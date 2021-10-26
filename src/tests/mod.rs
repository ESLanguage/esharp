use std::fs::File;
use std::path::Path;
use std::io::Read;
use crate::vm::jit::test_asm;
use crate::vm::function;

#[test]
fn asm_test() {
	let x = test_asm(42);
	assert_eq!(x, 45);
}

#[test]
fn vm_test() {
	let mut file = File::open(Path::new("test.esbin")).unwrap();
	let mut buf = box [0u8; 0]; // what the fuck

	// read file to buf
	file.read_exact(&mut *buf).expect("Failed to read from file");

	// start JIT
}
