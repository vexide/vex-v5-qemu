use std::time::Instant;

use vex_v5_display_simulator::{Display, DEFAULT_BACKGROUND, DEFAULT_FOREGROUND};
use vex_v5_qemu_protocol::{display::Shape, geometry::Point2};

pub fn main() {
    let mut display = Display::new(DEFAULT_FOREGROUND, DEFAULT_BACKGROUND, Instant::now());

    display.draw(
        Shape::Rectangle {
            top_left: Point2 { x: 50, y: 50 },
            bottom_right: Point2 { x: 150, y: 150 },
        },
        false,
    );

    display.foreground_color = [0, 0, 255];

    display.draw(
        Shape::Rectangle {
            top_left: Point2 { x: 75, y: 75 },
            bottom_right: Point2 { x: 175, y: 175 },
        },
        false,
    );

    display.draw(
        Shape::Circle {
            center: Point2 { x: 100, y: 100 },
            radius: 50,
        },
        true,
    );

    display.render(false);
    display.canvas.show();
}
