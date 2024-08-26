pub mod battery;

use battery::Battery;

#[derive(Default, Debug)]
pub struct Peripherals {
    pub battery: Battery,
}
