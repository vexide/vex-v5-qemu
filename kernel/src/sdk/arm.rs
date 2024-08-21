//! CTE Workcell Arm

use core::ffi::c_double;
use vex_sdk::*;

pub extern "C" fn vexDeviceArmMoveTipCommandLinearAdv(
    device: V5_DeviceT,
    position: *mut V5_DeviceArmTipPosition,
    j6_rotation: c_double,
    j6_velocity: u16,
    relative: bool,
) {
}
pub extern "C" fn vexDeviceArmMoveTipCommandJointAdv(
    device: V5_DeviceT,
    position: *mut V5_DeviceArmTipPosition,
    j6_rotation: c_double,
    j6_velocity: u16,
    relative: bool,
) {
}
pub extern "C" fn vexDeviceArmTipPositionGetAdv(
    device: V5_DeviceT,
    position: *mut V5_DeviceArmTipPosition,
) {
}
pub extern "C" fn vexDeviceArmPoseSet(device: V5_DeviceT, pose: u8, velocity: u16) {}
pub extern "C" fn vexDeviceArmMoveTipCommandLinear(
    device: V5_DeviceT,
    x: i32,
    y: i32,
    z: i32,
    pose: u8,
    velocity: u16,
    rotation: c_double,
    rot_velocity: u16,
    relative: bool,
) {
}
pub extern "C" fn vexDeviceArmMoveTipCommandJoint(
    device: V5_DeviceT,
    x: i32,
    y: i32,
    z: i32,
    pose: u8,
    velocity: u16,
    rotation: c_double,
    rot_velocity: u16,
    relative: bool,
) {
}
pub extern "C" fn vexDeviceArmMoveJointsCommand(
    device: V5_DeviceT,
    positions: *mut c_double,
    velocities: *mut u16,
    j6_rotation: c_double,
    j6_velocity: u16,
    j7_volts: c_double,
    j7_timeout: u16,
    j7_i_limit: u16,
    relative: bool,
) {
}
pub extern "C" fn vexDeviceArmSpinJoints(device: V5_DeviceT, velocities: *mut c_double) {}
pub extern "C" fn vexDeviceArmSetJointPositions(device: V5_DeviceT, new_positions: *mut c_double) {}
pub extern "C" fn vexDeviceArmPickUpCommand(device: V5_DeviceT) {}
pub extern "C" fn vexDeviceArmDropCommand(device: V5_DeviceT) {}
pub extern "C" fn vexDeviceArmMoveVoltsCommand(device: V5_DeviceT, voltages: *mut c_double) {}
pub extern "C" fn vexDeviceArmFullStop(device: V5_DeviceT, brakeMode: u8) {}
pub extern "C" fn vexDeviceArmEnableProfiler(device: V5_DeviceT, enable: u8) {}
pub extern "C" fn vexDeviceArmProfilerVelocitySet(
    device: V5_DeviceT,
    linear_velocity: u16,
    joint_velocity: u16,
) {
}
pub extern "C" fn vexDeviceArmSaveZeroValues(device: V5_DeviceT) {}
pub extern "C" fn vexDeviceArmForceZeroCommand(device: V5_DeviceT) {}
pub extern "C" fn vexDeviceArmClearZeroValues(device: V5_DeviceT) {}
pub extern "C" fn vexDeviceArmBootload(device: V5_DeviceT) {}
pub extern "C" fn vexDeviceArmTipPositionGet(
    device: V5_DeviceT,
    x: *mut i32,
    y: *mut i32,
    z: *mut i32,
) {
}
pub extern "C" fn vexDeviceArmJointInfoGet(
    device: V5_DeviceT,
    positions: *mut c_double,
    velocities: *mut c_double,
    currents: *mut i32,
) {
}
pub extern "C" fn vexDeviceArmJ6PositionGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceArmBatteryGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceArmServoFlagsGet(device: V5_DeviceT, servoID: u32) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceArmStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceArmDebugGet(device: V5_DeviceT, id: i32) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceArmJointErrorsGet(device: V5_DeviceT, errors: *mut u8) {}
pub extern "C" fn vexDeviceArmJ6PositionSet(device: V5_DeviceT, position: i16) {}
pub extern "C" fn vexDeviceArmStopJointsCommand(device: V5_DeviceT, brakeModes: *mut i16) {}
pub extern "C" fn vexDeviceArmReboot(device: V5_DeviceT) {}
pub extern "C" fn vexDeviceArmTipOffsetSet(device: V5_DeviceT, x: i32, y: i32, z: i32) {}
