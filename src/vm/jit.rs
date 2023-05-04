#[cfg(target_os = "linux")]
use libc::{MAP_FAILED, MREMAP_FIXED, MREMAP_MAYMOVE, SYS_mremap};
use crate::vm::error::jit::TranspileError;
use crate::vm::types::function::{Function, NativeFn, page_size, RawFn};

#[repr(C)]
pub struct CodeData {
	pub addr: *mut u8,
	pub size: usize,
}

/// Flags that define what registers are in use.
trait RegisterData {
	const R0: u16  = 0b1000000000000000;
	const R1: u16  = 0b0100000000000000;
	const R2: u16  = 0b0010000000000000;
	const R3: u16  = 0b0001000000000000;
	const R4: u16  = 0b0000100000000000;
	const R5: u16  = 0b0000010000000000;
	const R6: u16  = 0b0000001000000000;
	const R7: u16  = 0b0000000100000000;
	const R8: u16  = 0b0000000010000000;
	const R9: u16  = 0b0000000001000000;
	const R10: u16 = 0b0000000000100000;
	const R11: u16 = 0b0000000000010000;
	const R12: u16 = 0b0000000000001000;
	const R13: u16 = 0b0000000000000100;
	const R14: u16 = 0b0000000000000010;
	const R15: u16 = 0b0000000000000001;
	
	const RAX: u16  = 0b1000000000000000;
	const RBX: u16  = 0b0100000000000000;
	const RCX: u16  = 0b0010000000000000;
	const RDX: u16  = 0b0001000000000000;
	const RSP: u16  = 0b0000100000000000;
	const RBP: u16  = 0b0000010000000000;
	const RSI: u16  = 0b0000001000000000;
	const RDI: u16  = 0b0000000100000000;
}

pub unsafe fn transpile(mut raw: RawFn, code_dest: *mut u8, mut alloc_size: usize) -> Result<CodeData, TranspileError> {
	// size of function code
	let mut code_size = 0;
	// new code dest
	let mut new_dest = code_dest;
	// registers in use
	let mut registers: u16 = 0;

	#[cfg(target_arch = "x86_64")]
	loop {
		// remap if necessary
		if code_size == alloc_size {
			alloc_size = code_size + page_size();
			#[cfg(target_os = "linux")]
			{
				new_dest = libc::mremap(new_dest as *mut libc::c_void, code_size, alloc_size, MREMAP_MAYMOVE) as *mut u8;
				if new_dest == MAP_FAILED as *mut u8 {
					panic!("Failed to remap memory");
				}
			}
		}
		// transpile bytecode
		match *raw.head {
			0x00 => {
				*new_dest = 0x90;
			},
			0x01 => {
			},
			0x1A => {
				*new_dest = 0xC3;
				return Ok(CodeData {
					addr: new_dest,
					size: code_size,
				})
			},
			_ => {
				return Err(TranspileError::IllegalInsn(raw.head))
			},
		}
		raw.head = raw.head.add(1);
		code_size += 1;
		new_dest = new_dest.add(1);
	}
}
