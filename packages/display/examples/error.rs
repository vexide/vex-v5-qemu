use vex_v5_display_simulator::{ColorTheme, DisplayRenderer, TextOptions};
use vex_v5_qemu_protocol::{
    display::{Color, Shape},
    geometry::Point2,
};

pub fn main() {
    let mut display = DisplayRenderer::new(ColorTheme::Dark);
    display.draw_header();

    display.context.foreground_color = Color(0x8B0000);

    let container = Shape::Rectangle {
        top_left: Point2 { x: 50, y: 50 },
        bottom_right: Point2 {
            x: 340,
            y: 120,
        },
    };
    display.draw(container, false);

    display.context.foreground_color = Color(0xFFFFFF);
    display.draw(container, true);

    display.write_text(
        "Memory Permission error !".to_string(),
        Point2 {
            x: 80,
            y: 70,
        },
        true,
        TextOptions::default(),
    );

    display.write_text(
        "03800128".to_string(),
        Point2 {
            x: 80,
            y: 90,
        },
        true,
        TextOptions::default(),
    );

    display.context.foreground_color = Color(0x44FF44);
    let container = Shape::Rectangle {
        top_left: Point2 {
            x: 80,
            y: 70,
        },
        bottom_right: Point2 {
            x: 81,
            y: 71,
        },
    };
    display.draw(container, false);

    let pix = display.render(false).unwrap();
    pix.save_png("result.png").unwrap();
}
