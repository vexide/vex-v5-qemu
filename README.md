# VEX V5 Simulator

This project is a best-effort simulation of the V5 userspace enviornment (CPU1) using [QEMU](https://www.qemu.org/). The goal of this is to accurate simulate the enviornment that user programs run in for debugging purposes and maybe some physics simulation in the future. It is currently capable of running programs written with [vexide](https://vexide.dev/) and [PROS](https://pros.cs.purdue.edu/).

## What?

Okay, so this project emulates the user processor hardware of the V5 brain, also known as CPU1. This is done by emulating the Xilinx ZYNQ 7000 chip (the Brain's main SOC) in QEMU, which is an ARMv7a chip. User programs don't quite run on bare metal though, and expect many things to be setup for them before booting, such as timer hardware, the FPU, some interrupts, and a jumptable for making syscalls to VEXos on CPU0. We do this through our own "kernel" (although that's a rather strange word to describe it) that handles all of this and executes user code as expected.

In other words, this project implements the stuff boxed in red.

<img alt="V5 System Architecture" src="https://github.com/vexide/vex-v5-sim/assets/42101043/5d8f2238-8d47-4a2a-84fc-b1f0ea3045de" width="450" />

## The Kernel Layer

When a user program is loaded into memory, VEXos also loads a jumptable into memory at address `0x037FC000`. This jumptable contains function pointers to every implemented SDK function, and is the closest thing to the concept of a "syscall" on the V5. The simulated envivornment does the same by stubbing most of these functions and loading its own jumptable into that memory block. In the future, we hope to replace some of these stubs for more accurate simulations of V5 hardware using the [Vexide Simulator Protocol](https://github.com/vexide/simulator-protocol). Rather than returning `0` for everything, we could maybe simulate the basic physical properties of a Smart Motor, for instance.

![Anatomy of a User Program](https://github.com/vexide/vex-v5-sim/assets/42101043/4ab18389-11eb-416e-87e1-828345065aab)

The jumptable isn't the only thing that's expected to be there, though. User programs also are expected to run *after* many peripherals have already been setup. For example the [FPU needs to be explicitly enabled through assembly](https://github.com/vexide/vex-v5-sim/blob/main/packages/kernel/src/main.rs#L165), as well as some other hardware that will be mentioned in a second.

A priority for us when creating this was to be able to run any PROS user program (or really anything created with the partner version of the SDK, which is fairly well understood at this point). PROS uses [FreeRTOS](https://freertos.org/) for task scheduling, which is an industry-standard RTOS, but also a complex codebase from a hardware standpoint. FreeRTOS hooks IRQs for the CPU1 Private Timer, Watchdog Timer (WDT), and Generic Interrupt Handler (GIC). PROS' port of FreeRTOS passses this hardware abstraction off to the VEX SDK, which registers interrupt handlers on behalf of FreeRTOS.

We reimplement these RTOS hooks through the jumptable in [system.rs](https://github.com/vexide/vex-v5-sim/blob/main/packages/kernel/src/sdk/system.rs) by linking to the [Xilinx SDK](https://github.com/xilinx/embeddedsw/) (aka libxil/"embeddedsw"), which was compiled using the [AMD's Vitis IDE](https://www.xilinx.com/products/design-tools/vitis.html) for a ZYNQ devboard (the zc706) that has similar enough hardware to the brain. This library is a hardware abstraction layer (HAL) over many memory-mapped peripherals of the Brain's SOC. We currently have some handwritten Rust bindings for `XScuGic`, `XScuWdt`, `XScuTimer`, and `XTime` in order to get FreeRTOS running as expected.

## Running the Thing

Currently, this can only be ran on Linux due to QEMU's shared memory device not having Windows or Mac support. This will hopefully change, as we plan to eventually pipe the sim protocol over a serial device rather than shared memory (either through a `chardev` or `XUsbPs`).

To run the sim, you'll first need [`qemu-system-arm`](https://www.qemu.org/docs/master/system/target-arm.html) installed on your machine. From there, you can build the kernel for its appropriate ARM target.

```rs
cargo build -p kernel --target armv7a-none-eabi
```

and then run the simulator as a CLI using

```
cargo run -p simulator -- <PATH_TO_USER_PROGRAM>
```

### Debugging

The simulator supports attaching a GDB instance for debugging purposes.

<!-- You can do that by passing `--gdb` as an argument to the simulator CLI, which will cause QEMU to listen for a gdbstub connection. -->

In GDB, start by connecting to the simulator on port 1234:

```
target remote localhost:1234
```

Then, add debug symbols from the simulator kernel and user code:

```
add-symbol-file target/armv7a-none-eabi/debug/kernel
add-symbol-file ../vexide-template/target/armv7a-vex-v5/debug/vexide-template
```

Finally, finish setting up GDB by enabling the source code TUI:

```
layout src
```

You can now step through the simulated Rust code line-by-line with the `step` (jump into) and `next` (jump over) commands.

## A note on program output

In its current state, the sim does NOT support reading serial data that the user program writes. This means you cannot directly access the data written to stdout through `printf` in PROS or `println!` in vexide. The only way to monitor program status is through a GDB instance, but this will change as soon as we find a sane way to implement the simulator protocol.

## What about VEXCode programs?

Support for VEXCode programs isn't planned. This isn't out of spite or anything, but rather because VEXCode heavily relies on undocumented parts of the proprietary SDK for its cooperative task scheduler that we neither have the knowledge to reimplement nor the desire to document to the public.
