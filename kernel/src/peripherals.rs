use core::{
    cell::UnsafeCell,
    ffi::c_void,
    sync::atomic::{AtomicU32, Ordering},
};

use vexide_core::sync::LazyLock;

use crate::{
    hardware::{gic::{GenericInterruptController, InterruptTrigger}, uart::UartDriver},
    xil::{
        gic::XPAR_SCUGIC_0_DIST_BASEADDR,
        timer::{
            XScuTimer, XScuTimer_CfgInitialize, XScuTimer_ClearInterruptStatus,
            XScuTimer_EnableAutoReload, XScuTimer_EnableInterrupt, XScuTimer_IsExpired,
            XScuTimer_LoadTimer, XScuTimer_LookupConfig, XScuTimer_SetPrescaler, XScuTimer_Start,
            XScuTimer_Stop, XPAR_SCUTIMER_INTR, XPAR_XSCUTIMER_0_BASEADDR,
        },
        uart::{UART0_BASE_ADDR, UART1_BASE_ADDR},
        wdt::XScuWdt,
    },
    Mutex,
};

// SAFETY: This is the only place these devices are being initialized from.

/// UART1 (Universal Asynchronous Receiver/Transmitter) stream 1
pub static UART1: LazyLock<Mutex<UartDriver>> = LazyLock::new(|| {
    Mutex::new(unsafe { UartDriver::new(UART1_BASE_ADDR).unwrap() })
});

/// Generic Interrupt Controller
pub static GIC: LazyLock<Mutex<GenericInterruptController>> = LazyLock::new(|| {
    Mutex::new(unsafe { GenericInterruptController::new(XPAR_SCUGIC_0_DIST_BASEADDR).unwrap() })
});

pub static mut PRIVATE_TIMER: UnsafeCell<XScuTimer> =
    UnsafeCell::new(unsafe { core::mem::zeroed() });
pub static mut WATCHDOG_TIMER: UnsafeCell<XScuWdt> =
    UnsafeCell::new(unsafe { core::mem::zeroed() });

pub static SYSTEM_TIME: AtomicU32 = AtomicU32::new(0);

/// Handles a timer interrupt
pub extern "C" fn timer_interrupt_handler(timer: *mut c_void) {
    let timer = timer as *mut XScuTimer;

    if unsafe { XScuTimer_IsExpired(timer) } {
        unsafe {
            // Clear interrupt
            XScuTimer_ClearInterruptStatus(timer);
        }

        // Increment system timer
        _ = SYSTEM_TIME.fetch_add(1, Ordering::Relaxed);
    }

    // NOTE: I think (?) vexos offers a way for users
    // to register a callback here through some part
    // of the SDK, but nobody really uses that and its
    // not public API.
}

/// Configures the Private Timer peripheral and registers an interrupt handler
/// for timer ticks using the Generic Interrupt Controller (GIC).
pub fn setup_timer() {
    unsafe {
        let timer = PRIVATE_TIMER.get_mut();
        let mut gic = GIC.lock();

        let timer_config = XScuTimer_LookupConfig(XPAR_XSCUTIMER_0_BASEADDR as *mut u32);
        let status = XScuTimer_CfgInitialize(timer, timer_config, (*timer_config).BaseAddr);

        if status == 0 {
            XScuTimer_Stop(timer);

            // Ensure there is no prescaler.
            XScuTimer_SetPrescaler(timer, 0);

            // Configure timer
            XScuTimer_EnableAutoReload(timer); // Enable auto-reload mode.
            XScuTimer_LoadTimer(timer, 333333); // Load the timer counter register with the correct tick rate.
            XScuTimer_ClearInterruptStatus(timer);

            // Register timer handler with interrupt controller
            let result = gic.set_handler(
                XPAR_SCUTIMER_INTR,
                0,
                InterruptTrigger::RisingEdge,
                timer_interrupt_handler,
                timer as *mut XScuTimer as _
            );

            // Start timer and enable timer IRQs on this CPU.
            if result.is_ok() {
                XScuTimer_Start(timer);
                gic.enable_interrupt(XPAR_SCUTIMER_INTR);
                XScuTimer_EnableInterrupt(timer);
            }
        }
    }
}
