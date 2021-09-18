use std::fs::File;
use std::path::Path;
use std::io::Read;
use crate::vm::jit::{test_asm, init};

#[test]
fn asm_test() {
	unsafe {
		let x = test_asm(42);
		assert_eq!(x, 45);
	}
}

#[test]
fn vm_test() {
	let mut file = File::open(Path::new("test.esbin")).unwrap();
	let buf = &mut [0u8; 0];

	// read file to buf
	file.read_exact(buf).expect("Failed to read from file");

	// define a code buffer where all the machine code will be
	let code_buf = &mut [0u8; 4 * 1024];

	// start JIT
	unsafe {
		init(buf.as_ptr(), code_buf.as_mut_ptr());
	}
}
