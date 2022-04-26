#![cfg_attr(not(feature = "std"), no_std)]

use alloc::vec::Vec;

extern crate alloc;

pub trait XCVMProtocol {
	fn serialize(&self, network: &XCVMNetwork) -> Vec<u8>;
}

#[derive(Copy, Clone)]
pub struct Stableswap<Assets>(Assets, Assets);

impl<Assets> XCVMProtocol for Stableswap<Assets> {
	fn serialize(&self, network: &XCVMNetwork) -> Vec<u8> {
		match network {
			XCVMNetwork::Substrate(XCVMSubstrateChain::Picasso) => todo!("hardcoded"),
			XCVMNetwork::Cosmos(XCVMCosmChain::Terra) => todo!("hardcoded"),
		}
	}
}

#[derive(Copy, Clone)]
pub enum XCVMSubstrateChain {
	Picasso,
}

#[derive(Copy, Clone)]
pub enum XCVMCosmChain {
	Terra,
}

#[derive(Copy, Clone)]
pub enum XCVMNetwork {
	Substrate(XCVMSubstrateChain),
	Cosmos(XCVMCosmChain),
}

#[derive(Clone)]
pub enum XCVMInstruction<Account, Assets> {
	Transfer(Account, Assets),
	Bridge(XCVMNetwork, Assets),
	Call(Vec<u8>),
}

#[derive(Clone)]
pub struct XCVMBuilder<Account, Assets>(XCVMNetwork, Vec<XCVMInstruction<Account, Assets>>);

impl<Account, Assets> XCVMBuilder<Account, Assets> {
	pub fn here() -> Self {
		XCVMBuilder(XCVMNetwork::Substrate(XCVMSubstrateChain::Picasso), Vec::new())
	}

	pub fn transfer(&mut self, account: Account, assets: Assets) -> &mut Self {
		self.1.push(XCVMInstruction::Transfer(account, assets));
		self
	}

	pub fn bridge(&mut self, network: XCVMNetwork, assets: Assets) -> &mut Self {
		self.0 = network;
		self.1.push(XCVMInstruction::Bridge(network, assets));
		self
	}

	pub fn call(&mut self, protocol: impl XCVMProtocol) -> &mut Self {
		self.1.push(XCVMInstruction::Call(protocol.serialize(&self.0)));
		self
	}
}

#[test]
fn test() {
	let _ = XCVMBuilder::<(), ()>::here()
		.call(Stableswap((), ()))
		.bridge(XCVMNetwork::Cosmos(XCVMCosmChain::Terra), ())
		.call(Stableswap((), ()))
		.transfer((), ());
}
