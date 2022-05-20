#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct XCVMNetwork(u8);

impl XCVMNetwork {
	pub const PICASSO: XCVMNetwork = XCVMNetwork(1);
	pub const ETHEREUM: XCVMNetwork = XCVMNetwork(2);
}
