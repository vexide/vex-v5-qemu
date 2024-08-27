use bincode::{Decode, Encode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DistanceSensorData {
    pub object: Option<DistanceSensorObject>,
    pub status: u32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DistanceSensorObject {
    pub distance: u32,
    pub size: u32,
    pub velocity: f64,
    pub confidence: u32,
}
