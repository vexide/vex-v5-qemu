//! V5 Distance Sensor

use core::ffi::c_double;

use vex_sdk::*;
use vex_v5_qemu_protocol::{distance_sensor::DistanceSensorData, SmartPortData};

use super::SMARTPORTS;

pub unsafe extern "C" fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32 {
    if let Some(port) = SMARTPORTS.get(unsafe { *device }.zero_indexed_port as usize) {
        if let Some(SmartPortData::DistanceSensor(data)) = &port.lock().data {
            return if let Some(object) = &data.object {
                object.distance
            } else {
                9999
            }
        }
    }

    0
}
pub unsafe extern "C" fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32 {
    if let Some(port) = SMARTPORTS.get(unsafe { *device }.zero_indexed_port as usize) {
        if let Some(SmartPortData::DistanceSensor(data)) = &port.lock().data {
            if let Some(object) = &data.object {
                return object.confidence;
            }
        }
    }

    0
}
pub unsafe extern "C" fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32 {
    if let Some(port) = SMARTPORTS.get(unsafe { *device }.zero_indexed_port as usize) {
        if let Some(SmartPortData::DistanceSensor(data)) = &port.lock().data {
            return data.status;
        }
    }

    0
}
pub unsafe extern "C" fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32 {
    if let Some(port) = SMARTPORTS.get(unsafe { *device }.zero_indexed_port as usize) {
        if let Some(SmartPortData::DistanceSensor(data)) = &port.lock().data {
            return if let Some(object) = &data.object {
                object.relative_size as _
            } else {
                0
            }
        }
    }

    0 // TODO: test if a -1 return is actually feasable here
}

pub unsafe extern "C" fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double {
    if let Some(port) = SMARTPORTS.get(unsafe { *device }.zero_indexed_port as usize) {
        if let Some(SmartPortData::DistanceSensor(data)) = &port.lock().data {
            if let Some(object) = &data.object {
                return object.velocity;
            }
        }
    }

    0.0
}
