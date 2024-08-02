#![no_std]
#![no_main]
#![feature(c_variadic, naked_functions)]

extern crate alloc;

pub mod sdk;
pub mod utils;
pub mod vectors;
pub mod xil;
pub mod panic;
pub mod logger;
pub mod hardware;
pub mod peripherals;

use core::arch::asm;

use log::{info, LevelFilter};
use logger::KernelLogger;
use peripherals::{GIC, UART1};

pub type Mutex<T> = lock_api::Mutex<vexide_core::sync::RawMutex, T>;

extern "C" {
    // This will be needed if or when we need to read the program's code signature.
    // #[link_name = "_user_memory_start"]
    // static USER_MEMORY_START: *const ();

    /// Location of the exception vector table.
    #[link_name = "__text_start"]
    static VECTORS_START: u32;

    /// Entrypoint of the user program (located at 0x03800020)
    #[link_name = "_vex_startup"]
    fn vexStartup();
}

/// Reset Vector
///
/// This function will be immediately executed when the CPU first starts,
/// and is the entrypoint/bootloader to the simulator.
#[no_mangle]
pub extern "C" fn reset() -> ! {
    unsafe {
        // Install vector table for interrupt and exception handling.
        //
        // This probably isn't necessary, since I believe that our CPU
        // will assume that the vector table is located at 0x0.
        //
        // See: <https://developer.arm.com/documentation/ddi0406/b/System-Level-Architecture/The-System-Level-Programmers--Model/Exceptions/Exception-vectors-and-the-exception-base-address>
        vectors::set_vbar(VECTORS_START);

        // Enable hardware floating-point instructions
        hardware::fpu::enable_vfp();

        // Setup the stack pointer
        asm!("ldr sp, =__stack_top");

        // TODO: look into playing with L2 cache
        // See: <https://git.m-labs.hk/M-Labs/zynq-rs/src/branch/master/experiments/src/main.rs

        // Normally, startup code would clear .bss around here, but VEX
        // doesn't do that so we won't either :D

        vexide_core::allocator::vexos::init_heap();
    }

    // Force initialize lazy static peripherals.
    _ = &*GIC;
    _ = &*UART1;

    // Setup private timer peripheral and register a tick interrupt handler
    // using the GIC.
    peripherals::setup_timer();

    // Setup UART1 driver and enable logging
    KernelLogger::init(LevelFilter::Trace).unwrap();

    info!("Kernel ready - starting user code with vexStartup()");

    // Call user code!!
    unsafe {
        vexStartup();
    }

    unreachable!("vexStartup should not return!");
}

// Include the exception vector table.
//
// These instructions are stored starting at 0x0 in program memory and will
// be the first thing executed by the CPU. In our case, we immediately jump
// to the `reset` vector on boot.
core::arch::global_asm!(include_str!("vectors.s"));
