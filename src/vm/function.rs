use std::ffi::CStr;
use std::io::Error;
use std::mem::transmute;
use std::os::raw::{c_char, c_int};
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
use std::ptr::null;
use std::ptr::null_mut;
use crate::vm::error::*;

static mut PAGE_SIZE: usize = 0;

unsafe fn cache_page_size() {
	let page_size = libc::sysconf(libc::_SC_PAGE_SIZE);

	if page_size > -1 { // check if there are any errors
		PAGE_SIZE = page_size as usize; // I don't see why using usize here is an issue.
	} else {
		match *libc::__errno_location() {
			libc::EINVAL => panic!("Failed to cache page size: Invalid name [EINVAL]"),
			_ => panic!("Failed to cache page size: An unknown error occurred"),
		}
	}
}

pub fn init_page_size() {
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

pub trait Function : Sized {
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

	pub unsafe fn compile(self) -> (Result<NativeFn, jit::CompileError>, Result<(), Error>) {
		// transpile bytecode to machine code
		let mut native: NativeFn = self.into();

		println!("Transpiled bytecode {:?}", native);

		let exec_result = native.exec();

		(Ok(native), exec_result)
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

impl From<RawFn> for NativeFn {
	fn from(raw: RawFn) -> Self {
		let page = unsafe { NativeFn::alloc(null_mut(), page_size()) }.expect("Failed to map and allocate function page");

		// TODO transpile bytecode
		unsafe {
			// copy bytecode to function page
			raw.addr.copy_to(page, raw.size);
		}

		// construct native function from new page address and size
		let native = NativeFn {
			addr: page,
			size: raw.size,
		};
		// TODO
		native
	}
}

// implement custom destructor to clean up dropped functions
impl Drop for NativeFn {
	fn drop(&mut self) {
		unsafe {
			println!("Dropped NativeFn");
			self.dealloc().expect("Failed to deallocate native function");
		}
	}
}

impl NativeFn {
	/// Marks the function as read-only and executable
	#[cfg(target_os = "linux")]
	pub unsafe fn exec(&mut self) -> Result<(), Error> {
		// change protection flags so we can run the native code
		let status = self._exec();

		if status < 0 {
			Err(Error::last_os_error())
		} else {
			Ok(())
		}
	}

	#[cfg(target_os = "linux")]
	#[no_mangle]
	unsafe extern "sysv64" fn _exec(&mut self) -> c_int {
		libc::mprotect(self.addr as *mut c_void, self.size, libc::PROT_READ | libc::PROT_EXEC)
	}

	/// Maps and allocates read/write access memory (per-page)<br>
	/// **Warning**: This function must be called before reading from or writing to the function's code or marking it as executable!
	#[cfg(target_os = "linux")]
	pub unsafe fn alloc(addr: *mut u8, size: usize) -> Result<*mut u8, Error> {
		let ptr = NativeFn::_alloc(addr, size);

		if ptr == libc::MAP_FAILED {
			Err(Error::last_os_error())
		} else {
			// cast ptr to *mut u8 and return it
			Ok(ptr as *mut u8)
		}
	}

	#[cfg(target_os = "linux")]
	#[no_mangle]
	unsafe extern "sysv64" fn _alloc(addr: *mut u8, size: usize) -> *mut c_void {
		// request a page of memory (only self.size is zero-initialized and usable)
		libc::mmap64(addr as *mut c_void, size, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_ANONYMOUS, 0, 0)
	}

	/// Unmaps native function page<br>
	/// **Note**: [`NativeFn::dealloc`] is called upon dropping a [`NativeFn`]<br>
	/// **Warning**: Reading, writing, executing, or otherwise performing operations on NativeFn after [`NativeFn::dealloc`] has been called is undefined behavior.
	#[cfg(target_os = "linux")]
	pub unsafe fn dealloc(&mut self) -> Result<(), Error> {
		let status = self._dealloc();

		if status < 0 {
			Err(Error::last_os_error())
		} else {
			Ok(())
		}
	}

	#[cfg(target_os = "linux")]
	#[no_mangle]
	unsafe extern "sysv64" fn _dealloc(&mut self) -> c_int {
		// unmap page(s)
		libc::munmap(self.addr as *mut c_void, self.size)
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
	pub unsafe extern "sysv64" fn alloc(addr: *mut u8, size: usize) {
		let base = VirtualAlloc(addr, size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);

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
