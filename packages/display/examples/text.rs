use std::time::Instant;

use kurbo::Point;
use vex_v5_display_simulator::{
    ColorTheme, DisplayRenderer, TextOptions, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND
};

pub fn main() {
    let mut display = DisplayRenderer::new(ColorTheme::Dark);

    display.draw_header();

    display.write_text(
        "Hello, world!".to_string(),
        Point::new(50.0, 50.0),
        false,
        TextOptions::default(),
    );

    let pix = display.render(false).unwrap();
    pix.save_png("result.png").unwrap();
}
