use snafu::Snafu;

use crate::xil::{
    wdt::{
        XScuWdt, XScuWdt_CfgInitialize, XScuWdt_DisableAutoReload, XScuWdt_EnableAutoReload,
        XScuWdt_GetControlReg, XScuWdt_IsTimerExpired, XScuWdt_IsWdtExpired, XScuWdt_LoadWdt,
        XScuWdt_LookupConfig, XScuWdt_RestartWdt, XScuWdt_SetControlReg, XScuWdt_SetTimerMode,
        XScuWdt_SetWdMode, XScuWdt_Start, XScuWdt_Stop, XSCUWDT_CONTROL_OFFSET,
        XSCUWDT_CONTROL_PRESCALER_MASK, XSCUWDT_CONTROL_PRESCALER_SHIFT, XSCUWDT_COUNTER_OFFSET,
    },
    XST_SUCCESS,
};

pub enum WatchdogTimerMode {
    Timer,
    Watchdog,
}

#[derive(Debug, Snafu)]
pub enum WatchdogTimerError {
    /// The watchdog timer cannot be initialized with the given base address.
    #[snafu(display(
        "The peripheral cannot be initialized with the base address 0x{base_address:08X}.",
    ))]
    InvalidBaseAddress { base_address: u32 },
    /// The watchdog timer failed to initialize.
    InitializeFailed,
}

pub struct WatchdogTimer {
    instance: XScuWdt,
}

impl WatchdogTimer {
    pub const INTERRUPT_ID: u32 = 30;

    pub unsafe fn new(base_address: u32) -> Result<Self, WatchdogTimerError> {
        // SAFETY: The driver is initialized before it is returned.
        let mut instance = unsafe { core::mem::zeroed() };

        let config = unsafe { XScuWdt_LookupConfig(base_address) };
        if config.is_null() {
            return InvalidBaseAddressSnafu { base_address }.fail();
        }

        // SAFETY: The timer is a pointer to owned mutable memory and the config is valid.
        match unsafe { XScuWdt_CfgInitialize(&mut instance, config, (*config).BaseAddr) } {
            XST_SUCCESS => {}
            _ => return Err(WatchdogTimerError::InitializeFailed),
        }

        Ok(Self { instance })
    }

    pub fn load(&mut self, value: u32) {
        unsafe {
            XScuWdt_LoadWdt(&mut self.instance, value);
        }
    }

    pub fn restart(&mut self) {
        unsafe {
            XScuWdt_RestartWdt(&mut self.instance);
        }
    }

    pub fn start(&mut self) {
        unsafe {
            XScuWdt_Start(&mut self.instance);
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            XScuWdt_Stop(&mut self.instance);
        }
    }

    pub fn counter(&self) -> u32 {
        unsafe {
            core::ptr::read_volatile(
                (self.instance.Config.BaseAddr + XSCUWDT_COUNTER_OFFSET) as *const u32,
            )
        }
    }

    pub fn set_prescaler(&mut self, prescaler: u8) {
        let mut control = unsafe { XScuWdt_GetControlReg(&self.instance) };

        control &= !XSCUWDT_CONTROL_PRESCALER_MASK;
        control |= (prescaler as u32) << XSCUWDT_CONTROL_PRESCALER_SHIFT;

        unsafe {
            XScuWdt_SetControlReg(&mut self.instance, control);
        }
    }

    pub fn set_auto_reload(&mut self, enable: bool) {
        unsafe {
            match enable {
                true => XScuWdt_EnableAutoReload(&mut self.instance),
                false => XScuWdt_DisableAutoReload(&mut self.instance),
            }
        }
    }

    pub fn set_interrupt_enabled(&mut self, enable: bool) {
        let mask = if enable { 0x00000004 } else { !0x00000004 };

        unsafe {
            core::ptr::write_volatile(
                (self.instance.Config.BaseAddr + XSCUWDT_CONTROL_OFFSET) as *mut u32,
                core::ptr::read_volatile(
                    (self.instance.Config.BaseAddr + XSCUWDT_CONTROL_OFFSET) as *mut u32,
                ) | mask,
            );
        }
    }

    pub fn set_mode(&mut self, mode: WatchdogTimerMode) {
        unsafe {
            match mode {
                WatchdogTimerMode::Timer => XScuWdt_SetTimerMode(&mut self.instance),
                WatchdogTimerMode::Watchdog => XScuWdt_SetWdMode(&mut self.instance),
            }
        }
    }

    pub fn is_timer_expired(&self) -> bool {
        unsafe { XScuWdt_IsTimerExpired(&self.instance) }
    }

    pub fn is_watchdog_expired(&self) -> bool {
        unsafe { XScuWdt_IsWdtExpired(&self.instance) }
    }

    pub unsafe fn raw_mut(&mut self) -> &mut XScuWdt {
        &mut self.instance
    }
}

// SAFETY: The timer does not access or store any raw pointers that could be sent between
// threads (Doesn't access or set the name, doesn't use interrupt mode.)
unsafe impl Send for WatchdogTimer {}
