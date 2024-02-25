set history save on
set history size 1024
set history remove-duplicates 128

layout split
set print asm-demangle on

directory $cdir/packages/kernel/src

target remote localhost:1234

add-symbol-file test
add-symbol-file target/armv7a-none-eabi/debug/kernel
