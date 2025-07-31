use std::time::Instant;

use vex_v5_display_simulator::{ColorTheme, DisplayRenderer, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND};
use vex_v5_qemu_protocol::{display::{Color, Shape}, Circle, Rect};

pub fn main() {
    let mut display = DisplayRenderer::new(ColorTheme::Dark);

    display.draw_header();

    display.draw(
        Rect::new(50.0, 50.0, 150.0, 150.0).into(),
        false,
    );

    display.context.foreground_color = Color::from_rgb8(0, 0, 255);

    display.draw(
        Rect::new(75.0, 75.0, 175.0, 175.0).into(),
        false,
    );

    display.draw(
        Circle::new((50.0, 50.0), 50.0).into(),
        true,
    );

    let pix = display.render(false).unwrap();
    pix.save_png("result.png").unwrap();
}
