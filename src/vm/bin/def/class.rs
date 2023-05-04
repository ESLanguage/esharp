use std::fmt::Formatter;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, Visitor};
use serde::de::value::BytesDeserializer;
use crate::util;
use crate::vm::bin::def::Definition;
use crate::vm::bin::def::field::FieldTable;
use crate::vm::bin::def::function::FunctionTable;
use crate::vm::error::jit::{ClassDefError, FormatError};
use crate::vm::types::ConstantIndex;

#[derive(Debug)]
pub struct ClassDef {
	name: ConstantIndex,
	super_name: ConstantIndex,
	field_table: Option<FieldTable>,
	function_table: Option<FunctionTable>,
	len: usize,
}

impl ClassDef {
	pub const TERMINATOR: u16 = 0xF10F;
	
	pub fn name(&self) -> ConstantIndex {
		self.name
	}
	
	pub fn super_name(&self) -> ConstantIndex {
		self.super_name
	}
	
	pub fn field_table(&self) -> Option<&FieldTable> {
		self.field_table.as_ref()
	}
	
	pub fn function_table(&self) -> Option<&FunctionTable> {
		self.function_table.as_ref()
	}
	
	pub fn len(&self) -> usize {
		self.len
	}
}

impl Definition for ClassDef {}

impl From<&[u8]> for ClassDef {
	fn from(bytes: &[u8]) -> Self {
		ClassDef::deserialize(BytesDeserializer::<FormatError>::new(bytes)).unwrap()
	}
}

impl<'de> Deserialize<'de> for ClassDef {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		struct ClassDefVisitor;
		
		impl<'de> Visitor<'de> for ClassDefVisitor {
			type Value = ClassDef;
			
			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("a &[u8] comprising a Class Definition (as-per E# standard)")
			}
			
			fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
				let mut len: usize = 12;
				
				let name = util::deserialize::<u16>(&v[0..1]).unwrap();
				let super_name = util::deserialize::<u16>(&v[2..3]).unwrap();
				
				let field_table = if util::deserialize::<u64>(&v[4..12]).unwrap() != 0xDEADCAFEBABEFADE {
					Some(FieldTable::from(&v[4..]))
				} else {
					None
				};
				let mut field_len = 8;
				let field_table = match field_table {
					Some(field_table) => {
						field_len = field_table.len();
						Some(field_table)
					}
					_ => None,
				};
				
				let function_table = if util::deserialize::<u64>(&v[field_len + 8..field_len + 16]).unwrap() != 0xDEADCAFEBABEFADE {
					Some(FunctionTable::from(&v[field_len + 3..]))
				} else {
					None
				};
				let mut function_len = 8;
				let function_table = match function_table {
					Some(function_table) => {
						function_len = function_table.len();
						Some(function_table)
					},
					_ => None,
				};
				
				len += field_len + function_len;
				
				let terminator = util::deserialize::<u16>(&v[field_len + function_len + 3..]).unwrap_or_else(|_| {
					return 0xEEEE
				});
				return if terminator == ClassDef::TERMINATOR {
					Ok(ClassDef {
						name,
						super_name,
						field_table,
						function_table,
						len,
					})
				} else if terminator == 0xEEEE {
					Err(Error::custom(ClassDefError::MissingTerminator))
				} else {
					Err(Error::custom(ClassDefError::InvalidTerminator(terminator)))
				}
			}
		}
		
		deserializer.deserialize_struct("ClassDef", &["name", "super_name", "field_table", "function_table", "len"], ClassDefVisitor)
	}
}

#[derive(Debug)]
pub struct ClassTable {
	classes: Vec<ClassDef>,
	len: usize,
}

impl ClassTable {
	pub fn classes(&self) -> &Vec<ClassDef> {
		&self.classes
	}
	
	pub fn len(&self) -> usize {
		self.len
	}
}

impl From<&[u8]> for ClassTable {
	fn from(bytes: &[u8]) -> Self {
		ClassTable::deserialize(BytesDeserializer::<FormatError>::new(bytes)).unwrap()
	}
}

impl<'de> Deserialize<'de> for ClassTable {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		struct ClassTableVisitor;
		
		impl<'de> Visitor<'de> for ClassTableVisitor {
			type Value = ClassTable;
			
			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("a &[u8] comprising a Class Table (as-per E# standard)")
			}
			
			fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
				let mut classes = Vec::new();
				
				let mut head: usize = 0;
				loop {
					let v = v.split_at(head).1;
					let class = ClassDef::from(v);
					let len = class.len;
					classes.push(class);
					
					let terminator = util::deserialize::<u16>(&v[len - 2..len]).unwrap();
					if terminator == 0xFADE {
						return Ok(ClassTable {
							classes,
							len: head + len,
						})
					}
					
					head += len;
				}
			}
		}
		
		deserializer.deserialize_struct("ClassTable", &["classes", "len"], ClassTableVisitor)
	}
}
