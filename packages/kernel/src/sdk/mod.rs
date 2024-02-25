pub mod types;

use types::*;

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
    let mut table = [core::ptr::null(); 0x1000];

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
    });
    table
};

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
    0x03800000
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
