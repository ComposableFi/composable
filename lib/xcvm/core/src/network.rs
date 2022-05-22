use crate::AbiEncoded;

pub trait Callable {
	type EncodedCall;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct XCVMNetwork(u8);

impl XCVMNetwork {
	pub const PICASSO: XCVMNetwork = XCVMNetwork(1);
	pub const ETHEREUM: XCVMNetwork = XCVMNetwork(2);
}

impl Callable for XCVMNetwork {
	type EncodedCall = AbiEncoded;
}
