//! VEXos System Functions

use core::{
    arch::asm,
    ffi::c_void,
    sync::atomic::{AtomicBool, Ordering},
};

use log::{error, info};
use vex_sdk::*;

use crate::{
    hardware::{
        gic::InterruptTrigger,
        timers::{global_timer_counter, PrivateTimer, WatchdogTimerMode},
    },
    peripherals::{timer_interrupt_handler, GIC, PERIPHCLOCK, PRIVATE_TIMER, SYSTEM_TIME, WATCHDOG_TIMER},
    utils::exit,
    xil::{gic::XSCUGIC_MAX_NUM_INTR_INPUTS, timer::XScuTimer, XST_FAILURE, XST_SUCCESS},
};

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
pub fn vexSystemExitRequest() {
    exit(0);
}
pub fn vexSystemHighResTimeGet() -> u64 {
    global_timer_counter() / (PERIPHCLOCK as u64 / 1000000)
}
pub fn vexSystemPowerupTimeGet() -> u64 {
    // powerup time is the same as execution time in our case
    vexSystemHighResTimeGet()
}
pub fn vexSystemLinkAddrGet() -> u32 {
    // no multi-file support yet
    0x0
}
pub fn vexSystemTimerGet(param_1: u32) -> u32 {
    Default::default()
}
pub fn vexSystemUsbStatus() -> u32 {
    Default::default()
}
pub fn vexSystemTimerStop() {
    let mut timer = PRIVATE_TIMER.lock();
    timer.set_interrupt_enabled(false);
    timer.stop();
}

/// Clears the timer interrupt status if the timer has expired.
pub fn vexSystemTimerClearInterrupt() {
    // Realistically I think this should be a call to [`PrivateTimer::clear_interrupt_status()`]
    // and NOT the timer interrupt handler (which also increments the time for vexSystemTimeGet),
    // but this is supposedly the behavior observed in VEXos, so we'll do it too.
    unsafe { timer_interrupt_handler(&mut PRIVATE_TIMER.lock().raw() as *mut XScuTimer as _) }
}

/// Reinitializes the timer interrupt with a given tick handler and priority for the private timer instance.
pub fn vexSystemTimerReinitForRtos(
    priority: u32,
    handler: extern "C" fn(data: *mut c_void),
) -> i32 {
    let mut gic = GIC.lock();
    let mut timer = PRIVATE_TIMER.lock();

    // Install user-provided tick handler with provided priority
    let result = gic.set_handler(
        PrivateTimer::INTERRUPT_ID,
        priority as u8,
        InterruptTrigger::RisingEdge,
        timer_interrupt_handler,
        &mut unsafe { timer.raw() } as *mut XScuTimer as _,
    );

    // Restart the timer and enable the timer interrupt
    if result.is_ok() {
        timer.start();

        if timer.is_expired() {
            timer.clear_interrupt_status();
        }

        timer.set_interrupt_enabled(true);

        XST_SUCCESS
    } else {
        XST_FAILURE
    }
}

/// Handles an IRQ using the interrupt controller's handler table.
pub fn vexSystemApplicationIRQHandler(ulICCIAR: u32) {
    let gic = GIC.lock();

    // The ID of the interrupt is obtained by bitwise anding the ICCIAR value
    let interrupt_id = ulICCIAR & 0x3FF;

    unsafe {
        // Re-enable interrupts.
        asm!("cpsie i");

        // Check for a valid interrupt ID.
        if interrupt_id < (XSCUGIC_MAX_NUM_INTR_INPUTS as u32) {
            // Call respective interrupt handler from the vector table.
            let entry = (*gic.raw().Config).HandlerTable[interrupt_id as usize];
            (entry.handler).unwrap()(entry.callback_ref);
        }
    }
}

static WDT_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initializes the CPU1 watchdog timer.
pub fn vexSystemWatchdogReinitRtos() -> i32 {
    // Since the watchdog timer can only be initialized a single time, this function should only
    // ever be called once. In the event that it is called more than once, we check and bail early
    // by returning XST_FAILURE.
    if WDT_INITIALIZED.load(Ordering::Relaxed) {
        return XST_FAILURE;
    } else {
        WDT_INITIALIZED.store(true, Ordering::Relaxed);
    }

    let mut wdt = WATCHDOG_TIMER.lock();

    // Configure prescaler and LOAD values to match the requirements of the zc706 FreeRTOS port.
    wdt.set_prescaler(u8::MAX);
    wdt.load(u32::MAX);

    // The FreeRTOS port isn't actually using the watchdog timer for its "intended purpose" (identifying
    // program deadlocks), but rather just uses it as a secondary timer peripheral for reporting statistics
    // about task usage.
    //
    // As such, we run the watchdog timer in "timer mode" (rather than watchdog mode) to treat it like a normal
    // decrementing clock.
    wdt.set_mode(WatchdogTimerMode::Timer);

    wdt.start(); // you get the idea

    XST_SUCCESS
}
pub fn vexSystemWatchdogGet() -> u32 {
    Default::default()
}
pub fn vexSystemBoot() {}

pub fn vexSystemUndefinedException() {
    error!("Undefined Instruction Exception");
}
pub fn vexSystemFIQInterrupt() {
    info!("FIQ");
}
pub fn vexSystemIQRQnterrupt() {
    info!("IRQ");
}
pub fn vexSystemSWInterrupt() {
    info!("Software Interrupt");
}
pub fn vexSystemDataAbortInterrupt() {
    error!("Data Abort Exception");
}
pub fn vexSystemPrefetchAbortInterrupt() {
    error!("Prefetch Abort Exception");
}
