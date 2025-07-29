//! USB Serial Communication

use alloc::string::String;
use core::ffi::{c_char, VaList};

use embedded_io::{ErrorKind, ErrorType, Write};
use embedded_io_extras::Cursor;
use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use vex_v5_qemu_protocol::HostBoundPacket;

use crate::{protocol, sync::Mutex};

pub struct SerialChannel<const TX: usize, const RX: usize> {
    pub tx: Cursor<[u8; TX]>,
    pub rx: ConstGenericRingBuffer<u8, RX>,
}

impl<const TX: usize, const RX: usize> SerialChannel<TX, RX> {
    const fn new() -> Self {
        Self {
            tx: Cursor::new([0; TX]),
            rx: ConstGenericRingBuffer::new(),
        }
    }

    pub fn write_free(&self) -> usize {
        TX - self.tx.position() as usize
    }

    pub const fn read_capacity(&self) -> usize {
        RX
    }

    pub const fn write_capacity(&self) -> usize {
        TX
    }
}

impl<const TX: usize, const RX: usize> ErrorType for SerialChannel<TX, RX> {
    type Error = ErrorKind;
}

impl<const TX: usize, const RX: usize> Write for SerialChannel<TX, RX> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.tx.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        let pos = self.tx.position() as usize;

        if pos != 0 {
            protocol::send_packet(HostBoundPacket::UsbSerial(
                self.tx.get_ref()[0..pos].to_vec(),
            ))
            .unwrap();
        }

        self.tx.set_position(0);

        Ok(())
    }
}

pub static USB1: Mutex<SerialChannel<2048, 4096>> = Mutex::new(SerialChannel::new());

pub extern "C" fn vexSerialWriteChar(channel: u32, c: u8) -> i32 {
    match channel {
        1 => USB1.lock().write(&[c]).unwrap() as _,
        _ => 0,
    }
}

/// # Safety
///
/// - `data` must be a valid pointer to a buffer of length `data_len`.
pub unsafe fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32 {
    if data.is_null() || data_len == 0 {
        return 0;
    }

    match channel {
        1 => USB1
            .lock()
            .write(unsafe { core::slice::from_raw_parts(data, data_len as _) })
            .unwrap() as _,
        _ => 0,
    }
}

pub extern "C" fn vexSerialReadChar(channel: u32) -> i32 {
    match channel {
        1 => USB1.lock().rx.dequeue().map(|val| val as i32).unwrap_or(-1),
        _ => -1,
    }
}
pub extern "C" fn vexSerialPeekChar(channel: u32) -> i32 {
    match channel {
        1 => USB1
            .lock()
            .rx
            .peek()
            .copied()
            .map(|val| val as i32)
            .unwrap_or(-1),
        _ => -1,
    }
}
pub extern "C" fn vexSerialWriteFree(channel: u32) -> i32 {
    match channel {
        1 => USB1.lock().write_free() as _,
        _ => 0,
    }
}

/// # Safety
///
/// [`VaList`]s are *very* unsafe. The passed `format` and `args` parameter must be a valid [`printf` format string](http://www.cplusplus.com/reference/cstdio/printf/).
pub unsafe extern "C" fn vex_printf(format: *const c_char, args: VaList<'_, '_>) -> i32 {
    let mut buffer = String::new();

    let result = unsafe {
        printf_compat::format(format, args, printf_compat::output::fmt_write(&mut buffer))
    };

    protocol::send_packet(HostBoundPacket::KernelSerial(buffer.as_bytes().to_vec())).unwrap();

    result
}
/// # Safety
///
/// [`VaList`]s are *very* unsafe. The passed `format` and `args` parameter must be a valid [`printf` format string](http://www.cplusplus.com/reference/cstdio/printf/).
pub unsafe extern "C" fn vex_sprintf(
    out: *mut c_char,
    format: *const c_char,
    mut args: VaList<'_, '_>,
) -> i32 {
    unsafe {
        let mut buf = String::new();
        let num_bytes = printf_compat::format(
            format,
            args.as_va_list(),
            printf_compat::output::fmt_write(&mut buf),
        );
        core::ptr::copy_nonoverlapping(buf.as_ptr(), out.cast(), num_bytes as usize);
        num_bytes
    }
}
/// # Safety
///
/// [`VaList`]s are *very* unsafe. The passed `format` and `args` parameter must be a valid [`printf` format string](http://www.cplusplus.com/reference/cstdio/printf/).
pub unsafe extern "C" fn vex_snprintf(
    out: *mut c_char,
    max_len: u32,
    format: *const c_char,
    mut args: VaList<'_, '_>,
) -> i32 {
    let mut buf = String::new();
    let num_bytes = unsafe {
        printf_compat::format(
            format,
            args.as_va_list(),
            printf_compat::output::fmt_write(&mut buf),
        )
    };

    if num_bytes >= 0 {
        buf.push('\0');

        unsafe {
            core::ptr::copy_nonoverlapping(
                buf.as_ptr(),
                out.cast(),
                core::cmp::min(num_bytes as usize + 1, max_len as _),
            );
        }
    }

    num_bytes
}
