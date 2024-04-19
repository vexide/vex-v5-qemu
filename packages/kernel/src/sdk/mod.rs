#![deny(unsafe_op_in_unsafe_fn)]

pub(crate) mod abs_encoder;
pub(crate) mod display;
pub(crate) mod distance;
pub(crate) mod gps;
pub(crate) mod imu;
pub(crate) mod magnet;
pub(crate) mod motor;
pub(crate) mod optical;
pub(crate) mod serial;
pub(crate) mod types;
pub(crate) mod vision;

use abs_encoder::*;
use display::*;
pub use distance::*;
use gps::*;
use imu::*;
use magnet::*;
use motor::*;
use optical::*;
use serial::*;
use types::*;
use vision::*;

use crate::{hal::{self, gic::GenericInterruptController, timer::Timer, wdt::WatchdogTimer}, INTERRUPT_CONTROLLER, SYSTEM_TIME, WATCHDOG_TIMER};
use core::{arch::asm, ffi::c_void, sync::atomic::{AtomicBool, AtomicU32, Ordering}};

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
        0x10 => stdlib_mismatch_error,
        0x20 => private_api_disable,
        0x5c => vexos_background_processing,
        0xf0 => vexos_debug,
        0x118 => system_time,
        0x11c => time,
        0x120 => date,
        0x124 => system_memory_dump,
        0x130 => request_exit,
        0x134 => high_res_time,
        0x138 => system_powerup_time,
        0x13c => cold_link_addr,
        0x168 => system_timer,
        0x16c => enable_system_timer,
        0x170 => disable_system_timer,
        0x8c8 => reinit_system_timer_for_rtos,
        0x8cc => system_application_irq_handler,
        0x174 => usb_status,
        0x190 => get_num_devices,
        0x194 => get_num_devices_by_type,
        0x198 => devices,
        0x19c => device_by_index,
        0x1a0 => device_status,
        0x1a4 => controller_channel,
        0x1a8 => controller_connection_status,
        0x1ac => set_controller_text,
        0x1b0 => device_timestamp,
        0x1b4 => button_state,
        0x1e0 => led_set,
        0x1e4 => led,
        0x1ec => led_rgb,
        0x208 => adi_port_config_set,
        0x20c => adi_port_config,
        0x210 => adi_value_set,
        0x214 => adi_value,
        0x218 => adi_voltage,
        0x21c => adi_addr_led_set,
        // Cant infer args
        // 0x230 => vexDeviceBumperGet,
        // 0x258 => vexDeviceGyroReset,
        0x25c => gyro_heading,
        0x260 => gyro_degrees,
        0x280 => sonar_value,
        0x2a8 => generic_smart_value,
        0x2d0 => set_motor_velocity,
        0x2d4 => motor_velocity,
        0x2d8 => motor_actual_velocity,
        0x2dc => motor_direction,
        0x2e0 => set_motor_mode,
        0x2e4 => motor_mode,
        0x2e8 => set_motor_pwm,
        0x2ec => motor_pwm,
        0x2f0 => set_motor_current_limit,
        0x2f4 => motor_current_limit,
        0x2f8 => motor_current,
        0x2fc => motor_power,
        0x300 => motor_torque,
        0x304 => motor_efficiency,
        0x308 => motor_temperature,
        0x30c => motor_over_temp_flag,
        0x310 => motor_current_limit_flag,
        0x314 => motor_zero_velocity_flag,
        0x318 => motor_zero_position_flag,
        0x31c => set_motor_reverse_flag,
        0x320 => motor_reverse_flag,
        0x324 => set_motor_encoder_units,
        0x328 => motor_encoder_units,
        0x32c => set_motor_brake_mode,
        0x330 => motor_brake_mode,
        0x334 => set_motor_position_pid,
        0x338 => motor_position,
        0x33c => motor_position_raw,
        0x340 => motor_position_reset,
        0x344 => motor_target,
        0x348 => set_motor_servo_target,
        0x34c => set_motor_absolute_target,
        0x350 => set_motor_relative_target,
        0x354 => motor_faults,
        0x358 => motor_flags,
        0x35c => set_motor_voltage,
        0x360 => motor_voltage,
        0x364 => set_motor_gearing,
        0x368 => motor_gearing,
        0x36c => set_motor_voltage_limit,
        0x370 => motor_voltage_limit,
        0x374 => motor_velocity_update,
        0x378 => set_motor_position_pid,
        0x37c => set_motor_velocity_pid,
        0x380 => set_motor_external_profile,
        0x398 => set_vision_mode,
        0x39c => vision_mode,
        0x3a0 => vision_object_count,
        0x3a4 => vision_object,
        0x3a8 => set_vision_signature,
        0x3ac => vision_signature,
        0x3b0 => set_vision_brightness,
        0x3b4 => vision_brightness,
        0x3b8 => set_vision_white_balance_mode,
        0x3bc => vision_white_balance_mode,
        0x3c0 => set_vision_white_balance,
        0x3c4 => vision_white_balance,
        0x3c8 => set_vision_led_mod,
        0x3cc => vision_led_mode,
        0x3d0 => set_vision_led_brigntness,
        0x3d4 => vision_led_brigntness,
        0x3d8 => set_vision_led_color,
        0x3dc => vision_led_color,
        0x3e0 => set_vision_wifi_mode,
        0x3e4 => vision_wifi_mode,
        0x410 => imu_reset,
        0x414 => imu_heading,
        0x418 => imu_degrees,
        0x41c => imu_quaternion,
        0x420 => imu_attitude,
        0x424 => imu_raw_gyro,
        0x428 => imu_raw_accel,
        0x42c => imu_status,
        // 0x430 => vexDeviceImuTemperatureGet,
        // 0x434 => vexDeviceImuDebugGet,
        0x438 => set_imu_mode,
        0x43c => imu_mode,
        // 0x440 => vexDeviceImuCollisionDataGet,
        0x444 => set_imu_data_rate,
        0x4d8 => range_value,
        // 0x460 => vexDeviceRadioUserDataReceive,
        // 0x464 => vexDeviceRadioModeSet,
        0x488 => abs_enc_reset,
        0x48c => set_abs_enc_position,
        0x490 => abs_enc_position,
        0x494 => abs_enc_velocity,
        0x498 => abs_enc_angle,
        0x49c => set_abs_enc_reversed,
        0x4a0 => abs_enc_reversed,
        0x4a4 => abs_enc_status,
        // 0x4a8 => vexDeviceAbsEncTemperatureGet,
        // 0x4ac => vexDeviceAbsEncDebugGet,
        // 0x4b0 => vexDeviceAbsEncModeSet,
        // 0x4b4 => vexDeviceAbsEncModeGet,
        // 0x4b8 => vexDeviceAbsEncOffsetSet,
        // 0x4bc => vexDeviceAbsEncOffsetGet,
        0x4c0 => set_abs_enc_data_rate,
        0x500 => distance_distance,
        0x504 => distance_confidence,
        0x508 => distance_status,
        // 0x50c => vexDeviceDistanceDebugGet,
        // 0x510 => vexDeviceDistanceModeSet,
        // 0x514 => vexDeviceDistanceModeGet,
        0x518 => distance_object_size,
        0x51c => distance_object_velocity,
        0x528 => optical_hue,
        0x52c => optical_sat,
        0x530 => optical_brightness,
        0x534 => optical_proximity,
        0x538 => optical_rgb,
        0x53c => set_optical_led_pwm,
        0x540 => optical_led_pwm,
        0x544 => optical_status,
        0x548 => optical_raw,
        // 0x54c => vexDeviceOpticalDebugGet,
        0x550 => set_optical_mode,
        0x554 => optical_mode,
        0x558 => optical_gesture,
        0x55c => optical_gesture_enable,
        0x560 => optical_gesture_disable,
        0x564 => optical_proximity_threshold,
        // 0x568 => vexDeviceOpticalGainSet,
        // 0x56c => vexDeviceOpticalMatrixSet,
        // 0x570 => vexDeviceOpticalMatrixGet,
        0x57c => magnet_power,
        0x580 => magnet_pickup,
        0x584 => magnet_drop,
        0x588 => magnet_temperature,
        0x58c => magnet_current,
        0x590 => magnet_status,
        // 0x594 => vexDeviceMagnetDebugGet,
        // 0x598 => vexDeviceMagnetModeSet,
        // 0x59c => vexDeviceMagnetModeGet,
        0x5c8 => gps_reset,
        0x5cc => gps_heading,
        0x5d0 => gps_degrees,
        0x5d4 => gps_quaternion,
        0x5d8 => gps_attitude,
        0x5dc => gps_raw_gyro,
        0x5e0 => gps_raw_accel,
        0x5e4 => gps_status,
        // 0x5e8 => vexDeviceGpsTemperatureGet,
        // 0x5ec => vexDeviceGpsDebugGet,
        0x5f0 => set_gps_mode,
        0x5f4 => gps_mode,
        0x5f8 => set_gps_data_rate,
        0x5fc => set_gps_origin,
        0x600 => gps_origin,
        0x604 => set_gps_rotation,
        0x608 => gps_rotation,
        0x60c => set_gps_initial_position,
        0x610 => gps_error,
        0x640 => set_display_foreground_color,
        0x644 => set_display_background_color,
        0x648 => display_erase,
        0x64c => display_scroll,
        0x650 => display_scroll_rect,
        0x654 => display_copy_rect,
        0x658 => display_set_pixel,
        0x65c => display_clear_pixel,
        0x660 => display_draw_line,
        0x664 => display_clear_line,
        0x668 => display_draw_rect,
        0x66c => display_clear_rect,
        0x670 => display_fill_rect,
        0x674 => display_draw_circle,
        0x678 => display_clear_circle,
        0x67c => display_fill_circle,
        0x680 => display_printf,
        0x684 => display_string,
        0x688 => display_string_at,
        0x68c => display_big_string,
        0x690 => display_big_string_at,
        0x694 => display_centered_string,
        0x698 => display_big_centered_string,
        // 0x69c => display_text_smoothing,
        // 0x6a0 => display_text_reference,
        // 0x6a4 => display_screen_grab,
        0x6a8 => display_set_text_size,
        // 0x6ac => display_text_spacing,
        0x6b0 => display_small_string_at,
        0x6b4 => display_set_font_named,
        0x6b8 => display_foreground_color,
        0x6bc => display_background_color,
        0x6c0 => display_string_width_get,
        0x6c4 => display_string_height_get,
        // 0x6c8 => set_display_pen_size,
        // 0x6cc => display_pen_size,
        // 0x6d0 => display_font_custom_set,
        0x7a0 => render_display,
        0x7a4 => disable_display_double_buffer,
        0x794 => display_set_clip_region,
        0xb40 => set_optical_integration_time,
        0xb44 => optical_integration_time,
        0x898 => serial_write_char,
        0x89c => serial_write_buffer,
        0x8a0 => serial_read_char,
        0x8a4 => serial_peek_char,
        // 0x8a8 => serial_enable_remote_console,
        0x8ac => serial_write_free,
        0x8c0 => system_timer_stop,
        0x8c4 => system_timer_clear_interrupt,
        0x8d0 => system_watchdog_reinit_rtos,
        0x8d4 => system_watchdog_get,
        0x990 => read_bmp_image,
        0x994 => read_png_image,
        //TODO: usd functions
    });
    table
};

pub unsafe extern "C" fn unshimmed_syscall() -> ! {
    loop {}
}

pub unsafe extern "C" fn stdlib_mismatch_error(param_1: u32, param_2: u32) {}

pub unsafe extern "C" fn private_api_disable() {}

pub unsafe extern "C" fn vexos_background_processing() {}

pub unsafe extern "C" fn vexos_debug(fmt: *const u8, ...) {}

pub unsafe extern "C" fn system_time() -> u32 {
    SYSTEM_TIME.load(Ordering::Acquire)
}

pub unsafe extern "C" fn time(time: *mut Time) {
    unsafe {
        *time = Time {
            hour: 0,
            minute: 0,
            second: 0,
            hundredths: 0,
        };
    }
}

pub unsafe extern "C" fn date(date: *mut Date) {
    unsafe {
        *date = Date {
            year: 0,
            day: 0,
            month: 0,
        };
    }
}

pub unsafe extern "C" fn system_memory_dump() {}

pub unsafe extern "C" fn request_exit() -> ! {
    loop {}
}

pub unsafe extern "C" fn high_res_time() -> u64 {
    0
}
pub unsafe extern "C" fn system_powerup_time() -> u64 {
    0
}

pub unsafe extern "C" fn cold_link_addr() -> u32 {
    unsafe { crate::COLD_MEMORY_START as _ }
}

pub unsafe extern "C" fn system_timer(param_1: u32) -> u32 {
    0
}
pub unsafe extern "C" fn enable_system_timer(param_1: u32) -> u32 {
    0
}
pub unsafe extern "C" fn disable_system_timer(param_1: u32) {}

pub unsafe extern "C" fn system_watchdog_reinit_rtos() -> i32 {
    let wdt = unsafe { WATCHDOG_TIMER.get_mut().unwrap() };

    if wdt.is_started() {
        return 1;
    }
    
    let mut control_reg = wdt.control_register();
    control_reg |= 0xff << 0x08;
    wdt.set_control_register(control_reg);

    wdt.load(core::ffi::c_uint::MAX);
    wdt.set_timer_mode();
    wdt.start();

    0
}

/// Handles an IRQ.
pub unsafe extern "C" fn system_application_irq_handler(icciar: u32) -> u32 {
    let gic = unsafe { INTERRUPT_CONTROLLER.get_mut().unwrap() };

    // Re-enable interrupts.
    unsafe {
        asm!("cpsie i");
    }

    // The ID of the interrupt is obtained by bitwise anding the ICCIAR value
    let interrupt_id = icciar & 0x3FF;

    // Check for a valid interrupt ID.
	if interrupt_id < GenericInterruptController::MAX_INTERRUPT_INPUTS {
		// Call respective interrupt handler from the vector table.
        let handler_entry = unsafe {
            gic.handler_table[interrupt_id as usize].unwrap()
        };

        unsafe {
            (handler_entry.handler)(handler_entry.callback_ref);
        }

        return 0;
	}

    1
}

pub unsafe extern "C" fn system_watchdog_get() -> u32 {
    0
}

pub unsafe extern "C" fn usb_status() -> u32 {
    1
}

pub unsafe extern "C" fn get_num_devices() -> u32 {
    0
}
pub unsafe extern "C" fn get_num_devices_by_type(device_type: V5DeviceType) -> u32 {
    0
}
pub unsafe extern "C" fn devices() -> V5Device {
    V5Device {
        port: 0,
        exists: true,
        device_type: V5DeviceType::UndefinedSensor,
        timestamp: 0,
        device_specific_data: DeviceData { vision: () },
    }
}
pub unsafe extern "C" fn device_by_index(index: u32) -> V5Device {
    V5Device {
        port: 0,
        exists: true,
        device_type: V5DeviceType::UndefinedSensor,
        timestamp: 0,
        device_specific_data: DeviceData { vision: () },
    }
}

pub unsafe extern "C" fn device_status(devices: *const V5DeviceType) -> i32 {
    0
}

pub unsafe extern "C" fn controller_channel(id: ControllerID, channel: ControllerChannel) -> i32 {
    0
}
pub unsafe extern "C" fn controller_connection_status(id: ControllerID) -> i32 {
    1
}
pub unsafe extern "C" fn set_controller_text(id: u32, line: u32, col: u32, buf: *const u8) -> u32 {
    1
}

pub unsafe extern "C" fn device_timestamp(device: V5DeviceHandle) -> u32 {
    0
}

pub unsafe extern "C" fn button_state() -> u32 {
    0
}

pub unsafe extern "C" fn led_set(device: V5DeviceHandle, color: u32) {}

pub unsafe extern "C" fn led(device: V5DeviceHandle) -> u32 {
    0
}
pub unsafe extern "C" fn led_rgb(device: V5DeviceHandle) -> u32 {
    0
}

pub unsafe extern "C" fn adi_port_config_set(
    device: V5DeviceHandle,
    port: u32,
    config: AdiPortConfiguration,
) -> u32 {
    0
}
pub unsafe extern "C" fn adi_port_config(
    device: V5DeviceHandle,
    port: u32,
) -> AdiPortConfiguration {
    AdiPortConfiguration::AnalogIn
}

pub unsafe extern "C" fn adi_value_set(device: V5DeviceHandle, port: u32, value: i32) {}
pub unsafe extern "C" fn adi_value(device: V5DeviceHandle, port: u32) -> i32 {
    0
}
pub unsafe extern "C" fn adi_voltage(device: V5DeviceHandle, port: u32) -> i32 {
    0
}

pub unsafe extern "C" fn adi_addr_led_set(
    device: V5DeviceHandle,
    port: u32,
    pixel_data: *const u32,
    offset: u32,
    len: u32,
    opts: u32,
) -> i32 {
    0
}

pub unsafe extern "C" fn gyro_heading(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn gyro_degrees(device: V5DeviceHandle) -> f64 {
    0.0
}

pub unsafe extern "C" fn sonar_value(device: V5DeviceHandle) -> i32 {
    0
}

pub unsafe extern "C" fn generic_smart_value(device: V5DeviceHandle) -> f64 {
    0.0
}

pub unsafe extern "C" fn range_value(device: V5DeviceHandle) -> i32 {
    0
}

pub unsafe extern "C" fn system_timer_stop() {}
pub unsafe extern "C" fn system_timer_clear_interrupt() {}

/// Reinitializes the timer interrupt with a given tick handler and priority for the private timer instance.
pub unsafe extern "C" fn reinit_system_timer_for_rtos(
    priority: u32,
    handler: unsafe extern "C" fn(*mut c_void),
) -> i32 {
    let gic = unsafe { INTERRUPT_CONTROLLER.get_mut().unwrap() };
    // Set tick interrupt priority
	// PROS sets this to the lowest possible priority (portLOWEST_USABLE_INTERRUPT_PRIORITY << portPRIORITY_SHIFT).
    gic.set_priority_trigger_type(Timer::IRQ_INTERRUPT_ID, priority as u8, 3);
    gic.connect(Timer::IRQ_INTERRUPT_ID, handler, core::ptr::null_mut());
    0
}
