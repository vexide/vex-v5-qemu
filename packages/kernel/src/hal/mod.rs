use core::cell::OnceCell;

pub mod critical_section;
pub mod gic;
pub mod timer;
pub mod wdt;

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

pub struct UnsafePeripheral<T>(OnceCell<T>);

impl<T> UnsafePeripheral<T> {
    pub const unsafe fn new() -> Self {
        Self(OnceCell::new())
    }

    pub fn set(&mut self, value: T) -> Result<(), T> {
        self.0.set(value)
    }

    pub fn get(&self) -> Option<&T> {
        self.0.get()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.0.get_mut()
    }
}

// lol
unsafe impl<T> Send for UnsafePeripheral<T> {}
unsafe impl<T> Sync for UnsafePeripheral<T> {}
