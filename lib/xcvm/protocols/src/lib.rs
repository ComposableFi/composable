#![no_std]

extern crate alloc;

#[cfg(test)]
mod tests;

use ethabi::{encode, ethereum_types::H160, Token};
use xcvm_core::{AbiEncoded, XCVMNetwork, XCVMProtocol};

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

impl<Assets> XCVMProtocol<XCVMNetwork> for Stableswap<Assets> {
	fn serialize(&self, network: XCVMNetwork) -> AbiEncoded {
		match network {
			XCVMNetwork::PICASSO => AbiEncoded::empty(),
			XCVMNetwork::ETHEREUM => encode(&[Token::Address(H160::zero())]).into(),
			_ => todo!("handle error of invalid network id"),
		}
	}
}
