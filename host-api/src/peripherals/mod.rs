pub mod battery;
pub mod motor;

use battery::Battery;

#[derive(Default, Debug)]
pub struct Peripherals {
    pub battery: Battery,
}
