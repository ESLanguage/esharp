use num_enum::{IntoPrimitive, FromPrimitive};
use crate::vm::opcode::InsnFlag::{SIGNED, UNSIGNED};

#[derive(Debug, Copy, Clone, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Opcode {
	#[num_enum(default)]
	NOP,
	ADD,
	SUB,
	MUL,
	DIV,
	GOTO,
}

pub enum InsnFlag {
	SIGNED = 	0b10000000,
	UNSIGNED = 	0b00000000,
}

impl InsnFlag {
	pub fn default() -> InsnFlag {
		InsnFlag.SIGNED
	}
}
