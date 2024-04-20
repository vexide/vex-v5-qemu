//! VEXos System Functions

use crate::{
    hal::{gic::GenericInterruptController, timer::Timer}, timer_interrupt_handler, INTERRUPT_CONTROLLER, PRIVATE_TIMER, SYSTEM_TIME, WATCHDOG_TIMER
};
use core::{arch::asm, ffi::c_void, sync::atomic::Ordering};

use vex_sdk::*;

pub fn vexPrivateApiDisable(sig: u32) {}
pub fn vexStdlibMismatchError(param_1: u32, param_2: u32) {}
pub fn vexScratchMemoryPtr(ptr: *mut *mut core::ffi::c_void) -> i32 {
    Default::default()
}
pub fn vexScratchMemoryLock() -> bool {
    Default::default()
}
pub fn vexScratchMemoryUnock() {}
pub fn vexSystemTimeGet() -> u32 {
    SYSTEM_TIME.load(Ordering::Acquire)
}
pub fn vexGettime() -> time {
    Default::default()
}
pub fn vexGetdate() -> date {
    Default::default()
}
pub fn vexSystemMemoryDump() {}
pub fn vexSystemDigitalIO(pin: u32, value: u32) {}
pub fn vexSystemStartupOptions() -> u32 {
    Default::default()
}
pub fn vexSystemExitRequest() {}
pub fn vexSystemHighResTimeGet() -> u64 {
    Default::default()
}
pub fn vexSystemPowerupTimeGet() -> u64 {
    Default::default()
}
pub fn vexSystemLinkAddrGet() -> u32 {
    Default::default()
}
pub fn vexSystemTimerGet(param_1: u32) -> u32 {
    Default::default()
}
pub fn vexSystemTimerEnable(param_1: u32) -> u32 {
    let timer = unsafe { PRIVATE_TIMER.get_mut().unwrap() };
    timer.enable_interrupt();
    0
}
pub fn vexSystemTimerDisable(param_1: u32) {
    let timer = unsafe { PRIVATE_TIMER.get_mut().unwrap() };
    timer.disable_interrupt();
}
pub fn vexSystemUsbStatus() -> u32 {
    Default::default()
}
pub fn vexSystemTimerStop() {
    let timer = unsafe { PRIVATE_TIMER.get_mut().unwrap() };
    timer.stop();
}
pub fn vexSystemTimerClearInterrupt() {
    unsafe {
        timer_interrupt_handler(core::ptr::null_mut());
    }
}

/// Reinitializes the timer interrupt with a given tick handler and priority for the private timer instance.
pub fn vexSystemTimerReinitForRtos(
    priority: u32,
    handler: extern "C" fn(data: *mut c_void),
) -> i32 {
    let gic = unsafe { INTERRUPT_CONTROLLER.get_mut().unwrap() };
    let timer = unsafe { PRIVATE_TIMER.get_mut().unwrap() };
    // Set tick interrupt priority
    // PROS sets this to the lowest possible priority (portLOWEST_USABLE_INTERRUPT_PRIORITY << portPRIORITY_SHIFT).
    gic.set_priority_trigger_type(Timer::IRQ_INTERRUPT_ID, priority as u8, 3);
    gic.connect(Timer::IRQ_INTERRUPT_ID, handler, core::ptr::null_mut());

    timer.start();
    if timer.is_expired() {
        timer.clear_interrupt_status();
    }
    timer.enable_interrupt();

    0
}

/// Handles an IRQ using the interrupt controller's handler table.
pub fn vexSystemApplicationIRQHandler(ulICCIAR: u32) {
    let gic = unsafe { INTERRUPT_CONTROLLER.get_mut().unwrap() };

    // Re-enable interrupts.
    unsafe {
        asm!("cpsie i");
    }

    // The ID of the interrupt is obtained by bitwise anding the ICCIAR value
    let interrupt_id = ulICCIAR & 0x3FF;

    // Check for a valid interrupt ID.
    if interrupt_id < GenericInterruptController::MAX_INTERRUPT_INPUTS {
        // Call respective interrupt handler from the vector table.
        let handler_entry = gic.handler_table[interrupt_id as usize].unwrap();

        unsafe {
            (handler_entry.handler)(handler_entry.callback_ref);
        }
    }
}

/// Initializes the CPU1 watchdog timer.
pub fn vexSystemWatchdogReinitRtos() -> i32 {
    let wdt = unsafe { WATCHDOG_TIMER.get_mut().unwrap() };

    if wdt.is_started() {
        return 1;
    }

    let mut control_reg = wdt.control_register();
    control_reg |= 0xff << 0x08;
    wdt.set_control_register(control_reg);

    wdt.load(core::ffi::c_uint::MAX);
    wdt.set_timer_mode();
    wdt.start();

    0
}
pub fn vexSystemWatchdogGet() -> u32 {
    Default::default()
}
pub fn vexSystemBoot() {}
pub fn vexSystemUndefinedException() {}
pub fn vexSystemFIQInterrupt() {}
pub fn vexSystemIQRQnterrupt() {}
pub fn vexSystemSWInterrupt() {}
pub fn vexSystemDataAbortInterrupt() {}
pub fn vexSystemPrefetchAbortInterrupt() {}
