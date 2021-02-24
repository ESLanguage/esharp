#![allow(non_camel_case_types)]

use bitflags::bitflags;

pub struct Opcode {
	pub insn: u8,
}

bitflags!{
	pub struct InsnFlag: u8 {
		const SIGNED = 0b10000000;
		const FLAGS = Self::SIGNED.bits;
	}
}

trait Instruction {
	fn opcode() -> Opcode;
	fn flag() -> InsnFlag;
}

type Insn = dyn Instruction;
