#![rust_analyzer::skip]

use image::DynamicImage;
use tauri::{Emitter, Manager};
use tauri_plugin_log::TimezoneStrategy;
use tokio::sync::Mutex;
use vex_v5_qemu_host::brain::Brain;

pub mod protocol;
pub mod qemu;

pub struct AppState {
    pub brain: Brain,
}

const ESCAPES: [Option<&str>; 6] = [
    None,             // Default foreground
    Some("\x1B[31m"), // Error (red)
    Some("\x1B[33m"), // Warn (yellow)
    Some("\x1B[34m"), // Info (blue)
    Some("\x1B[36m"), // Debug (cyan)
    Some("\x1B[37m"), // Trace (white)
];

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .format(|out, message, record| {
                    let time_format =
                        time::format_description::parse("[hour]:[minute]:[second]").unwrap();

                    out.finish(format_args!(
                        "{} {}[{}]\x1B[0m {}: {}",
                        TimezoneStrategy::UseUtc
                            .get_now()
                            .format(&time_format)
                            .unwrap(),
                        ESCAPES[record.level() as usize].unwrap_or_default(),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let mut brain = Brain::new();
            let peripherals = brain.peripherals.take().unwrap();

            app.manage(Mutex::new(AppState { brain }));

            let app_handle = app.handle().to_owned();
            tauri::async_runtime::spawn(async move {
                let mut usb = peripherals.usb;
                let mut display = peripherals.display;
                loop {
                    tokio::select! {
                        Some(data) = usb.recv() => {
                            app_handle.emit("brain_usb_recv", data).unwrap();
                        },
                        Some(frame) = display.next_frame() => {
                            app_handle.emit("brain_display_frame", DynamicImage::ImageRgb8(frame).to_rgba8().to_vec()).unwrap();
                        }
                        else => break,
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![qemu::spawn_qemu, qemu::kill_qemu])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
