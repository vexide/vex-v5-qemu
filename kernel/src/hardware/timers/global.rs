use crate::peripherals::PERIPHBASE;

use crate::hardware::mmu::HighMemUnlock;

const GLOBAL_TIMER_BASE_ADDRESS: u32 = PERIPHBASE + 0x0200;

const GLOBAL_TIMER_COUNTER_REGISTER_LOW: u32 = GLOBAL_TIMER_BASE_ADDRESS /* + 0x00 */;
const GLOBAL_TIMER_COUNTER_REGISTER_HIGH: u32 = GLOBAL_TIMER_BASE_ADDRESS + 0x04;

/// Reads the value of the incrementing 64-bit global timer counter.
// TODO: maybe turn this into a global, since there's much more to
// this peripheral than just reading it.
pub fn global_timer_counter() -> u64 {
    let _unlock_mem = HighMemUnlock::new();
    let mut low: u32;
    let mut high: u32;

    loop {
        unsafe {
            high = core::ptr::read_volatile(GLOBAL_TIMER_COUNTER_REGISTER_HIGH as *const u32);
            low = core::ptr::read_volatile(GLOBAL_TIMER_COUNTER_REGISTER_LOW as *const u32);

            if core::ptr::read_volatile(GLOBAL_TIMER_COUNTER_REGISTER_HIGH as *const u32) == high {
                break;
            }
        }
    }

    ((high as u64) << 32_u32) | (low as u64)
}
