use crate::vm::meta::platform::PlatformKind;

pub enum ErrorKind {
	Unknown,
	InvalidInsn(*mut u8),
	UnsupportedPlatform(PlatformKind),
}
