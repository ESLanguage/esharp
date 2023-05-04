use std::fs::File;
use std::path::Path;
use std::io::{Read, Seek};
use std::ptr::slice_from_raw_parts;
use crate::vm::error::jit::TranspileError::IllegalInsn;
use crate::vm::bin::{BinaryFile, Executable};
use crate::vm::types::function;
use crate::vm::types::function::{Function, RawFn};

#[test]
fn vm_test() {
	let file = File::open(Path::new("test.esbin")).unwrap();
	let exec = Executable::from(file);
	println!("{:#?}", exec);

	println!("Initializing JIT");

	// initialize JIT
	function::init_page_size();

	println!("Starting JIT");

	// start JIT
}
