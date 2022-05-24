#![no_std]

extern crate alloc;

mod instruction;
mod network;
mod program;
mod protocol;
mod types;

pub use crate::{instruction::*, network::*, program::*, protocol::*, types::*};
use alloc::vec::Vec;

#[derive(Clone)]
pub struct XCVMProgramBuilder<Network, Instruction> {
	pub network: Network,
	pub instructions: Vec<Instruction>,
	pub instruction_pointer: u32,
}

impl<Network, Account, Assets>
	XCVMProgramBuilder<Network, XCVMInstruction<Network, Network::EncodedCall, Account, Assets>>
where
	Network: Copy + Callable,
{
	pub fn from(network: Network) -> Self {
		XCVMProgramBuilder { network, instructions: Vec::new(), instruction_pointer: 0 }
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

	pub fn call<T>(mut self, protocol: T) -> Result<Self, T::Error>
	where
		T: XCVMProtocol<Network>,
	{
		protocol.serialize(self.network).map(|encoded_call| {
			self.instructions.push(XCVMInstruction::Call(encoded_call));
			self
		})
	}

	pub fn with_instruction_pointer(mut self, instruction_pointer: u32) -> Self {
		self.instruction_pointer = instruction_pointer;
		self
	}

	pub fn build(
		self,
	) -> XCVMProgram<XCVMInstruction<Network, Network::EncodedCall, Account, Assets>> {
		XCVMProgram {
			instructions: self.instructions,
			instruction_pointer: self.instruction_pointer,
		}
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	use alloc::vec;

	#[test]
	fn test() {
		struct DummyProtocol1;
		#[derive(Debug)]
		struct DummyProtocol1Error;
		impl XCVMProtocol<XCVMNetwork> for DummyProtocol1 {
			type Error = DummyProtocol1Error;
			fn serialize(&self, network: XCVMNetwork) -> Result<AbiEncoded, Self::Error> {
				match network {
					XCVMNetwork::PICASSO => Ok(AbiEncoded::from(vec![0xCA, 0xFE, 0xBA, 0xBE])),
					XCVMNetwork::ETHEREUM => Ok(AbiEncoded::from(vec![0xDE, 0xAD, 0xC0, 0xDE])),
					_ => Err(DummyProtocol1Error),
				}
			}
		}

		struct DummyProtocol2;
		#[derive(Debug)]
		struct DummyProtocol2Error;
		impl XCVMProtocol<XCVMNetwork> for DummyProtocol2 {
			type Error = DummyProtocol2Error;
			fn serialize(&self, network: XCVMNetwork) -> Result<AbiEncoded, Self::Error> {
				match network {
					XCVMNetwork::PICASSO => Ok(AbiEncoded::from(vec![0xCA, 0xFE, 0xBA, 0xBE])),
					XCVMNetwork::ETHEREUM => Ok(AbiEncoded::from(vec![0xDE, 0xAD, 0xC0, 0xDE])),
					_ => Err(DummyProtocol2Error),
				}
			}
		}

		#[derive(Debug)]
		enum ContractBuildError {
			DummyProtocol1(DummyProtocol1Error),
			DummyProtocol2(DummyProtocol2Error),
		}
		impl From<DummyProtocol1Error> for ContractBuildError {
			fn from(x: DummyProtocol1Error) -> Self {
				ContractBuildError::DummyProtocol1(x)
			}
		}
		impl From<DummyProtocol2Error> for ContractBuildError {
			fn from(x: DummyProtocol2Error) -> Self {
				ContractBuildError::DummyProtocol2(x)
			}
		}

		let contract = || -> Result<_, ContractBuildError> {
			Ok(XCVMProgramBuilder::<XCVMNetwork, XCVMInstruction<XCVMNetwork, _, (), ()>>::from(
				XCVMNetwork::PICASSO,
			)
			.call(DummyProtocol1)?
			.bridge(XCVMNetwork::ETHEREUM, ())
			.call(DummyProtocol2)?
			.transfer((), ()))
		}()
		.expect("valid contract");

		assert_eq!(
			contract.instructions,
			vec![
				XCVMInstruction::Call(AbiEncoded::from(vec![202, 254, 186, 190])),
				XCVMInstruction::Bridge(XCVMNetwork::ETHEREUM, ()),
				XCVMInstruction::Call(AbiEncoded::from(vec![222, 173, 192, 222])),
				XCVMInstruction::Transfer((), ()),
			]
		);
	}
}
