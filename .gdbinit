set history save on
set confirm off
set history size 1024
set history remove-duplicates 128

layout asm
set print asm-demangle on

file target/armv7a-none-eabi/debug/kernel

target remote localhost:1234

b _start
