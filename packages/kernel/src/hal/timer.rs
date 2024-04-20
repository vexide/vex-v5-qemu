//! Cortex-A9 Private Timer

#![allow(const_item_mutation)]

use super::Register;

#[derive(Debug, Eq, PartialEq)]
pub struct Timer {
    is_started: bool,
    is_ready: bool,
}

impl Timer {
    /// XPAR_XSCUTIMER_0_DEVICE_ID
    pub const DEVICE_ID: u32 = 0;
    /// XPAR_XSCUTIMER_0_BASEADDR
    pub const BASE_ADDR: u32 = 0xF8F00600;
    /// XPAR_XSCUTIMER_0_HIGHADDR
    pub const HIGH_ADDR: u32 = 0xF8F0061F;

    /// Timer Load Register
    pub const LOAD_REGISTER: Register = Register::new(Self::BASE_ADDR, 0x00);
    /// Timer Counter Register
    pub const COUNTER_REGISTER: Register = Register::new(Self::BASE_ADDR, 0x04);
    /// Timer Control Register
    pub const CONTROL_REGISTER: Register = Register::new(Self::BASE_ADDR, 0x08);
    /// Timer Interrupt Status Register
    pub const ISR_REGISTER: Register = Register::new(Self::BASE_ADDR, 0x0C);

    /// Prescaler
    pub const CONTROL_PRESCALER_MASK: u32 = 0x0000FF00;
    /// Prescaler shift
    pub const CONTROL_PRESCALER_SHIFT: u32 = 8;
    /// Intr enable
    pub const CONTROL_IRQ_ENABLE_MASK: u32 = 0x00000004;
    /// Auto-reload
    pub const CONTROL_AUTO_RELOAD_MASK: u32 = 0x00000002;
    /// Timer enable
    pub const CONTROL_ENABLE_MASK: u32 = 0x00000001;

    /// Event flag
    pub const ISR_EVENT_FLAG_MASK: u32 = 0x00000001;

    pub const IRQ_INTERRUPT_ID: u32 = 29;

    /// Clock frequency of the Cortex-A9.
    pub const CPU_CLOCK_FREQ_HZ: u32 = 666666687;
    /// Clock frequency of the private timer. This is half of the CPU's clock speed.
    pub const CLOCK_FREQ_HZ: u32 = Self::CPU_CLOCK_FREQ_HZ / 2;
    /// Tick rate of the private timer.
    pub const TICK_RATE_HZ: u32 = Self::CLOCK_FREQ_HZ / 1000;

    /// Initialize a specific timer instance/driver.
    pub fn new() -> Self {
        Self {
            is_started: false,
            is_ready: false,
        }
    }

    /// Stop the timer.
    pub fn stop(&mut self) {
        // Read the contents of the Control register.
        let mut control_register = unsafe { Self::CONTROL_REGISTER.read() };

        // Clear the 'timer enable' bit in the register.
        control_register &= !Self::CONTROL_ENABLE_MASK;

        // Update the Control register with the new value.
        unsafe {
            Self::CONTROL_REGISTER.write(control_register);
        }

        // Indicate that the device is stopped.
        self.is_started = false;
    }

    /// Start the timer.
    pub fn start(&mut self) {
        // Read the contents of the Control register.
        let mut control_register = unsafe { Self::CONTROL_REGISTER.read() };

        // Set the 'timer enable' bit in the register.
        control_register |= Self::CONTROL_ENABLE_MASK;

        // Update the Control register with the new value.
        unsafe {
            Self::CONTROL_REGISTER.write(control_register);
        }

        // Indicate that the device is stopped.
        self.is_started = true;
    }

    /// This function sets the prescaler bits in the timer control register.
    pub fn set_prescaler(&mut self, value: u8) {
        // Read the Timer control register.
        let mut control_register = unsafe { Self::CONTROL_REGISTER.read() };

        // Clear all of the prescaler control bits in the register.
        control_register &= !Self::CONTROL_PRESCALER_MASK;

        // Set the prescaler value.
        control_register |= (value as u32) << Self::CONTROL_PRESCALER_SHIFT;

        // Write the register with the new values.
        unsafe {
            Self::CONTROL_REGISTER.write(control_register);
        }
    }

    /// Write to the timer load register. This will also update the
    /// timer counter register with the new value. This macro can be used to
    /// change the time-out value.
    pub fn load(&mut self, value: u32) {
        unsafe {
            Self::LOAD_REGISTER.write(value);
        }
    }

    /// Enable auto-reload mode.
    pub fn enable_auto_reload(&mut self) {
        // Read the Timer control register.
        let mut control_register = unsafe { Self::CONTROL_REGISTER.read() };

        // Apply the auto-reload mask.
        control_register |= Self::CONTROL_AUTO_RELOAD_MASK;

        // Write the register with the new values.
        unsafe {
            Self::CONTROL_REGISTER.write(control_register);
        }
    }

    /// This function clears the interrupt status.
    pub fn clear_interrupt_status(&mut self) {
        unsafe {
            Self::ISR_REGISTER.write(Self::ISR_EVENT_FLAG_MASK);
        }
    }

    /// Check if the timer has expired.
    pub fn is_expired(&self) -> bool {
        unsafe {
            (Self::ISR_REGISTER.read() & Self::ISR_EVENT_FLAG_MASK) == Self::ISR_EVENT_FLAG_MASK
        }
    }

    /// Enables the private timer IRQ.
    pub fn enable_interrupt(&mut self) {
        // Read the Timer control register.
        let mut control_register = unsafe { Self::CONTROL_REGISTER.read() };

        // Apply the irq enable mask.
        control_register |= Self::CONTROL_IRQ_ENABLE_MASK;

        // Write the register with the new values.
        unsafe {
            Self::CONTROL_REGISTER.write(control_register);
        }
    }
}
