use alloc::{string::String, vec::Vec};
use core::num::NonZeroU16;

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
        position: Point2<i32>,
        opaque: bool,
        background: Color,
    },
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
