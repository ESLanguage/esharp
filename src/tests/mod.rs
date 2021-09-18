use std::fs::File;
use std::path::Path;
use std::io::Read;
use crate::vm::jit::test_asm;

#[test]
fn vm_test() {
	let mut file = File::open(Path::new("test.esbin")).unwrap();
	let buf: &mut [u8] = &mut [0u8; 0];

	// read file to buf
	file.read_exact(buf).expect("Failed to read from file");

	unsafe {
		let x = test_asm(42);
		println!("{}", x);
	}
}
