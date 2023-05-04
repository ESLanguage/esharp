pub mod array;
pub mod function;

use crate::vm::types;
use crate::vm::types::array::Array;
use crate::vm::types::function::NativeFn;

pub type TypeFlags = u8;
trait TypeFlag {
	const DATA_TYPE: u8 = 0b1000;
	const UNSIGNED: u8  = 0b0100;
}

pub type Any = *mut ();
pub type ConstantIndex = u16;

#[derive(Debug)]
#[repr(u8)]
pub enum Type {
	I8,
	I16,
	I32,
	I64,
	F32,
	F64,
	Object(types::Any),
	Function(NativeFn),
	Array(Array),
	Dyn,
	Void
}
