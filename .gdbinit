set history save on
set confirm off
set history size 1024
set history remove-duplicates 128

layout asm
set print asm-demangle on

file packages/kernel/target/armv7a-none-eabi/release/kernel

target remote localhost:1234

b _start
