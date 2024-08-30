use std::{borrow::BorrowMut, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::{
    process::Command,
    sync::Mutex,
};

use crate::AppState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QemuOptions {
    gdb: bool,
    kernel: PathBuf,
    qemu: PathBuf,
    binary: PathBuf,
    qemu_args: Vec<String>,
}

/// Kills the currently running QEMU subprocess.
#[tauri::command]
pub async fn kill_qemu(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    state
        .lock()
        .await
        .brain
        .kill_program()
        .await
        .map_err(|_| "Failed to kill QEMU process.".into())
}

/// Spawns a new QEMU subprocess.
#[tauri::command]
pub async fn spawn_qemu(
    state: State<'_, Mutex<AppState>>,
    opts: QemuOptions,
) -> Result<(), String> {
    let brain = &mut state.lock().await.brain;

    let mut cmd = Command::new(opts.qemu.clone());
    cmd.borrow_mut().args(opts.qemu_args);

    brain
        .run_program(cmd, opts.kernel, opts.binary)
        .await
        .map_err(|_| "Failed to start QEMU process.")?;

    Ok(())
}
