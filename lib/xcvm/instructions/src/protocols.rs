use crate::network::XCVMNetwork;
use crate::protocol::XCVMProtocol;
use alloc::vec::Vec;

#[derive(Copy, Clone)]
pub struct Stableswap<Assets> {
	input: Assets,
	output: Assets,
}

impl<Assets> Stableswap<Assets> {
	pub fn new(input: Assets, output: Assets) -> Self {
		Stableswap { input, output }
	}
}

impl<Assets> XCVMProtocol for Stableswap<Assets> {
	fn serialize(&self, network: XCVMNetwork) -> Vec<u8> {
		match network {
			XCVMNetwork::PICASSO => todo!("hardcoded"),
			XCVMNetwork::ETHEREUM => todo!("hardcoded"),
			_ => todo!("handle error of invalid network id"),
		}
	}
}
