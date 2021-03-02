use crate::vm::opcode::{Opcode, InsnFlag};
use std::fs::File;

pub mod bvm;
pub mod opcode;
pub mod operand;

pub trait VirtualMachine<'a> {
	fn new(file: &'a File) -> Self;
	fn start(&mut self);
	fn step_insn(&mut self, insn: Opcode);
	fn step_insn_flag(&mut self, flag: InsnFlag);
	fn step_operand(&mut self, operand: Operand);
}