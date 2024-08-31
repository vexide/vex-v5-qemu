//! VEXos Task Scheduler Functions

use core::ffi::{c_char, c_int, c_void};

use vex_v5_qemu_protocol::KernelBoundPacket;

use super::{BATTERY, SMARTPORTS};
use crate::{peripherals::UART1, protocol::recv_packet};

/// Adds a new simple task to the task scheduler.
pub extern "C" fn vexTaskAdd(
    callback: unsafe extern "C" fn() -> c_int,
    interval: c_int,
    label: *const c_char,
) {
}

/// Gets a tasks's callback function and internal ID.
pub extern "C" fn vexTaskGetCallbackAndId(index: u32, callback_id: *mut c_int) -> *mut c_void {
    core::ptr::null_mut()
}

/// Yields execution away from the current task for a given number of
/// milliseconds.
pub extern "C" fn vexTaskSleep(time: u32) {}

/// Returns the maximum number of threads that are supported by the VEXos
/// task scheduler.
///
/// Real V5 enviornments set this at 128, but since we don't currently emulate
/// the VEXcode task scheduler, we just return 0.
pub extern "C" fn vexTaskHardwareConcurrency() -> i32 {
    0
}

/// Unknown use; on the partner SDK this is aliased to vexTasksRun,
/// but this real version on the jumptable isn't understood well.
pub extern "C" fn vexBackgroundProcessing() {}

/// Ticks's the (non-existent in this environment) internal VEXos task
/// scheduler
///
/// This more importantly handles many device reads and flushes
/// serial.
pub extern "C" fn vexTasksRun() {
    while !UART1.lock().is_rx_empty() {
        if let Some(packet) = recv_packet().unwrap() {
            match packet {
                KernelBoundPacket::BatteryUpdate { data, timestamp } => {
                    let mut battery = BATTERY.lock();
                    battery.data = Some(data);
                    battery.timestamp = timestamp;
                }
                KernelBoundPacket::SmartPortUpdate {
                    port_index,
                    data,
                    timestamp,
                } => {
                    if let Some(port) = SMARTPORTS.get(port_index as usize) {
                        port.lock().update(data, timestamp);
                    }
                }
                KernelBoundPacket::ControllerUpdate {
                    id,
                    data,
                    timestamp,
                } => {}
                _ => panic!("Unexpected kernel-bound packet {:?}", packet),
            }
        }
    }
}
