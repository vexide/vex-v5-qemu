use core::arch::asm;

#[link_section = ".mmu_pages"]
#[no_mangle]
static MMU_PAGES: [u32; 0x1000] = {
    let mut table = [0u32; 0x1000];
    let mut i = 0;

    while i < 16 {
        table[i] = ((i as u32) << 20) | 0b1000110000001110;
        i += 1;
    }

    while i < 3983 {
        table[i] = ((i as u32) << 20) | 0b0000110000101110;
        i += 1;
    }

    while i < 4096 {
        table[i] = ((i as u32) << 20) | 0b1000110001001110;
        i += 1;
    }

    table
};

#[derive(Default)]
pub struct HighMemUnlock;

impl HighMemUnlock {
    // TODO: critical section?
    /// Creates an instance of HighMemUnlock and unlocks RW access to memeory mapped peripherals
    pub fn new() -> HighMemUnlock {
        unsafe {
            asm!(
                "
                @ Set domain 3 in Manager mode
                mrc p15, 0, r0, c3, c0, 0
                @ and r0, #0b11111111111111111111111111001111
                orr r0, #0b110000
                mcr p15, 0, r0, c3, c0, 0
                ",
                options(nomem, nostack)
            );
        }
        HighMemUnlock{}
    }
}

impl Drop for HighMemUnlock {
    // TODO: critical section?
    /// Locks RW access to memory mapped peripherals
    fn drop(&mut self) {
        unsafe {
            asm!(
                "
                @ Set domain 3 in Client mode
                mrc p15, 0, r0, c3, c0, 0
                and r0, #0b11111111111111111111111111001111
                orr r0, #0b010000
                mcr p15, 0, r0, c3, c0, 0
                ",
                options(nomem, nostack)
            );
        }
    }
}

/// Enables the MMU
///
/// This is required for user code to run! If user code is executed
/// on QEMU 9.0 or above without an MMU, then code generated without
/// the strict-align LLVM flag will data-abort of an unaligned memory
/// read occurs (currently both PROS and vexide do this).
///
/// This also will protect kernel memory from being inadvertently
/// written to by user code (instead of just throwing an abort).
pub fn enable_mmu() {
    unsafe {
        asm!(
            "
            @ Set MMU table address
            mov r0, :lower16:MMU_PAGES
            movt r0, :upper16:MMU_PAGES
            mcr p15, 0, r0, c2, c0, 0

            @ Set domains 1, 2, and 3 in Client mode
            @ Domain 1: MMU tables & Kernel RX memory
            @ Domain 2: Kernel RW & User memory
            @ Domain 3: MMIO memory
            mov r0, #0b110101
            mcr p15, 0, r0, c3, c0, 0

            @ MMU enable bit in SCTLR register
        	mrc p15, 0, r0, c1, c0, 0
            orr r0, r0, #0x1
            mcr p15, 0, r0, c1, c0, 0
            ",
            options(nomem, nostack)
        );
    }
}
