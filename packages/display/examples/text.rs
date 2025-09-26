use vex_v5_display_simulator::{ColorTheme, DisplayRenderer, TextOptions};
use vex_v5_qemu_protocol::geometry::Point2;

pub fn main() {
    let mut display = DisplayRenderer::new(ColorTheme::Dark);

    display.draw_header();

    display.draw_text(
        "Hello, world!".to_string(),
        Point2 { x: 50, y: 50 },
        false,
        TextOptions::default(),
    );

    let pix = display.render(false).unwrap();
    pix.save_png("result.png").unwrap();
}
