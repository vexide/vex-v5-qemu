//! VEXos System Functions

use core::{
    arch::asm,
    ffi::c_void,
    sync::atomic::{AtomicBool, Ordering},
};

use vex_sdk::*;

use crate::{
    hardware::{
        gic::InterruptTrigger,
        timers::{global_timer_counter, PrivateTimer, WatchdogTimerMode},
    },
    peripherals::{
        timer_interrupt_handler, GIC, PERIPHCLK, PRIVATE_TIMER, SYSTEM_TIME, WATCHDOG_TIMER,
    },
    protocol::exit,
    xil::{
        exception::{DataAbortAddr, PrefetchAbortAddr, UndefinedExceptionAddr},
        gic::XSCUGIC_MAX_NUM_INTR_INPUTS,
        timer::XScuTimer,
        XST_FAILURE, XST_SUCCESS,
    },
};

pub extern "C" fn vexPrivateApiDisable(sig: u32) {}
pub extern "C" fn vexStdlibMismatchError(param_1: u32, param_2: u32) {}
pub extern "C" fn vexScratchMemoryPtr(ptr: *mut *mut core::ffi::c_void) -> i32 {
    Default::default()
}
pub extern "C" fn vexScratchMemoryLock() -> bool {
    Default::default()
}
pub extern "C" fn vexScratchMemoryUnock() {}
pub extern "C" fn vexSystemTimeGet() -> u32 {
    SYSTEM_TIME.load(Ordering::Acquire)
}
pub extern "C" fn vexGettime() -> time {
    Default::default()
}
pub extern "C" fn vexGetdate() -> date {
    Default::default()
}
pub extern "C" fn vexSystemMemoryDump() {}
pub extern "C" fn vexSystemDigitalIO(pin: u32, value: u32) {}
pub extern "C" fn vexSystemStartupOptions() -> u32 {
    Default::default()
}
pub extern "C" fn vexSystemExitRequest() {
    exit(0);
}
pub extern "C" fn vexSystemHighResTimeGet() -> u64 {
    global_timer_counter() / (PERIPHCLK as u64 / 1000000)
}
pub extern "C" fn vexSystemPowerupTimeGet() -> u64 {
    // powerup time is the same as execution time in our case
    vexSystemHighResTimeGet()
}
pub extern "C" fn vexSystemLinkAddrGet() -> u32 {
    // no multi-file support yet
    0x0
}
pub extern "C" fn vexSystemTimerGet(param_1: u32) -> u32 {
    Default::default()
}
pub extern "C" fn vexSystemUsbStatus() -> u32 {
    Default::default()
}
pub extern "C" fn vexSystemTimerStop() {
    let mut timer = PRIVATE_TIMER.lock();
    timer.set_interrupt_enabled(false);
    timer.stop();
}

/// Clears the timer interrupt status if the timer has expired.
pub extern "C" fn vexSystemTimerClearInterrupt() {
    // Realistically I think this should be a call to
    // [`PrivateTimer::clear_interrupt_status()`] and NOT the timer interrupt
    // handler (which also increments the time for vexSystemTimeGet),
    // but this is supposedly the behavior observed in VEXos, so we'll do it too.
    timer_interrupt_handler(PRIVATE_TIMER.lock().raw_mut() as *mut XScuTimer as _)
}

/// Reinitializes the timer interrupt with a given tick handler and priority for
/// the private timer instance.
pub extern "C" fn vexSystemTimerReinitForRtos(
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
        handler,
        timer.raw_mut() as *mut XScuTimer as _,
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
pub extern "C" fn vexSystemApplicationIRQHandler(ulICCIAR: u32) {
    let mut gic = GIC.lock();

    // The ID of the interrupt is obtained by bitwise anding the ICCIAR value
    let interrupt_id = ulICCIAR & 0x3FF;

    unsafe {
        // Re-enable interrupts.
        asm!("cpsie i");

        // Check for a valid interrupt ID.
        if interrupt_id < (XSCUGIC_MAX_NUM_INTR_INPUTS as u32) {
            // Call respective interrupt handler from the vector table.
            let entry = (*(gic.raw_mut().Config)).HandlerTable[interrupt_id as usize];
            (entry.handler).unwrap()(entry.callback_ref);
        }
    }
}

static WDT_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initializes the CPU1 watchdog timer.
pub extern "C" fn vexSystemWatchdogReinitRtos() -> i32 {
    // Since the watchdog timer can only be initialized a single time, this function
    // should only ever be called once. In the event that it is called more than
    // once, we check and bail early by returning XST_FAILURE.
    if WDT_INITIALIZED.load(Ordering::Relaxed) {
        return XST_FAILURE;
    } else {
        WDT_INITIALIZED.store(true, Ordering::Relaxed);
    }

    let mut wdt = WATCHDOG_TIMER.lock();

    // Configure prescaler and LOAD values to match the requirements of the zc706
    // FreeRTOS port.
    wdt.set_prescaler(u8::MAX);
    wdt.load(u32::MAX);

    // The FreeRTOS port isn't actually using the watchdog timer for its "intended
    // purpose" (identifying program deadlocks), but rather just uses it as a
    // secondary timer peripheral for reporting statistics about task usage.
    //
    // As such, we run the watchdog timer in "timer mode" (rather than watchdog
    // mode) to treat it like a normal decrementing clock.
    wdt.set_mode(WatchdogTimerMode::Timer);

    wdt.start(); // you get the idea

    XST_SUCCESS
}
pub extern "C" fn vexSystemWatchdogGet() -> u32 {
    WATCHDOG_TIMER.lock().counter()
}
pub extern "C" fn vexSystemBoot() {}

pub extern "C" fn vexSystemUndefinedException() {
    unsafe {
        log::error!("Undefined instruction exception: {UndefinedExceptionAddr:#x}");
    }
    // TODO: draw the funny red box to the screen
    loop {
        core::hint::spin_loop();
    }
}

// These three are noops for now since to my knowledge
// they aren't used or another handler takes its place
// (in the case of IRQs XScuGic_InterruptHandler) is
// registered on the exception table for IRQs.
pub extern "C" fn vexSystemFIQInterrupt() {}
pub extern "C" fn vexSystemIQRQnterrupt() {}
pub extern "C" fn vexSystemSWInterrupt() {}

pub extern "C" fn vexSystemDataAbortInterrupt() {
    unsafe {
        log::error!("Data abort exception: {DataAbortAddr:#x}");
    }
    // TODO: draw the funny red box to the screen
    loop {
        core::hint::spin_loop();
    }
}
pub extern "C" fn vexSystemPrefetchAbortInterrupt() {
    unsafe {
        log::error!("Prefetch abort exception: {PrefetchAbortAddr:#x}");
    }
    // TODO: draw the funny red box to the screen
    loop {
        core::hint::spin_loop();
    }
}
