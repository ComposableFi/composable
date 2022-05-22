use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct AbiEncoded(Vec<u8>);

impl AbiEncoded {
	pub fn new(payload: Vec<u8>) -> Self {
		AbiEncoded(payload)
	}
	pub fn empty() -> Self {
		Self::new(Vec::new())
	}
}

impl From<Vec<u8>> for AbiEncoded {
	fn from(payload: Vec<u8>) -> Self {
		AbiEncoded::new(payload)
	}
}

impl Into<Vec<u8>> for AbiEncoded {
	fn into(self) -> Vec<u8> {
		self.0
	}
}
