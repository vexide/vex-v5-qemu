#![deny(unsafe_op_in_unsafe_fn)]
#![allow(
    unused_variables,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]

pub mod abs_enc;
pub mod adi;
pub mod ai_vision;
pub mod arm;
pub mod battery;
pub mod competition;
pub mod controller;
pub mod device;
pub mod display;
pub mod distance;
pub mod file;
pub mod generic_radio;
pub mod generic_serial;
pub mod gps;
pub mod imu;
pub mod led;
pub mod light_tower;
pub mod magnet;
pub mod motor;
pub mod optical;
pub mod pneumatic;
pub mod range;
pub mod serial;
pub mod system;
pub mod task;
pub mod touch;
pub mod vision;

pub use abs_enc::*;
pub use adi::*;
pub use ai_vision::*;
pub use arm::*;
pub use battery::*;
pub use competition::*;
pub use controller::*;
pub use device::*;
pub use display::*;
pub use distance::*;
pub use file::*;
pub use generic_radio::*;
pub use generic_serial::*;
pub use gps::*;
pub use imu::*;
pub use led::*;
pub use light_tower::*;
pub use magnet::*;
pub use motor::*;
pub use optical::*;
pub use pneumatic::*;
pub use range::*;
pub use serial::*;
pub use system::*;
pub use task::*;
pub use touch::*;
pub use vision::*;

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
        // Rotation Sensor
        0x488 => vexDeviceAbsEncReset,
        0x48c => vexDeviceAbsEncPositionSet,
        0x490 => vexDeviceAbsEncPositionGet,
        0x494 => vexDeviceAbsEncVelocityGet,
        0x498 => vexDeviceAbsEncAngleGet,
        0x49c => vexDeviceAbsEncReverseFlagSet,
        0x4a0 => vexDeviceAbsEncReverseFlagGet,
        0x4a4 => vexDeviceAbsEncStatusGet,
        0x4a8 => vexDeviceAbsEncTemperatureGet,
        0x4c0 => vexDeviceAbsEncDataRateSet,

        // ADI
        0x208 => vexDeviceAdiPortConfigSet,
        0x20c => vexDeviceAdiPortConfigGet,
        0x210 => vexDeviceAdiValueSet,
        0x214 => vexDeviceAdiValueGet,
        0x21c => vexDeviceAdiAddrLedSet,
        0x230 => vexDeviceBumperGet,
        0x258 => vexDeviceGyroReset,
        0x25c => vexDeviceGyroHeadingGet,
        0x260 => vexDeviceGyroDegreesGet,
        0x280 => vexDeviceSonarValueGet,

        // AI Vision
        0xcd4 => vexDeviceAiVisionClassNameGet,
        0xcc4 => vexDeviceAiVisionCodeGet,
        0xcc0 => vexDeviceAiVisionCodeSet,
        0xcbc => vexDeviceAiVisionColorGet,
        0xcb8 => vexDeviceAiVisionColorSet,
        0xcac => vexDeviceAiVisionModeGet,
        0xca8 => vexDeviceAiVisionModeSet,
        0xcb0 => vexDeviceAiVisionObjectCountGet,
        0xcb4 => vexDeviceAiVisionObjectGet,
        0xcd8 => vexDeviceAiVisionSensorSet,
        0xcc8 => vexDeviceAiVisionStatusGet,
        0xccc => vexDeviceAiVisionTemperatureGet,

        // CTE Workcell Arm
        0xb54 => vexDeviceArmMoveTipCommandLinearAdv,
        0xb58 => vexDeviceArmMoveTipCommandJointAdv,
        0xb5c => vexDeviceArmTipPositionGetAdv,
        0xc30 => vexDeviceArmPoseSet,
        0xc34 => vexDeviceArmMoveTipCommandLinear,
        0xc38 => vexDeviceArmMoveTipCommandJoint,
        0xc3c => vexDeviceArmMoveJointsCommand,
        0xc40 => vexDeviceArmSpinJoints,
        0xc44 => vexDeviceArmSetJointPositions,
        0xc48 => vexDeviceArmPickUpCommand,
        0xc4c => vexDeviceArmDropCommand,
        0xc50 => vexDeviceArmMoveVoltsCommand,
        0xc54 => vexDeviceArmFullStop,
        0xc58 => vexDeviceArmEnableProfiler,
        0xc5c => vexDeviceArmProfilerVelocitySet,
        0xc60 => vexDeviceArmSaveZeroValues,
        0xc64 => vexDeviceArmForceZeroCommand,
        0xc68 => vexDeviceArmClearZeroValues,
        0xc6c => vexDeviceArmBootload,
        0xc70 => vexDeviceArmTipPositionGet,
        0xc74 => vexDeviceArmJointInfoGet,
        0xc78 => vexDeviceArmJ6PositionGet,
        0xc7c => vexDeviceArmBatteryGet,
        0xc80 => vexDeviceArmServoFlagsGet,
        0xc84 => vexDeviceArmStatusGet,
        0xc88 => vexDeviceArmDebugGet,
        0xc8c => vexDeviceArmJointErrorsGet,
        0xc90 => vexDeviceArmJ6PositionSet,
        0xc94 => vexDeviceArmStopJointsCommand,
        0xc98 => vexDeviceArmReboot,
        0xc9c => vexDeviceArmTipOffsetSet,

        // Battery
        0xa00 => vexBatteryVoltageGet,
        0xa04 => vexBatteryCurrentGet,
        0xa08 => vexBatteryTemperatureGet,
        0xa0c => vexBatteryCapacityGet,

        // Competition
        0x9d8 => vexCompetitionStatus,
        0x9dc => vexCompetitionControl,

        // Controller
        0x1a4 => vexControllerGet,
        0x1a8 => vexControllerConnectionStatusGet,
        0x1ac => vexControllerTextSet,

        // Device
        0x190 => vexDevicesGetNumber,
        0x194 => vexDevicesGetNumberByType,
        0x198 => vexDevicesGet,
        0x19c => vexDeviceGetByIndex,
        0x1d8 => vexDeviceFlagsGetByIndex,
        0x1a0 => vexDeviceGetStatus,
        0x1b0 => vexDeviceGetTimestamp,
        0x2a8 => vexDeviceGenericValueGet,
        0x1b8 => vexDeviceTypeGetByIndex,
        0x1b4 => vexDeviceButtonStateGet,

        // Display
        0x640 => vexDisplayForegroundColor,
        0x644 => vexDisplayBackgroundColor,
        0x648 => vexDisplayErase,
        0x64c => vexDisplayScroll,
        0x650 => vexDisplayScrollRect,
        0x654 => vexDisplayCopyRect,
        0x658 => vexDisplayPixelSet,
        0x65c => vexDisplayPixelClear,
        0x660 => vexDisplayLineDraw,
        0x664 => vexDisplayLineClear,
        0x668 => vexDisplayRectDraw,
        0x66c => vexDisplayRectClear,
        0x670 => vexDisplayRectFill,
        0x674 => vexDisplayCircleDraw,
        0x678 => vexDisplayCircleClear,
        0x67c => vexDisplayCircleFill,
        0x6a8 => vexDisplayTextSize,
        0x6b4 => vexDisplayFontNamedSet,
        0x6b8 => vexDisplayForegroundColorGet,
        0x6bc => vexDisplayBackgroundColorGet,
        0x6c0 => vexDisplayStringWidthGet,
        0x6c4 => vexDisplayStringHeightGet,
        0x6c8 => vexDisplayPenSizeSet,
        0x6cc => vexDisplayPenSizeGet,
        0x794 => vexDisplayClipRegionSet,
        0x7a0 => vexDisplayRender,
        0x7a4 => vexDisplayDoubleBufferDisable,
        0x7a8 => vexDisplayClipRegionSetWithIndex,
        0x990 => vexImageBmpRead,
        0x994 => vexImagePngRead,
        0x680 => vexDisplayVPrintf,
        0x684 => vexDisplayVString,
        0x688 => vexDisplayVStringAt,
        0x68c => vexDisplayVBigString,
        0x690 => vexDisplayVBigStringAt,
        0x6b0 => vexDisplayVSmallStringAt,
        0x694 => vexDisplayVCenteredString,
        0x698 => vexDisplayVBigCenteredString,

        // Distance Sensor
        0x500 => vexDeviceDistanceDistanceGet,
        0x504 => vexDeviceDistanceConfidenceGet,
        0x508 => vexDeviceDistanceStatusGet,
        0x518 => vexDeviceDistanceObjectSizeGet,
        0x51c => vexDeviceDistanceObjectVelocityGet,

        // FAT32 xilffs filesystem
        0x7d0 => vexFileMountSD,
        0x7d4 => vexFileDirectoryGet,
        0x7d8 => vexFileOpen,
        0x7dc => vexFileOpenWrite,
        0x7e0 => vexFileOpenCreate,
        0x7e4 => vexFileClose,
        0x7ec => vexFileWrite,
        0x7f0 => vexFileSize,
        0x7f4 => vexFileSeek,
        0x7f8 => vexFileRead,
        0x7fc => vexFileDriveStatus,
        0x800 => vexFileTell,
        0x804 => vexFileSync,
        0x808 => vexFileStatus,

        // VEXLink
        0xaa4 => vexDeviceGenericRadioConnection,
        0xaac => vexDeviceGenericRadioWriteFree,
        0xab0 => vexDeviceGenericRadioTransmit,
        0xabc => vexDeviceGenericRadioReceiveAvail,
        0xac0 => vexDeviceGenericRadioReceive,
        0xac8 => vexDeviceGenericRadioLinkStatus,

        // Serial
        0xa50 => vexDeviceGenericSerialEnable,
        0xa54 => vexDeviceGenericSerialBaudrate,
        0xa58 => vexDeviceGenericSerialWriteChar,
        0xa5c => vexDeviceGenericSerialWriteFree,
        0xa60 => vexDeviceGenericSerialTransmit,
        0xa64 => vexDeviceGenericSerialReadChar,
        0xa68 => vexDeviceGenericSerialPeekChar,
        0xa6c => vexDeviceGenericSerialReceiveAvail,
        0xa70 => vexDeviceGenericSerialReceive,
        0xa74 => vexDeviceGenericSerialFlush,

        // GPS Sensor
        0x5c8 => vexDeviceGpsReset,
        0x5cc => vexDeviceGpsHeadingGet,
        0x5d0 => vexDeviceGpsDegreesGet,
        0x5d4 => vexDeviceGpsQuaternionGet,
        0x5d8 => vexDeviceGpsAttitudeGet,
        0x5dc => vexDeviceGpsRawGyroGet,
        0x5e0 => vexDeviceGpsRawAccelGet,
        0x5e4 => vexDeviceGpsStatusGet,
        0x5e8 => vexDeviceGpsTemperatureGet,
        0x5f0 => vexDeviceGpsModeSet,
        0x5f4 => vexDeviceGpsModeGet,
        0x5f8 => vexDeviceGpsDataRateSet,
        0x5fc => vexDeviceGpsOriginSet,
        0x600 => vexDeviceGpsOriginGet,
        0x604 => vexDeviceGpsRotationSet,
        0x608 => vexDeviceGpsRotationGet,
        0x60c => vexDeviceGpsInitialPositionSet,
        0x614 => vexDeviceGpsErrorGet,

        // Inertial Sensor
        0x410 => vexDeviceImuReset,
        0x414 => vexDeviceImuHeadingGet,
        0x418 => vexDeviceImuDegreesGet,
        0x41c => vexDeviceImuQuaternionGet,
        0x420 => vexDeviceImuAttitudeGet,
        0x424 => vexDeviceImuRawGyroGet,
        0x428 => vexDeviceImuRawAccelGet,
        0x42c => vexDeviceImuStatusGet,
        0x430 => vexDeviceImuTemperatureGet,
        0x438 => vexDeviceImuModeSet,
        0x43c => vexDeviceImuModeGet,
        0x444 => vexDeviceImuDataRateSet,

        // LED (unused)
        0x1e0 => vexDeviceLedSet,
        0x1e4 => vexDeviceLedRgbSet,
        0x1e8 => vexDeviceLedGet,
        0x1ec => vexDeviceLedRgbGet,

        // CTE Workcell Light Tower
        0x5b8 => vexDeviceLightTowerBlinkSet,
        0x5a4 => vexDeviceLightTowerColorSet,
        0x5a8 => vexDeviceLightTowerRgbGet,
        0x5a0 => vexDeviceLightTowerRgbSet,
        0x5b0 => vexDeviceLightTowerStatusGet,
        0x5b4 => vexDeviceLightTowerDebugGet,
        0x5ac => vexDeviceLightTowerXywGet,

        // Electromagnet
        0x578 => vexDeviceMagnetPowerSet,
        0x57c => vexDeviceMagnetPowerGet,
        0x580 => vexDeviceMagnetPickup,
        0x584 => vexDeviceMagnetDrop,
        0x588 => vexDeviceMagnetTemperatureGet,
        0x58c => vexDeviceMagnetCurrentGet,
        0x590 => vexDeviceMagnetStatusGet,

        // Smart Motor
        0x2d0 => vexDeviceMotorVelocitySet,
        0x2d4 => vexDeviceMotorVelocityGet,
        0x2d4 => vexDeviceMotorActualVelocityGet,
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

        // Optical Sensor
        0x528 => vexDeviceOpticalHueGet,
        0x52c => vexDeviceOpticalSatGet,
        0x530 => vexDeviceOpticalBrightnessGet,
        0x534 => vexDeviceOpticalProximityGet,
        0x538 => vexDeviceOpticalRgbGet,
        0x53c => vexDeviceOpticalLedPwmSet,
        0x540 => vexDeviceOpticalLedPwmGet,
        0x544 => vexDeviceOpticalStatusGet,
        0x548 => vexDeviceOpticalRawGet,
        0x550 => vexDeviceOpticalModeSet,
        0x554 => vexDeviceOpticalModeGet,
        0x558 => vexDeviceOpticalGestureGet,
        0x55c => vexDeviceOpticalGestureEnable,
        0x560 => vexDeviceOpticalGestureDisable,
        0x564 => vexDeviceOpticalProximityThreshold,
        0xb40 => vexDeviceOpticalIntegrationTimeSet,
        0xb44 => vexDeviceOpticalIntegrationTimeGet,

        // CTE Workcell Pneumatics
        0xc28 => vexDevicePneumaticActuationStatusGet,
        0xc08 => vexDevicePneumaticCompressorSet,
        0xc10 => vexDevicePneumaticCtrlSet,
        0xc20 => vexDevicePneumaticCylinderPwmSet,
        0xc0c => vexDevicePneumaticCylinderSet,
        0xc1c => vexDevicePneumaticPwmGet,
        0xc18 => vexDevicePneumaticPwmSet,
        0xc14 => vexDevicePneumaticStatusGet,

        // Unused LIDAR stuff
        0x4d8 => vexDeviceRangeValueGet,

        // Serial
        0x898 => vexSerialWriteChar,
        0x89c => vexSerialWriteBuffer,
        0x8a0 => vexSerialReadChar,
        0x8a4 => vexSerialPeekChar,
        0x8ac => vexSerialWriteFree,
        0x0f0 => vex_vprintf,
        0x0f4 => vex_vsprintf,
        0x0f8 => vex_vsnprintf,

        // System
        0x10 => vexStdlibMismatchError,
        0x01c => vexScratchMemoryPtr,
        0x998 => vexScratchMemoryLock,
        0x99c => vexScratchMemoryUnock,
        0x118 => vexSystemTimeGet,
        0x11c => vexGettime,
        0x120 => vexGetdate,
        0x124 => vexSystemMemoryDump,
        0x128 => vexSystemDigitalIO,
        0x12c => vexSystemStartupOptions,
        0x130 => vexSystemExitRequest,
        0x134 => vexSystemHighResTimeGet,
        0x138 => vexSystemPowerupTimeGet,
        0x13c => vexSystemLinkAddrGet,
        0x168 => vexSystemTimerGet,
        0x16c => vexSystemTimerEnable,
        0x170 => vexSystemTimerDisable,
        0x174 => vexSystemUsbStatus,
        0x8c0 => vexSystemTimerStop,
        0x8c4 => vexSystemTimerClearInterrupt,
        0x8c8 => vexSystemTimerReinitForRtos,
        0x8cc => vexSystemApplicationIRQHandler,
        0x8d0 => vexSystemWatchdogReinitRtos,
        0x8d4 => vexSystemWatchdogGet,
        0x910 => vexSystemBoot,
        0x914 => vexSystemUndefinedException,
        0x918 => vexSystemFIQInterrupt,
        0x91c => vexSystemIQRQnterrupt,
        0x920 => vexSystemSWInterrupt,
        0x924 => vexSystemDataAbortInterrupt,
        0x928 => vexSystemPrefetchAbortInterrupt,

        // Task Scheduler
        0x028 => vexTaskAdd,
        0x084 => vexTaskGetCallbackAndId,
        0x06c => vexTaskSleep,
        0x140 => vexTaskHardwareConcurrency,
        0xf74 => vexBackgroundProcessing,
        0x05c => vexTasksRun,

        // Touch
        0x960 => vexTouchUserCallbackSet,
        0x964 => vexTouchDataGet,

        // Vision
        0x398 => vexDeviceVisionModeSet,
        0x39c => vexDeviceVisionModeGet,
        0x3a0 => vexDeviceVisionObjectCountGet,
        0x3a4 => vexDeviceVisionObjectGet,
        0x3a8 => vexDeviceVisionSignatureSet,
        0x3ac => vexDeviceVisionSignatureGet,
        0x3c0 => vexDeviceVisionBrightnessSet,
        0x3c4 => vexDeviceVisionBrightnessGet,
        0x3c8 => vexDeviceVisionWhiteBalanceModeSet,
        0x3cc => vexDeviceVisionWhiteBalanceModeGet,
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
    });
    table
};

pub unsafe extern "C" fn unshimmed_syscall() -> ! {
    loop {}
}