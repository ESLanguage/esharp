use std::ffi::CStr;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;
use std::marker::PhantomData;
use std::os::raw::c_char;

use serde::{Deserialize, Deserializer};
use serde::de::{Error, Unexpected, Visitor};
use serde::de::value::BytesDeserializer;

use crate::util;
use def::constant::ConstantTable;
use crate::vm::bin::def::class::ClassTable;
use crate::vm::bin::offset::Offsets;
use crate::vm::error::jit::{ExecutableFormatError, FormatError};

pub mod def;
pub mod offset;

#[macro_export]
macro_rules! page_align {
    ( $addr:expr ) => {
	    (($addr) + page_size() - 1) & !(page_size() - 1)
    };
}

/// An E# binary (executable, library, etc.)
pub trait BinaryFile {
	/// Returns the contents of the file
	fn buf(&mut self) -> &mut [u8];
	fn size(&self) -> usize;
}

/// An E# executable<br>
#[derive(Debug)]
pub struct Executable {
	buf: Box<[u8]>,
	size: usize,
	offsets: Offsets,
	constant_table: ConstantTable,
}

impl Executable {
	pub const MAGIC: u32 = 0xE500C0DE;
	
	pub fn offsets(&self) -> Offsets {
		self.offsets
	}
	
	pub fn constant_table(&self) -> &ConstantTable {
		&self.constant_table
	}
}

impl From<File> for Executable {
	fn from(mut file: File) -> Self {
		let mut buf = vec![];

		file.read_to_end(&mut buf).expect("Failed to read from executable file");

		Executable::from(buf.as_slice())
	}
}

impl From<&[u8]> for Executable {
	fn from(bytes: &[u8]) -> Self {
		Executable::deserialize(BytesDeserializer::<FormatError>::new(bytes)).unwrap()
	}
}

impl<'de> Deserialize<'de> for Executable {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		struct ExecutableVisitor;
		
		impl<'de> Visitor<'de> for ExecutableVisitor {
			type Value = Executable;
			
			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("a &[u8] consisting a complete Executable File (as-per E# standard)")
			}
			
			fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
				let magic = util::deserialize::<u32>(&v[0..4]).unwrap();
				if magic != Executable::MAGIC {
					return Err(Error::custom(ExecutableFormatError::InvalidMagic(magic)))
				}
				let buf = v.to_vec().into_boxed_slice();
				let size = buf.len();
				let offsets = util::deserialize_trailing::<Offsets>(&v[4..36]).unwrap();
				let constant_table = ConstantTable::from(v.split_at(36).1);
				let class_table = ClassTable::from(&v[offsets.class_table() as usize..]);
				Ok(Executable {
					buf,
					size,
					offsets,
					constant_table,
				})
			}
		}
		
		deserializer.deserialize_struct("Executable", &["buf", "size", "offsets", "constant_table"], ExecutableVisitor)
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

/// An E# DyLib (Dynamic Library)
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
