#![no_std]

extern crate alloc;

pub mod instruction;
pub mod network;
pub mod protocol;
pub mod types;

#[cfg(test)]
mod tests;

pub use crate::{instruction::*, network::*, protocol::*, types::*};
use alloc::vec::Vec;

#[derive(Clone)]
pub struct XCVMContractBuilder<Network, Instruction> {
	pub network: Network,
	pub instructions: Vec<Instruction>,
}

impl<Network, Account, Assets>
	XCVMContractBuilder<Network, XCVMInstruction<Network, Network::EncodedCall, Account, Assets>>
where
	Network: Copy + Callable,
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

	pub fn call(mut self, protocol: impl XCVMProtocol<Network>) -> Self {
		self.instructions.push(XCVMInstruction::Call(protocol.serialize(self.network)));
		self
	}
}
