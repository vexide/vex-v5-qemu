//! VEXos System Functions

use core::{arch::asm, ffi::c_void, sync::atomic::Ordering};

use vex_sdk::*;

use crate::{
    timer_interrupt_handler,
    xil::{
        gic::{
            self, XScuGic_Connect, XScuGic_LookupConfig, XScuGic_SetPriorityTriggerType, XPAR_SCUGIC_0_DIST_BASEADDR, XSCUGIC_MAX_NUM_INTR_INPUTS
        },
        time::{XTime, XTime_GetTime},
        timer::{
            XScuTimer, XScuTimer_ClearInterruptStatus, XScuTimer_EnableInterrupt, XScuTimer_IsExpired, XScuTimer_Start, XScuTimer_Stop, XPAR_SCUTIMER_INTR
        },
        wdt::{
            XScuWdt_CfgInitialize, XScuWdt_GetControlReg, XScuWdt_LoadWdt, XScuWdt_LookupConfig,
            XScuWdt_SetControlReg, XScuWdt_SetTimerMode, XScuWdt_Start, XPAR_XSCUWDT_0_BASEADDR,
        },
    },
    INTERRUPT_CONTROLLER, PRIVATE_TIMER, SYSTEM_TIME, WATCHDOG_TIMER,
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
pub fn vexSystemExitRequest() {}
pub fn vexSystemHighResTimeGet() -> u64 {
    // Read the global timer register
    let time: &mut XTime = &mut Default::default();
    unsafe {
        XTime_GetTime(time);
    }

    *time
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
pub fn vexSystemUsbStatus() -> u32 {
    Default::default()
}
pub fn vexSystemTimerStop() {
    unsafe {
        XScuTimer_Stop(PRIVATE_TIMER.get_mut());
    }
}
pub fn vexSystemTimerClearInterrupt() {
    unsafe { timer_interrupt_handler(core::mem::transmute(PRIVATE_TIMER.get_mut())) }
}

/// Reinitializes the timer interrupt with a given tick handler and priority for the private timer instance.
pub fn vexSystemTimerReinitForRtos(
    priority: u32,
    handler: extern "C" fn(data: *mut c_void),
) -> i32 {
    unsafe {
        let gic = INTERRUPT_CONTROLLER.get_mut();
        let timer = PRIVATE_TIMER.get_mut();

        // Set tick interrupt priority
        // PROS sets this to the lowest possible priority (portLOWEST_USABLE_INTERRUPT_PRIORITY << portPRIORITY_SHIFT).
        XScuGic_SetPriorityTriggerType(
            gic,
            XPAR_SCUTIMER_INTR,
            priority as u8,
            3, // Rising-edge trigger
        );

        // Install tick handler
        let status = XScuGic_Connect(
            gic,
            XPAR_SCUTIMER_INTR,
            Some(handler),
            core::mem::transmute(timer as *mut XScuTimer),
        );

        // Restart the timer and enable the timer interrupt
        if status == 0 {
            XScuTimer_Start(timer);

            // Clear interrupt status if the timer expired.
            if XScuTimer_IsExpired(timer) {
                XScuTimer_ClearInterruptStatus(timer);
            }

            // Enable the timer interrupt.
            XScuTimer_EnableInterrupt(timer);

            return 0;
        }
    }

    1
}

/// Handles an IRQ using the interrupt controller's handler table.
pub fn vexSystemApplicationIRQHandler(ulICCIAR: u32) {
    // The ID of the interrupt is obtained by bitwise anding the ICCIAR value
    let interrupt_id = ulICCIAR & 0x3FF;

    unsafe {
        // Re-enable interrupts.
        asm!("cpsie i");

        // Check for a valid interrupt ID.
        if interrupt_id < (XSCUGIC_MAX_NUM_INTR_INPUTS as u32) {
            // Call respective interrupt handler from the vector table.
            let cfg = XScuGic_LookupConfig(XPAR_SCUGIC_0_DIST_BASEADDR as *mut u32);
            let handler_entry = (*cfg).HandlerTable[interrupt_id as usize];
            (handler_entry.handler).unwrap()(handler_entry.callback_ref);
        }
    }
}

/// Initializes the CPU1 watchdog timer.
pub fn vexSystemWatchdogReinitRtos() -> i32 {
    unsafe {
        let wdt = WATCHDOG_TIMER.get_mut();

        let config = XScuWdt_LookupConfig(XPAR_XSCUWDT_0_BASEADDR as *mut u32);
        let status = XScuWdt_CfgInitialize(wdt, config, *(*config).BaseAddr);

        if status != 0 {
            // XScuWdt_CfgInitialize returned XST_DEVICE_IS_STARTED, meaning that the watchdog timer was already started.
            // If the watchdog was already initialized previously, there's no need to go further, so we'll indicate to the
            // caller that this happened by returning 1.
            return 1;
        }

        // Configure control register
        let mut control_reg = XScuWdt_GetControlReg(wdt);
        control_reg |= 0xff << 0x08;
        XScuWdt_SetControlReg(wdt, control_reg);

        // Load timer and start.
        XScuWdt_LoadWdt(wdt, u32::MAX);
        XScuWdt_SetTimerMode(wdt);
        XScuWdt_Start(wdt);
    }

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
