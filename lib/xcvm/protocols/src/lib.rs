#![no_std]

extern crate alloc;

use alloc::{borrow::ToOwned, vec, vec::Vec};
use core::str::FromStr;
use ethabi::{encode, ethereum_types::H160, Function, StateMutability, Token};
use xcvm_core::{XCVMNetwork, XCVMProtocol};

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
	fn serialize(&self, network: XCVMNetwork) -> Result<Vec<u8>, Self::Error> {
		match network {
			XCVMNetwork::PICASSO => Ok(vec![]),
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
	use alloc::collections::VecDeque;
	use xcvm_core::{XCVMInstruction, XCVMProgram, XCVMProgramBuilder};

	#[test]
	fn test() {
		let program = || -> Result<_, StableswapError> {
			Ok(XCVMProgramBuilder::<XCVMNetwork, XCVMInstruction<XCVMNetwork, _, (), ()>>::from(
				None,
				XCVMNetwork::PICASSO,
			)
			.call(Stableswap::<(), ()>::new((), (), ()))?
			.spawn::<_, StableswapError>(None, XCVMNetwork::ETHEREUM, Vec::new(), (), |child| {
				Ok(child.call(Stableswap::<(), ()>::new((), (), ()))?.transfer((), ()))
			})?
			.build())
		}()
		.expect("valid program");

		assert_eq!(
			program,
			XCVMProgram {
				tag: None,
				instructions: VecDeque::from([
					XCVMInstruction::Call { encoded: vec![] },
					XCVMInstruction::Spawn {
						network: XCVMNetwork::ETHEREUM,
						salt: Vec::new(),
						assets: (),
						program: XCVMProgram {
							tag: None,
							instructions: VecDeque::from([
								XCVMInstruction::Call {
									encoded: vec![
										0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 74, 17, 213, 238,
										170, 194, 142, 195, 246, 29, 16, 13, 175, 77, 64, 71, 31,
										24, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
										0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0,
										0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
										0, 0, 0, 0, 0, 0, 0, 4, 129, 25, 192, 101, 0, 0, 0, 0, 0,
										0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
										0, 0, 0
									]
								},
								XCVMInstruction::Transfer { to: (), assets: () }
							])
						}
					},
				])
			}
		);
	}
}
