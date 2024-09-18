use std::cell::UnsafeCell;
use critical_section::RestoreState;

pub use lock_api::MutexGuard;
pub type Mutex<T> = lock_api::Mutex<RawMutex, T>;

struct MutexState(UnsafeCell<RestoreState>);
impl MutexState {
    const fn new() -> Self {
        Self(UnsafeCell::new(RestoreState::invalid()))
    }

    /// Returns true if the lock was acquired.
    fn try_lock(&self) -> bool {
        unsafe { *self.0.get() = critical_section::acquire() }
        true
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
        critical_section::with(|_| self.state.try_lock())
    }

    fn try_lock(&self) -> bool {
        critical_section::with(|_| self.state.try_lock())
    }

    unsafe fn unlock(&self) {
        critical_section::with(|_| self.state.unlock())
    }
}
