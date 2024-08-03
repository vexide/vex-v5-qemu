//! Cortex-A9 Watchdog Timer

use core::ffi::{c_char, c_uint};

pub const XPAR_XSCUWDT_0_BASEADDR: u32 = 0xF8F00620;

pub const XSCUWDT_LOAD_OFFSET: u32 = 0x00;
pub const XSCUWDT_COUNTER_OFFSET: u32 = 0x04;
pub const XSCUWDT_CONTROL_OFFSET: u32 = 0x08;
pub const XSCUWDT_ISR_OFFSET: u32 = 0x0C;
pub const XSCUWDT_RST_STS_OFFSET: u32 = 0x10;
pub const XSCUWDT_DISABLE_OFFSET: u32 = 0x14;

/// Watchdog mode disable value 1.
pub const XSCUWDT_DISABLE_VALUE_1: u32 = 0x12345678;
/// Watchdog mode disable value 2.
pub const XSCUWDT_DISABLE_VALUE_2: u32 = 0x87654321;

pub const XSCUWDT_CONTROL_WD_MODE_MASK: u32 = 0x00000008;
pub const XSCUWDT_CONTROL_AUTO_RELOAD_MASK: u32 = 0x00000002;
pub const XSCUWDT_CONTROL_PRESCALER_MASK: u32 = 0x0000FF00;
pub const XSCUWDT_CONTROL_PRESCALER_SHIFT: u32 = 8;
pub const XSCUWDT_CONTROL_IT_ENABLE_MASK: u32 = 0x00000004;
pub const XSCUWDT_CONTROL_WD_ENABLE_MASK: u32 = 0x00000001;
pub const XSCUWDT_ISR_EVENT_FLAG_MASK: u32 = 0x00000001;
pub const XSCUWDT_RST_STS_RESET_FLAG_MASK: u32 = 0x00000001;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XScuWdt_Config {
    pub Name: *const c_char,
    pub BaseAddr: u32,
    pub IntrId: u32,
    pub IntrParent: *mut c_uint,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XScuWdt {
    pub Config: XScuWdt_Config,
    pub IsReady: u32,
    pub IsStarted: u32,
}

extern "C" {
    pub fn XScuWdt_LookupConfig(BaseAddr: *mut c_uint) -> *mut XScuWdt_Config;
    pub fn XScuWdt_CfgInitialize(
        InstancePtr: *mut XScuWdt,
        ConfigPtr: *mut XScuWdt_Config,
        EffectiveAddress: u32,
    ) -> i32;
    pub fn XScuWdt_Start(InstancePtr: *mut XScuWdt);
    pub fn XScuWdt_Stop(InstancePtr: *mut XScuWdt);
}

pub unsafe extern "C" fn XScuWdt_GetControlReg(InstancePtr: *const XScuWdt) -> u32 {
    unsafe {
        core::ptr::read_volatile(
            ((*InstancePtr).Config.BaseAddr + XSCUWDT_CONTROL_OFFSET) as *const u32,
        )
    }
}

pub unsafe extern "C" fn XScuWdt_SetControlReg(InstancePtr: *mut XScuWdt, ControlReg: u32) {
    unsafe {
        core::ptr::write_volatile(
            ((*InstancePtr).Config.BaseAddr + XSCUWDT_CONTROL_OFFSET) as *mut u32,
            ControlReg,
        );
    }
}

pub unsafe extern "C" fn XScuWdt_LoadWdt(InstancePtr: *mut XScuWdt, Value: u32) {
    unsafe {
        core::ptr::write_volatile(
            ((*InstancePtr).Config.BaseAddr + XSCUWDT_LOAD_OFFSET) as *mut u32,
            Value,
        );
    }
}

pub unsafe extern "C" fn XScuWdt_SetTimerMode(InstancePtr: *mut XScuWdt) {
    unsafe {
        core::ptr::write_volatile(
            ((*InstancePtr).Config.BaseAddr + XSCUWDT_DISABLE_OFFSET) as *mut u32,
            XSCUWDT_DISABLE_VALUE_1,
        );
        core::ptr::write_volatile(
            ((*InstancePtr).Config.BaseAddr + XSCUWDT_DISABLE_OFFSET) as *mut u32,
            XSCUWDT_DISABLE_VALUE_2,
        );
    }
}

pub unsafe extern "C" fn XScuWdt_SetWdMode(InstancePtr: *mut XScuWdt) {
    unsafe {
        core::ptr::write_volatile(
            ((*InstancePtr).Config.BaseAddr + XSCUWDT_CONTROL_OFFSET) as *mut u32,
            core::ptr::read_volatile(
                ((*InstancePtr).Config.BaseAddr + XSCUWDT_CONTROL_OFFSET) as *mut u32,
            ) | XSCUWDT_CONTROL_WD_MODE_MASK,
        );
    }
}

pub unsafe extern "C" fn XScuWdt_RestartWdt(InstancePtr: *mut XScuWdt) {
    unsafe {
        XScuWdt_LoadWdt(
            InstancePtr,
            core::ptr::read_volatile(
                ((*InstancePtr).Config.BaseAddr + XSCUWDT_LOAD_OFFSET) as *const u32,
            ) as u32,
        )
    }
}

pub unsafe extern "C" fn XScuWdt_EnableAutoReload(InstancePtr: *mut XScuWdt) {
    unsafe {
        core::ptr::write_volatile(
            ((*InstancePtr).Config.BaseAddr + XSCUWDT_CONTROL_OFFSET) as *mut u32,
            core::ptr::read_volatile(
                ((*InstancePtr).Config.BaseAddr + XSCUWDT_CONTROL_OFFSET) as *mut u32,
            ) | XSCUWDT_CONTROL_AUTO_RELOAD_MASK,
        );
    }
}

pub unsafe extern "C" fn XScuWdt_DisableAutoReload(InstancePtr: *mut XScuWdt) {
    unsafe {
        core::ptr::write_volatile(
            ((*InstancePtr).Config.BaseAddr + XSCUWDT_CONTROL_OFFSET) as *mut u32,
            core::ptr::read_volatile(
                ((*InstancePtr).Config.BaseAddr + XSCUWDT_CONTROL_OFFSET) as *mut u32,
            ) | !XSCUWDT_CONTROL_AUTO_RELOAD_MASK,
        );
    }
}

pub unsafe extern "C" fn XScuWdt_IsTimerExpired(InstancePtr: *const XScuWdt) -> bool {
    unsafe {
        (core::ptr::read_volatile(
            ((*InstancePtr).Config.BaseAddr + XSCUWDT_ISR_OFFSET) as *const u32,
        ) & XSCUWDT_ISR_EVENT_FLAG_MASK)
            == XSCUWDT_ISR_EVENT_FLAG_MASK
    }
}

pub unsafe extern "C" fn XScuWdt_IsWdtExpired(InstancePtr: *const XScuWdt) -> bool {
    unsafe {
        (core::ptr::read_volatile(
            ((*InstancePtr).Config.BaseAddr + XSCUWDT_RST_STS_OFFSET) as *const u32,
        ) & XSCUWDT_RST_STS_RESET_FLAG_MASK)
            == XSCUWDT_RST_STS_RESET_FLAG_MASK
    }
}
