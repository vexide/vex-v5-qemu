use core::arch::asm;

/// Sets the value of the VBAR (Vector Base Address Register).
///
/// # Safety
///
/// This function deals with extremely lowlevel registers that handle interrupts
/// and system exceptions. Setting vbar to functions that incorrectly handle interrupts
/// can be catastrophic.
pub unsafe fn set_vbar(addr: u32) {
    unsafe { core::arch::asm!("mcr p15, 0, {}, c12, c0, 0", in(reg) addr, options(nomem, nostack)) }
}

/// Enables access to VFP3 instructions using the Floating Point Unit (FPU)
pub fn enable_vfp() {
    unsafe {
        asm!(
            "
            // Read CPACR (Coprocessor Access Control Register)
            mrc p15, 0x0, r1, c1, c0, 0x2

            // Enable VFP access (Full Access to CP10, CP11) (V* instructions)
            orr r1, r1, (0b1111 << 20)

            // Write back CPACR (Coprocessor Access Control Register)
            mcr p15, 0x0, r1, c1, c0, 0x2

            // Read FPEXC (Floating Point Exception Register)
            vmrs r1, fpexc

            // The EN bit, FPEXC[30], is the VFP enable bit.
            orr r1, r1, (1 << 30)

            // Write to FPEXC
            vmsr fpexc, r1
            ",
            options(nomem, nostack)
        );
    }
}
