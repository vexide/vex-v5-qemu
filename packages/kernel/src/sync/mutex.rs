use std::{cell::UnsafeCell, hint::spin_loop};
use critical_section::RestoreState;

pub use lock_api::MutexGuard;
pub type Mutex<T> = lock_api::Mutex<RawMutex, T>;

fn in_interrupt() -> bool {
    unsafe {
        let mut cpsr: u32;
        asm!("mrs {0}, cpsr", out(reg) cpsr);
        let is_system_mode = (cpsr & 0b11111) == 0b11111;
        !is_system_mode
    }
}

struct MutexState(UnsafeCell<RestoreState>, bool);
impl MutexState {
    const fn new() -> Self {
        Self(UnsafeCell::new(RestoreState::invalid()), false)
    }

    /// Returns true if the lock was acquired.
    fn try_lock(&self) -> bool {
        unsafe { 
            let state = critical_section::acquire();
            if self.1 {
                critical_section::release(state);
                false
            } else {
                *self.0.get() = state;
                self.1 = true;
                true
            }
        }
    }

    fn unlock(&self) {
        unsafe { critical_section::release(self.0.into_inner()) }
    }
}

/// A raw mutex type built on top of the critical section.
pub struct RawMutex {
    state: MutexState,
}
impl RawMutex {
    /// Creates a new raw mutex.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            state: MutexState::new(),
        }
    }
}
unsafe impl lock_api::RawMutex for RawMutex {
    // Allow this because we can't get around it
    #[allow(clippy::declare_interior_mutable_const)]
    const INIT: Self = Self::new();

    type GuardMarker = lock_api::GuardSend;

    fn lock(&self) {
        if self.state.try_lock() {
            ()
        } else if in_interrupt() {
            panic!("Deadlock in kernel detected!");
        } else {
            while !self.state.try_lock() {
                spin_loop()
            }
            ()
        }
    }

    fn try_lock(&self) -> bool {
        self.state.try_lock()
    }

    unsafe fn unlock(&self) {
        self.state.unlock()
    }
}
