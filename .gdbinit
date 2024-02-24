set history save on
set history size 1024
set history remove-duplicates 128
layout asm

target remote localhost:1234
file test
