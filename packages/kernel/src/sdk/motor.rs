//! V5 Smart Motor

use core::ffi::c_double;

use vex_sdk::*;

pub extern "C" fn vexDeviceMotorVelocitySet(device: V5_DeviceT, velocity: i32) {}
pub extern "C" fn vexDeviceMotorVelocityGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceMotorActualVelocityGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceMotorDirectionGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceMotorModeSet(device: V5_DeviceT, mode: V5MotorControlMode) {}
pub extern "C" fn vexDeviceMotorModeGet(device: V5_DeviceT) -> V5MotorControlMode {
    Default::default()
}
pub extern "C" fn vexDeviceMotorPwmSet(device: V5_DeviceT, pwm: i32) {}
pub extern "C" fn vexDeviceMotorPwmGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceMotorCurrentLimitSet(device: V5_DeviceT, limit: i32) {}
pub extern "C" fn vexDeviceMotorCurrentLimitGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceMotorCurrentGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceMotorPowerGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceMotorTorqueGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceMotorEfficiencyGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceMotorTemperatureGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceMotorOverTempFlagGet(device: V5_DeviceT) -> bool {
    Default::default()
}
pub extern "C" fn vexDeviceMotorCurrentLimitFlagGet(device: V5_DeviceT) -> bool {
    Default::default()
}
pub extern "C" fn vexDeviceMotorZeroVelocityFlagGet(device: V5_DeviceT) -> bool {
    Default::default()
}
pub extern "C" fn vexDeviceMotorZeroPositionFlagGet(device: V5_DeviceT) -> bool {
    Default::default()
}
pub extern "C" fn vexDeviceMotorReverseFlagSet(device: V5_DeviceT, reverse: bool) {}
pub extern "C" fn vexDeviceMotorReverseFlagGet(device: V5_DeviceT) -> bool {
    Default::default()
}
pub extern "C" fn vexDeviceMotorEncoderUnitsSet(device: V5_DeviceT, units: V5MotorEncoderUnits) {}
pub extern "C" fn vexDeviceMotorEncoderUnitsGet(device: V5_DeviceT) -> V5MotorEncoderUnits {
    Default::default()
}
pub extern "C" fn vexDeviceMotorBrakeModeSet(device: V5_DeviceT, mode: V5MotorBrakeMode) {}
pub extern "C" fn vexDeviceMotorBrakeModeGet(device: V5_DeviceT) -> V5MotorBrakeMode {
    Default::default()
}
pub extern "C" fn vexDeviceMotorPositionSet(device: V5_DeviceT, position: c_double) {}
pub extern "C" fn vexDeviceMotorPositionGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceMotorPositionRawGet(device: V5_DeviceT, timestamp: *mut u32) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceMotorPositionReset(device: V5_DeviceT) {}
pub extern "C" fn vexDeviceMotorTargetGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceMotorServoTargetSet(device: V5_DeviceT, position: c_double) {}
pub extern "C" fn vexDeviceMotorAbsoluteTargetSet(
    device: V5_DeviceT,
    position: c_double,
    veloctiy: i32,
) {
}
pub extern "C" fn vexDeviceMotorRelativeTargetSet(
    device: V5_DeviceT,
    position: c_double,
    velocity: i32,
) {
}
pub extern "C" fn vexDeviceMotorFaultsGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceMotorFlagsGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceMotorVoltageSet(device: V5_DeviceT, voltage: i32) {}
pub extern "C" fn vexDeviceMotorVoltageGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceMotorGearingSet(device: V5_DeviceT, gearset: V5MotorGearset) {}
pub extern "C" fn vexDeviceMotorGearingGet(device: V5_DeviceT) -> V5MotorGearset {
    Default::default()
}
pub extern "C" fn vexDeviceMotorVoltageLimitSet(device: V5_DeviceT, limit: i32) {}
pub extern "C" fn vexDeviceMotorVoltageLimitGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceMotorVelocityUpdate(device: V5_DeviceT, velocity: i32) {}
pub extern "C" fn vexDeviceMotorPositionPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid) {}
pub extern "C" fn vexDeviceMotorVelocityPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid) {}
pub extern "C" fn vexDeviceMotorExternalProfileSet(
    device: V5_DeviceT,
    position: c_double,
    velocity: i32,
) {
}
