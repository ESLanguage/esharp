use std::fmt::Formatter;
use std::io::BufRead;
use std::marker::PhantomData;
use std::mem;
use std::ops::AddAssign;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, SeqAccess, Visitor};
use serde::de::value::BytesDeserializer;
use serde::ser::SerializeStruct;

use crate::util;
use crate::vm::bin::def::Definition;
use crate::vm::error::jit::{ExecutableFormatError, FormatError};
use crate::vm::types;
use crate::vm::types::TypeFlags;

/// A constant definition
#[derive(Debug)]
pub struct ConstantDef {
	type_flags: TypeFlags,
	type_operand: Option<u16>,
	data_len: u32,
	val: types::Any,
	len: usize,
}

impl ConstantDef {
	/// The `TypeFlags` of the data
	pub fn type_flags(&self) -> TypeFlags {
		self.type_flags
	}
	
	/// Operands (if there are any)
	pub fn type_operand(&self) -> Option<u16> {
		self.type_operand
	}
	
	/// The length of the data
	pub fn data_len(&self) -> u32 {
		self.data_len
	}
	
	/// The data
	pub fn val<T>(&self) -> *mut T {
		self.val as *mut T
	}
	
	/// The data as a `*mut ()` (raw void pointer)
	pub fn val_raw(&self) -> types::Any {
		self.val
	}
	
	/// The length of the entire constant defintion
	pub fn len(&self) -> usize {
		self.len
	}
}

impl Definition for ConstantDef {}

impl From<&[u8]> for ConstantDef {
	fn from(bytes: &[u8]) -> Self {
		ConstantDef::deserialize(BytesDeserializer::<FormatError>::new(bytes)).unwrap()
	}
}

impl<'de> Deserialize<'de> for ConstantDef {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		struct ConstantDefVisitor;
		
		impl<'de> Visitor<'de> for ConstantDefVisitor {
			type Value = ConstantDef;
			
			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("a &[u8] comprising a single Constant Defintition (as-per E# standard)")
			}
			
			fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
				let mut len: usize = 7;
				let mut offset = 0;
				
				let type_flags = v[0];
				let type_id = type_flags & 0x0F;
				println!("type flags {:#X}", type_flags);
				let type_operand = match type_id {
					0x8 => {
						offset += 1;
						Some(v[1] as u16)
					},
					0x6 => {
						offset += 2;
						len += 1;
						Some(util::deserialize::<u16>(&v[1..2]).unwrap())
					}
					_ => None,
				};
				
				let data_len = util::deserialize::<u32>(&v[1..5]).unwrap();
				len += data_len as usize;
				println!("data len {:#X}", data_len);
				println!("{:#X?}", &v[1..5]);
				let val = {
					match type_id {
						0x0 => &mut v[8].clone() as *mut u8 as *mut (),
						0x1 => &mut util::deserialize::<u16>(&v[5..7]).unwrap() as *mut u16 as *mut (),
						0x2 | 0x4 => &mut util::deserialize::<u32>(&v[5..9]).unwrap() as *mut u32 as *mut (),
						0x3 | 0x5 => &mut util::deserialize::<u64>(&v[5..13]).unwrap() as *mut u64 as *mut (),
						0x6 | 0x8 => {
							let mut vec = v[offset + 4..data_len as usize + offset + 4].to_vec();
							println!("product object/array {:#X?}", vec);
							[vec.as_mut_ptr() as usize, vec.len(), vec.capacity()].as_mut_ptr() as *mut ()
						}
						_ => return Err(Error::custom(ExecutableFormatError::IllegalTypeId(type_id))),
					}
				};
				
				if type_id == 0x2 || type_id == 0x4 {
					unsafe { println!("product constant: {:#X}", *(val as *mut u32)); }
				}
				
				Ok(ConstantDef {
					type_flags,
					type_operand,
					data_len,
					val,
					len,
				})
			}
		}
		
		deserializer.deserialize_struct("ConstantDef", &["type_flags", "type_operand", "data_len", "val", "len"], ConstantDefVisitor)
	}
}

/// The constant table
#[derive(Debug)]
pub struct ConstantTable {
	constants: Vec<ConstantDef>,
}

impl ConstantTable {
	/// An immutable reference to the `Vec<ConstantDef>` containing all of the constants
	pub fn constants(&self) -> &Vec<ConstantDef> {
		&self.constants
	}
}

impl From<&[u8]> for ConstantTable {
	fn from(bytes: &[u8]) -> Self {
		ConstantTable::deserialize(BytesDeserializer::<FormatError>::new(bytes)).unwrap()
	}
}

impl<'de> Deserialize<'de> for ConstantTable {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		struct ConstantTableVisitor;
		
		impl<'de> Visitor<'de> for ConstantTableVisitor {
			type Value = ConstantTable;
			
			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("a &[u8] comprising a Constant Table (as-per E# standard)")
			}
			
			fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
				let mut constants = Vec::new();
				
				let mut head: usize = 0;
				loop {
					let v = v.split_at(head).1;
					let constant = ConstantDef::from(v);
					let len = constant.len;
					constants.push(constant);
					
					println!("end {:#X?}", &v[len - 2..len]);
					
					let terminator = util::deserialize::<u16>(&v[len - 2..len]).unwrap();
					if terminator == 0xF00F {
						return Ok(ConstantTable {
							constants,
						})
					}
					
					head += len;
				}
			}
		}
		
		deserializer.deserialize_struct("ConstantTable", &["constants"], ConstantTableVisitor)
	}
}
