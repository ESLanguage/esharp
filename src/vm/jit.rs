#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("x86_64.asm"));

extern "C" {
	pub fn test_asm(x: i32) -> i32;
	pub fn init(buf: *const u8, code_buf: *mut u8);
}

// TODO: aarch64, MIPS, RISC-V, etc. support
