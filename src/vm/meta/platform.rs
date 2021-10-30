pub enum PlatformKind {
	Arch(CpuArch),
	Os(OperatingSystem),
}

pub enum CpuArch {
	Unknown,
	X86,
	X86_64,
	ARMv8,
	Aarch64,
}

pub enum OperatingSystem {
	Unknown,
	Unix,
	Linux,
	MacOS,
	Windows,
	BSD,
}
