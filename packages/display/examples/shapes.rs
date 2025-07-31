use vex_v5_display_simulator::{ColorTheme, DisplayRenderer};
use vex_v5_qemu_protocol::{
    display::{Color, Shape},
    geometry::Point2,
};

pub fn main() {
    let mut display = DisplayRenderer::new(ColorTheme::Dark);
    display.draw_header();

    display.draw(
        Shape::Rectangle {
            top_left: Point2 { x: 50, y: 50 },
            bottom_right: Point2 { x: 150, y: 150 },
        },
        false,
    );

    display.context.foreground_color = Color(0x0000FF);
    display.draw(
        Shape::Rectangle {
            top_left: Point2 { x: 75, y: 75 },
            bottom_right: Point2 { x: 175, y: 175 },
        },
        false,
    );
    display.draw(
        Shape::Circle {
            center: Point2 { x: 50, y: 50 },
            radius: 50,
        },
        true,
    );

    let pix = display.render(false).unwrap();
    pix.save_png("result.png").unwrap();
}
