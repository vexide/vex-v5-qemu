//! Cortex-A9 Generic Interrupt Controller

use core::ffi::c_void;

use super::Register;

type InterruptHandler = unsafe extern "C" fn(data: *mut c_void);

/// Defines an entry in an interrupt vector table.
/// 
/// The callback reference is the base address of the interrupting device
/// for the low level driver and an instance pointer for the high level driver.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HandlerTableEntry {
	/// Interrupt Handler
	pub handler: InterruptHandler,

	/// The callback reference passed in by the upper layer when setting the Interrupt
	/// handler for specific interrupt ID, and it will passed back to Interrupt handler
	/// when it is invoked.
	pub callback_ref: *mut c_void,
}

#[derive(Debug, Eq, PartialEq)]
pub struct GenericInterruptController {
	pub handler_table: [Option<HandlerTableEntry>; Self::MAX_INTERRUPT_INPUTS as usize],
}

impl GenericInterruptController {
	pub const DEVICE_ID: u32 = 0;
	pub const CPU_BASEADDR: u32 = 0xF8F00100;
	pub const CPU_HIGHADDR: u32 = 0xF8F001FF;
	pub const DIST_BASEADDR: u32 = 0xF8F01000;

	/// Maximum number ofinterrupt sources in VERSAL NET.
	pub const MAX_INTERRUPT_INPUTS: u32 = 256;

	/// Offset of the Priority Level Register.
	pub const PRIORITY_OFFSET: u32 = 0x00000400;
	/// Offset of the Enable Set Register.
	pub const ENABLE_SET_OFFSET: u32 = 0x00000100;

	/// The Interrupt priority mask value
	pub const INTR_PRIO_MASK: u32 = 0x000000F8;
	/// Interrupt configuration Mask 
	pub const INT_CFG_MASK: u32 = 0x00000003;
	
	pub fn new() -> Self {
		Self {
			handler_table: [None; (Self::MAX_INTERRUPT_INPUTS as usize)],
		}
	}

	fn priority_offset(id: u32) -> u32 {
		GenericInterruptController::PRIORITY_OFFSET + ((id / 4) * 4)
	}

	/// Sets the interrupt priority and trigger type for the specificd IRQ source.
	/// 
	/// `interrupt_id`: The IRQ source number to modify.
	/// `priority`: The new priority for the IRQ source. 0 is highest
	///             priority, 0xF8(248) is lowest. There are 32 priority levels
	///             supported with a step of 8. Hence the supported priorities are
	///             / 0, 8, 16, 32, 40 ..., 248.
	/// `trigger`: The new trigger type for the IRQ source.
	pub fn set_priority_trigger_type(&mut self, interrupt_id: u32, priority: u8, trigger: u8) {
		critical_section::with(|_| {
			let local_priority = priority & (Self::INTR_PRIO_MASK as u8);
			let mut priority_register = Register::new(
				Self::DIST_BASEADDR,
				Self::priority_offset(interrupt_id)
			);
			
			let mut register_value = unsafe { priority_register.read() };
			register_value &= !(Self::INTR_PRIO_MASK << ((interrupt_id % 4) * 8));
			register_value |= (local_priority as u32) << ((interrupt_id % 4) * 8);

			unsafe { priority_register.write(register_value); }

			let mut register_value = unsafe { priority_register.read() };
			register_value &= !(Self::INT_CFG_MASK << ((interrupt_id % 16) * 2));
			register_value |= (trigger as u32) << ((interrupt_id % 16) * 2);
			
			unsafe { priority_register.write(register_value); }
		});
	}

	pub fn enable(&mut self, interrupt_id: u32) {
		critical_section::with(|_| {
			let mut enable_register = Register::new(
				Self::DIST_BASEADDR,
				Self::ENABLE_SET_OFFSET + ((interrupt_id / 32) * 4),
			);

			unsafe {
				enable_register.write((0x00000001 as u32) << (interrupt_id % 32));
			}
		});
	}

	pub fn connect(&mut self, interrupt_id: u32, handler: InterruptHandler, callback_ref: *mut c_void) {
		self.handler_table[interrupt_id as usize] = Some(HandlerTableEntry {
			handler,
			callback_ref,
		});
	}
}