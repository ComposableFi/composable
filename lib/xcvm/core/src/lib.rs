#![no_std]

extern crate alloc;

mod asset;
mod instruction;
mod network;
mod program;
mod protocol;
mod types;

pub use crate::{asset::*, instruction::*, network::*, program::*, protocol::*, types::*};
use alloc::collections::VecDeque;

#[derive(Clone)]
pub struct XCVMProgramBuilder<Network, Instruction> {
	pub network: Network,
	pub instructions: VecDeque<Instruction>,
}

impl<Network, Account, Assets>
	XCVMProgramBuilder<Network, XCVMInstruction<Network, Network::EncodedCall, Account, Assets>>
where
	Network: Copy + Callable,
{
	pub fn from(network: Network) -> Self {
		XCVMProgramBuilder { network, instructions: VecDeque::new() }
	}

	pub fn transfer(mut self, account: Account, assets: Assets) -> Self {
		self.instructions.push_back(XCVMInstruction::Transfer(account, assets));
		self
	}

	pub fn spawn<F, E>(mut self, network: Network, assets: Assets, f: F) -> Result<Self, E>
	where
		F: FnOnce(Self) -> Result<Self, E>,
	{
		self.instructions.push_back(XCVMInstruction::Spawn(
			network,
			assets,
			f(Self::from(network))?.build().instructions,
		));
		Ok(self)
	}

	pub fn call_raw(mut self, encoded_call: Network::EncodedCall) -> Self {
		self.instructions.push_back(XCVMInstruction::Call(encoded_call));
		self
	}

	pub fn call<T>(self, protocol: T) -> Result<Self, T::Error>
	where
		T: XCVMProtocol<Network>,
	{
		protocol.serialize(self.network).map(|encoded_call| self.call_raw(encoded_call))
	}

	pub fn build(
		self,
	) -> XCVMProgram<XCVMInstruction<Network, Network::EncodedCall, Account, Assets>> {
		XCVMProgram { instructions: self.instructions }
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
					XCVMNetwork::PICASSO => Ok(AbiEncoded::from(vec![0xCA, 0xFE, 0xBE, 0xEF])),
					XCVMNetwork::ETHEREUM => Ok(AbiEncoded::from(vec![0xC0, 0xDE, 0xC0, 0xDE])),
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
		enum ProgramBuildError {
			DummyProtocol1(DummyProtocol1Error),
			DummyProtocol2(DummyProtocol2Error),
		}
		impl From<DummyProtocol1Error> for ProgramBuildError {
			fn from(x: DummyProtocol1Error) -> Self {
				ProgramBuildError::DummyProtocol1(x)
			}
		}
		impl From<DummyProtocol2Error> for ProgramBuildError {
			fn from(x: DummyProtocol2Error) -> Self {
				ProgramBuildError::DummyProtocol2(x)
			}
		}

		let program = || -> Result<_, ProgramBuildError> {
			XCVMProgramBuilder::<XCVMNetwork, XCVMInstruction<XCVMNetwork, _, (), ()>>::from(
				XCVMNetwork::PICASSO,
			)
			.call(DummyProtocol1)?
			.spawn::<_, ProgramBuildError>(XCVMNetwork::ETHEREUM, (), |child| {
				Ok(child.call(DummyProtocol2)?.call(DummyProtocol1)?.transfer((), ()))
			})
		}()
		.expect("valid program");

		assert_eq!(
			program.instructions,
			VecDeque::from([
				// Protocol 1 on picasso
				XCVMInstruction::Call(AbiEncoded::from(vec![202, 254, 190, 239])),
				XCVMInstruction::Spawn(
					XCVMNetwork::ETHEREUM,
					(),
					VecDeque::from([
						// Protocol 2 on eth
						XCVMInstruction::Call(AbiEncoded::from(vec![222, 173, 192, 222])),
						// Protocol 1 on eth, different encoding than on previous network
						XCVMInstruction::Call(AbiEncoded::from(vec![192, 222, 192, 222])),
						XCVMInstruction::Transfer((), ())
					])
				),
			])
		);
	}
}
