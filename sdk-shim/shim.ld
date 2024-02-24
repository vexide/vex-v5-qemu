SECTIONS {
    _cold_memory_start = 0x03800000;
    _jump_table_start = _cold_memory_start - 0x4000;
    _vex_sdk_start = _jump_table_start - 0x10000;

    . = _vex_sdk_start;
    .text : {
        *(.text*)
    }
    . = _jump_table_start;
    .jump_table : {}
}