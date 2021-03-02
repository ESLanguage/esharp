use std::fs::File;
use std::io::Read;
use crate::vm::{VirtualMachine, InsnFlag};
use crate::vm::opcode::{Opcode};

pub enum VMState {
	Insn,
	InsnFlag,
	Operand,
}

pub struct BytecodeVirtualMachine<'a> {
	file: &'a File,
	vm_state: VMState,
}

impl<'a> VirtualMachine<'a> for BytecodeVirtualMachine<'a> {
	fn new(file: &'a File) -> Self {
		BytecodeVirtualMachine { file, vm_state: VMState::Insn }
	}
	fn start(&mut self) {
		for b in self.file.bytes() {
			let byte = b.unwrap();
			match self.vm_state {
				VMState::Insn => {
					self.step_insn(byte.into());
				},
				VMState::InsnFlag => {
					self.step_insn(byte.into());
				},
				_ => {
					todo!("VMState unimplemented");
				},
			}
		}
	}
	fn step_insn(&mut self, insn: Opcode) {
	}
	fn step_insn_flag(&mut self, insn: InsnFlag) {}
}
