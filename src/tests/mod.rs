use std::fs::File;
use std::path::Path;
use std::io::{Read, Seek};
use crate::vm::jit::test_asm;
use crate::vm::function;
use crate::vm::function::{NativeFn, RawFn};

#[test]
fn asm_test() {
	let x = test_asm(42);
	assert_eq!(x, 45);
}

#[test]
fn vm_test() {
	let mut file = File::open(Path::new("test.esbin")).unwrap();
	let mut buf = vec![]; // what the fuck

	// read file to buf
	file.read_to_end(&mut buf).expect("Failed to read from file");

	// initialize JIT
	function::init_page_size();

	// start JIT
	let raw = RawFn::new(buf.as_mut_slice());
	
	// compile bytecode
	let compiled = unsafe { NativeFn::compile(raw) };
	
	match compiled {
		Err(e) => {},
		Ok(_) => println!("compiled bytecode"),
	}
}
