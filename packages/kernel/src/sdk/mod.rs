pub mod abs_encoder;
pub mod imu;
pub mod motor;
pub mod optical;
pub mod types;
pub mod vision;

use abs_encoder::*;
use imu::*;
use motor::*;
use optical::*;
use types::*;
use vision::*;

macro_rules! jump_table {
    ($table:ident, { $($offset:expr => $fun:ident,)* }) => {
        $(
            $table[$offset / 4] = $fun as _;
        )*
    };
}

#[link_section = ".jump_table"]
#[no_mangle]
pub static mut JUMP_TABLE: [*const (); 0x1000] = {
    let mut table = [unshimmed_syscall as _; 0x1000];

    jump_table!(table, {
        0x10 => vexStdlibMismatchError,
        0x20 => vexPrivateApiDisable,
        0x5c => vexBackgroundProcessing,
        0xf0 => vexDebug,
        0x118 => vexSystemTimeGet,
        0x11c => vexGettime,
        0x120 => vexGetdate,
        0x124 => vexSystemMemoryDump,
        0x130 => vexSystemExitRequest,
        0x134 => vexSystemHighResTimeGet,
        0x138 => vexSystemPowerupTimeGet,
        0x13c => vexSystemLinkAddrGet,
        0x168 => vexSystemTimerGet,
        0x16c => vexSystemTimerEnable,
        0x170 => vexSystemTimerDisable,
        0x174 => vexSystemUsbStatus,
        0x190 => vexDevicesGetNumber,
        0x194 => vexDevicesGetNumberByType,
        0x198 => vexDevicesGet,
        0x18c => vexDeviceGetByIndex,
        0x1a0 => vexDeviceGetStatus,
        0x1a4 => vexControllerGet,
        0x1a8 => vexControllerConnectionStatusGet,
        0x1ac => vexControllerTextSet,
        0x1b0 => vexDeviceGetTimestamp,
        0x1b4 => vexDeviceButtonStateGet,
        0x1e0 => vexDeviceLedSet,
        0x1e4 => vexDeviceLedGet,
        0x1ec => vexDeviceLedRgbGet,
        0x208 => vexDeviceAdiPortConfigSet,
        0x20c => vexDeviceAdiPortConfigGet,
        0x210 => vexDeviceAdiValueSet,
        0x214 => vexDeviceAdiValueGet,
        0x218 => vexDeviceAdiVoltageGet,
        0x21c => vexDeviceAdiAddrLedSet,
        // Cant infer args
        // 0x230 => vexDeviceBumperGet,
        // 0x258 => vexDeviceGyroReset,
        0x25c => vexDeviceGyroHeadingGet,
        0x260 => vexDeviceGyroDegreesGet,
        0x280 => vexDeviceSonarValueGet,
        0x2a8 => vexDeviceGenericValueGet,
        0x2d0 => vexDeviceMotorVelocitySet,
        0x2d4 => vexDeviceMotorVelocityGet,
        0x2d8 => vexDeviceMotorActualVelocityGet,
        0x2dc => vexDeviceMotorDirectionGet,
        0x2e0 => vexDeviceMotorModeSet,
        0x2e4 => vexDeviceMotorModeGet,
        0x2e8 => vexDeviceMotorPwmSet,
        0x2ec => vexDeviceMotorPwmGet,
        0x2f0 => vexDeviceMotorCurrentLimitSet,
        0x2f4 => vexDeviceMotorCurrentLimitGet,
        0x2f8 => vexDeviceMotorCurrentGet,
        0x2fc => vexDeviceMotorPowerGet,
        0x300 => vexDeviceMotorTorqueGet,
        0x304 => vexDeviceMotorEfficiencyGet,
        0x308 => vexDeviceMotorTemperatureGet,
        0x30c => vexDeviceMotorOverTempFlagGet,
        0x310 => vexDeviceMotorCurrentLimitFlagGet,
        0x314 => vexDeviceMotorZeroVelocityFlagGet,
        0x318 => vexDeviceMotorZeroPositionFlagGet,
        0x31c => vexDeviceMotorReverseFlagSet,
        0x320 => vexDeviceMotorReverseFlagGet,
        0x324 => vexDeviceMotorEncoderUnitsSet,
        0x328 => vexDeviceMotorEncoderUnitsGet,
        0x32c => vexDeviceMotorBrakeModeSet,
        0x330 => vexDeviceMotorBrakeModeGet,
        0x334 => vexDeviceMotorPositionSet,
        0x338 => vexDeviceMotorPositionGet,
        0x33c => vexDeviceMotorPositionRawGet,
        0x340 => vexDeviceMotorPositionReset,
        0x344 => vexDeviceMotorTargetGet,
        0x348 => vexDeviceMotorServoTargetSet,
        0x34c => vexDeviceMotorAbsoluteTargetSet,
        0x350 => vexDeviceMotorRelativeTargetSet,
        0x354 => vexDeviceMotorFaultsGet,
        0x358 => vexDeviceMotorFlagsGet,
        0x35c => vexDeviceMotorVoltageSet,
        0x360 => vexDeviceMotorVoltageGet,
        0x364 => vexDeviceMotorGearingSet,
        0x368 => vexDeviceMotorGearingGet,
        0x36c => vexDeviceMotorVoltageLimitSet,
        0x370 => vexDeviceMotorVoltageLimitGet,
        0x374 => vexDeviceMotorVelocityUpdate,
        0x378 => vexDeviceMotorPositionPidSet,
        0x37c => vexDeviceMotorVelocityPidSet,
        0x380 => vexDeviceMotorExternalProfileSet,
        0x398 => vexDeviceVisionModeSet,
        0x39c => vexDeviceVisionModeGet,
        0x3a0 => vexDeviceVisionObjectCountGet,
        0x3a4 => vexDeviceVisionObjectGet,
        0x3a8 => vexDeviceVisionSignatureSet,
        0x3ac => vexDeviceVisionSignatureGet,
        0x3b0 => vexDeviceVisionBrightnessSet,
        0x3b4 => vexDeviceVisionBrightnessGet,
        0x3b8 => vexDeviceVisionWhiteBalanceModeSet,
        0x3bc => vexDeviceVisionWhiteBalanceModeGet,
        0x3c0 => vexDeviceVisionWhiteBalanceSet,
        0x3c4 => vexDeviceVisionWhiteBalanceGet,
        0x3c8 => vexDeviceVisionLedModeSet,
        0x3cc => vexDeviceVisionLedModeGet,
        0x3d0 => vexDeviceVisionLedBrigntnessSet,
        0x3d4 => vexDeviceVisionLedBrigntnessGet,
        0x3d8 => vexDeviceVisionLedColorSet,
        0x3dc => vexDeviceVisionLedColorGet,
        0x3e0 => vexDeviceVisionWifiModeSet,
        0x3e4 => vexDeviceVisionWifiModeGet,
        0x410 => vexDeviceImuReset,
        0x414 => vexDeviceImuHeadingGet,
        0x418 => vexDeviceImuDegreesGet,
        0x41c => vexDeviceImuQuaternionGet,
        0x420 => vexDeviceImuAttitudeGet,
        0x424 => vexDeviceImuRawGyroGet,
        0x428 => vexDeviceImuRawAccelGet,
        0x42c => vexDeviceImuStatusGet,
        // 0x430 => vexDeviceImuTemperatureGet,
        // 0x434 => vexDeviceImuDebugGet,
        0x438 => vexDeviceImuModeSet,
        0x43c => vexDeviceImuModeGet,
        // 0x440 => vexDeviceImuCollisionDataGet,
        0x444 => vexDeviceImuDataRateSet,
        0x4d8 => vexDeviceRangeValueGet,
        // 0x460 => vexDeviceRadioUserDataReceive,
        // 0x464 => vexDeviceRadioModeSet,
        0x488 => vexDeviceAbsEncReset,
        0x48c => vexDeviceAbsEncPositionSet,
        0x490 => vexDeviceAbsEncPositionGet,
        0x494 => vexDeviceAbsEncVelocityGet,
        0x498 => vexDeviceAbsEncAngleGet,
        0x49c => vexDeviceAbsEncReverseFlagSet,
        0x4a0 => vexDeviceAbsEncReverseFlagGet,
        0x4a4 => vexDeviceAbsEncStatusGet,
        // 0x4a8 => vexDeviceAbsEncTemperatureGet,
        // 0x4ac => vexDeviceAbsEncDebugGet,
        // 0x4b0 => vexDeviceAbsEncModeSet,
        // 0x4b4 => vexDeviceAbsEncModeGet,
        // 0x4b8 => vexDeviceAbsEncOffsetSet,
        // 0x4bc => vexDeviceAbsEncOffsetGet,
        0x4c0 => vexDeviceAbsEncDataRateSet,
        0x528 => vexDeviceOpticalHueGet,
        0x52c => vexDeviceOpticalSatGet,
        0x530 => vexDeviceOpticalBrightnessGet,
        0x534 => vexDeviceOpticalProximityGet,
        0x538 => vexDeviceOpticalRgbGet,
        0x53c => vexDeviceOpticalLedPwmSet,
        0x540 => vexDeviceOpticalLedPwmGet,
        0x544 => vexDeviceOpticalStatusGet,
        0x548 => vexDeviceOpticalRawGet,
        // 0x54c => vexDeviceOpticalDebugGet,
        0x550 => vexDeviceOpticalModeSet,
        0x554 => vexDeviceOpticalModeGet,
        0x558 => vexDeviceOpticalGestureGet,
        0x55c => vexDeviceOpticalGestureEnable,
        0x560 => vexDeviceOpticalGestureDisable,
        0x564 => vexDeviceOpticalProximityThreshold,
        // 0x568 => vexDeviceOpticalGainSet,
        // 0x56c => vexDeviceOpticalMatrixSet,
        // 0x570 => vexDeviceOpticalMatrixGet,
        0x8c0 => vexSystemTimerStop,
        0xb40 => vexDeviceOpticalIntegrationTimeSet,
        0xb44 => vexDeviceOpticalIntegrationTimeGet,
    });
    table
};

pub unsafe extern "C" fn unshimmed_syscall() -> ! {
    loop {}
}

pub unsafe extern "C" fn vexStdlibMismatchError(param_1: u32, param_2: u32) {}

pub unsafe extern "C" fn vexPrivateApiDisable() {}

pub unsafe extern "C" fn vexBackgroundProcessing() {}

pub unsafe extern "C" fn vexDebug(fmt: *const u8, ...) {}

//TODO: Find the right return types for all date/time functions
pub unsafe extern "C" fn vexSystemTimeGet() -> u32 {
    0
}

pub unsafe extern "C" fn vexGettime(time: *mut Time) {
    *time = Time {
        hour: 0,
        minute: 0,
        second: 0,
        hundredths: 0,
    };
}

pub unsafe extern "C" fn vexGetdate(date: *mut Date) {
    *date = Date {
        year: 0,
        day: 0,
        month: 0,
    };
}

pub unsafe extern "C" fn vexSystemMemoryDump() {}

pub unsafe extern "C" fn vexSystemExitRequest() -> ! {
    loop {}
}

pub unsafe extern "C" fn vexSystemHighResTimeGet() -> u64 {
    0
}
pub unsafe extern "C" fn vexSystemPowerupTimeGet() -> u64 {
    0
}

pub unsafe extern "C" fn vexSystemLinkAddrGet() -> u32 {
    crate::COLD_MEMORY_START as _
}

pub unsafe extern "C" fn vexSystemTimerGet(param_1: u32) -> u32 {
    0
}
pub unsafe extern "C" fn vexSystemTimerEnable(param_1: u32) -> u32 {
    0
}
pub unsafe extern "C" fn vexSystemTimerDisable(param_1: u32) {}

pub unsafe extern "C" fn vexSystemUsbStatus() -> u32 {
    1
}

pub unsafe extern "C" fn vexDevicesGetNumber() -> u32 {
    0
}
pub unsafe extern "C" fn vexDevicesGetNumberByType(device_type: V5DeviceType) -> u32 {
    0
}
pub unsafe extern "C" fn vexDevicesGet() -> V5Device {
    V5Device {
        port: 0,
        exists: true,
        device_type: V5DeviceType::UndefinedSensor,
        timestamp: 0,
        device_specific_data: DeviceData { vision: () },
    }
}
pub unsafe extern "C" fn vexDeviceGetByIndex(index: u32) -> V5Device {
    V5Device {
        port: 0,
        exists: true,
        device_type: V5DeviceType::UndefinedSensor,
        timestamp: 0,
        device_specific_data: DeviceData { vision: () },
    }
}

pub unsafe extern "C" fn vexDeviceGetStatus(devices: *const V5DeviceType) -> i32 {
    0
}

pub unsafe extern "C" fn vexControllerGet(id: ControllerID, channel: ControllerChannel) -> i32 {
    0
}
pub unsafe extern "C" fn vexControllerConnectionStatusGet(id: ControllerID) -> i32 {
    1
}
pub unsafe extern "C" fn vexControllerTextSet(id: u32, line: u32, col: u32, buf: *const u8) -> u32 {
    1
}

pub unsafe extern "C" fn vexDeviceGetTimestamp(device: V5Device) -> u32 {
    0
}

pub unsafe extern "C" fn vexDeviceButtonStateGet() -> u32 {
    0
}

pub unsafe extern "C" fn vexDeviceLedSet(device: V5Device, color: u32) {}

pub unsafe extern "C" fn vexDeviceLedGet(device: V5Device) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceLedRgbGet(device: V5Device) -> u32 {
    0
}

pub unsafe extern "C" fn vexDeviceAdiPortConfigSet(
    device: V5Device,
    port: u32,
    config: AdiPortConfiguration,
) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceAdiPortConfigGet(
    device: V5Device,
    port: u32,
) -> AdiPortConfiguration {
    AdiPortConfiguration::AnalogIn
}

pub unsafe extern "C" fn vexDeviceAdiValueSet(device: V5Device, port: u32, value: i32) {}
pub unsafe extern "C" fn vexDeviceAdiValueGet(device: V5Device, port: u32) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceAdiVoltageGet(device: V5Device, port: u32) -> i32 {
    0
}

pub unsafe extern "C" fn vexDeviceAdiAddrLedSet(
    device: V5Device,
    port: u32,
    pixel_data: *const u32,
    offset: u32,
    len: u32,
    opts: u32,
) -> i32 {
    0
}

pub unsafe extern "C" fn vexDeviceGyroHeadingGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceGyroDegreesGet(device: V5Device) -> f64 {
    0.0
}

pub unsafe extern "C" fn vexDeviceSonarValueGet(device: V5Device) -> i32 {
    0
}

pub unsafe extern "C" fn vexDeviceGenericValueGet(device: V5Device) -> f64 {
    0.0
}

pub unsafe extern "C" fn vexDeviceRangeValueGet(device: V5Device) -> i32 {
    0
}

pub unsafe extern "C" fn vexSystemTimerStop() {}
