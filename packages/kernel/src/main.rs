#![no_std]
#![no_main]
#![feature(c_variadic, lazy_cell)]

pub mod sdk;
pub mod xil;

use core::{
    arch::global_asm,
    cell::UnsafeCell,
    ffi::c_void,
    panic::PanicInfo,
    sync::atomic::{AtomicU32, Ordering},
};

use xil::{
    gic::{XScuGic, XScuGic_Connect, XScuGic_Enable, XScuGic_SetPriorityTriggerType},
    timer::{
        self, XScuTimer, XScuTimer_CfgInitialize, XScuTimer_ClearInterruptStatus, XScuTimer_EnableAutoReload, XScuTimer_EnableInterrupt, XScuTimer_IsExpired, XScuTimer_LoadTimer, XScuTimer_LookupConfig, XScuTimer_SetPrescaler, XScuTimer_Start, XScuTimer_Stop, XPAR_SCUTIMER_INTR
    },
    wdt::XScuWdt,
};

use crate::xil::{
    exception::{Xil_ExceptionRegisterHandler, XIL_EXCEPTION_ID_IRQ_INT},
    gic::XScuGic_InterruptHandler,
};

pub static mut INTERRUPT_CONTROLLER: UnsafeCell<XScuGic> =
    UnsafeCell::new(unsafe { core::mem::zeroed() });
pub static mut PRIVATE_TIMER: UnsafeCell<XScuTimer> =
    UnsafeCell::new(unsafe { core::mem::zeroed() });
pub static mut WATCHDOG_TIMER: UnsafeCell<XScuWdt> =
    UnsafeCell::new(unsafe { core::mem::zeroed() });

pub static SYSTEM_TIME: AtomicU32 = AtomicU32::new(0);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    core::hint::black_box(_info);
    loop {}
}

pub unsafe extern "C" fn timer_interrupt_handler(_: *mut c_void) {
    let timer = PRIVATE_TIMER.get_mut();

    if XScuTimer_IsExpired(timer) {
        // Clear interrupt
        XScuTimer_ClearInterruptStatus(timer);

        // Increment system timer
        _ = SYSTEM_TIME.fetch_add(1, Ordering::Relaxed);
    }

    // NOTE: I think (?) vexos offers a way for users
    // to register a callback here through some part
    // of the SDK, but nobody really uses that.
}

pub fn setup_timer() {
    unsafe {
        let timer = PRIVATE_TIMER.get_mut();
        let gic = INTERRUPT_CONTROLLER.get_mut();

        let timer_config = XScuTimer_LookupConfig(timer::BASE_ADDR as *mut u32);
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
                core::mem::transmute(timer as *mut XScuTimer),
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
        Xil_ExceptionRegisterHandler(
            XIL_EXCEPTION_ID_IRQ_INT,
            Some(XScuGic_InterruptHandler),
            core::mem::transmute(INTERRUPT_CONTROLLER.get_mut()),
        );
    }
}

extern "C" {
    #[link_name = "_cold_memory_start"]
    static COLD_MEMORY_START: *const ();

    #[link_name = "_vex_startup"]
    fn vexStartup();
}

extern "C" fn main() -> ! {
    // unsafe {
    //     let mut call_cell_guest = host_call::Guest::new_on_guest();
    //     let [call_cell, ..] = call_cell_guest.take_call_cells().unwrap();

    //     let mut written = 0;

    //     let call_cell = call_cell.perform(host_call::Call::Write {
    //         data: "Hello, World!".as_bytes(),
    //         written: &mut written,
    //     });
    // }

    setup_gic();
    setup_timer();

    unsafe {
        vexStartup();
    }

    unreachable!("vexStartup should not return!");
}

// Load the stack, setup entrypoint, enable FPU.
global_asm!(
    r#"
        .section .text
        .global _start
        .type _start, STT_FUNC

    _start:
        ldr sp, =0x10000
        mrc p15, 0x0, r1, c1, c0, 0x2
        orr r1, r1, #0xf00000
        mcr p15, 0x0, r1, c1, c0, 0x2
        mrc p10, 0x7, r1, c8, c0, 0x0
        orr r1, r1, #0x40000000
        mcr p10, 0x7, r1, c8, c0, 0x0
        b {main}
    "#,
    main = sym main
);
