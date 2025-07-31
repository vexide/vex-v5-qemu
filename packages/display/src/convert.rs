use tiny_skia::{Color, Path, PathBuilder, Rect};

pub trait ToSkia {
    type Skia;

    fn to_skia(self) -> Self::Skia;
}

impl ToSkia for vex_v5_qemu_protocol::display::Color {
    type Skia = Color;

    fn to_skia(self) -> Color {
        let [r, g, b, a] = self.components;
        Color::from_rgba(r, g, b, a).expect("components out of range")
    }
}

impl ToSkia for kurbo::Rect {
    type Skia = Rect;

    fn to_skia(self) -> Self::Skia {
        Rect::from_ltrb(
            self.x0 as f32,
            self.y0 as f32,
            self.x1 as f32,
            self.y1 as f32,
        )
        .unwrap()
    }
}

impl ToSkia for kurbo::Circle {
    type Skia = Path;

    fn to_skia(self) -> Self::Skia {
        PathBuilder::from_circle(
            self.center.x as f32,
            self.center.y as f32,
            self.radius as f32,
        )
        .unwrap()
    }
}

impl ToSkia for kurbo::Line {
    type Skia = Path;

    fn to_skia(self) -> Self::Skia {
        let mut builder = PathBuilder::new();
        builder.move_to(self.p0.x as f32, self.p0.y as f32);
        builder.line_to(self.p1.x as f32, self.p1.y as f32);
        builder.finish().unwrap()
    }
}
