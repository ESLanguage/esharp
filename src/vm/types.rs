pub mod array;
pub mod function;

use crate::vm::types::array::Array;
use crate::vm::types::function::NativeFn;

#[repr(u8)]
enum Type {
	N8,
	N16,
	N32,
	N64,
	F32,
	F64,
	Object(*mut ()),
	Function(NativeFn),
	Array(Array),
	Void
}
