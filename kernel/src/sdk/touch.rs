//! Brain Screen Touchscreen

use vex_sdk::*;

pub fn vexTouchUserCallbackSet(callback: unsafe extern "C" fn(V5_TouchEvent, i32, i32)) {}
pub fn vexTouchDataGet(status: *mut V5_TouchStatus) {}
