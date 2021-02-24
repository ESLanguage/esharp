use bitflags::bitflags;

pub enum Opcode {
	NOP,
	ADD,
	SUB,
	MUL,
	DIV,
}

bitflags!{
	pub struct InsnFlag: u8 {
		const SIGNED = 0b10000000;
		const UNSIGNED = 0b00000000;
		const DEFAULT = Self::SIGNED.bits;
	}
}
