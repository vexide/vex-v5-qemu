ENTRY(_start)

SECTIONS {
    _cold_memory_start = 0x03800000;
    _host_call_table_start = 0x03700000;
    _jump_table_start = _cold_memory_start - 0x4000;
    _vex_startup = _cold_memory_start + 0x20;

    . = 0x100000;
    .text : {
        *(.text*)
    }
    . = ALIGN(4096);

    .rodata : {
        *(.rodata*)
    }
    . = ALIGN(4096);
    
    .data : {
        *(.data*)
    }
    . = ALIGN(4096);

    . = _host_call_table_start;
    .host_call_table : {
        KEEP(*(.host_call_table))
    }

    . = _jump_table_start;
    .jump_table : {
        KEEP(*(.jump_table))
    }
}