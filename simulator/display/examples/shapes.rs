use std::time::Instant;

use vex_v5_display_simulator::{Display, Path, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND};

pub fn main() {
    let mut display = Display::new(DEFAULT_FOREGROUND, DEFAULT_BACKGROUND, Instant::now());

    display.draw(
        Path::Rect {
            x1: 50,
            y1: 50,
            x2: 150,
            y2: 150,
        },
        false,
    );

    display.foreground_color = [0, 0, 255];

    display.draw(
        Path::Rect {
            x1: 75,
            y1: 75,
            x2: 175,
            y2: 175,
        },
        false,
    );

    display.draw(
        Path::Circle {
            cx: 100,
            cy: 100,
            radius: 50,
        },
        true,
    );

    display.render(false);
    display.canvas.show();
}
