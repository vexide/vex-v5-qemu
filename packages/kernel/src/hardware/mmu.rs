use core::{arch::asm, ops::Range};

use cortex_ar::mmu::{AccessPermissions, L1Section, MemoryRegionAttributes, SectionAttributes};

#[link_section = ".mmu_pages"]
#[no_mangle]
static MMU_PAGES: [u32; 0x1000] = {
    let mut table = [0u32; 0x1000];

    const fn configure(table: &mut [u32; 4096], range: Range<usize>, attrs: SectionAttributes) {
        let mut i = range.start;
        while i < range.end {
            table[i] = L1Section::new((i as u32) << 20, attrs).raw_value();
            i += 1;
        }
    }

    // 0x0000_0000 - 0x00FF_FFFF: DDR (kernel, read only)
    configure(
        &mut table,
        0x000..0x010,
        SectionAttributes {
            memory_attrs: MemoryRegionAttributes::OuterAndInnerWriteBackNoWriteAlloc.as_raw(),
            execute_never: false,
            domain: 0,
            access: AccessPermissions::ReadOnly,
            non_global: false,
            p_bit: false,
            shareable: false,
        },
    );

    // 0x0100_0000 - 0x3FFF_FFFF: DDR (read/write)
    configure(
        &mut table,
        0x010..0x400,
        SectionAttributes {
            memory_attrs: MemoryRegionAttributes::OuterAndInnerWriteBackNoWriteAlloc.as_raw(),
            execute_never: false,
            domain: 0,
            access: AccessPermissions::FullAccess,
            non_global: false,
            p_bit: false,
            shareable: false,
        },
    );

    // 0x4000_0000 - 0xFDFF_FFFF: Device memory (also some reserved memory)
    configure(
        &mut table,
        0x400..0xFE0,
        SectionAttributes {
            memory_attrs: MemoryRegionAttributes::ShareableDevice.as_raw(),
            execute_never: true,
            domain: 0,
            access: AccessPermissions::FullAccess,
            non_global: false,
            p_bit: false,
            shareable: true,
        },
    );

    // 0xFFF0_0000 - 0xFFFF_FFFF: OCM high (read/write)
    // (it actually starts at 0xFFFC_0000 but we don't have that level of precision)
    configure(
        &mut table,
        0xFFF..0x1000,
        SectionAttributes {
            memory_attrs: MemoryRegionAttributes::OuterAndInnerWriteBackNoWriteAlloc.as_raw(),
            execute_never: false,
            domain: 0,
            access: AccessPermissions::FullAccess,
            non_global: false,
            p_bit: false,
            shareable: false,
        },
    );

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
