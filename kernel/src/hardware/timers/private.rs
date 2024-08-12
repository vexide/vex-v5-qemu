use snafu::Snafu;

use crate::xil::{
    timer::{
        XScuTimer, XScuTimer_CfgInitialize, XScuTimer_ClearInterruptStatus,
        XScuTimer_DisableAutoReload, XScuTimer_DisableInterrupt, XScuTimer_EnableAutoReload,
        XScuTimer_EnableInterrupt, XScuTimer_GetCounterValue, XScuTimer_IsExpired,
        XScuTimer_LoadTimer, XScuTimer_LookupConfig, XScuTimer_RestartTimer,
        XScuTimer_SetPrescaler, XScuTimer_Start, XScuTimer_Stop,
    },
    XST_SUCCESS,
};

#[derive(Debug, Snafu)]
pub enum PrivateTimerError {
    /// The private timer cannot be initialized with the given base address.
    #[snafu(display(
        "The peripheral cannot be initialized with the base address 0x{base_address:08X}.",
    ))]
    InvalidBaseAddress { base_address: u32 },
    /// The private timer failed to initialize.
    InitializeFailed,
}

pub struct PrivateTimer {
    instance: XScuTimer,
}

impl PrivateTimer {
    pub const INTERRUPT_ID: u32 = 29;

    /// Create a new private timer peripheral at the given base address.
    ///
    /// # Safety
    ///
    /// This function must only be called once per given base address.
    pub unsafe fn new(base_address: u32) -> Result<Self, PrivateTimerError> {
        // SAFETY: The timer is initialized before it is returned.
        let mut instance = unsafe { core::mem::zeroed() };

        let config = unsafe { XScuTimer_LookupConfig(base_address) };
        if config.is_null() {
            return InvalidBaseAddressSnafu { base_address }.fail();
        }

        // SAFETY: The timer is a pointer to owned mutable memory and the config is
        // valid.
        match unsafe { XScuTimer_CfgInitialize(&mut instance, config, (*config).BaseAddr) } {
            XST_SUCCESS => {}
            _ => return Err(PrivateTimerError::InitializeFailed),
        }

        Ok(Self { instance })
    }

    pub fn load(&mut self, value: u32) {
        unsafe {
            XScuTimer_LoadTimer(&mut self.instance, value);
        }
    }

    pub fn restart(&mut self) {
        unsafe {
            XScuTimer_RestartTimer(&mut self.instance);
        }
    }

    pub fn start(&mut self) {
        unsafe {
            XScuTimer_Start(&mut self.instance);
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            XScuTimer_Stop(&mut self.instance);
        }
    }

    pub fn counter(&self) -> u32 {
        unsafe { XScuTimer_GetCounterValue(&self.instance) }
    }

    pub fn set_prescaler(&mut self, prescaler: u8) {
        unsafe {
            XScuTimer_SetPrescaler(&mut self.instance, prescaler);
        }
    }

    pub fn set_auto_reload(&mut self, enable: bool) {
        unsafe {
            match enable {
                true => XScuTimer_EnableAutoReload(&mut self.instance),
                false => XScuTimer_DisableAutoReload(&mut self.instance),
            }
        }
    }

    pub fn set_interrupt_enabled(&mut self, enable: bool) {
        unsafe {
            match enable {
                true => XScuTimer_EnableInterrupt(&mut self.instance),
                false => XScuTimer_DisableInterrupt(&mut self.instance),
            }
        }
    }

    pub fn clear_interrupt_status(&mut self) {
        unsafe { XScuTimer_ClearInterruptStatus(&mut self.instance) }
    }

    pub fn is_expired(&self) -> bool {
        unsafe { XScuTimer_IsExpired(&self.instance) }
    }

    pub fn raw_mut(&mut self) -> &mut XScuTimer {
        &mut self.instance
    }
}

// SAFETY: The timer does not access or store any raw pointers that could be
// sent between threads (Doesn't access or set the name, doesn't use interrupt
// mode.)
unsafe impl Send for PrivateTimer {}
