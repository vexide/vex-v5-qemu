use tiny_skia::{Color, Path, PathBuilder, Rect};
use vex_v5_qemu_protocol::display::Shape;

pub trait ToSkia {
    type Skia;

    fn to_skia(self) -> Self::Skia;
}

const fn color_components(c: vex_v5_qemu_protocol::display::Color) -> (u8, u8, u8) {
    let r = ((c.0 >> 16) & 0xFF) as u8;
    let g = ((c.0 >> 8) & 0xFF) as u8;
    let b = (c.0 & 0xFF) as u8;

    (r, g, b)
}

impl ToSkia for vex_v5_qemu_protocol::display::Color {
    type Skia = Color;

    fn to_skia(self) -> Self::Skia {
        let (r, g, b) = color_components(self);
        Color::from_rgba8(r, g, b, 0xFF)
    }
}

impl ToSkia for Shape {
    type Skia = Path;

    fn to_skia(self) -> Self::Skia {
        match self {
            Shape::Rectangle {
                top_left,
                bottom_right,
            } => PathBuilder::from_rect(
                Rect::from_ltrb(
                    top_left.x as _,
                    top_left.y as _,
                    // ensure that rects of width/height 0 are drawn as hairline borders
                    // TODO: test if this applies to circles with r=0
                    (top_left.x + (bottom_right.x - top_left.x).max(1)) as _,
                    (top_left.y + (bottom_right.y - top_left.y).max(1)) as _,
                )
                .unwrap(),
            ),
            Shape::Circle { center, radius } => {
                PathBuilder::from_circle(center.x as _, center.y as _, radius as _).unwrap()
            }
            Shape::Line { start, end } => {
                let mut builder = PathBuilder::new();
                builder.move_to(start.x as f32, start.y as f32);
                builder.line_to(end.x as f32, end.y as f32);
                builder.finish().unwrap()
            }
        }
    }
}
