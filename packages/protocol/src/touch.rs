use bincode::{Decode, Encode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::geometry::Point2;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TouchData {
    pub point: Point2<i16>,
    pub event: TouchEvent,
}

impl Default for TouchData {
    fn default() -> Self {
        TouchData {
            event: TouchEvent::Release,
            point: Point2 { x: 0, y: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TouchEvent {
    Press,
    Release,
}
