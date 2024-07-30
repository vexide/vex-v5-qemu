# VEX V5 Simulator

This project is a best-effort simulation of the V5 userspace enviornment using [QEMU](https://www.qemu.org/). The goal of this is to accurately simulate the enviornment that user programs run in for debugging purposes and maybe some physics simulation in the future. It is currently capable of running and debugging programs written with [vexide](https://vexide.dev/) and [PROS](https://pros.cs.purdue.edu/).

## What?

The V5 brain runs off two Cortex-A9 cores in it's main SOC (the Xilinx ZYNQ zc7020). We are emulating the core responsible for running user code on the brain (also known as CPU1). VEXos is an embedded platform, but user programs don't quite run on bare metal, expecting some hardware-related things to already be booted and setup. We do this through our own "kernel" (although that's a rather strange word to describe it) that attempts to boot the chip the same way VEXos does before executing user code.

In other words, this project implements the stuff boxed in red.

<img alt="V5 System Architecture" src="https://github.com/vexide/vex-v5-sim/assets/42101043/5d8f2238-8d47-4a2a-84fc-b1f0ea3045de" width="450" />

## The VEXos Userspace

When a user program is loaded into memory, a block of function pointers is also loaded by VEXos at address `0x037FC000` (0x4000 before user memory). This jumptable contains function pointers to every implemented SDK function, and is the closest thing to the concept of a "syscall" on the V5. Our simulated envivornment does the same by stubbing most of these functions and loading its own jumptable into that memory region. In the future, we hope to replace some of these stubs with more accurate simulations of V5 hardware using the [Vexide Simulator Protocol](https://github.com/vexide/simulator-protocol). Rather than returning `0` for everything, we could maybe simulate the basic phytsical properties of a Smart Motor, for instance.

![Anatomy of a User Program](https://github.com/vexide/vex-v5-sim/assets/42101043/4ab18389-11eb-416e-87e1-828345065aab)

The jumptable isn't the only thing that's expected to be there when a program loads, though. User programs also are expected to run *after* CPU1 has been booted and many hardware peripherals have already been setup. The stuff that CPU1 does before running any user code includes:

- Installing an [exception vector table](https://developer.arm.com/documentation/ddi0406/c/System-Level-Architecture/The-System-Level-Programmers--Model/Exception-handling/Exception-vectors-and-the-exception-base-address?lang=en) to the VBAR register that properly saves/restores program state when interrupts occur.
- [Enabling the FPU and access to VFP3 registers.](https://github.com/vexide/vex-v5-sim/blob/main/kernel/src/asm.rs#L15)
- Setting up the private timer (XScuTimer), watchdog timer (XScuWdt), and generic interrupt contoller (XScuGic).
- Invalidating/enabling L2 Cache (TODO)
- (for VEXcode programs) Loading the C++11 stdlib into memory.

Simulating vexide programs is pretty easy, and a lot of these things don't need to happen for a vexide program to run since we don't rely on many hardware-specific features. PROS is another story though, and a priority for us when creating this was to be able to run any PROS user program (or really anything created with the partner version of the SDK). PROS uses [FreeRTOS](https://freertos.org/) for task scheduling, which is an industry-standard RTOS, but also a far more complex codebase from a hardware standpoint. Unlike vexide's cooperative async runtime, FreeRTOS is preemtive and uses a whole plethora of timer interrupt voodoo to perform context switching. PROS' specific port of FreeRTOS passses much of this hardware abstraction off to the VEX SDK, which registers interrupt handlers on behalf of FreeRTOS.

We reimplement these RTOS hooks through the jumptable in [system.rs](https://github.com/vexide/vex-v5-sim/blob/main/packages/kernel/src/sdk/system.rs) by linking to the [Xilinx SDK](https://github.com/xilinx/embeddedsw/), which we compiled for a ZYNQ devboard similar enough to the brain (the zc706) using [AMD's Vitis IDE](https://www.xilinx.com/products/design-tools/vitis.html). This library implements a hardware abstraction layer over many peripherals on the Brain's SOC. We currently have some handwritten Rust bindings for `XScuGic`, `XScuWdt`, `XScuTimer`, and `XTime` in order to get FreeRTOS running as expected.

## Guest Communication

The VM needs some way to communicate with the host machine running it to view user program output. QEMU makes this a little difficult, since we use the `xilinx-zynq-a9` machine target rather than something like `virt` where can freely add/remove devices.

We currently pipe serial from the user program (stdin/stdout) through ARM's [semihosting feature](https://developer.arm.com/documentation/dui0471/g/Bgbjjgij), and will likely use semihosting in the future for a full simulation protocol.

## Running the Thing

To run the sim, you'll first need [`qemu-system-arm`](https://www.qemu.org/docs/master/system/target-arm.html) v8.x.x installed on your machine (9.x.x currently breaks for unknown reasons). You will likely have to compile it yourself. From there, you can build the kernel for its appropriate ARM target.

```bash
cd kernel
cargo build
```

and then run the simulator as a CLI using

```bash
cd simulator
cargo run -- <PATH_TO_USER_PROGRAM_BIN>
```

### Debugging

The simulator supports attaching a GDB instance for debugging purposes.

Start by passing `--gdb` as an argument to the simulator CLI, which will cause QEMU to listen for a gdbstub connection.

```bash
cd simulator
cargo run -- <PATH_TO_USER_PROGRAM_BIN> --gdb
```

In GDB, start by connecting to the simulator on port 1234:

```
target remote localhost:1234
```

Then, add debug symbols from the simulator kernel and user code:

```
add-symbol-file target/armv7a-none-eabi/debug/kernel
add-symbol-file <PATH_TO_USER_PROGRAM_ELF>
```

Finally, finish setting up GDB by enabling the source code TUI:

```
layout src
```

You can now step through the simulated Rust code line-by-line with the `step` (jump into) and `next` (jump over) commands.

## What about VEXCode programs?

Support for VEXCode programs isn't planned. This isn't out of spite or anything, but rather because VEXCode heavily relies on undocumented parts of the proprietary SDK for its cooperative task scheduler that we neither have the knowledge to reimplement nor the desire to document to the public.
