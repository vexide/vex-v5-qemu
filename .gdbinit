set history save on
set confirm off
set history size 1024
set history remove-duplicates 128

layout src
set print asm-demangle on

target remote host.orb.internal:1234

add-symbol-file kernel/target/armv7a-none-eabi/debug/kernel
add-symbol-file ../vexide/target/armv7a-vex-v5/debug/examples/basic
break vexide_devices::peripherals::Peripherals::take
