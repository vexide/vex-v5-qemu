pub unsafe extern "C" fn serial_write_char(channel: u32, c: u8) -> i32 {
    0
}
pub unsafe extern "C" fn serial_write_buffer(channel: u32, data: *const u8, data_len: u32) -> i32 {
    0
}
pub unsafe extern "C" fn serial_read_char(channel: u32) -> i32 {
    0
}
pub unsafe extern "C" fn serial_peek_char(channel: u32) -> i32 {
    0
}
pub unsafe extern "C" fn serial_write_free(channel: u32) -> i32 {
    0
}
