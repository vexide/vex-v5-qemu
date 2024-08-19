use core::num::NonZeroU16;

use alloc::{string::String, vec::Vec};
use bincode::{Decode, Encode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::geometry::{Point2, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Encode, Decode, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color(pub u32);

/// An instruction for drawing to the robot LCD screen.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DrawCommand {
    Fill(Shape),
    Stroke(Shape),
    CopyBuffer {
        top_left: Point2<i32>,
        bottom_right: Point2<i32>,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode, Decode, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextLocation {
    Coordinates(Point2<i32>),
    Line(i32),
}

impl Default for TextLocation {
    fn default() -> Self {
        Self::Coordinates(Point2 { x: 0, y: 0 })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode, Decode, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScrollLocation {
    Rect(Rect),
    Line(i32),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Encode, Decode, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextSize {
    Small,
    #[default]
    Normal,
    Large,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Encode, Decode, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DisplayRenderMode {
    #[default]
    Immediate,
    DoubleBuffered,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Encode, Decode, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextMeasurement {
    pub width: i32,
    pub height: i32,
}

/// A shape that can be drawn to the robot LCD screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Shape {
    Rectangle {
        top_left: Point2<i32>,
        bottom_right: Point2<i32>,
    },
    Circle {
        center: Point2<i32>,
        radius: u16,
    },
    Line {
        start: Point2<i32>,
        end: Point2<i32>,
    },
}
