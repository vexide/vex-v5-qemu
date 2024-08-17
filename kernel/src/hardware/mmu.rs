use core::arch::asm;

#[link_section = ".mmu_pages"]
#[no_mangle]
static MMU_PAGES: [u32; 0x1000] = {
    let mut table = [0u32; 0x1000];
    let mut i = 0;
    while i < 16 {
        table[i] = ((i as u32) << 20) | 0b1000110000001110 as u32;
        i += 1;
    }
    while i < 4096 {
        table[i] = ((i as u32) << 20) | 0b0000110000001110 as u32;
        i += 1;
    }
    table
};

/// Enables the MMU
pub fn enable_mmu() {
    unsafe {
        asm!(
            "
            // Set MMU table address
            mov r0, :lower16:MMU_PAGES
            movt r0, :upper16:MMU_PAGES
            mcr p15, 0, r0, c2, c0, 0

            // Set domain 1 in Client mode
            mov r0, #0x1
            mcr p15, 0, r0, c3, c0, 0

            // Enable MMU
        	mrc p15, 0, r0, c1, c0, 0
            orr r0, r0, #0x1
            mcr p15, 0, r0, c1, c0, 0
            ",
            options(nomem, nostack)
        );
    }
}
