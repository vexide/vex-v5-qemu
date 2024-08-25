use bincode::{Decode, Encode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BatteryData {
    pub voltage: i32,
    pub current: i32,
    pub temperature: f64,
    pub capacity: f64,
}
