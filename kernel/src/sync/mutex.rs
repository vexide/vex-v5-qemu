use core::sync::atomic::{AtomicU8, Ordering};

pub use lock_api::MutexGuard;
pub type Mutex<T> = lock_api::Mutex<RawMutex, T>;

struct MutexState(AtomicU8);
impl MutexState {
    const fn new() -> Self {
        Self(AtomicU8::new(0))
    }

    /// Returns true if the lock was acquired.
    fn try_lock(&self) -> bool {
        self.0
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Acquire)
            .is_ok()
    }

    fn unlock(&self) {
        self.0.store(0, Ordering::Release);
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
        critical_section::with(|_| {
            while !self.state.try_lock() {
                core::hint::spin_loop();
            }
        })
    }

    fn try_lock(&self) -> bool {
        critical_section::with(|_| self.state.try_lock())
    }

    unsafe fn unlock(&self) {
        critical_section::with(|_| {
            self.state.unlock();
        })
    }
}
