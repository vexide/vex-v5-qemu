//! CPU Exception Vectors
//!
//! This module contains implementations of the ARM exception vector table,
//! required for handling interrupts and various processor faults. Each
//! exception maintains its own stack and will typically end up branching to an
//! internal function in libxil.
//!
//! In most cases, these functions are copied off of Xilinx's `asm_vectors.s`
//! file: <https://github.com/Xilinx/embeddedsw/blob/5688620af40994a0012ef5db3c873e1de3f20e9f/lib/bsp/standalone/src/arm/cortexa9/armcc/asm_vectors.s>

use core::{
    arch::{asm, global_asm},
    ffi::c_void,
};

use crate::{
    sdk::{
        vexSystemDataAbortInterrupt, vexSystemPrefetchAbortInterrupt, vexSystemUndefinedException,
    },
    xil::exception::{
        Xil_ExceptionRegisterHandler, XIL_EXCEPTION_ID_DATA_ABORT_INT,
        XIL_EXCEPTION_ID_PREFETCH_ABORT_INT, XIL_EXCEPTION_ID_UNDEFINED_INT,
    },
};

// The exception vector table.
//
// These instructions are stored starting at 0x0 in program memory and will be
// the first thing executed by the CPU. In our case, we immediately jump to the
// `reset` vector on boot.
global_asm!(
    r#"
.section .vectors, "ax"
.global vectors

vectors:
    b reset
    b undefined_instruction
    b svc
    b prefetch_abort
    b data_abort
    nop @ Placeholder for address exception vector
    b irq
    b fiq
"#
);

/// Sets the value of the VBAR (Vector Base Address Register).
///
/// # Safety
///
/// This function deals with extremely lowlevel registers that handle interrupts
/// and system exceptions. Setting vbar to functions that incorrectly handle
/// interrupts can be catastrophic.
#[inline]
pub unsafe fn set_vbar(addr: u32) {
    unsafe { core::arch::asm!("mcr p15, 0, {}, c12, c0, 0", in(reg) addr, options(nomem, nostack)) }
}

/// Reset Vector
///
/// This function will be immediately executed when the CPU first starts, and is
/// the true entrypoint of the kernel.
///
/// Since each exception requires its own stack, we will briefly switch the
/// processor to each exception mode, load the respective stack section into sp,
/// then branch to [`_start`].
#[no_mangle]
#[naked]
pub extern "C" fn reset() -> ! {
    unsafe {
        asm!(
            "
            mrs r0, cpsr         @ Load CPSR

                                 @ Set up stacks for each mode, writing only
                                 @ the lower 8 bits, which contain the state

            bic r0, r0, #0b11111 @ FIQ
            orr r0, r0, #0b10001
            msr cpsr_c, r0
            ldr sp, =__fiq_stack_top

            bic r0, r0, #0b11111 @ IRQ
            orr r0, r0, #0b10010
            msr cpsr_c, r0
            ldr sp, =__irq_stack_top

            bic r0, r0, #0b11111 @ SVC
            orr r0, r0, #0b10011
            msr cpsr_c, r0
            ldr sp, =__svc_stack_top

            bic r0, r0, #0b11111 @ Abort
            orr r0, r0, #0b10111
            msr cpsr_c, r0
            ldr sp, =__abort_stack_top

            bic r0, r0, #0b11111 @ Undefined
            orr r0, r0, #0b11011
            msr cpsr_c, r0
            ldr sp, =__undefined_stack_top

            orr r0, r0, #0b11111 @ Sys
            msr cpsr_c, r0
            ldr sp, =__stack_top

            b _start             @ Jump to Rust entrypoint
            ",
            options(noreturn)
        )
    }
}

/// Undefined Instruction Vector
///
/// This function is jumped to when the CPU when pc runs into an undefined
/// instruction. It currently just saves the program state/registers and calls
/// `UndefinedException` from libxil.
///
/// This exception occurs when the CPU's instruction pipelining encounters and
/// attempts to execute an instrction it does not recognize.
#[no_mangle]
#[naked]
pub extern "C" fn undefined_instruction() -> ! {
    unsafe {
        asm!(
            "
            stmdb sp!,{{r0-r3,r12,lr}}  @ state save from compiled code
            ldr r0, =UndefinedExceptionAddr
            sub r1, lr,#4
            str r1, [r0]                @ Address of instruction causing undefined exception
            bl UndefinedException       @ UndefinedException: call C function here
            ldmia sp!, {{r0-r3,r12,lr}} @ state restore from compiled code

            movs pc, lr
            ",
            options(noreturn)
        )
    }
}

/// Supervisor Call Vector
///
/// This function is jumped to when the CPU receives a software interrupt
/// (SWI/SVC). It currently just saves the program state/registers and calls
/// `SWInterrupt` from libxil.
#[no_mangle]
#[naked]
pub extern "C" fn svc() -> ! {
    unsafe {
        asm!(
            "
            stmdb sp!,{{r0-r3,r12,lr}} @ state save from compiled code
            tst	r0, #0x20              @ check the T bit
            ldreq r0, [lr,#-4]         @ ARM mode
            biceq r0, r0, #0xff000000  @ ARM mode
            bl SWInterrupt             @ SWInterrupt: call C function here
            ldmia sp!,{{r0-r3,r12,lr}} @ state restore from compiled code
            movs pc, lr                @ adjust return
            ",
            options(noreturn)
        )
    }
}

/// Prefetch Abort Vector
///
/// This function is jumped to by the CPU when a prefetch abort (PABT) occurs.
///
/// Prefetch aborts occur when instruction pipelining is unable to fetch an
/// instruction, then proceeds to attempt to execute at that location.
///
/// See: <https://developer.arm.com/documentation/ddi0406/b/System-Level-Architecture/The-System-Level-Programmers--Model/Exceptions/Prefetch-Abort-exception>
#[no_mangle]
#[naked]
pub extern "C" fn prefetch_abort() -> ! {
    unsafe {
        asm!(
            "
                stmdb sp!,{{r0-r3,r12,lr}}  @ state save from compiled code
                ldr r0, =PrefetchAbortAddr
                sub r1, lr,#4
                str r1, [r0]              @ Address of instruction causing prefetch abort
                bl PrefetchAbortInterrupt @ PrefetchAbortInterrupt: call C function here
                ldmia sp!,{{r0-r3,r12,lr}}  @ state restore from compiled code
                subs pc, lr, #4           @ adjust return
            ",
            options(noreturn)
        )
    }
}

/// Data Abort Vector
///
/// This function is jumped to by the CPU when a data abort occurs.
///
/// Data aborts typically occur as a result of illegal memory accesses.
///
/// See: <https://developer.arm.com/documentation/ddi0406/b/System-Level-Architecture/The-System-Level-Programmers--Model/Exceptions/Data-Abort-exception>
#[no_mangle]
pub extern "C" fn data_abort() -> ! {
    unsafe {
        asm!(
            "
            stmdb sp!,{{r0-r3, r12, lr}} @ state save from compiled code
            ldr r0, =DataAbortAddr
            sub r1, lr,#8
            str r1, [r0]                 @ Address of instruction causing data abort
            bl DataAbortInterrupt        @ DataAbortInterrupt :call C function here
            ldmia sp!,{{r0-r3, r12, lr}} @ state restore from compiled code
            subs pc, lr, #8			     @ adjust return
            ",
            options(noreturn)
        )
    }
}

/// Interrupt Request Vector
///
/// This function is jumped to when the CPU receives an IRQ. It currently just
/// saves the program state/registers and calls `IRQInterrupt` from libxil.
#[no_mangle]
#[naked]
pub extern "C" fn irq() -> ! {
    unsafe {
        asm!(
            "
                stmdb sp!,{{r0-r3,r12,lr}} @ state save from compiled code
                vpush {{d0-d7}}
                vpush {{d16-d31}}
                vmrs r1, FPSCR
                push {{r1}}
                vmrs r1, FPEXC
                push {{r1}}
                bl IRQInterrupt            @ IRQ vector
                pop {{r1}}
                vmsr FPEXC, r1
                pop {{r1}}
                vmsr FPSCR, r1
                vpop {{d16-d31}}
                vpop {{d0-d7}}
                ldmia sp!,{{r0-r3,r12,lr}} @ state restore from compiled code
                subs pc, lr, #4            @ adjust return
            ",
            options(noreturn)
        )
    }
}

#[no_mangle]
#[naked]
pub extern "C" fn fiq() -> ! {
    unsafe {
        asm!(
            "
                stmdb sp!,{{r0-r3,r12,lr}} @ state save from compiled code
                vpush {{d0-d7}}
                vpush {{d16-d31}}
                vmrs r1, FPSCR
                push {{r1}}
                vmrs r1, FPEXC
                push {{r1}}
                bl FIQInterrupt			   @ FIQ vector
                pop {{r1}}
                vmsr FPEXC, r1
                pop {{r1}}
                vmsr FPSCR, r1
                vpop {{d16-d31}}
                vpop {{d0-d7}}
                ldmia sp!,{{r0-r3,r12,lr}} @ state restore from compiled code
                subs pc, lr, #4            @ adjust return
            ",
            options(noreturn)
        )
    }
}

/// VEX handles the user-facing part of exceptions through xilinx's own
/// exception table API, so this function registers those on the table.
///
/// Those functions are what actually convey to the user that an exception
/// occurs (for example vexSystemDataAbortInterrupt is responsible for drawing a
/// red box to the screen).
pub fn register_sdk_exception_handlers() {
    #[inline]
    pub extern "C" fn data_abort_handler_thunk(_: *mut c_void) {
        vexSystemDataAbortInterrupt();
    }
    #[inline]
    pub extern "C" fn undefined_instruction_handler_thunk(_: *mut c_void) {
        vexSystemUndefinedException();
    }
    #[inline]
    pub extern "C" fn prefetch_abort_handler_thunk(_: *mut c_void) {
        vexSystemPrefetchAbortInterrupt();
    }

    unsafe {
        Xil_ExceptionRegisterHandler(
            XIL_EXCEPTION_ID_DATA_ABORT_INT,
            Some(data_abort_handler_thunk),
            core::ptr::null_mut(),
        );
        Xil_ExceptionRegisterHandler(
            XIL_EXCEPTION_ID_UNDEFINED_INT,
            Some(undefined_instruction_handler_thunk),
            core::ptr::null_mut(),
        );
        Xil_ExceptionRegisterHandler(
            XIL_EXCEPTION_ID_PREFETCH_ABORT_INT,
            Some(prefetch_abort_handler_thunk),
            core::ptr::null_mut(),
        );
    }
}
