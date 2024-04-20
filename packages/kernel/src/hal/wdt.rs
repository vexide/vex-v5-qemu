//! Cortex-A9 Watchdog Timer

#![allow(const_item_mutation)]

use super::Register;

#[derive(Debug, Eq, PartialEq)]
pub struct WatchdogTimer {
    is_started: bool,
}

impl WatchdogTimer {
    /// XPAR_SCUWDT_0_BASEADDR
    pub const BASE_ADDR: u32 = 0xF8F00620;
    /// XPAR_SCUWDT_0_HIGHADDR
    pub const HIGH_ADDR: u32 = 0xF8F006FF;
    /// XPAR_SCUWDT_0_DEVICE_ID
    pub const DEVICE_ID: u32 = 0;

    /// Watchdog Load Register
    pub const LOAD_REGISTER: Register = Register::new(Self::BASE_ADDR, 0x00);
    /// Watchdog Counter Register
    pub const COUNTER_REGISTER: Register = Register::new(Self::BASE_ADDR, 0x04);
    /// Watchdog Control Register
    pub const CONTROL_REGISTER: Register = Register::new(Self::BASE_ADDR, 0x08);
    /// Watchdog Interrupt Status Register
    pub const ISR_REGISTER: Register = Register::new(Self::BASE_ADDR, 0x0C);
    /// Watchdog Reset Status Register
    pub const RST_STS_REGISTER: Register = Register::new(Self::BASE_ADDR, 0x10);
    /// Watchdog Disable Register
    pub const DISABLE_REGISTER: Register = Register::new(Self::BASE_ADDR, 0x14);

    /// Prescaler Mask
    pub const CONTROL_PRESCALER_MASK: u32 = 0x0000FF00;
    /// Prescaler Shift
    pub const CONTROL_PRESCALER_SHIFT: u32 = 8;
    /// Watchdog/Timer mode
    pub const CONTROL_WD_MODE_MASK: u32 = 0x00000008;
    /// Intr enable (in timer mode)
    pub const CONTROL_IT_ENABLE_MASK: u32 = 0x00000004;
    /// Auto-reload (in timer mode)
    pub const CONTROL_AUTO_RELOAD_MASK: u32 = 0x00000002;
    /// Watchdog enable
    pub const CONTROL_WD_ENABLE_MASK: u32 = 0x00000001;

    /// Watchdog mode disable value 1.
    pub const DISABLE_VALUE_1: u32 = 0x12345678;
    /// Watchdog mode disable value 2.
    pub const DISABLE_VALUE_2: u32 = 0x87654321;

    /// Initialize a specific watchdog timer instance/driver. This function enables the
    /// watchdog mode.
    pub fn initialize() -> Self {
        let mut instance = Self { is_started: false };

        instance.set_mode();

        instance
    }

    /// Start the watchdog counter of the device.
    ///
    /// User needs to select the appropriate mode (watchdog/timer) before using this API.
    pub fn start(&mut self) {
        let mut control_reg = unsafe { Self::CONTROL_REGISTER.read() };
        control_reg |= Self::CONTROL_WD_ENABLE_MASK;
        self.set_control_register(control_reg);

        self.is_started = true;
    }

    pub fn is_started(&self) -> bool {
        self.is_started
    }

    /// Put the watchdog timer in Watchdog mode by setting the WD mode bit of the
    /// Watchdog control register.
    pub fn set_mode(&mut self) {
        unsafe {
            Self::CONTROL_REGISTER
                .write(Self::CONTROL_REGISTER.read() | Self::CONTROL_WD_MODE_MASK);
        }
    }

    /// Put the watchdog timer in Watchdog mode by setting the WD mode bit of the
    /// Watchdog control register.
    pub fn set_timer_mode(&mut self) {
        unsafe {
            Self::DISABLE_REGISTER.write(Self::DISABLE_VALUE_1);
            Self::DISABLE_REGISTER.write(Self::DISABLE_VALUE_2);
        }
    }

    /// Get the contents of the watchdog control register.
    pub fn control_register(&self) -> u32 {
        unsafe { Self::CONTROL_REGISTER.read() }
    }

    /// Write to the watchdog control register.
    pub fn set_control_register(&mut self, value: u32) {
        unsafe {
            Self::CONTROL_REGISTER.write(value);
        }
    }

    /// Write to the watchdog timer load register. This will also update the
    /// watchdog counter register with the new value. This macro can be used to
    /// change the time-out value.
    pub fn load(&mut self, value: u32) {
        unsafe {
            Self::LOAD_REGISTER.write(value);
        }
    }
}
