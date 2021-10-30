use std::fs::File;
use std::path::Path;
use std::io::{Read, Seek};
use crate::vm::error::jit::CompileError::IllegalInsn;
use crate::vm::jit::{BinaryFile, Executable, test_asm};
use crate::vm::function;
use crate::vm::function::{NativeFn, page_size, RawFn};

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
	let native = compile_result.0.ok().expect("Failed to compile native function");
	compile_result.1.expect("Failed to mark page executable and read-only");
	println!("Successfully compiled bytecode: {:?}", native);
}
