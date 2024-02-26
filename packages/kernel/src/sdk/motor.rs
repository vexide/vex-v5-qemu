use super::types::*;

pub unsafe extern "C" fn vexDeviceMotorVelocitySet(device: V5DeviceHandle, velocity: i32) {}
pub unsafe extern "C" fn vexDeviceMotorVelocityGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorActualVelocityGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorDirectionGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorModeSet(device: V5DeviceHandle, mode: MotorControlMode) {}
pub unsafe extern "C" fn vexDeviceMotorModeGet(device: V5DeviceHandle) -> MotorControlMode {
    MotorControlMode::Undefined
}
pub unsafe extern "C" fn vexDeviceMotorPwmSet(device: V5DeviceHandle, pwm: i32) {}
pub unsafe extern "C" fn vexDeviceMotorPwmGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorCurrentLimitSet(device: V5DeviceHandle, limit: i32) {}
pub unsafe extern "C" fn vexDeviceMotorCurrentLimitGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorCurrentGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorPowerGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorTorqueGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorEfficiencyGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorTemperatureGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorOverTempFlagGet(device: V5DeviceHandle) -> bool {
    false
}
pub unsafe extern "C" fn vexDeviceMotorCurrentLimitFlagGet(device: V5DeviceHandle) -> bool {
    false
}
pub unsafe extern "C" fn vexDeviceMotorZeroVelocityFlagGet(device: V5DeviceHandle) -> bool {
    false
}
pub unsafe extern "C" fn vexDeviceMotorZeroPositionFlagGet(device: V5DeviceHandle) -> bool {
    false
}
pub unsafe extern "C" fn vexDeviceMotorReverseFlagSet(device: V5DeviceHandle, reverse: bool) {}
pub unsafe extern "C" fn vexDeviceMotorReverseFlagGet(device: V5DeviceHandle) -> bool {
    false
}
pub unsafe extern "C" fn vexDeviceMotorEncoderUnitsSet(device: V5DeviceHandle, units: MotorEncoderUnits) {
}
pub unsafe extern "C" fn vexDeviceMotorEncoderUnitsGet(device: V5DeviceHandle) -> MotorEncoderUnits {
    MotorEncoderUnits::Degrees
}
pub unsafe extern "C" fn vexDeviceMotorBrakeModeSet(device: V5DeviceHandle, mode: MotorBrakeMode) {}
pub unsafe extern "C" fn vexDeviceMotorBrakeModeGet(device: V5DeviceHandle) -> MotorBrakeMode {
    MotorBrakeMode::Coast
}
pub unsafe extern "C" fn vexDeviceMotorPositionSet(device: V5DeviceHandle, position: f64) {}
pub unsafe extern "C" fn vexDeviceMotorPositionGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorPositionRawGet(
    device: V5DeviceHandle,
    timestamp: *const u32,
) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorPositionReset(device: V5DeviceHandle) {}
pub unsafe extern "C" fn vexDeviceMotorTargetGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorServoTargetSet(device: V5DeviceHandle, position: f64) {}
pub unsafe extern "C" fn vexDeviceMotorAbsoluteTargetSet(
    device: V5DeviceHandle,
    position: f64,
    veloctiy: i32,
) {
}
pub unsafe extern "C" fn vexDeviceMotorRelativeTargetSet(
    device: V5DeviceHandle,
    position: f64,
    velocity: i32,
) {
}
pub unsafe extern "C" fn vexDeviceMotorFaultsGet(device: V5DeviceHandle) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorFlagsGet(device: V5DeviceHandle) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorVoltageSet(device: V5DeviceHandle, voltage: i32) {}
pub unsafe extern "C" fn vexDeviceMotorVoltageGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorGearingSet(device: V5DeviceHandle, gearset: MotorGearset) {}
pub unsafe extern "C" fn vexDeviceMotorGearingGet(device: V5DeviceHandle) -> MotorGearset {
    MotorGearset::Gearing36
}
pub unsafe extern "C" fn vexDeviceMotorVoltageLimitSet(device: V5DeviceHandle, limit: i32) {}
pub unsafe extern "C" fn vexDeviceMotorVoltageLimitGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorVelocityUpdate(device: V5DeviceHandle, velocity: i32) {}
pub unsafe extern "C" fn vexDeviceMotorPositionPidSet(device: V5DeviceHandle, pid: *mut MotorPid) {}
pub unsafe extern "C" fn vexDeviceMotorVelocityPidSet(device: V5DeviceHandle, pid: *mut MotorPid) {}
pub unsafe extern "C" fn vexDeviceMotorExternalProfileSet(
    device: V5DeviceHandle,
    position: f64,
    velocity: i32,
) {
}
