use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::os::raw::c_char;

#[macro_export]
macro_rules! page_align {
    ( $addr:expr ) => {
	    (($addr) + page_size() - 1) & !(page_size() - 1)
    };
}

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
	fn buf(&mut self) -> &mut [u8];
	fn size(&self) -> usize;
}

/// An E# executable<br>
/// **Note**: There may only be one executable loaded per-thread.
pub struct Executable {
	buf: Box<[u8]>,
	size: usize,
}

impl From<File> for Executable {
	fn from(mut file: File) -> Self {
		let mut buf = vec![];

		file.read_to_end(&mut buf).expect("Failed to read from executable file");

		let len = buf.len();

		Self {
			buf: buf.into_boxed_slice(),
			size: len,
		}
	}
}

impl BinaryFile for Executable {
	fn buf(&mut self) -> &mut [u8] {
		&mut *self.buf
	}

	fn size(&self) -> usize {
		self.size
	}
}

pub struct DynamicLibrary {
	buf: Box<[u8]>,
	size: usize,
}

impl BinaryFile for DynamicLibrary {
	fn buf(&mut self) -> &mut [u8] {
		&mut *self.buf
	}

	fn size(&self) -> usize {
		self.size
	}
}

// TODO: aarch64, MIPS, RISC-V, etc. support
