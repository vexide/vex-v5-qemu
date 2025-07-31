#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ControllerId {
    Master,
    Partner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ControllerData {
    pub axis_1: u8,
    pub axis_2: u8,
    pub axis_3: u8,
    pub axis_4: u8,
    pub axis_spare_1: u8,
    pub axis_spare_2: u8,
    pub button_a: bool,
    pub button_b: bool,
    pub button_x: bool,
    pub button_y: bool,
    pub button_up: bool,
    pub button_down: bool,
    pub button_left: bool,
    pub button_right: bool,
    pub button_l1: bool,
    pub button_l2: bool,
    pub button_r1: bool,
    pub button_r2: bool,
    pub button_all: bool,
    pub button_level: bool,
    pub button_sel: bool,
    pub battery_capacity: u8,
    pub flags: u32,
}
