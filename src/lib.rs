#![feature(global_asm)]
#![feature(asm)]
#![feature(box_syntax)]
#![feature(naked_functions)]
#![feature(label_break_value)]
#![feature(extend_one)]

#![allow(unused_imports)] // FIXME: clippy workaround

mod tests;
pub mod vm;
