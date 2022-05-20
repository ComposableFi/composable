#![cfg_attr(not(feature = "std"), no_std)]

use alloc::vec::Vec;

extern crate alloc;

pub mod instruction;
pub mod network;
pub mod protocol;
pub mod protocols;

use crate::instruction::XCVMInstruction;
use crate::network::XCVMNetwork;
use crate::protocol::XCVMProtocol;
use crate::protocols::Stableswap;

#[derive(Clone)]
pub struct XCVMContractBuilder<Account, Assets> {
	network: XCVMNetwork,
	instructions: Vec<XCVMInstruction<Account, Assets>>,
}

impl<Account, Assets> XCVMContractBuilder<Account, Assets> {
	pub fn here() -> Self {
		XCVMContractBuilder { network: XCVMNetwork::PICASSO, instructions: Vec::new() }
	}

	pub fn transfer(&mut self, account: Account, assets: Assets) -> &mut Self {
		self.instructions.push(XCVMInstruction::Transfer(account, assets));
		self
	}

	pub fn bridge(&mut self, network: XCVMNetwork, assets: Assets) -> &mut Self {
		self.network = network;
		self.instructions.push(XCVMInstruction::Bridge(network, assets));
		self
	}

	pub fn call(&mut self, protocol: impl XCVMProtocol) -> &mut Self {
		self.instructions.push(XCVMInstruction::Call(protocol.serialize(self.network)));
		self
	}
}

#[test]
fn test() {
	let _ = XCVMContractBuilder::<(), ()>::here()
		.call(Stableswap::new((), ()))
		.bridge(XCVMNetwork::ETHEREUM, ())
		.call(Stableswap::new((), ()))
		.transfer((), ());
}
