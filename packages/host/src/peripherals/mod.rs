pub mod battery;
pub mod display;
pub mod smartport;
pub mod usb;

use battery::Battery;
use display::Display;
use smartport::SmartPort;
use usb::Usb;

pub struct Peripherals {
    pub battery: Battery,
    pub usb: Usb,

    pub port_1: SmartPort,
    pub port_2: SmartPort,
    pub port_3: SmartPort,
    pub port_4: SmartPort,
    pub port_5: SmartPort,
    pub port_6: SmartPort,
    pub port_7: SmartPort,
    pub port_8: SmartPort,
    pub port_9: SmartPort,
    pub port_10: SmartPort,
    pub port_11: SmartPort,
    pub port_12: SmartPort,
    pub port_13: SmartPort,
    pub port_14: SmartPort,
    pub port_15: SmartPort,
    pub port_16: SmartPort,
    pub port_17: SmartPort,
    pub port_18: SmartPort,
    pub port_19: SmartPort,
    pub port_20: SmartPort,
    pub port_21: SmartPort,

    pub display: Display,
    // TODO: onboard ADI, controllers, display/touch, usb, sdcard
}
