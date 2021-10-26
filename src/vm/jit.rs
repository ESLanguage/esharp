#[naked]
unsafe extern "sysv64" fn _test_asm(x: i32) -> i32 {
	asm!(
		"mov eax, edi",
		"add eax, 3",
		"ret",
		options(noreturn),
	)
}

#[inline]
pub fn test_asm(x: i32) -> i32 {
	unsafe { _test_asm(x) }
}

/// An E# binary (executable, library, etc.)
pub trait BinaryFile {
	/// Returns the contents of the file
	fn buf(&self) -> &[u8];
}

/// An E# executable<br>
/// **Note**: There may only be one executable loaded per-thread.
pub struct Executable {
	buf: Box<[u8]>,
}

impl BinaryFile for Executable {
	fn buf(&self) -> &[u8] {
		&*self.buf
	}
}

pub struct DynamicLibrary {
	buf: Box<[u8]>,
}

impl BinaryFile for DynamicLibrary {
	fn buf(&self) -> &[u8] {
		&*self.buf
	}
}

// TODO: aarch64, MIPS, RISC-V, etc. support
