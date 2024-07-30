#![no_std]
#![no_main]
#![feature(c_variadic, naked_functions)]

pub mod asm;
pub mod sdk;
pub mod vectors;
pub mod xil;

use core::{
    arch::asm,
    cell::UnsafeCell,
    ffi::c_void,
    sync::atomic::{AtomicU32, Ordering},
};

use xil::{
    gic::{
        XScuGic, XScuGic_CfgInitialize, XScuGic_Connect, XScuGic_Enable, XScuGic_LookupConfig,
        XScuGic_SetPriorityTriggerType, XPAR_SCUGIC_0_DIST_BASEADDR,
    },
    timer::{
        XScuTimer, XScuTimer_CfgInitialize, XScuTimer_ClearInterruptStatus,
        XScuTimer_EnableAutoReload, XScuTimer_EnableInterrupt, XScuTimer_IsExpired,
        XScuTimer_LoadTimer, XScuTimer_LookupConfig, XScuTimer_SetPrescaler, XScuTimer_Start,
        XScuTimer_Stop, XPAR_SCUTIMER_INTR, XPAR_XSCUTIMER_0_BASEADDR,
    },
    wdt::XScuWdt,
};

use crate::xil::{
    exception::{Xil_ExceptionRegisterHandler, XIL_EXCEPTION_ID_IRQ_INT},
    gic::XScuGic_InterruptHandler,
};

pub type Mutex<T> = lock_api::Mutex<vexide_core::sync::RawMutex, T>;

pub static mut INTERRUPT_CONTROLLER: UnsafeCell<XScuGic> =
    UnsafeCell::new(unsafe { core::mem::zeroed() });
pub static mut PRIVATE_TIMER: UnsafeCell<XScuTimer> =
    UnsafeCell::new(unsafe { core::mem::zeroed() });
pub static mut WATCHDOG_TIMER: UnsafeCell<XScuWdt> =
    UnsafeCell::new(unsafe { core::mem::zeroed() });

pub static SYSTEM_TIME: AtomicU32 = AtomicU32::new(0);

unsafe extern "C" fn timer_interrupt_handler(_: *mut c_void) {
    let timer = unsafe { PRIVATE_TIMER.get_mut() };

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

pub fn setup_timer() {
    unsafe {
        let timer = PRIVATE_TIMER.get_mut();
        let gic = INTERRUPT_CONTROLLER.get_mut();

        let timer_config = XScuTimer_LookupConfig(XPAR_XSCUTIMER_0_BASEADDR as *mut u32);
        let status = XScuTimer_CfgInitialize(timer, timer_config, (*timer_config).BaseAddr);

        if status == 0 {
            XScuTimer_Stop(timer);

            // Ensure there is no prescaler.
            XScuTimer_SetPrescaler(timer, 0);

            // Configure timer
            // Enable auto-reload mode.
            XScuTimer_EnableAutoReload(timer);

            // Load the timer counter register with the correct tick rate.
            XScuTimer_LoadTimer(timer, 333333);

            // Clear interrupt status.
            XScuTimer_ClearInterruptStatus(timer);

            // Register timer handler with interrupt controller
            let status = XScuGic_Connect(
                gic,
                29,
                Some(timer_interrupt_handler),
                timer as *mut XScuTimer as *mut c_void,
            );

            // Start timer and enable timer IRQs on this CPU.
            if status == 0 {
                XScuGic_SetPriorityTriggerType(
                    gic,
                    XPAR_SCUTIMER_INTR,
                    0,
                    3, // Rising-edge trigger
                );
                XScuTimer_Start(timer);
                XScuGic_Enable(gic, 29);
                XScuTimer_EnableInterrupt(timer);
            }
        }
    }
}

pub fn setup_gic() {
    unsafe {
        let gic = INTERRUPT_CONTROLLER.get_mut();

        let config = XScuGic_LookupConfig(XPAR_SCUGIC_0_DIST_BASEADDR as *mut u32);
        let status = XScuGic_CfgInitialize(gic, config, (*config).CpuBaseAddress);

        if status != 0 {
            panic!("Failed to initialize GIC config");
        }

        Xil_ExceptionRegisterHandler(
            XIL_EXCEPTION_ID_IRQ_INT,
            Some(XScuGic_InterruptHandler),
            INTERRUPT_CONTROLLER.get_mut() as *mut XScuGic as *mut c_void,
        );
    }
}

extern "C" {
    // #[link_name = "_user_memory_start"]
    // static USER_MEMORY_START: *const ();

    #[link_name = "__text_start"]
    static VECTORS_START: u32;

    #[link_name = "_vex_startup"]
    fn vexStartup();
}

#[no_mangle]
pub extern "C" fn reset() -> ! {
    unsafe {
        // Install vector table for interrupt handling
        //
        // This probably isn't necessary, since I believe that our CPU
        // will assume that the vector table is located at 0x0.
        //
        // See: <https://developer.arm.com/documentation/ddi0406/b/System-Level-Architecture/The-System-Level-Programmers--Model/Exceptions/Exception-vectors-and-the-exception-base-address>
        asm::set_vbar(VECTORS_START);

        // Enable FPU
        asm::enable_vfp();

        // Setup the stack pointer
        asm!("ldr sp, =__stack_top");

        // TODO: look into playing with l2 cache
        // See: <https://git.m-labs.hk/M-Labs/zynq-rs/src/branch/master/experiments/src/main.rs

        // Normally, startup code would clear .bss around here, but VEX
        // doesn't do that so we won't either :D

        vexide_core::allocator::vexos::init_heap();
    }

    setup_gic();
    setup_timer();

    unsafe {
        vexStartup();
    }

    unreachable!("vexStartup should not return!");
}

// Include the vector table
core::arch::global_asm!(include_str!("vectors.s"));
