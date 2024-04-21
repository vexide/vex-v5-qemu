//! Cortex-A9 Generic Interrupt Controller

#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_uint, c_void};

pub const XSCUGIC_MAX_NUM_INTR_INPUTS: usize = 95;

pub type Xil_InterruptHandler = Option<unsafe extern "C" fn(data: *mut c_void)>;

/// Defines an entry in an interrupt vector table.
///
/// The callback reference is the base address of the interrupting device
/// for the low level driver and an instance pointer for the high level driver.
#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XScuGic_VectorTableEntry {
    /// Interrupt Handler
    pub handler: Xil_InterruptHandler,

    /// The callback reference passed in by the upper layer when setting the Interrupt
    /// handler for specific interrupt ID, and it will passed back to Interrupt handler
    /// when it is invoked.
    pub callback_ref: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XScuGic_Config {
    pub Name: *const c_char,
    pub DistBaseAddress: u32,
    pub CpuBaseAddress: u32,
    pub HandlerTable: [XScuGic_VectorTableEntry; XSCUGIC_MAX_NUM_INTR_INPUTS],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct XScuGic {
    pub Config: *mut XScuGic_Config,
    pub IsReady: u32,
    pub UnhandledInterrupts: u32,
}

extern "C" {
    pub fn XScuGic_CfgInitialize(
        InstancePtr: *mut XScuGic,
        ConfigPtr: *mut XScuGic_Config,
        EffectiveAddr: u32,
    ) -> i32;
    pub fn XScuGic_LookupConfig(BaseAddr: *mut c_uint) -> *mut XScuGic_Config;
    // This should be `InstancePtr: *mut XScuGic`, but rust can't transmute function pointers making this difficult
    // to cast to a Xil_ExceptionHandler.
    pub fn XScuGic_InterruptHandler(InstancePtr: *mut c_void);
    pub fn XScuGic_Connect(
        InstancePtr: *mut XScuGic,
        Int_Id: u32,
        Handler: Xil_InterruptHandler,
        CallBackRef: *mut c_void,
    ) -> i32;
    pub fn XScuGic_SetPriorityTriggerType(
        InstancePtr: *mut XScuGic,
        Int_Id: u32,
        Priority: u8,
        Trigger: u8,
    ) -> i32;
    pub fn XScuGic_Enable(InstancePtr: *mut XScuGic, Int_Id: u32);
}
