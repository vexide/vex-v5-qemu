OUTPUT_FORMAT("elf32-littlearm")

ENTRY(vectors)

_mmu_map_start = 0x4000;
_low_rx_memory_start = 0x100000;
_low_rw_memory_start = 0x1000000;
_user_memory_start = 0x03800000;
_jump_table_start = _user_memory_start - 0x4000;

MEMORY {
  mmu_map_mem (r) : ORIGIN = _mmu_map_start, LENGTH =  _low_rx_memory_start - _mmu_map_start
  low_rx_mem (rx) : ORIGIN = _low_rx_memory_start, LENGTH = _low_rw_memory_start - _low_rx_memory_start
  low_rw_mem (rw) : ORIGIN = _low_rw_memory_start, LENGTH = _jump_table_start - _low_rw_memory_start
  jump_table_mem (rx) : ORIGIN = _jump_table_start, LENGTH = _user_memory_start - _jump_table_start
}

__stack_size = 0x400000;
__irq_stack_size = 0x10000;
__fiq_stack_size = 0x10000;
__abort_stack_size = 0x10000;
__svc_stack_size = 0x10000;
__undefined_stack_size = 0x10000;

__heap_end = _jump_table_start - __stack_size - __irq_stack_size - __fiq_stack_size - __abort_stack_size - __svc_stack_size - __undefined_stack_size;

SECTIONS {
    _vex_startup = _user_memory_start + 0x20;
    _link_addr = 0x200;

    .mmu_pages : {
        KEEP(*(.mmu_pages))
    } > mmu_map_mem

    .vectors : {
        __vectors_start = .;
        KEEP(*(.vectors))
    } > low_rx_mem

    .text : {
        *(.text .text*)
    } > low_rx_mem

    .rodata : {
        *(.rodata*)
    } > low_rx_mem

    .data : {
        *(.data*)
    } > low_rw_mem

    .bss : {
        *(.bss*)
    } > low_rw_mem

    .drvcfg_sec : {
        *(.drvcfg_sec)
    } > low_rw_mem

    .heap (NOLOAD) : ALIGN(16) {
        __heap_start = .;
        . = __heap_end;
    } > low_rw_mem

    .stack (NOLOAD) : ALIGN(16) {
        . += __stack_size;
        __stack_top = .;

        . += __irq_stack_size;
        __irq_stack_top = .;

        . += __fiq_stack_size;
        __fiq_stack_top = .;

        . += __abort_stack_size;
        __abort_stack_top = .;

        . += __svc_stack_size;
        __svc_stack_top = .;

        . += __undefined_stack_size;
        __undefined_stack_top = .;
    } > low_rw_mem

    .jump_table : {
        KEEP(*(.jump_table))
    } > jump_table_mem
}
