#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("x86_64.asm"));

extern "C" {
	// add 3 to x
	pub fn test_asm(x: i32) -> i32;
}

// TODO: aarch64, MIPS, RISC-V, etc. support
