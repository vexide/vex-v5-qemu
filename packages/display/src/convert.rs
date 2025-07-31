use tiny_skia::{Color, ColorU8, Path, PathBuilder, Rect};
use vex_v5_qemu_protocol::display::Shape;

pub trait ToSkia {
    type Skia;

    fn to_skia(self) -> Self::Skia;
}

pub fn color_components(c: vex_v5_qemu_protocol::display::Color) -> (u8, u8, u8, u8) {
    let r = ((c.0 >> 24) & 0xFF) as u8;
    let g = ((c.0 >> 16) & 0xFF) as u8;
    let b = ((c.0 >> 8) & 0xFF) as u8;
    let a = (c.0 & 0xFF) as u8;

    (r, g, b, a)
}

/// `const` implementation of [`Color::from_rgba8`] because for some reason that's not a thing yet.
// TODO: swap this for [`Color::from_rgba_unchecked`] once
// https://github.com/linebender/tiny-skia/pull/135 is published.
pub const fn const_color_from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Option<Color> {
    // thin implementation of `NormalizedF32::new(n)` from the strict-num crate.
    const fn validate_component_range(n: f32) -> Option<f32> {
        if n.is_finite() && n >= 0.0 && n <= 1.0 {
            Some(n)
        } else {
            None
        }
    }

    struct _Color {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    }

    // SAFETY: _Color is identical in layout and size to Color
    unsafe {
        // NOTE: expr? in const fn is not stable.
        Some(std::mem::transmute(_Color {
            r: if let Some(c) = validate_component_range(r as f32 / 255.0) { c } else { return None },
            g: if let Some(c) = validate_component_range(g as f32 / 255.0) { c } else { return None },
            b: if let Some(c) = validate_component_range(b as f32 / 255.0) { c } else { return None },
            a: if let Some(c) = validate_component_range(a as f32 / 255.0) { c } else { return None },
        }))
    }
}

impl ToSkia for vex_v5_qemu_protocol::display::Color {
    type Skia = ColorU8;

    fn to_skia(self) -> Self::Skia {
        let (r, g, b, a) = color_components(self);
        ColorU8::from_rgba(r, g, b, a)
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
                    bottom_right.x as _,
                    bottom_right.y as _,
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
