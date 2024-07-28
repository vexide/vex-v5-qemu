//! V5 Controller

use vex_sdk::*;

pub fn vexControllerGet(id: V5_ControllerId, index: V5_ControllerIndex) -> i32 {
    Default::default()
}
pub fn vexControllerConnectionStatusGet(id: V5_ControllerId) -> V5_ControllerStatus {
    Default::default()
}
pub fn vexControllerTextSet(id: u32, line: u32, col: u32, buf: *const u8) -> u32 {
    Default::default()
}
