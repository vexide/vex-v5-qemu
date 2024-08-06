use snafu::Snafu;
use vexide_core::io::{self, Read, Write};

use crate::xil::{uart::*, XST_SUCCESS};

#[derive(Debug, Snafu)]
pub enum UartDriverError {
    /// The UART device cannot be initialized with the given base address.
    #[snafu(display(
        "The UART driver cannot be initialized with the base address 0x{base_address:08X}.",
    ))]
    InvalidBaseAddress { base_address: u32 },
    /// The UART device failed to pass a self-test.
    SelfTestFailed,
    /// The specified baud rate is not possible because the input clock frequency is not
    /// divisible with an acceptable amount of error.
    InvalidBaudRate,
    /// The UART driver failed to initialize.
    InitializeFailed,
}

impl UartDriverError {
    /// Convert an XST status code to a UART driver error Result.
    ///
    /// Returns `Ok(())` if the status code is [`XST_SUCCESS`].
    pub const fn try_from_xst_status(status: i32) -> Result<(), Self> {
        use crate::xil::uart;
        match status {
            XST_SUCCESS => Ok(()),
            status => Err(match status {
                uart::XST_UART_TEST_FAIL => Self::SelfTestFailed,
                uart::XST_UART_BAUD_ERROR => Self::InvalidBaudRate,
                _ => Self::InitializeFailed,
            }),
        }
    }
}

/// A safe wrapper around the Xilinx UART driver.
pub struct UartDriver {
    instance: XUartPs,
}

impl UartDriver {
    /// Initialize the UART driver with the given base address.
    ///
    /// # Parameters
    ///
    /// - `base_address`: The base address of the UART device.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the UART driver is only initialized once for a given base address.
    pub unsafe fn new(base_address: u32) -> Result<Self, UartDriverError> {
        // SAFETY: The driver is initialized before it is returned.
        let mut driver = unsafe { core::mem::zeroed() };
        let config = unsafe { XUartPs_LookupConfig(base_address) };
        if config.is_null() {
            return InvalidBaseAddressSnafu { base_address }.fail();
        }
        // SAFETY: The driver is a pointer to owned mutable memory and the config is valid.
        let status = unsafe { XUartPs_CfgInitialize(&mut driver, config, (*config).BaseAddress) };
        UartDriverError::try_from_xst_status(status)?;

        // Adding the self-test causes linker errors (missing strlen, etc.)
        // if self_test {
        //     // SAFETY: The driver is fully initialized.
        //     let status = unsafe { XUartPs_SelfTest(&mut driver) };
        //     UartDriverError::try_from_xst_status(status)?;
        // }

        Ok(Self { instance: driver })
    }

    pub fn set_baud_rate(&mut self, baud_rate: u32) -> Result<(), UartDriverError> {
        // SAFETY: The instance is fully initialized.
        let status = unsafe { XUartPs_SetBaudRate(&mut self.instance, baud_rate) };
        UartDriverError::try_from_xst_status(status)
    }

    pub fn raw_mut(&mut self) -> &mut XUartPs {
        &mut self.instance
    }
}

// SAFETY: The UART driver does not access or store any raw pointers that could be sent between
// threads (Doesn't access or set the name, doesn't use interrupt mode.)
unsafe impl Send for UartDriver {}

impl Write for UartDriver {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut sent_count = 0;
        while sent_count < buf.len() {
            // SAFETY: The instance is fully initialized.
            sent_count += unsafe { XUartPs_Send(&mut self.instance, &buf[sent_count], 1) as usize };
        }
        Ok(sent_count)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for UartDriver {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut read_count = 0;
        while read_count < buf.len() {
            // SAFETY: The instance is fully initialized.
            let num_read =
                unsafe { XUartPs_Recv(&mut self.instance, &mut buf[read_count], 1) as usize };
            read_count += num_read;
        }
        Ok(read_count)
    }
}
