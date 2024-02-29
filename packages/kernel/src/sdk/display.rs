use super::Image;

pub unsafe extern "C" fn set_display_foreground_color(color: u32) {}
pub unsafe extern "C" fn set_display_background_color(color: u32) {}
pub unsafe extern "C" fn display_foreground_color() -> u32 {
    0
}
pub unsafe extern "C" fn display_background_color() -> u32 {
    0
}

pub unsafe extern "C" fn display_erase() {}
pub unsafe extern "C" fn display_scroll(start_line: i32, num_lines: i32) {}
pub unsafe extern "C" fn display_scroll_rect(
    top_left_x: i32,
    top_left_y: i32,
    bottom_right_x: i32,
    bottom_right_y: i32,
    nLines: i32,
) {
}
pub unsafe extern "C" fn display_copy_rect(
    top_left_x: i32,
    top_left_y: i32,
    bottom_right_x: i32,
    bottom_right_y: i32,
    pixel_buffer: *const i32,
    stride: i32,
) {
}
pub unsafe extern "C" fn display_set_pixel(x: u32, y: u32) {}
pub unsafe extern "C" fn display_clear_pixel(x: u32, y: u32) {}
pub unsafe extern "C" fn display_draw_line(
    top_left_x: i32,
    top_left_y: i32,
    bottom_right_x: i32,
    bottom_right_y: i32,
) {
}
pub unsafe extern "C" fn display_clear_line(
    top_left_x: i32,
    top_left_y: i32,
    bottom_right_x: i32,
    bottom_right_y: i32,
) {
}
pub unsafe extern "C" fn display_draw_rect(
    top_left_x: i32,
    top_left_y: i32,
    bottom_right_x: i32,
    bottom_right_y: i32,
) {
}
pub unsafe extern "C" fn display_clear_rect(
    top_left_x: i32,
    top_left_y: i32,
    bottom_right_x: i32,
    bottom_right_y: i32,
) {
}
pub unsafe extern "C" fn display_fill_rect(
    top_left_x: i32,
    top_left_y: i32,
    bottom_right_x: i32,
    bottom_right_y: i32,
) {
}
pub unsafe extern "C" fn display_draw_circle(center_x: i32, center_y: i32, radius: i32) {}
pub unsafe extern "C" fn display_clear_circle(center_x: i32, center_y: i32, radius: i32) {}
pub unsafe extern "C" fn display_fill_circle(center_x: i32, center_y: i32, radius: i32) {}

pub unsafe extern "C" fn display_printf(
    x_pos: i32,
    y_pos: i32,
    b_opaque: u32,
    format: *const u8,
    ...
) {
}
pub unsafe extern "C" fn display_string(line_number: i32, format: *const u8, ...) {}
pub unsafe extern "C" fn display_string_at(x_pos: i32, y_pos: i32, format: *const u8, ...) {}
pub unsafe extern "C" fn display_big_string(line_number: i32, format: *const u8, ...) {}
pub unsafe extern "C" fn display_big_string_at(x_pos: i32, y_pos: i32, format: *const u8, ...) {}
pub unsafe extern "C" fn display_small_string_at(x_pos: i32, y_pos: i32, format: *const u8, ...) {}
pub unsafe extern "C" fn display_centered_string(line_number: i32, format: *const u8, ...) {}
pub unsafe extern "C" fn display_big_centered_string(line_number: i32, format: *const u8, ...) {}

pub unsafe extern "C" fn display_set_text_size(n: u32, d: u32) {}
pub unsafe extern "C" fn display_set_font_named(font_name: *const u8) {}
pub unsafe extern "C" fn display_string_width_get(string: *const u8) -> i32 {
    0
}
pub unsafe extern "C" fn display_string_height_get(string: *const u8) -> i32 {
    0
}

pub unsafe extern "C" fn render_display(wait_for_vsync: bool, run_scheduler: bool) -> bool {
    false
}
pub unsafe extern "C" fn disable_display_double_buffer() {}

pub unsafe extern "C" fn display_set_clip_region(
    top_left_x: i32,
    top_left_y: i32,
    bottom_right_x: i32,
    bottom_right_y: i32,
) {
}

pub unsafe extern "C" fn read_bmp_image(
    image_buffer: *const u8,
    out_buffer: *mut Image,
    maxw: u32,
    maxh: u32,
) -> i32 {
    0
}
pub unsafe extern "C" fn read_png_image(
    image_buffer: *const u8,
    image: *mut Image,
    maxw: u32,
    maxh: u32,
    ibuflen: u32,
) -> i32 {
    0
}
