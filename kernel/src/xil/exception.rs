#![allow(non_camel_case_types)]

use core::ffi::c_void;

pub const XIL_EXCEPTION_ID_RESET: u32 = 0;
pub const XIL_EXCEPTION_ID_UNDEFINED_INT: u32 = 1;
pub const XIL_EXCEPTION_ID_SWI_INT: u32 = 2;
pub const XIL_EXCEPTION_ID_PREFETCH_ABORT_INT: u32 = 3;
pub const XIL_EXCEPTION_ID_DATA_ABORT_INT: u32 = 4;
pub const XIL_EXCEPTION_ID_IRQ_INT: u32 = 5;
pub const XIL_EXCEPTION_ID_FIQ_INT: u32 = 6;
pub const XIL_EXCEPTION_ID_LAST: u32 = 6;

pub type Xil_ExceptionHandler = Option<unsafe extern "C" fn(data: *mut c_void)>;

extern "C" {
    pub fn Xil_ExceptionRegisterHandler(
        Exception_id: u32,
        Handler: Xil_ExceptionHandler,
        Data: *mut c_void,
    );
}
