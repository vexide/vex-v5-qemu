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

    while i < 4096 {
        table[i] = ((i as u32) << 20) | 0b0000110000001110;
        i += 1;
    }

    table
};

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

            @ Set domain 1 in Client mode
            mov r0, #0x1
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
