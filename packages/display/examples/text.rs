use std::time::Instant;

use vex_v5_display_simulator::{Display, TextOptions, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND};
use vex_v5_qemu_protocol::geometry::Point2;

pub fn main() {
    let mut display = Display::new(DEFAULT_FOREGROUND, DEFAULT_BACKGROUND, Instant::now());

    display.write_text(
        "Hello, world!".to_string(),
        Point2 { x: 50, y: 50 },
        false,
        TextOptions::default(),
    );

    display.render(false);
    display.canvas.show();
}
