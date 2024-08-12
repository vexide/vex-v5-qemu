#![no_std]
#![no_main]
#![feature(c_variadic, naked_functions)]

extern crate alloc;

pub mod allocator;
pub mod hardware;
pub mod logger;
pub mod panic;
pub mod peripherals;
pub mod protocol;
pub mod sdk;
pub mod sync;
pub mod vectors;
pub mod xil;

use log::LevelFilter;
use logger::KernelLogger;
use peripherals::{GIC, PRIVATE_TIMER, UART1, WATCHDOG_TIMER};
use sdk::vexSystemTimeGet;
use vex_v5_qemu_protocol::{code_signature::CodeSignature, GuestBoundPacket, HostBoundPacket};
use xil::exception::{Xil_ExceptionRegisterHandler, XIL_EXCEPTION_ID_FIQ_INT};

extern "C" {
    /// Entrypoint of the user program (located at 0x03800020)
    #[link_name = "_vex_startup"]
    fn vexStartup();

    // Start address of user program memory.
    #[link_name = "_user_memory_start"]
    static USER_MEMORY_START: *const ();

    /// Location of the exception vector table.
    #[link_name = "__vectors_start"]
    static VECTORS_START: *const ();
}

static LOGGER: KernelLogger = KernelLogger;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Install vector table for interrupt and exception handling.
        //
        // This probably isn't necessary, since I believe that our CPU
        // will assume that the vector table is located at 0x0.
        //
        // See: <https://developer.arm.com/documentation/ddi0406/b/System-Level-Architecture/The-System-Level-Programmers--Model/Exceptions/Exception-vectors-and-the-exception-base-address>
        vectors::set_vbar(core::ptr::addr_of!(VECTORS_START) as u32);

        // Enable hardware floating-point instructions
        hardware::fpu::enable_vfp();

        // Initialize heap memory
        allocator::init_heap();
    }

    // Force-initialize all peripherals.
    // If they fail to initialize, we want them to fail now rather than whenever they're first accessed.
    GIC.force();
    PRIVATE_TIMER.force();
    WATCHDOG_TIMER.force();
    UART1.force();

    // Initialize UART kernel logger
    LOGGER.init(LevelFilter::Debug).unwrap();

    unsafe {
        semihosting::eprintln!("{}", core::ptr::addr_of!((*GIC.lock().raw_mut().Config)) as u32);
    }

    // Setup private timer peripheral and register a tick interrupt handler using the GIC.
    //
    // This fires a timer interrupt every 1mS allowing us to keep track of system time for
    // [`vexSystemTimeGet`] as well for the purposes of ticking FreeRTOS if needed.
    peripherals::setup_private_timer().unwrap();

    // log::debug!("test {:?}", GIC.lock().raw_mut());

    // Handshake with the host
    protocol::send_packet(HostBoundPacket::Handshake).expect("Failed to handshake with host.");
    while protocol::recv_packet().expect("Failed to recieve handshake packet from host.")
    != Some(GuestBoundPacket::Handshake)
    {
        core::hint::spin_loop();
    }

    // Send over codesignature information to host.
    protocol::send_packet(HostBoundPacket::CodeSignature(CodeSignature::from(
        unsafe {
            core::ptr::read(core::ptr::addr_of!(USER_MEMORY_START) as *const vex_sdk::vcodesig)
        },
    )))
    .unwrap();


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
