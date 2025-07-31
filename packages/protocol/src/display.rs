use alloc::{string::String, vec::Vec};
use derive_more::From;
use core::num::NonZeroU16;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type Color = color::AlphaColor<color::Srgb>;

/// An instruction for drawing to the robot LCD screen.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DrawCommand {
    Fill(Shape),
    Stroke(Shape),
    CopyBuffer {
        top_left: kurbo::Point,
        bottom_right: kurbo::Point,
        stride: NonZeroU16,
        buffer: Vec<u32>,
    },
    Text {
        data: String,
        size: TextSize,
        location: TextLocation,
        opaque: bool,
        background: Color,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextLocation {
    Coordinates(kurbo::Point),
    Line(i32),
}

impl Default for TextLocation {
    fn default() -> Self {
        Self::Coordinates(kurbo::Point::ZERO)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScrollLocation {
    Rect(kurbo::Rect),
    Line(i32),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextSize {
    Small,
    #[default]
    Normal,
    Large,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DisplayRenderMode {
    #[default]
    Immediate,
    DoubleBuffered,
}

/// A shape that can be drawn to the robot LCD screen.
#[derive(Debug, Clone, Copy, PartialEq, From)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Shape {
    Rectangle(kurbo::Rect),
    Circle(kurbo::Circle),
    Line(kurbo::Line),
}
