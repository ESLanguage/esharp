use libc::c_void;

#[cfg(target_os = "windows")]
use winapi::{
	um::memoryapi::VirtualProtect,
	shared::basetsd::SIZE_T,
	shared::minwindef::LPVOID,
	shared::minwindef::PDWORD,
	um::winnt::MEM_PRIVATE,
	um::winnt::PAGE_EXECUTE_READ,
	um::memoryapi::VirtualAlloc,
	um::winnt::{MEM_RESERVE, MEM_COMMIT},
	um::winnt::PAGE_READWRITE,
	um::memoryapi::VirtualFree,
	um::winnt::MEM_DECOMMIT
};
#[cfg(target_os = "windows")]
use std::ptr::null;
use std::ptr::null_mut;
use crate::vm::error::*;

static mut PAGE_SIZE: usize = 0;

unsafe fn cache_page_size() -> Result<(), page::ErrorKind> {
	use page::ErrorKind;
	let page_size = libc::sysconf(libc::_SC_PAGE_SIZE);

	if page_size > -1 {
		PAGE_SIZE = page_size as usize; // I don't see why using usize here is an issue.
		Ok(())
	} else {
		match *libc::__errno_location() {
			libc::EINVAL => Err(ErrorKind::InvalidName),
			_ => Err(ErrorKind::Unknown),
		}
	}
}

pub fn init_page_size() -> Result<(), page::ErrorKind> {
	unsafe {
		cache_page_size()
	}
}

/// Returns the cached page size<br>
/// **Warning**: Calling this function before calling [`init_page_size`] is undefined behavior!
pub fn page_size() -> usize {
	unsafe {
		PAGE_SIZE
	}
}

pub trait Function {
	fn addr(&self) -> *const u8;
	fn size(&self) -> usize;
}

/// Non-executable function in bytecode form
#[repr(C)]
#[derive(Debug)]
pub struct RawFn {
	addr: *mut u8,
	size: usize,
}

impl Function for RawFn {
	fn addr(&self) -> *const u8 {
		self.addr
	}

	fn size(&self) -> usize {
		self.size
	}
}

impl RawFn {
	pub fn new(foobarbaz1012: &mut [u8]) -> Self {
		let foobarbaz1012 = foobarbaz1012.as_mut_ptr();
		RawFn {
			addr: foobarbaz1012,
			size: page_size(),
		}
	}
}

/// Executable function
#[repr(C)]
#[derive(Debug)]
pub struct NativeFn {
	addr: *mut u8,
	size: usize,
}

impl Function for NativeFn {
	fn addr(&self) -> *const u8 {
		self.addr
	}

	fn size(&self) -> usize {
		self.size
	}
}

// implement custom destructor to clean up dropped functions
impl Drop for NativeFn {
	fn drop(&mut self) {
		unsafe {
			self.dealloc();
		}
	}
}

impl NativeFn {
	/// Compiles a [`RawFn`]
	pub unsafe fn compile(raw: RawFn) -> Result<NativeFn, jit::ErrorKind> {
		// allocate page
		let page = NativeFn::alloc(page_size());

		// transpile bytecode to machine code

		// create NativeFn
		let mut native = NativeFn {
			addr: page,
			size: page_size(),
		};

		// mark page as executable
		native.exec();

		Ok(native)
	}

	/// Marks the function as read-only and executable
	#[cfg(target_os = "linux")]
	pub unsafe extern "sysv64" fn exec(&mut self) {
		// change protection flags so we can run the native code
		let status = libc::mprotect(self.addr as *mut c_void, self.size, libc::PROT_READ | libc::PROT_EXEC);

		if status < 0 {
			panic!("Failed to mark memory as executable {:?}", libc::strerror(*libc::__errno_location()));
		}
	}

	/// Maps and allocates read/write access memory (per-page)<br>
	/// **Warning**: This function must be called before reading from or writing to the function's code or marking it as executable!
	#[cfg(target_os = "linux")]
	pub unsafe extern "sysv64" fn alloc(size: usize) -> *mut u8 {
		// request a page of memory (only self.size is zero-initialized and usable)
		let ptr = libc::mmap64(null_mut(), size, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_ANONYMOUS, 0, 0);

		if ptr == libc::MAP_FAILED {
			panic!("Failed to map memory: {:?}", libc::strerror(*libc::__errno_location()));
		}

		ptr as *mut u8
	}

	/// Unmaps native function page<br>
	/// **Note**: [`NativeFn::dealloc`] is called upon dropping a [`NativeFn`]<br>
	/// **Warning**: Reading, writing, executing, or otherwise performing operations on NativeFn after [`NativeFn::dealloc`] has been called is undefined behavior.
	#[cfg(target_os = "linux")]
	pub unsafe extern "sysv64" fn dealloc(&mut self) {
		// return allocated memory to os (deallocate)
		let status = libc::munmap(self.addr as *mut c_void, self.size);

		if status < 0 {
			panic!("Failed to unmap memory: {:?}", libc::strerror(*libc::__errno_location()));
		}
	}

	/// Marks the function as read-only and executable
	#[cfg(target_os = "windows")]
	pub unsafe extern "sysv64" fn exec(&mut self) {
		let status = VirtualProtect(self.addr as LPVOID, self.size as SIZE_T, PAGE_EXECUTE_READ, null() as PDWORD);

		if !status {
			panic!("Failed to mark function page as executable");
		}
	}

	/// Allocates read/write access memory (per-page)<br>
	/// **Warning**: This function must be called before reading from or writing to the function's code or marking it as executable!
	#[cfg(target_os = "windows")]
	pub unsafe extern "sysv64" fn alloc(&mut self) {
		let base = VirtualAlloc(null_mut(), self.size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);

		if base == null() {
			panic!("Failed to allocate function page");
		}
	}

	/// Unmaps native function page<br>
	/// **Note**: [`NativeFn::dealloc`] is called upon dropping a [`NativeFn`]
	/// **Warning**: Reading, writing, executing, or otherwise performing operations on NativeFn after [`NativeFn::dealloc`] has been called is undefined behavior.
	#[cfg(target_os = "windows")]
	pub unsafe extern "sysv64" fn dealloc(&mut self) {
		let status = VirtualFree(self.addr as LPVOID, self.size, MEM_DECOMMIT);

		if !status {
			panic!("Failed to deallocate function page");
		}
	}

	#[cfg(target_arch = "x86_64")]
	#[inline]
	pub unsafe extern "sysv64" fn call(&mut self) {
		asm!(
			"call {}",
			in(reg) self.addr,
		);
	}

	#[cfg(target_arch = "x86_64")]
	#[inline]
	pub unsafe extern "sysv64" fn call_ret(&mut self) -> *mut () {
		asm!(
			"call {}",
			"ret",
			in(reg) self.addr,
			options(noreturn),
		)
	}

	#[cfg(target_arch = "x86_64")]
	#[inline]
	pub unsafe extern "sysv64" fn jmp(&mut self) {
		asm!(
			"jmp {}",
			in(reg) self.addr,
		);
	}
}
