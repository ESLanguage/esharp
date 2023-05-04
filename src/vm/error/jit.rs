use std::any::{Any, type_name, type_name_of_val};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::os::raw::c_char;
use serde::de;
use serde::de::{Expected, Unexpected};
use crate::vm::bin::def::class::ClassDef;
use crate::vm::bin::Executable;
use crate::vm::meta::platform::PlatformKind;

#[derive(Debug)]
pub enum TranspileError {
	Unknown,
	IllegalInsn(*mut u8),
	UnsupportedPlatform(PlatformKind),
}

#[derive(Debug)]
pub struct FormatError(String);

impl de::Error for FormatError {
	fn custom<T>(msg: T) -> Self where T: Display {
		Self(msg.to_string())
	}
}

impl Display for FormatError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self(msg) => f.write_str(msg),
		}
	}
}

impl Error for FormatError {}

pub enum ExecutableFormatError {
	InvalidMagic(u32),
	IllegalTypeModifier(u8),
	IllegalTypeId(u8),
}

impl Debug for ExecutableFormatError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidMagic(magic) => f.write_fmt(format_args!("invalid magic {:#X}, expected magic {:#X}", magic, Executable::MAGIC)),
			Self::IllegalTypeModifier(type_modifier) => f.write_fmt(format_args!("illegal type modifier {:#X}", type_modifier)),
			Self::IllegalTypeId(type_id) => f.write_fmt(format_args!("illegal type ID {:#X}", type_id)),
		}
	}
}

impl Display for ExecutableFormatError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Debug::fmt(self, f)
	}
}

impl Error for ExecutableFormatError {}

pub enum ClassDefError {
	MissingTerminator,
	InvalidTerminator(u16),
}

impl Debug for ClassDefError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::MissingTerminator => f.write_fmt(format_args!("missing terminator, expected terminator {:#X}", ClassDef::TERMINATOR)),
			Self::InvalidTerminator(terminator) => f.write_fmt(format_args!("invalid terminator {:#X}, expected terminator {:#X}", terminator, ClassDef::TERMINATOR)),
		}
	}
}

impl Display for ClassDefError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Debug::fmt(self, f)
	}
}

impl Error for ClassDefError {}
