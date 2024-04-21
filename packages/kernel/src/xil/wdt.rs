//! Cortex-A9 Watchdog Timer

use core::ffi::c_uint;

pub const BASE_ADDR: u32 = 0xF8F00620;

pub const XSCUWDT_CONTROL_OFFSET: u32 = 0x08;
pub const XSCUWDT_LOAD_OFFSET: u32 = 0x00;
pub const XSCUWDT_DISABLE_OFFSET: u32 = 0x14;

/// Watchdog mode disable value 1.
pub const XSCUWDT_DISABLE_VALUE_1: u32 = 0x12345678;
/// Watchdog mode disable value 2.
pub const XSCUWDT_DISABLE_VALUE_2: u32 = 0x87654321;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XScuWdt_Config {
    pub DeviceId: u16,
    pub BaseAddr: *mut c_uint,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XScuWdt {
    pub Config: XScuWdt_Config,
    pub IsReady: u32,
    pub IsStarted: u32,
}

extern "C" {
    pub fn XScuWdt_LookupConfig(DeviceId: u16) -> *mut XScuWdt_Config;
    pub fn XScuWdt_CfgInitialize(
        InstancePtr: *mut XScuWdt,
        ConfigPtr: *mut XScuWdt_Config,
        EffectiveAddress: u32,
    ) -> i32;
    pub fn XScuWdt_Start(InstancePtr: *mut XScuWdt);
}

pub unsafe extern "C" fn XScuWdt_GetControlReg(InstancePtr: *const XScuWdt) -> u32 {
    core::ptr::read_volatile((BASE_ADDR + XSCUWDT_CONTROL_OFFSET) as *const u32)
}

pub unsafe extern "C" fn XScuWdt_SetControlReg(InstancePtr: *mut XScuWdt, ControlReg: u32) {
    core::ptr::write_volatile((BASE_ADDR + XSCUWDT_CONTROL_OFFSET) as *mut u32, ControlReg);
}

pub unsafe extern "C" fn XScuWdt_LoadWdt(InstancePtr: *mut XScuWdt, Value: u32) {
    core::ptr::write_volatile((BASE_ADDR + XSCUWDT_LOAD_OFFSET) as *mut u32, Value);
}

pub unsafe extern "C" fn XScuWdt_SetTimerMode(InstancePtr: *mut XScuWdt) {
    unsafe {
        core::ptr::write_volatile(
            (BASE_ADDR + XSCUWDT_DISABLE_OFFSET) as *mut u32,
            XSCUWDT_DISABLE_VALUE_1,
        );
        core::ptr::write_volatile(
            (BASE_ADDR + XSCUWDT_DISABLE_OFFSET) as *mut u32,
            XSCUWDT_DISABLE_VALUE_2,
        );
    }
}
