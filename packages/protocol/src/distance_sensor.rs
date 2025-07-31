#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DistanceSensorData {
    pub object: Option<DistanceObject>,
    pub status: u32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DistanceObject {
    pub distance: u32,
    pub relative_size: u32,
    pub velocity: f64,
    pub confidence: u32,
}
