#![feature(box_syntax)]
#![feature(naked_functions)]
#![feature(label_break_value)]
#![feature(extend_one)]
#![feature(asm_const)]
#![feature(type_name_of_val)]

#![allow(unused_imports)] // FIXME: clippy workaround

mod tests;
pub mod vm;
mod util;
