//! USB Serial Communication

use core::ffi::{c_char, VaList};

use semihosting::io::Write;
use snafu::{OptionExt, ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum FlushError {
    /// Stdio is not supported on this CPU architecture.
    StdioNotSupported,
    /// Failed to write to stdio.
    #[snafu(display("Failed to write to stdio: {inner}"))]
    Write { inner: semihosting::io::Error },
}

fn flush_stdout_buffer(buffer: &[u8]) -> Result<usize, FlushError> {
    let mut stdout = semihosting::io::stdout()
        .ok()
        .context(StdioNotSupportedSnafu)?;
    stdout
        .write(buffer)
        .map_err(|inner| WriteSnafu { inner }.build())
}

pub const INTERNAL_STDOUT_BUFFER_SIZE: usize = 2048;

pub fn vexSerialWriteChar(channel: u32, c: u8) -> i32 {
    if channel == 1 {
        match flush_stdout_buffer(&[c]) {
            Ok(_) => 1,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

/// # Safety
///
/// - `data` must be a valid pointer to a buffer of length `data_len`.
pub unsafe fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32 {
    let data = unsafe { core::slice::from_raw_parts(data, data_len as usize) };
    if channel == 1 {
        match flush_stdout_buffer(data) {
            Ok(bytes_written) => bytes_written as i32,
            Err(_) => -1,
        }
    } else {
        -1
    }
}
pub fn vexSerialReadChar(channel: u32) -> i32 {
    Default::default()
}
pub fn vexSerialPeekChar(channel: u32) -> i32 {
    Default::default()
}
pub fn vexSerialWriteFree(channel: u32) -> i32 {
    if channel == 1 {
        INTERNAL_STDOUT_BUFFER_SIZE as i32
    } else {
        -1
    }
}
pub fn vex_vprintf(format: *const c_char, args: VaList) -> i32 {
    Default::default()
}
pub fn vex_vsprintf(out: *mut c_char, format: *const c_char, args: VaList) -> i32 {
    Default::default()
}
pub fn vex_vsnprintf(out: *mut c_char, max_len: u32, format: *const c_char, args: VaList) -> i32 {
    Default::default()
}
