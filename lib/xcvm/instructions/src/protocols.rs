use crate::network::XCVMNetwork;
use crate::protocol::XCVMProtocol;
use crate::types::AbiEncoded;

use ethabi::{encode, ethereum_types::H160, Token};

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

impl<Assets> XCVMProtocol<XCVMNetwork, AbiEncoded> for Stableswap<Assets> {
	fn serialize(&self, network: XCVMNetwork) -> AbiEncoded {
		match network {
			XCVMNetwork::PICASSO => AbiEncoded::empty(),
			XCVMNetwork::ETHEREUM => encode(&[Token::Address(H160::zero())]).into(),
			_ => todo!("handle error of invalid network id"),
		}
	}
}
