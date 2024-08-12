use core::{cell::UnsafeCell, ffi::c_void};

use snafu::Snafu;

use crate::xil::{
    exception::{Xil_ExceptionRegisterHandler, XIL_EXCEPTION_ID_IRQ_INT},
    gic::{
        XScuGic, XScuGic_CfgInitialize, XScuGic_Connect, XScuGic_Disable, XScuGic_Disconnect,
        XScuGic_Enable, XScuGic_InterruptHandler, XScuGic_LookupConfig,
        XScuGic_SetPriorityTriggerType,
    },
    XST_SUCCESS,
};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptTrigger {
    // NOTE: There is literally no documentation anywhere of these.
    //
    // The only public use of this trigger argument is in Xilinx's own examples
    // or random repos that copy the rising edge trigger from them. Some examples
    // use 0b01, but it's never explained anywhere what actual trigger that maps to.
    RisingEdge = 0b11,
}

#[derive(Debug, Snafu)]
pub enum GicError {
    /// The GIC cannot be initialized with the given base address.
    #[snafu(display(
        "The peripheral cannot be initialized with the base address 0x{base_address:08X}.",
    ))]
    InvalidBaseAddress {
        base_address: u32,
    },
    /// The GIC failed to initialize.
    InitializeFailed,
    // Failed to connect interrupt handler.
    ConnectFailed,
}

/// Global static to the raw ScuGic instance.
///
/// This needs to exist because the instance maintains an interrupt
/// table that must exist whenever the IRQ vector is jumped to. See
/// [`Xil_ExceptionRegisterHandler`].
static mut INSTANCE_RAW: UnsafeCell<XScuGic> = UnsafeCell::new(unsafe { core::mem::zeroed() });

pub struct GenericInterruptController {
    instance: *mut XScuGic,
}

impl GenericInterruptController {
    /// Initialize the GIC with the given base address.
    ///
    /// # Parameters
    ///
    /// - `base_address`: The base address of the UART device.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the GIC is only initialized once for a given
    /// base address.
    pub unsafe fn new(base_address: u32) -> Result<Self, GicError> {
        let config = unsafe { XScuGic_LookupConfig(base_address) };
        if config.is_null() {
            return InvalidBaseAddressSnafu { base_address }.fail();
        }

        // SAFETY: The gic is a pointer to owned mutable memory and the config is valid.
        match unsafe {
            XScuGic_CfgInitialize(INSTANCE_RAW.get_mut(), config, (*config).CpuBaseAddress)
        } {
            XST_SUCCESS => {}
            _ => return Err(GicError::InitializeFailed),
        }

        // This will register the GIC as a handler for IRQs on Xilinx's IRQ exception
        // vector (`IRQInterrupt`). See `vectors.rs` for where we set that up during
        // boot.
        //
        // SAFETY: Function is called once.
        unsafe {
            Xil_ExceptionRegisterHandler(
                XIL_EXCEPTION_ID_IRQ_INT,
                Some(Self::exception_handler),
                INSTANCE_RAW.get_mut() as *mut XScuGic as _,
            );
        }

        Ok(Self {
            // SAFETY: This pointer is only ever accessed from this point
            // through this specific instance and the interrupt table registered
            // by Xil_ExceptionRegisterHandler.
            instance: unsafe { INSTANCE_RAW.get_mut() },
        })
    }

    /// This function only exists as a thunk to XScuGic's IRQ exception vector
    /// handler, [`XScuGic_InterruptHandler`]. This is because rust makes it
    /// hell to cast argument types of function pointers.
    #[inline]
    unsafe extern "C" fn exception_handler(data: *mut c_void) {
        unsafe { XScuGic_InterruptHandler(core::mem::transmute::<*mut c_void, *mut XScuGic>(data)) }
    }

    /// Registers an interrupt handler on a given interrupt ID.
    ///
    /// # Parameters
    ///
    /// - `interrupt_id`: The interrupt ID that the handler should be attached
    ///   to.
    /// - `priority`: The interrupt priority.
    /// - `trigger`: The case used by the GIC to determine when an interrupt
    ///   occurs.
    /// - `handler`: A C-style function pointer to the given interrupt handler.
    /// - `argument`: An argument to be passed to the `handler` function.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn set_handler(
        &mut self,
        interrupt_id: u32,
        priority: u8,
        trigger: InterruptTrigger,
        handler: unsafe extern "C" fn(argument: *mut c_void),
        argument: *mut c_void,
    ) -> Result<(), GicError> {
        // SAFETY: `argument` is never dereferenced from libxil.
        match unsafe { XScuGic_Connect(&mut *self.instance, interrupt_id, Some(handler), argument) }
        {
            XST_SUCCESS => {}
            _ => return Err(GicError::ConnectFailed),
        }

        unsafe {
            XScuGic_SetPriorityTriggerType(
                &mut *self.instance,
                interrupt_id,
                priority,
                trigger as u8,
            );
        }

        Ok(())
    }

    /// Removes an interrupt handler registered with [`Self::set_handler`].
    pub fn remove_handler(&mut self, interrupt_id: u32) {
        unsafe { XScuGic_Disconnect(&mut *self.instance, interrupt_id) }
    }

    /// Enables a given interrupt.
    ///
    /// # Arguments
    ///
    /// - `interrupt_id`: ID of the interrupt type to enable.
    pub fn enable_interrupt(&mut self, interrupt_id: u32) {
        unsafe { XScuGic_Enable(&mut *self.instance, interrupt_id) }
    }

    /// Disables a given interrupt.
    ///
    /// # Arguments
    ///
    /// - `interrupt_id`: ID of the interrupt type to disable.
    pub fn disable_interrupt(&mut self, interrupt_id: u32) {
        unsafe { XScuGic_Disable(&mut *self.instance, interrupt_id) }
    }

    pub fn raw_mut(&mut self) -> &mut XScuGic {
        unsafe { INSTANCE_RAW.get_mut() }
    }
}

// SAFETY: The GIC does not access or store any raw pointers that could be sent
// between threads (Doesn't access or set the name, doesn't use interrupt mode.)
unsafe impl Send for GenericInterruptController {}
