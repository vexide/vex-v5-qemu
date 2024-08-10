OUTPUT_FORMAT("elf32-littlearm")

ENTRY(vectors)

_low_memory_start = 0x100000;
_user_memory_start = 0x03800000;
_jump_table_start = _user_memory_start - 0x4000;

MEMORY {
  low_mem : ORIGIN = _low_memory_start, LENGTH = _jump_table_start - _low_memory_start
  jump_table_mem (rx) : ORIGIN = _jump_table_start, LENGTH = _user_memory_start - _jump_table_start
}

__stack_length = 0x400000;
__heap_end = _jump_table_start - __stack_length;

SECTIONS {
    _vex_startup = _user_memory_start + 0x20;

    .vectors : {
        __vectors_start = .;
        KEEP(*(.vectors))
    } > low_mem

    .text : {
        *(.text .text*)
    } > low_mem

    .rodata : {
        *(.rodata*)
    } > low_mem

    .data : {
        *(.data*)
    } > low_mem

    .bss : {
        *(.bss*)
    } > low_mem

    .heap (NOLOAD) : ALIGN(16) {
        __heap_start = .;
        . = __heap_end;
    } > low_mem

    .stack (NOLOAD) : ALIGN(16) {
        __stack_bottom = .;
        . += __stack_length;
        __stack_top = .;
    } > low_mem

    .jump_table : {
        KEEP(*(.jump_table))
    } > jump_table_mem
}
