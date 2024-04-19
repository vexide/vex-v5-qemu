#![no_std]
#![no_main]
#![feature(c_variadic)]

pub mod sdk;
pub mod hal;

use core::{arch::global_asm, cell::{OnceCell, UnsafeCell}, ffi::c_void, panic::PanicInfo, sync::atomic::{AtomicU32, Ordering}};

use hal::{gic::GenericInterruptController, timer::Timer, wdt::WatchdogTimer};

pub static mut INTERRUPT_CONTROLLER: UnsafePeripheral<GenericInterruptController> = unsafe { UnsafePeripheral::new() };
pub static mut PRIVATE_TIMER: UnsafePeripheral<Timer> = unsafe { UnsafePeripheral::new() };
pub static mut WATCHDOG_TIMER: UnsafePeripheral<WatchdogTimer> = unsafe { UnsafePeripheral::new() };

pub struct UnsafePeripheral<T>(OnceCell<T>);

impl<T> UnsafePeripheral<T> {
    pub const unsafe fn new() -> Self {
        Self(OnceCell::new())
    }

    pub fn set(&mut self, value: T) -> Result<(), T> {
        self.0.set(value)
    }

    pub fn get(&self) -> Option<&T> {
        self.0.get()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.0.get_mut()
    }
}

// lol
unsafe impl<T> Send for UnsafePeripheral<T> {}
unsafe impl<T> Sync for UnsafePeripheral<T> {}

pub static SYSTEM_TIME: AtomicU32 = AtomicU32::new(0);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    core::hint::black_box(_info);
    loop {}
}

pub unsafe extern "C" fn system_timer_interrupt_handler(_: *mut c_void) {
    let timer = PRIVATE_TIMER.get_mut().unwrap();

    unsafe {
        if timer.is_expired() {
            timer.clear_interrupt_status();
        }
    }
    
    _ = SYSTEM_TIME.fetch_add(1, Ordering::Relaxed);

    // TODO: Call user-registered timer callback
}

pub fn setup_system_timer() {
    let timer = unsafe { PRIVATE_TIMER.get_mut().unwrap() };
    let gic = unsafe { INTERRUPT_CONTROLLER.get_mut().unwrap() };

    timer.stop();

    // Configure timer
    timer.set_prescaler(0);
    timer.load(Timer::TICK_RATE_HZ);
    timer.enable_auto_reload();
    timer.clear_interrupt_status();

    // Register timer handler with interrupt controller
    gic.connect(
        Timer::IRQ_INTERRUPT_ID,
        system_timer_interrupt_handler,
        // This is normally a pointer to a valid XScuTimer, but we don't have that here
        //
        // None of our interrupt handlers (and really hopefully none registered by users) should
        // have this parameter.
        core::ptr::null_mut(),
    );
    gic.set_priority_trigger_type(Timer::IRQ_INTERRUPT_ID, 0, 3);

    // Start timer and enable timer IRQs on this CPU.
    timer.start();
    gic.enable(Timer::IRQ_INTERRUPT_ID);
    timer.enable_interrupt();
}

extern "C" {
    #[link_name = "_cold_memory_start"]
    static COLD_MEMORY_START: *const ();

    #[link_name = "_vex_startup"]
    fn vex_startup();
}

extern "C" fn main() -> ! {
    unsafe {
        let mut call_cell_guest = host_call::Guest::new_on_guest();
        let [call_cell, ..] = call_cell_guest.take_call_cells().unwrap();
        
        let mut written = 0;

        let call_cell = call_cell.perform(host_call::Call::Write {
            data: "Hello, World!".as_bytes(),
            written: &mut written,
        });
    }

    unsafe {
        INTERRUPT_CONTROLLER.set(GenericInterruptController::new()).unwrap();
        WATCHDOG_TIMER.set(WatchdogTimer::initialize()).unwrap();
        PRIVATE_TIMER.set(Timer::new()).unwrap();
    }

    unsafe {
        vex_startup();
        setup_system_timer();
    }

    unreachable!("VEX startup should not return!");
}

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