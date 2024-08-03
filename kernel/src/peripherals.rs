use core::{
    ffi::c_void,
    sync::atomic::{AtomicU32, Ordering},
};

use vexide_core::sync::LazyLock;

use crate::{
    hardware::{
        gic::{GenericInterruptController, GicError, InterruptTrigger},
        timers::{private::PrivateTimer, WatchdogTimer},
        uart::UartDriver,
    },
    xil::{
        gic::XPAR_SCUGIC_0_DIST_BASEADDR,
        timer::{XScuTimer, XPAR_XSCUTIMER_0_BASEADDR},
        uart::UART1_BASE_ADDR,
        wdt::XPAR_XSCUWDT_0_BASEADDR,
    },
    Mutex,
};

/// Clock frequency of Cortex-A9 timers (in Hz).
///
/// This is half the CPU frequency (which is 666.6MHz).
pub const PERIPHCLK: u32 = 666666687 / 2;

/// Base address of MMIO peripherals.
pub const PERIPHBASE: u32 = 0xF8F00000;

// SAFETY: This is the only place these devices are being initialized from.

/// UART1 (Universal Asynchronous Receiver/Transmitter) stream 1
pub static UART1: LazyLock<Mutex<UartDriver>> =
    LazyLock::new(|| Mutex::new(unsafe { UartDriver::new(UART1_BASE_ADDR).unwrap() }));

/// Generic Interrupt Controller
pub static GIC: LazyLock<Mutex<GenericInterruptController>> = LazyLock::new(|| {
    Mutex::new(unsafe { GenericInterruptController::new(XPAR_SCUGIC_0_DIST_BASEADDR).unwrap() })
});

/// Private Timer
pub static PRIVATE_TIMER: LazyLock<Mutex<PrivateTimer>> =
    LazyLock::new(|| Mutex::new(unsafe { PrivateTimer::new(XPAR_XSCUTIMER_0_BASEADDR).unwrap() }));

/// Watchdog Timer
pub static WATCHDOG_TIMER: LazyLock<Mutex<WatchdogTimer>> =
    LazyLock::new(|| Mutex::new(unsafe { WatchdogTimer::new(XPAR_XSCUWDT_0_BASEADDR).unwrap() }));

pub static SYSTEM_TIME: AtomicU32 = AtomicU32::new(0);

/// Handles a timer interrupt
pub extern "C" fn timer_interrupt_handler(_: *mut c_void) {
    let mut timer = PRIVATE_TIMER.lock();

    // Verify that the timer has in fact reached zero.
    if timer.is_expired() {
        // Clear ISR flag to ensure that future interrupts fire.
        timer.clear_interrupt_status();

        // This interrupt is configured to fire every 1mS, so we'll
        // tick the low resolution system timer to track the number of
        // milliseconds since program start.
        _ = SYSTEM_TIME.fetch_add(1, Ordering::Relaxed);
    }

    // NOTE: I think (?) vexos offers a way for users to register a callback
    // here through some part of the SDK, but nobody really uses that and its
    // not a publicly exposed API. PROS just uses [`vexSystemTimerReinitForRtos`]
    // anyways...
}

/// Configures the Private Timer peripheral and registers an interrupt handler
/// for timer ticks using the Generic Interrupt Controller (GIC).
pub fn setup_private_timer() -> Result<(), GicError> {
    let mut gic = GIC.lock();
    let mut timer = PRIVATE_TIMER.lock();

    // These two calls are very likely not necessary, but are a good sanity check to
    // ensure the timer wasn't previously configured.
    timer.stop();
    timer.set_prescaler(0);

    // This is a decrementing counter. Once it reaches zero, an interrupt will fire
    // which will then be handled from the GIC.
    //
    // After the timer reaches zero (every millisecond), we still want it to keep
    // ticking, so we enable auto-reload to reset the counter back to the provided
    // LOAD value.
    timer.set_auto_reload(true);

    // The LOAD value specifies where the timer should start at when counting
    // down. In our case, the timer's clock speed is half the CPU frequency
    // (666.6 MHz) and we want this timer to tick every millisecond, so we
    // configure it to start counting down from 333333 resulting in 1mS periods.
    timer.load(PERIPHCLK / 1000);

    // Another sanity check. This realistically shouldn't be needed unless a timer
    // IRQ has already occurred, which we haven't configured to happen yet.
    timer.clear_interrupt_status();

    // Register a handler for timer interrupts. See [`timer_interrupt_handler`] for
    // the handler code itself.
    gic.set_handler(
        PrivateTimer::INTERRUPT_ID,
        0, // priority
        InterruptTrigger::RisingEdge,
        timer_interrupt_handler,
        timer.raw_mut() as *mut XScuTimer as _,
    )?;

    // Begin counting down from the provided LOAD value.
    timer.start();

    // Enable timer IRQ.
    gic.enable_interrupt(PrivateTimer::INTERRUPT_ID);
    timer.set_interrupt_enabled(true);

    Ok(())
}
