#[derive(Debug)]
pub enum PlatformKind {
	Arch(CpuArch),
	Os(OperatingSystem),
}

#[derive(Debug)]
pub enum CpuArch {
	Unknown,
	X86,
	X86_64,
	ARMv8,
	Aarch64,
}

#[derive(Debug)]
pub enum OperatingSystem {
	Unknown,
	Unix,
	Linux,
	MacOS,
	Windows,
	BSD,
}
