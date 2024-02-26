use super::types::*;

pub unsafe extern "C" fn vexDeviceMotorVelocitySet(device: V5Device, velocity: i32) {}
pub unsafe extern "C" fn vexDeviceMotorVelocityGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorActualVelocityGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorDirectionGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorModeSet(device: V5Device, mode: MotorControlMode) {}
pub unsafe extern "C" fn vexDeviceMotorModeGet(device: V5Device) -> MotorControlMode {
    MotorControlMode::Undefined
}
pub unsafe extern "C" fn vexDeviceMotorPwmSet(device: V5Device, pwm: i32) {}
pub unsafe extern "C" fn vexDeviceMotorPwmGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorCurrentLimitSet(device: V5Device, limit: i32) {}
pub unsafe extern "C" fn vexDeviceMotorCurrentLimitGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorCurrentGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorPowerGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorTorqueGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorEfficiencyGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorTemperatureGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorOverTempFlagGet(device: V5Device) -> bool {
    false
}
pub unsafe extern "C" fn vexDeviceMotorCurrentLimitFlagGet(device: V5Device) -> bool {
    false
}
pub unsafe extern "C" fn vexDeviceMotorZeroVelocityFlagGet(device: V5Device) -> bool {
    false
}
pub unsafe extern "C" fn vexDeviceMotorZeroPositionFlagGet(device: V5Device) -> bool {
    false
}
pub unsafe extern "C" fn vexDeviceMotorReverseFlagSet(device: V5Device, reverse: bool) {}
pub unsafe extern "C" fn vexDeviceMotorReverseFlagGet(device: V5Device) -> bool {
    false
}
pub unsafe extern "C" fn vexDeviceMotorEncoderUnitsSet(device: V5Device, units: MotorEncoderUnits) {
}
pub unsafe extern "C" fn vexDeviceMotorEncoderUnitsGet(device: V5Device) -> MotorEncoderUnits {
    MotorEncoderUnits::Degrees
}
pub unsafe extern "C" fn vexDeviceMotorBrakeModeSet(device: V5Device, mode: MotorBrakeMode) {}
pub unsafe extern "C" fn vexDeviceMotorBrakeModeGet(device: V5Device) -> MotorBrakeMode {
    MotorBrakeMode::Coast
}
pub unsafe extern "C" fn vexDeviceMotorPositionSet(device: V5Device, position: f64) {}
pub unsafe extern "C" fn vexDeviceMotorPositionGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorPositionRawGet(
    device: V5Device,
    timestamp: *const u32,
) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorPositionReset(device: V5Device) {}
pub unsafe extern "C" fn vexDeviceMotorTargetGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMotorServoTargetSet(device: V5Device, position: f64) {}
pub unsafe extern "C" fn vexDeviceMotorAbsoluteTargetSet(
    device: V5Device,
    position: f64,
    veloctiy: i32,
) {
}
pub unsafe extern "C" fn vexDeviceMotorRelativeTargetSet(
    device: V5Device,
    position: f64,
    velocity: i32,
) {
}
pub unsafe extern "C" fn vexDeviceMotorFaultsGet(device: V5Device) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorFlagsGet(device: V5Device) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorVoltageSet(device: V5Device, voltage: i32) {}
pub unsafe extern "C" fn vexDeviceMotorVoltageGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorGearingSet(device: V5Device, gearset: MotorGearset) {}
pub unsafe extern "C" fn vexDeviceMotorGearingGet(device: V5Device) -> MotorGearset {
    MotorGearset::Gearing36
}
pub unsafe extern "C" fn vexDeviceMotorVoltageLimitSet(device: V5Device, limit: i32) {}
pub unsafe extern "C" fn vexDeviceMotorVoltageLimitGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMotorVelocityUpdate(device: V5Device, velocity: i32) {}
pub unsafe extern "C" fn vexDeviceMotorPositionPidSet(device: V5Device, pid: *mut MotorPid) {}
pub unsafe extern "C" fn vexDeviceMotorVelocityPidSet(device: V5Device, pid: *mut MotorPid) {}
pub unsafe extern "C" fn vexDeviceMotorExternalProfileSet(
    device: V5Device,
    position: f64,
    velocity: i32,
) {
}
