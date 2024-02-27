use super::types::*;

pub unsafe extern "C" fn set_motor_velocity(device: V5DeviceHandle, velocity: i32) {}
pub unsafe extern "C" fn motor_velocity(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn motor_actual_velocity(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn motor_direction(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn set_motor_mode(device: V5DeviceHandle, mode: MotorControlMode) {}
pub unsafe extern "C" fn motor_mode(device: V5DeviceHandle) -> MotorControlMode {
    MotorControlMode::Undefined
}
pub unsafe extern "C" fn set_motor_pwm(device: V5DeviceHandle, pwm: i32) {}
pub unsafe extern "C" fn motor_pwm(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn set_motor_current_limit(device: V5DeviceHandle, limit: i32) {}
pub unsafe extern "C" fn motor_current_limit(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn motor_current(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn motor_power(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn motor_torque(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn motor_efficiency(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn motor_temperature(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn motor_over_temp_flag(device: V5DeviceHandle) -> bool {
    false
}
pub unsafe extern "C" fn motor_current_limit_flag(device: V5DeviceHandle) -> bool {
    false
}
pub unsafe extern "C" fn motor_zero_velocity_flag(device: V5DeviceHandle) -> bool {
    false
}
pub unsafe extern "C" fn motor_zero_position_flag(device: V5DeviceHandle) -> bool {
    false
}
pub unsafe extern "C" fn set_motor_reverse_flag(device: V5DeviceHandle, reverse: bool) {}
pub unsafe extern "C" fn motor_reverse_flag(device: V5DeviceHandle) -> bool {
    false
}
pub unsafe extern "C" fn set_motor_encoder_units(device: V5DeviceHandle, units: MotorEncoderUnits) {
}
pub unsafe extern "C" fn motor_encoder_units(device: V5DeviceHandle) -> MotorEncoderUnits {
    MotorEncoderUnits::Degrees
}
pub unsafe extern "C" fn set_motor_brake_mode(device: V5DeviceHandle, mode: MotorBrakeMode) {}
pub unsafe extern "C" fn motor_brake_mode(device: V5DeviceHandle) -> MotorBrakeMode {
    MotorBrakeMode::Coast
}
pub unsafe extern "C" fn set_motor_position(device: V5DeviceHandle, position: f64) {}
pub unsafe extern "C" fn motor_position(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn motor_position_raw(
    device: V5DeviceHandle,
    timestamp: *const u32,
) -> i32 {
    0
}
pub unsafe extern "C" fn motor_position_reset(device: V5DeviceHandle) {}
pub unsafe extern "C" fn motor_target(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn set_motor_servo_target(device: V5DeviceHandle, position: f64) {}
pub unsafe extern "C" fn set_motor_absolute_target(
    device: V5DeviceHandle,
    position: f64,
    veloctiy: i32,
) {
}
pub unsafe extern "C" fn set_motor_relative_target(
    device: V5DeviceHandle,
    position: f64,
    velocity: i32,
) {
}
pub unsafe extern "C" fn motor_faults(device: V5DeviceHandle) -> u32 {
    0
}
pub unsafe extern "C" fn motor_flags(device: V5DeviceHandle) -> u32 {
    0
}
pub unsafe extern "C" fn set_motor_voltage(device: V5DeviceHandle, voltage: i32) {}
pub unsafe extern "C" fn motor_voltage(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn set_motor_gearing(device: V5DeviceHandle, gearset: MotorGearset) {}
pub unsafe extern "C" fn motor_gearing(device: V5DeviceHandle) -> MotorGearset {
    MotorGearset::Gearing36
}
pub unsafe extern "C" fn set_motor_voltage_limit(device: V5DeviceHandle, limit: i32) {}
pub unsafe extern "C" fn motor_voltage_limit(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn motor_velocity_update(device: V5DeviceHandle, velocity: i32) {}
pub unsafe extern "C" fn set_motor_position_pid(device: V5DeviceHandle, pid: *mut MotorPid) {}
pub unsafe extern "C" fn set_motor_velocity_pid(device: V5DeviceHandle, pid: *mut MotorPid) {}
pub unsafe extern "C" fn set_motor_external_profile(
    device: V5DeviceHandle,
    position: f64,
    velocity: i32,
) {
}
