use crate::network::XCVMNetwork;
use alloc::vec::Vec;

pub trait XCVMProtocol {
	fn serialize(&self, network: XCVMNetwork) -> Vec<u8>;
}
