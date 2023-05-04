#[derive(Debug)]
#[repr(C)]
pub struct Array {
	start: *mut (),
	len: usize,
}
