#![cfg_attr(not(feature = "std"), no_std)]

use alloc::vec::Vec;

extern crate alloc;

pub mod instruction;
pub mod network;
pub mod protocol;
pub mod tests;
pub mod types;

pub use crate::instruction::XCVMInstruction;
pub use crate::network::XCVMNetwork;
pub use crate::protocol::XCVMProtocol;
pub use crate::types::AbiEncoded;

#[derive(Clone)]
pub struct XCVMContractBuilder<Network, Instruction> {
	pub network: Network,
	pub instructions: Vec<Instruction>,
}

impl<Network, AbiEncoded, Account, Assets>
	XCVMContractBuilder<Network, XCVMInstruction<Network, AbiEncoded, Account, Assets>>
where
	Network: Copy,
{
	pub fn from(network: Network) -> Self {
		XCVMContractBuilder { network, instructions: Vec::new() }
	}

	pub fn transfer(mut self, account: Account, assets: Assets) -> Self {
		self.instructions.push(XCVMInstruction::Transfer(account, assets));
		self
	}

	pub fn bridge(mut self, network: Network, assets: Assets) -> Self {
		self.network = network;
		self.instructions.push(XCVMInstruction::Bridge(network, assets));
		self
	}

	pub fn call(mut self, protocol: impl XCVMProtocol<Network, AbiEncoded>) -> Self {
		self.instructions.push(XCVMInstruction::Call(protocol.serialize(self.network)));
		self
	}
}
