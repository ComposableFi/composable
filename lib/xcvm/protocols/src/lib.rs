#![no_std]

extern crate alloc;

use core::str::FromStr;

use alloc::{borrow::ToOwned, vec};
use ethabi::{encode, ethereum_types::H160, Function, Param, ParamType, StateMutability, Token};
use xcvm_core::{AbiEncoded, XCVMNetwork, XCVMProtocol};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SlippageLimit {
	Unlimited,
	Limited {},
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Stableswap<Assets, Options> {
	input: Assets,
	output: Assets,
	options: Options,
}

impl<Assets, Options> Stableswap<Assets, Options> {
	pub fn new(input: Assets, output: Assets, options: Options) -> Self {
		Stableswap { input, output, options }
	}
	pub fn ethereum_prototype() -> Function {
		Function {
			name: "swap".to_owned(),
			inputs: vec![],
			outputs: vec![],
			constant: None,
			state_mutability: StateMutability::Payable,
		}
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum StableswapError {
	UnsupportedNetwork,
	EncodingFailed,
}

impl<Assets, Options> XCVMProtocol<XCVMNetwork> for Stableswap<Assets, Options> {
	type Error = StableswapError;
	fn serialize(&self, network: XCVMNetwork) -> Result<AbiEncoded, Self::Error> {
		match network {
			XCVMNetwork::PICASSO => Ok(AbiEncoded::empty()),
			XCVMNetwork::ETHEREUM => {
				let uniswap_v3_contract_address =
					H160::from_str("0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852")
						.expect("impossible");
				let encoded_call = Self::ethereum_prototype()
					.encode_input(&[])
					.map_err(|_| StableswapError::EncodingFailed)?;
				Ok(encode(&[
					Token::Address(uniswap_v3_contract_address),
					Token::Bytes(encoded_call),
				])
				.into())
			},
			_ => Err(StableswapError::UnsupportedNetwork),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use xcvm_core::{XCVMContractBuilder, XCVMInstruction};

	#[test]
	fn test() {
		let contract = || -> Result<_, StableswapError> {
			Ok(XCVMContractBuilder::<XCVMNetwork, XCVMInstruction<XCVMNetwork, _, (), ()>>::from(
				XCVMNetwork::PICASSO,
			)
			.call(Stableswap::<(), ()>::new((), (), ()))?
			.bridge(XCVMNetwork::ETHEREUM, ())
			.call(Stableswap::<(), ()>::new((), (), ()))?
			.transfer((), ()))
		}()
		.expect("valid contract");

		assert_eq!(
			contract.instructions,
			vec![
				XCVMInstruction::Call(AbiEncoded::empty()),
				XCVMInstruction::Bridge(XCVMNetwork::ETHEREUM, ()),
				XCVMInstruction::Call(AbiEncoded::from(vec![
					0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 74, 17, 213, 238, 170, 194, 142, 195,
					246, 29, 16, 13, 175, 77, 64, 71, 31, 24, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
					0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0,
					0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
					4, 129, 25, 192, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
					0, 0, 0, 0, 0, 0, 0, 0, 0
				])),
				XCVMInstruction::Transfer((), ()),
			]
		);
	}
}
