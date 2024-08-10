#![allow(non_camel_case_types)]

use core::ffi::c_void;

pub const XIL_EXCEPTION_ID_IRQ_INT: u32 = 2;

pub type Xil_ExceptionHandler = Option<unsafe extern "C" fn(data: *mut c_void)>;

extern "C" {
    pub fn Xil_ExceptionRegisterHandler(
        Exception_id: u32,
        Handler: Xil_ExceptionHandler,
        Data: *mut c_void,
    );
}
