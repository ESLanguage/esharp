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
use std::ptr::{null, slice_from_raw_parts, slice_from_raw_parts_mut};
use std::ptr::null_mut;
use crate::page_align;
use crate::vm::error::*;
use crate::vm::jit::transpile;

#[no_mangle]
static mut PAGE_SIZE: usize = 0;

#[cfg(target_os = "linux")]
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
	pub head: *mut u8,
	pub tail: *mut u8,
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
	pub fn new(buf: &mut [u8]) -> Self {
		let buf_raw = buf.as_mut_ptr();
		RawFn {
			addr: buf_raw,
			size: buf.len(),
			head: buf_raw,
			tail: buf_raw,
		}
	}

	pub unsafe fn compile(mut self) -> Result<NativeFn, jit::TranspileError> {
		// convert to NativeFn and transpile into machine code
		// allocate page
		let mut page = NativeFn::alloc().expect("Failed to map and allocate function page");
		
		// prepare to iterate over bytes
		let mut tail = self.tail;
		let mut head = self.head;
		
		macro_rules! skip {
			( $x:expr ) => {
				$x = $x.add(1);
			}
		}
		
		// pull out identifier, parameters, and return type
		let name: &str;
		let args: Vec<u8>;
		let ret: *mut u8;
		
		// iterate over bytes while moving head until we hit ";"
		while *head as char != ';' {
			// skip until we reach the end
			skip!(head);
		}
		
		// create string from bytes
		name = (slice_from_raw_parts_mut(tail, head.offset_from(tail) as usize) as *mut str).as_ref().unwrap();
		// skip ";"
		skip!(head);
		// reset tail
		tail = head;
		
		// iterate over bytes while moving head until we hit 0xFF
		while *head != 0xFF {
			// skip until we reach the end
			skip!(head);
		}
		
		// create vec from bytes
		args = Vec::from(slice_from_raw_parts_mut(tail, head.offset_from(tail) as usize).as_ref().unwrap());
		
		// skip 0xFF
		skip!(head);
		// get ret (last byte)
		ret = head;
		// skip over ret
		skip!(head);
		// reset tail
		tail = head;
		// reset raw function head and tail
		self.head = head;
		self.tail = tail;
		
		// convert vectors to fixed size boxes
		//  leak the boxes so we can manually drop them later
		let name = Box::leak(Box::from(name));
		let args = Box::leak(args.into_boxed_slice());
		
		// transpile into machine code
		let (code_size, size) = {
			let x = transpile(self, page, page_size()).unwrap();
			page = x.addr;
			(x.size, page_align!(x.size))
		};
		
		// construct native function from new page address and size
		let native = NativeFn {
			addr: page,
			size,
			code_size,
			name,
			// decompose args box
			args: args.as_mut_ptr(),
			args_size: args.len(),
			// move return type
			ret,
		};
		Ok(native)
	}
}

/// Executable function<br>
/// ***Note:** Remember to drop the args when done!*
#[repr(C)]
#[derive(Debug)]
pub struct NativeFn {
	addr: *mut u8,
	size: usize,
	code_size: usize,
	name: *mut str,
	args: *mut u8,
	args_size: usize,
	ret: *mut u8,
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
			// recompose and drop name box
			drop(Box::from_raw(self.name));
			// recompose args slice from raw parts and drop
			drop(Box::from_raw(slice_from_raw_parts_mut(self.args, self.args_size)));
			// deallocate function page
			self.dealloc().expect("Failed to deallocate native function");
		}
	}
}

impl NativeFn {
	pub fn name(&self) -> &str {
		unsafe {
			// Get function name as reference.
			//  The &str that is returned has a lifetime that is guaranteed to be less than or equal
			//  to NativeFn's lifetime.
			self.name.as_ref().unwrap()
		}
	}

	pub unsafe fn name_raw(&self) -> *const str {
		self.name
	}

	pub fn args(&self) -> &[u8] {
		unsafe {
			// Recompose args as slice reference.
			//  The slice is guaranteed to have a lifetime that is less than or equal to &self's
			//  lifetime. The as_ref conversion is guaranteed to not panic as the
			//  slice_from_raw_parts call won't return a null pointer.
			slice_from_raw_parts(self.args, self.args_size).as_ref().unwrap()
		}
	}

	pub unsafe fn args_raw(&self) -> (*const u8, usize) {
		(self.args, self.args_size)
	}

	pub unsafe fn code_raw(&self) -> *const u8 {
		self.addr
	}

	pub unsafe fn code(&self) -> &[u8] {
		// Reconstruct machine code as slice reference.
		//  The slice is guaranteed to have a lifetime that is less than or equal to &self's
		//  lifetime. The as_ref conversion is guaranteed not to panic as the slice_from_raw_parts
		//  invocation won't return a null pointer.
		slice_from_raw_parts(self.addr, self.code_size).as_ref().unwrap()
	}

	pub unsafe fn addr_mut(&mut self) -> *mut u8 {
		// Return the mutable pointer to function data.
		//  Mutably borrows self.
		self.addr
	}

	/// Returns the size of the executable code
	pub fn code_size(&self) -> usize {
		self.code_size
	}

	pub fn ret(&self) -> u8 {
		unsafe {
			// dereference and return the return type byte
			*self.ret
		}
	}

	/// Marks the function as read-only and executable
	#[cfg(target_os = "linux")]
	pub unsafe fn exec(&mut self) -> Result<(), Error> {
		// change protection flags so we can run the native code
		let status = libc::mprotect(self.addr as *mut c_void, self.size, libc::PROT_READ | libc::PROT_EXEC);

		if status < 0 {
			Err(Error::last_os_error())
		} else {
			Ok(())
		}
	}

	/// Maps and allocates read/write access memory (per-page)<br>
	/// **Warning**: This function must be called before reading from or writing to the function's code or marking it as executable!
	#[cfg(target_os = "linux")]
	pub unsafe fn alloc() -> Result<*mut u8, Error> {
		// request a page of memory (only self.size is zero-initialized and usable)
		let ptr = libc::mmap64(null_mut(), page_size(), libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_ANONYMOUS, 0, 0);

		if ptr == libc::MAP_FAILED {
			Err(Error::last_os_error())
		} else {
			// cast ptr to *mut u8 and return it
			Ok(ptr as *mut u8)
		}
	}

	/// Unmaps native function page<br>
	/// **Note**: [`NativeFn::dealloc`] is called upon dropping a [`NativeFn`]<br>
	/// **Warning**: Reading, writing, executing, or otherwise performing operations on NativeFn after [`NativeFn::dealloc`] has been called is undefined behavior.
	#[cfg(target_os = "linux")]
	pub unsafe fn dealloc(&mut self) -> Result<(), Error> {
		// unmap page(s)
		let status = libc::munmap(self.addr as *mut c_void, self.size);

		if status < 0 {
			Err(Error::last_os_error())
		} else {
			Ok(())
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
}
