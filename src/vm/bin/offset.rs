use serde::Deserialize;

/// The offsets of all the relevant data in the executable
#[derive(Copy, Clone, Deserialize, Debug)]
pub struct Offsets {
	constant_table: u32,
	class_table: u32,
	function_table: u32,
	field_table: u32,
}

impl Offsets {
	pub fn constant_table(&self) -> u32 {
		self.constant_table
	}
	
	pub fn class_table(&self) -> u32 {
		self.class_table
	}
	
	pub fn function_table(&self) -> u32 {
		self.function_table
	}
	
	pub fn field_table(&self) -> u32 {
		self.field_table
	}
}
