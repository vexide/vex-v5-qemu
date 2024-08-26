use std::time::Instant;

use vex_v5_display_simulator::{Display, TextOptions, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND};

pub fn main() {
    let mut display = Display::new(DEFAULT_FOREGROUND, DEFAULT_BACKGROUND, Instant::now());

    display.write_text(
        "Hello, world!".to_string(),
        (50, 50),
        false,
        TextOptions::default(),
    );

    display.render(false);
    display.canvas.show();
}
