//! Brain Screen Touchscreen

use crate::sync::Mutex;
use vex_sdk::*;
use vex_v5_qemu_protocol::{geometry::Point2, touch::{TouchData, TouchEvent}};

pub static TOUCH: Mutex<Touchscreen> = Mutex::new(Touchscreen::new());

pub struct Touchscreen {
    pub data: TouchData,
}

impl Touchscreen {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            data: TouchData {
                event: TouchEvent::Release,
                point: Point2 { x: 0, y: 0 },
            },
        }
    }
}

pub extern "C" fn vexTouchUserCallbackSet(callback: unsafe extern "C" fn(V5_TouchEvent, i32, i32)) {
}
/// # Safety
///
/// - `device` must be a valid, non-null pointer to a V5_TouchStatus instance
pub unsafe extern "C" fn vexTouchDataGet(status: *mut V5_TouchStatus) {
    let data = TOUCH.lock().data;
    unsafe {
        *status = V5_TouchStatus {
            lastEvent: match data.event {
                TouchEvent::Press => V5_TouchEvent::kTouchEventPress,
                TouchEvent::Release => V5_TouchEvent::kTouchEventRelease,
            },
            lastXpos: data.point.x,
            lastYpos: data.point.y,
            pressCount: 0, // TODO
            releaseCount: 0,
        }
    }
}
