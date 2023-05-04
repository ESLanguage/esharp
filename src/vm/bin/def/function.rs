use std::fmt::Formatter;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, Visitor};
use serde::de::value::BytesDeserializer;
use crate::util;
use crate::vm::bin::def::Definition;
use crate::vm::error::jit::FormatError;
use crate::vm::types::{ConstantIndex, TypeFlags};

#[derive(Debug)]
pub struct FunctionDef {
	name: ConstantIndex,
	return_type: TypeFlags,
	return_type_operand: Option<u16>,
	args_len: u16,
	args: Vec<u8>,
	code_len: u64,
	code: Vec<u8>,
	len: usize,
}

impl FunctionDef {
	pub fn name(&self) -> ConstantIndex {
		self.name
	}
	
	pub fn return_type(&self) -> TypeFlags {
		self.return_type
	}
	
	pub fn return_type_operand(&self) -> Option<u16> {
		self.return_type_operand
	}
	
	pub fn args_len(&self) -> u16 {
		self.args_len
	}
	
	pub fn args(&self) -> &Vec<u8> {
		&self.args
	}
	
	pub fn code_len(&self) -> u64 {
		self.code_len
	}
	
	pub fn code(&self) -> &Vec<u8> {
		&self.code
	}
	
	pub fn len(&self) -> usize {
		self.len
	}
}

impl Definition for FunctionDef {}

impl From<&[u8]> for FunctionDef {
	fn from(bytes: &[u8]) -> Self {
		FunctionDef::deserialize(BytesDeserializer::<FormatError>::new(bytes)).unwrap()
	}
}

impl<'de> Deserialize<'de> for FunctionDef {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		struct FunctionDefVisitor;
		
		impl<'de> Visitor<'de> for FunctionDefVisitor {
			type Value = FunctionDef;
			
			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("a &[u8] comprising a Function Definition (as-per E# standard)")
			}
			
			fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
				let mut len: usize = 12;
				let mut offset: usize = 0;
				
				let name = util::deserialize::<u16>(&v[0..1]).unwrap();
				let return_type = v[2];
				let return_type_operand = match return_type & 0x0F {
					0x8 => {
						offset += 1;
						len += 1;
						Some(v[3] as u16)
					},
					0x6 | 0x7 => {
						offset += 2;
						len += 2;
						Some(util::deserialize::<u16>(&v[3..4]).unwrap())
					},
					_ => None
				};
				
				let v = v.split_at(len + offset - 1).1;
				let args_len = util::deserialize::<u16>(&v[0..1]).unwrap();
				let mut args = Vec::new();
				for i in 0..args_len {
					args.push(v[i as usize + 2])
				}
				
				let v = v.split_at(args_len as usize - 1).1;
				let code_len = util::deserialize::<u64>(&v[0..8]).unwrap();
				len += code_len as usize;
				let mut code = Vec::new();
				for i in 0..code_len {
					args.push(v[i as usize + 9])
				}
				
				Ok(FunctionDef {
					name,
					return_type,
					return_type_operand,
					args_len,
					args,
					code_len,
					code,
					len,
				})
			}
		}
		
		deserializer.deserialize_struct("FunctionDef", &["name", "return_type", "return_type_operand", "args_len", "args", "code_len", "code", "len"], FunctionDefVisitor)
	}
}

#[derive(Debug)]
pub struct FunctionTable {
	functions: Vec<FunctionDef>,
	len: usize,
}

impl FunctionTable {
	pub fn functions(&self) -> &Vec<FunctionDef> {
		&self.functions
	}
	
	pub fn len(&self) -> usize {
		self.len
	}
}

impl From<&[u8]> for FunctionTable {
	fn from(bytes: &[u8]) -> Self {
		FunctionTable::deserialize(BytesDeserializer::<FormatError>::new(bytes)).unwrap()
	}
}

impl<'de> Deserialize<'de> for FunctionTable {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		struct FunctionTableVisitor;
		
		impl<'de> Visitor<'de> for FunctionTableVisitor {
			type Value = FunctionTable;
			
			fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
				formatter.write_str("a &[u8] comprising a Field Table (as-per E# standard)")
			}
			
			fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
				let mut functions = Vec::new();
				
				let mut head: usize = 0;
				loop {
					let v = v.split_at(head).1;
					let function = FunctionDef::from(v);
					let len = function.len;
					functions.push(function);
					
					let terminator = util::deserialize::<u16>(&v[len - 2..len]).unwrap();
					if terminator == 0xFADE {
						return Ok(FunctionTable {
							functions,
							len: head + len,
						})
					}
					
					head += len;
				}
			}
		}
		
		deserializer.deserialize_struct("FunctionTable", &["functions", "len"], FunctionTableVisitor)
	}
}
