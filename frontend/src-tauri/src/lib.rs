use tauri::Manager;
use tauri_plugin_log::TimezoneStrategy;
use tauri_plugin_shell::process::CommandChild;
use tokio::sync::Mutex;

pub mod protocol;
pub mod qemu;

#[derive(Default)]
pub struct AppState {
    /// QEMU child process (if running).
    qemu_process: Option<CommandChild>,
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
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![qemu::spawn_qemu, qemu::kill_qemu])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
