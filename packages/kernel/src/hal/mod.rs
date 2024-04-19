pub mod timer;
pub mod wdt;
pub mod gic;
pub mod critical_section;

/// A register associated with a peripheral.
pub struct Register(u32);

impl Register {
	/// Create a new register from a peripheral's base address and an offset.
	pub const fn new(base: u32, offset: u32) -> Self {
		Self(base + offset)
	}

	/// Read the contents of the register.
	pub unsafe fn read(&self) -> u32 {
		core::ptr::read_volatile(self.0 as *const u32)
	}

	/// Write to the register.
	pub unsafe fn write(&mut self, data: u32) {
		core::ptr::write_volatile(self.0 as *mut u32, data);
	}
}