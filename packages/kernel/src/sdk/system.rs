//! VEXos System Functions

use alloc::format;
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
    sdk::draw_error_box,
    xil::{
        gic::XSCUGIC_MAX_NUM_INTR_INPUTS,
        timer::XScuTimer,
        XST_FAILURE, XST_SUCCESS,
    },
};

extern "C" {
    #[link_name = "_link_addr"]
    static LINK_ADDR: u32;
}

pub extern "C" fn vexPrivateApiDisable(sig: u32) {}
pub extern "C" fn vexStdlibMismatchError(installed_version: u32, required_version: u32) {
    draw_error_box([
        Some("C++ Library mismatch !"),
        Some(&format!(
            "Installed {}.{}.{}",
            installed_version >> 0x18,
            (installed_version << 8) >> 24,
            (installed_version << 16) >> 24
        )),
        Some(&format!(
            "Required  {}.{}.{}",
            required_version >> 0x18,
            (required_version << 8) >> 24,
            (required_version << 16) >> 24
        )),
    ]);
}
pub extern "C" fn vexScratchMemoryLock() -> bool {
    false
}
pub extern "C" fn vexScratchMemoryUnock() {}
pub extern "C" fn vexSystemTimeGet() -> u32 {
    SYSTEM_TIME.load(Ordering::Acquire)
}
/// # Safety
///
/// `pTime` must be a valid pointer to a `date` structure.
pub unsafe extern "C" fn vexGettime(pTime: *mut time) {
    let time_ms = SYSTEM_TIME.load(Ordering::Acquire);
    let time_secs = time_ms / 1000;

    unsafe {
        *pTime = time {
            ti_hour: (time_secs / 3600) as _,
            ti_min: ((time_secs / 60) % 60) as _,
            ti_sec: (time_secs % 60) as _,
            ti_hund: ((time_ms % 1000) / 10) as _,
        }
    }
}
/// # Safety
///
/// `pDate` must be a valid pointer to a `date` structure.
pub unsafe extern "C" fn vexGetdate(pDate: *mut date) {
    unsafe {
        *pDate = date {
            da_year: 2016,
            da_day: 16,
            da_mon: 11,
        }
    }
}
pub extern "C" fn vexSystemMemoryDump() {
    unsafe {
        unsafe extern "C" {
            static mut __heap_start: u8;
            static mut __heap_end: u8;

            static mut __data_start: u32;
            static mut __data_end: u32;

            static mut __bss_start: u32;
            static mut __bss_end: u32;

            static mut __stack_top: u32;
            static mut __stack_bottom: u32;
        }

        let stack_size = (&raw const __stack_top).offset_from(&raw const __stack_bottom);
        let bss_size = (&raw const __bss_end).offset_from(&raw const __bss_start);
        let data_size = (&raw const __data_end).offset_from(&raw const __data_start);
        let heap_size = (&raw const __heap_end).offset_from(&raw const __heap_start);

        vex_printf(c"sizeof task        : %u\n".as_ptr(), 0);
        vex_printf(c"sizeof taskS       : %u\n".as_ptr(), 0);
        vex_printf(
            c"system data top    : %08X\n".as_ptr(),
            &raw const __data_start,
        );
        vex_printf(
            c"system data end    : %08X\n".as_ptr(),
            &raw const __data_end,
        );
        vex_printf(
            c"system data        : %08X (%8d)\n".as_ptr(),
            data_size,
            data_size,
        );
        vex_printf(c"system bss top     : %08X\n".as_ptr(), &__bss_start);
        vex_printf(c"system bss end     : %08X\n".as_ptr(), &__bss_end);
        vex_printf(
            c"system bss         : %08X (%8d)\n".as_ptr(),
            bss_size,
            bss_size,
        );
        vex_printf(c"system heap top    : %08X\n".as_ptr(), &__heap_start);
        vex_printf(c"system heap end    : %08X\n".as_ptr(), &__heap_end);
        vex_printf(
            c"system heap        : %08X (%8d)\n".as_ptr(),
            heap_size,
            heap_size,
        );
        vex_printf(c"system stack end   : %08X\n".as_ptr(), &__stack_top);
        vex_printf(c"system stack       : %08X\n".as_ptr(), &__stack_bottom);
        vex_printf(
            c"system stack size  : %08X (%8d)\n".as_ptr(),
            stack_size,
            stack_size,
        );
    }
}
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
    unsafe { LINK_ADDR }
}
pub extern "C" fn vexSystemTimerGet(timer: u32) -> u32 {
    // normally the timer argument here would select from a few different, counters
    // such as the vsync interrupt timer, but none of that is applicable here.
    0
}
pub extern "C" fn vexSystemUsbStatus() -> u32 {
    // USB connected, no controller
    //
    // TODO: wire this up to controller once that is implemented
    0x3
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

// These three are noops for now since to my knowledge
// they aren't used or another handler takes its place
// (in the case of IRQs XScuGic_InterruptHandler) is
// registered on the exception table for IRQs.
pub extern "C" fn vexSystemUndefinedException() {
    loop {}
}
pub extern "C" fn vexSystemFIQInterrupt() {
    loop {}
}
pub extern "C" fn vexSystemIQRQnterrupt() {
    loop {}
}
pub extern "C" fn vexSystemSWInterrupt() {
    loop {}
}
pub extern "C" fn vexSystemDataAbortInterrupt() {
    loop {}
}
pub extern "C" fn vexSystemPrefetchAbortInterrupt() {
    loop {}
}
