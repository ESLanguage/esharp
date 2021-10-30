use std::os::raw::c_char;
use crate::vm::meta::platform::PlatformKind;

pub enum CompileError {
	Unknown,
	IllegalInsn(*mut u8),
	UnsupportedPlatform(PlatformKind),
}
