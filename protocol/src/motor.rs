use bitflags::bitflags;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impl_bincode_bitflags;

bitflags! {
    /// The fault flags returned by a [`Motor`].
    #[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]

    pub struct MotorFaults: u32 {
        /// The motor's temperature is above its limit.
        const OVER_TEMPERATURE = 0x01;

        /// The motor is over current.
        const OVER_CURRENT = 0x04;

        /// The motor's H-bridge has encountered a fault.
        const DRIVER_FAULT = 0x02;

        /// The motor's H-bridge is over current.
        const DRIVER_OVER_CURRENT = 0x08;
    }
}
impl_bincode_bitflags!(MotorFaults);

bitflags! {
    /// The status bits returned by a [`Motor`].
    #[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct MotorFlags: u32 {
        /// Failed communicate with the motor
        const BUSY = 0x01;

        /// UNUSED: The motor is currently near zero velocity.
        const ZERO_VELOCITY = 0x02;

        /// UNUSED: The motor is at its zero position.
        const ZERO_POSITION = 0x04;
    }
}
impl_bincode_bitflags!(MotorFlags);
