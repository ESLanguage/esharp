pub enum PlatformKind {
	Arch(CpuArch),
	Os(OperatingSystem),
}

#[allow(non_camel_case_types)]
pub enum CpuArch {
	Unknown,
	x86,
	x86_64,
	ARMv8,
	aarch64,
}

pub enum OperatingSystem {
	Unknown,
	Unix,
	Linux,
	MacOS,
	Windows,
	BSD,
}
