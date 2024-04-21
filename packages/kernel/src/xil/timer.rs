//! Cortex-A9 Private Timer

use core::ffi::{c_char, c_uint};

pub const BASE_ADDR: u32 = 0xF8F00600;

pub const XSCUTIMER_LOAD_OFFSET: u32 = 0x00;
pub const XSCUTIMER_CONTROL_OFFSET: u32 = 0x08;
pub const XSCUTIMER_ISR_OFFSET: u32 = 0x0C;

pub const XSCUTIMER_CONTROL_AUTO_RELOAD_MASK: u32 = 0x00000002;
pub const XSCUTIMER_CONTROL_IRQ_ENABLE_MASK: u32 = 0x00000004;
pub const XSCUTIMER_ISR_EVENT_FLAG_MASK: u32 = 0x00000001;

pub const XPAR_SCUTIMER_INTR: u32 = 29;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XScuTimer_Config {
    pub Name: *const c_char,
    pub BaseAddr: u32,
    pub IntrId: u32,
    pub IntrParent: *mut c_uint,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XScuTimer {
    pub Config: XScuTimer_Config,
    pub IsReady: u32,
    pub IsStarted: u32,
}

extern "C" {
    pub fn XScuTimer_LookupConfig(BaseAddr: *mut c_uint) -> *mut XScuTimer_Config;
    pub fn XScuTimer_CfgInitialize(
        InstancePtr: *mut XScuTimer,
        ConfigPtr: *mut XScuTimer_Config,
        EffectiveAddress: u32,
    ) -> i32;
    pub fn XScuTimer_Stop(InstancePtr: *mut XScuTimer);
    pub fn XScuTimer_Start(InstancePtr: *mut XScuTimer);
    pub fn XScuTimer_SetPrescaler(InstancePtr: *mut XScuTimer, PrescalerValue: u8);
}

pub unsafe extern "C" fn XScuTimer_LoadTimer(InstancePtr: *mut XScuTimer, Value: u32) {
    core::ptr::write_volatile((BASE_ADDR + XSCUTIMER_LOAD_OFFSET) as *mut u32, Value);
}

pub unsafe extern "C" fn XScuTimer_IsExpired(InstancePtr: *mut XScuTimer) -> bool {
    (core::ptr::read_volatile((BASE_ADDR + XSCUTIMER_ISR_OFFSET) as *const u32)
        & XSCUTIMER_ISR_EVENT_FLAG_MASK)
        == XSCUTIMER_ISR_EVENT_FLAG_MASK
}

pub unsafe extern "C" fn XScuTimer_EnableAutoReload(InstancePtr: *mut XScuTimer) {
    core::ptr::write_volatile(
        (BASE_ADDR + XSCUTIMER_CONTROL_OFFSET) as *mut u32,
        core::ptr::read_volatile((BASE_ADDR + XSCUTIMER_CONTROL_OFFSET) as *mut u32)
            | XSCUTIMER_CONTROL_AUTO_RELOAD_MASK,
    );
}

pub unsafe extern "C" fn XScuTimer_ClearInterruptStatus(InstancePtr: *mut XScuTimer) {
    core::ptr::write_volatile(
        (BASE_ADDR + XSCUTIMER_ISR_OFFSET) as *mut u32,
        XSCUTIMER_ISR_EVENT_FLAG_MASK,
    );
}

pub unsafe extern "C" fn XScuTimer_EnableInterrupt(InstancePtr: *mut XScuTimer) {
    core::ptr::write_volatile(
        (BASE_ADDR + XSCUTIMER_CONTROL_OFFSET) as *mut u32,
        core::ptr::read_volatile((BASE_ADDR + XSCUTIMER_CONTROL_OFFSET) as *mut u32)
            | XSCUTIMER_CONTROL_IRQ_ENABLE_MASK,
    );
}

pub unsafe extern "C" fn XScuTimer_DisableInterrupt(InstancePtr: *mut XScuTimer) {
    core::ptr::write_volatile(
        (BASE_ADDR + XSCUTIMER_CONTROL_OFFSET) as *mut u32,
        core::ptr::read_volatile((BASE_ADDR + XSCUTIMER_CONTROL_OFFSET) as *mut u32)
            & !XSCUTIMER_CONTROL_IRQ_ENABLE_MASK,
    );
}
