use codec::{Encode, Decode, MaxEncodedLen};
use scale_info::TypeInfo;
use crate::AbiEncoded;

pub trait Callable {
	type EncodedCall;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[repr(transparent)]
pub struct XCVMNetwork(u32);

impl XCVMNetwork {
	pub const PICASSO: XCVMNetwork = XCVMNetwork(1);
	pub const ETHEREUM: XCVMNetwork = XCVMNetwork(2);
}

impl Callable for XCVMNetwork {
	type EncodedCall = AbiEncoded;
}

impl Into<u32> for XCVMNetwork {
	fn into(self) -> u32 {
		self.0
	}
}

impl From<u32> for XCVMNetwork {
	fn from(network: u32) -> Self {
		XCVMNetwork(network)
	}
}
