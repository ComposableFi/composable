#![no_std]

extern crate alloc;

use alloc::{borrow::ToOwned, vec, vec::Vec};
use core::str::FromStr;
use ethabi::{encode, ethereum_types::H160, Function, StateMutability, Token};
use xcvm_core::{XCVMNetwork, XCVMProtocol};

pub struct Ping;
impl Ping {
	pub fn ethereum_prototype() -> Function {
		Function {
			name: "ping".to_owned(),
			inputs: vec![],
			outputs: vec![],
			constant: None,
			state_mutability: StateMutability::Payable,
		}
	}
}
impl XCVMProtocol<XCVMNetwork> for Ping {
	type Error = ();
	fn serialize(
		&self,
		network: XCVMNetwork,
	) -> Result<<XCVMNetwork as xcvm_core::Callable>::EncodedCall, Self::Error> {
		match network {
			XCVMNetwork::ETHEREUM => {
				let contract_address = H160::from_str("0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9")
					.expect("impossible");
				let encoded_call = Self::ethereum_prototype().encode_input(&[]).map_err(|_| ())?;
				Ok(encode(&[Token::Address(contract_address), Token::Bytes(encoded_call)]).into())
			},
			_ => Err(()),
		}
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SlippageLimit {
	Unlimited,
	Limited {},
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Swap<Assets, Options> {
	input: Assets,
	output: Assets,
	options: Options,
}

impl<Assets, Options> Swap<Assets, Options> {
	pub fn new(input: Assets, output: Assets, options: Options) -> Self {
		Swap { input, output, options }
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

impl<Assets, Options> XCVMProtocol<XCVMNetwork> for Swap<Assets, Options> {
	type Error = StableswapError;
	fn serialize(&self, network: XCVMNetwork) -> Result<Vec<u8>, Self::Error> {
		match network {
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
		let program = || -> Result<_, ()> {
			Ok(XCVMProgramBuilder::<XCVMNetwork, XCVMInstruction<XCVMNetwork, _, (), ()>>::from(
				None,
				XCVMNetwork::PICASSO,
			)
			.spawn::<_, ()>(None, XCVMNetwork::ETHEREUM, Vec::new(), (), |child| {
				Ok(child.call(Ping)?.transfer((), ()))
			})?
			.build())
		}()
		.expect("valid program");

		assert_eq!(
			program,
			XCVMProgram {
				tag: None,
				instructions: VecDeque::from([XCVMInstruction::Spawn {
					network: XCVMNetwork::ETHEREUM,
					salt: Vec::new(),
					assets: (),
					program: XCVMProgram {
						tag: None,
						instructions: VecDeque::from([
							XCVMInstruction::Call {
								encoded: vec![
									0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 207, 126, 211, 172, 202,
									90, 70, 126, 158, 112, 76, 112, 62, 141, 135, 246, 52, 251, 15,
									201, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
									0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0,
									0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
									0, 0, 4, 92, 54, 177, 134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
									0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
								]
							},
							XCVMInstruction::Transfer { to: (), assets: () }
						])
					}
				},])
			}
		);
	}
}
