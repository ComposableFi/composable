use codec::{Encode, Decode};
use scale_info::TypeInfo;
use alloc::vec::Vec;

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo)]
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
