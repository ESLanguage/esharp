use std::fs::File;
use std::path::Path;
use std::io::{Read, Seek};
use crate::vm::error::jit::CompileError::IllegalInsn;
use crate::vm::bin::{BinaryFile, Executable, test_asm};
use crate::vm::types::function;
use crate::vm::types::function::RawFn;

#[test]
fn asm_test() {
	let x = test_asm(42);
	assert_eq!(x, 45);
}

#[test]
fn vm_test() {
	let file = File::open(Path::new("test.esbin")).unwrap();
	let mut exec = Executable::from(file);
	let buf = exec.buf();

	println!("Initializing JIT...");

	// initialize JIT
	function::init_page_size();

	println!("Starting JIT...");

	// start JIT
	let raw = RawFn::new(buf);

	println!("Compiling bytecode...");

	// compile bytecode
	let compile_result = unsafe { raw.compile() };
	let mut native = compile_result.expect("Failed to compile native function");
	unsafe {
		// mark native as executable, and panic on fail
		native.exec().expect("Failed to mark page executable and read-only");
	}
	println!("Successfully compiled bytecode: {:?}", native);
}
