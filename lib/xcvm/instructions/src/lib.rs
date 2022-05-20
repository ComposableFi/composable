#![cfg_attr(not(feature = "std"), no_std)]

use alloc::vec::Vec;

extern crate alloc;

pub trait XCVMProtocol {
	fn serialize(&self, network: XCVMNetwork) -> Vec<u8>;
}

#[derive(Copy, Clone)]
pub struct Stableswap<Assets>(Assets, Assets);

impl<Assets> XCVMProtocol for Stableswap<Assets> {
	fn serialize(&self, network: XCVMNetwork) -> Vec<u8> {
		match network {
			XCVMNetwork::PICASSO => todo!("hardcoded"),
			XCVMNetwork::ETHEREUM => todo!("hardcoded"),
			_ => todo!("handle error of invalid network id"),
		}
	}
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct XCVMNetwork(u8);

impl XCVMNetwork {
	pub const PICASSO: XCVMNetwork = XCVMNetwork(1);
	pub const ETHEREUM: XCVMNetwork = XCVMNetwork(2);
}

#[derive(Clone)]
pub enum XCVMInstruction<Account, Assets> {
	Transfer(Account, Assets),
	Bridge(XCVMNetwork, Assets),
	Call(Vec<u8>),
}

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
		.call(Stableswap((), ()))
		.bridge(XCVMNetwork::Cosmos(XCVMCosmChain::Terra), ())
		.call(Stableswap((), ()))
		.transfer((), ());
}
