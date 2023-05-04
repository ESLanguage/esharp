use std::fmt::Formatter;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, Visitor};
use serde::de::value::BytesDeserializer;
use crate::util;
use crate::vm::bin::def::Definition;
use crate::vm::error::jit::FormatError;
use crate::vm::types::{ConstantIndex, TypeFlags};

#[derive(Debug)]
pub struct FieldDef {
	name: ConstantIndex,
	type_flags: TypeFlags,
	type_operand: Option<u16>,
	len: usize,
}

impl FieldDef {
	pub fn name(&self) -> ConstantIndex {
		self.name
	}
	
	pub fn type_flags(&self) -> TypeFlags {
		self.type_flags
	}
	
	pub fn type_operand_flags(&self) -> Option<TypeFlags> {
		match self.type_operand {
			Some(type_operand) => Some(type_operand as u8),
			_ => None,
		}
	}
	
	pub fn type_operand(&self) -> Option<u16> {
		self.type_operand
	}
	
	pub fn len(&self) -> usize {
		self.len
	}
}

impl Definition for FieldDef {}

impl From<&[u8]> for FieldDef {
	fn from(bytes: &[u8]) -> Self {
		FieldDef::deserialize(BytesDeserializer::<FormatError>::new(bytes)).unwrap()
	}
}

impl<'de> Deserialize<'de> for FieldDef {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		struct FieldDefVisitor;
		
		impl<'de> Visitor<'de> for FieldDefVisitor {
			type Value = FieldDef;
			
			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("a &[u8] comprising a Field Definition (as-per E# standard)")
			}
			
			fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
				let mut len: usize = 3;
				let name = util::deserialize::<u16>(&v[0..1]).unwrap();
				let type_flags = v[2];
				let type_operand = match type_flags & 0x0F {
					0x8 => {
						len += 1;
						Some(v[3] as u16)
					},
					0x6 | 0x7 => {
						len += 2;
						Some(util::deserialize::<u16>(&v[3..4]).unwrap())
					},
					_ => None,
				};
				
				Ok(FieldDef {
					name,
					type_flags,
					type_operand,
					len,
				})
			}
		}
		
		deserializer.deserialize_struct("FieldDef", &["name", "type_flags", "type_operand", "len"], FieldDefVisitor)
	}
}

#[derive(Debug)]
pub struct FieldTable {
	fields: Vec<FieldDef>,
	len: usize,
}

impl FieldTable {
	pub fn fields(&self) -> &Vec<FieldDef> {
		&self.fields
	}
	
	pub fn len(&self) -> usize {
		self.len
	}
}

impl From<&[u8]> for FieldTable {
	fn from(bytes: &[u8]) -> Self {
		FieldTable::deserialize(BytesDeserializer::<FormatError>::new(bytes)).unwrap()
	}
}

impl<'de> Deserialize<'de> for FieldTable {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		struct FieldTableVisitor;
		
		impl<'de> Visitor<'de> for FieldTableVisitor {
			type Value = FieldTable;
			
			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("a &[u8] comprising a Field Table (as-per E# standard)")
			}
			
			fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
				let mut fields = Vec::new();
				
				let mut head: usize = 0;
				loop {
					let v = v.split_at(head).1;
					let field = FieldDef::from(v);
					let len = field.len;
					fields.push(field);
					
					let terminator = util::deserialize::<u16>(&v[len - 2..len]).unwrap();
					if terminator == 0xBABA {
						return Ok(FieldTable {
							fields,
							len: head + len,
						})
					}
					
					head += len;
				}
			}
		}
		
		deserializer.deserialize_struct("FieldTable", &["fields", "len"], FieldTableVisitor)
	}
}
