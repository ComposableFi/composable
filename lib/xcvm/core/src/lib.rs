#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod asset;
mod instruction;
mod network;
mod program;
mod protocol;

pub use crate::{asset::*, instruction::*, network::*, program::*, protocol::*};
use alloc::{collections::VecDeque, vec::Vec};

pub fn serialize_json<T: serde::Serialize>(
	program: &XCVMProgram<T>,
) -> Result<Vec<u8>, serde_json::Error> {
	serde_json::to_vec(program)
}

pub fn deserialize_json<T: serde::de::DeserializeOwned>(
	buffer: &[u8],
) -> Result<XCVMProgram<T>, serde_json::Error> {
	serde_json::from_slice(buffer)
}

#[derive(Clone)]
pub struct XCVMProgramBuilder<Network, Instruction> {
	pub tag: Option<Vec<u8>>,
	pub network: Network,
	pub instructions: VecDeque<Instruction>,
	pub nonce: u32,
}

impl<Network, Account, Assets>
	XCVMProgramBuilder<Network, XCVMInstruction<Network, Network::EncodedCall, Account, Assets>>
where
	Network: Copy + Callable,
{
	pub fn from(tag: Option<Vec<u8>>, network: Network, nonce: u32) -> Self {
		XCVMProgramBuilder { tag, network, instructions: VecDeque::new(), nonce }
	}

	pub fn transfer(mut self, to: Account, assets: Assets) -> Self {
		self.instructions.push_back(XCVMInstruction::Transfer { to, assets });
		self
	}

	pub fn spawn<F, E>(
		mut self,
		tag: Option<Vec<u8>>,
		network: Network,
		assets: Assets,
		f: F,
	) -> Result<Self, E>
	where
		F: FnOnce(Self) -> Result<Self, E>,
	{
		self.instructions.push_back(XCVMInstruction::Spawn {
			network,
			assets,
			program: f(Self::from(tag, network, self.nonce + 1))?.build(),
		});
		Ok(self)
	}

	pub fn call_raw(mut self, encoded: Network::EncodedCall) -> Self {
		self.instructions.push_back(XCVMInstruction::Call { encoded });
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
	) -> XCVMProgram<VecDeque<XCVMInstruction<Network, Network::EncodedCall, Account, Assets>>> {
		XCVMProgram { tag: self.tag, instructions: self.instructions, nonce: self.nonce }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::{collections::BTreeMap, vec};

	struct DummyProtocol1;
	#[derive(Debug)]
	struct DummyProtocol1Error;
	impl XCVMProtocol<XCVMNetwork> for DummyProtocol1 {
		type Error = DummyProtocol1Error;
		fn serialize(&self, network: XCVMNetwork) -> Result<Vec<u8>, Self::Error> {
			match network {
				XCVMNetwork::PICASSO => Ok(vec![0xCA, 0xFE, 0xBE, 0xEF]),
				XCVMNetwork::ETHEREUM => Ok(vec![0xC0, 0xDE, 0xC0, 0xDE]),
				_ => Err(DummyProtocol1Error),
			}
		}
	}

	struct DummyProtocol2;
	#[derive(Debug)]
	struct DummyProtocol2Error;
	impl XCVMProtocol<XCVMNetwork> for DummyProtocol2 {
		type Error = DummyProtocol2Error;
		fn serialize(&self, network: XCVMNetwork) -> Result<Vec<u8>, Self::Error> {
			match network {
				XCVMNetwork::PICASSO => Ok(vec![0xCA, 0xFE, 0xBA, 0xBE]),
				XCVMNetwork::ETHEREUM => Ok(vec![0xDE, 0xAD, 0xC0, 0xDE]),
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

	#[test]
	fn can_build() {
		let program = || -> Result<_, ProgramBuildError> {
			Ok(XCVMProgramBuilder::<
					XCVMNetwork,
					XCVMInstruction<XCVMNetwork, _, (), XCVMTransfer>,
				>::from(Some("Main program".as_bytes().to_vec()), XCVMNetwork::PICASSO, 0)
				.call(DummyProtocol1)?
				.spawn::<_, ProgramBuildError>(
					None,
					XCVMNetwork::ETHEREUM,
					XCVMTransfer::empty(),
					|child| {
						Ok(child
							.call(DummyProtocol2)?
							.call(DummyProtocol1)?
							.transfer((), XCVMTransfer::from([(1, u128::MAX)])))
					},
				)?
				.build())
		}()
		.expect("valid program");

		assert_eq!(
			program,
			XCVMProgram {
				tag: Some("Main program".as_bytes().to_vec()),
				nonce: 0,
				instructions: VecDeque::from([
					// Protocol 1 on picasso
					XCVMInstruction::Call { encoded: vec![202, 254, 190, 239] },
					XCVMInstruction::Spawn {
						network: XCVMNetwork::ETHEREUM,
						assets: XCVMTransfer::empty(),
						program: XCVMProgram {
							tag: None,
							nonce: 1,
							instructions: VecDeque::from([
								// Protocol 2 on eth
								XCVMInstruction::Call { encoded: vec![222, 173, 192, 222] },
								// Protocol 1 on eth, different encoding than on previous network
								XCVMInstruction::Call { encoded: vec![192, 222, 192, 222] },
								XCVMInstruction::Transfer {
									to: (),
									assets: XCVMTransfer::from(BTreeMap::from([(1, u128::MAX)]))
								}
							])
						}
					}
				])
			},
		);
	}

	#[test]
	fn json_iso() {
		let program = || -> Result<_, ProgramBuildError> {
			Ok(XCVMProgramBuilder::<
				XCVMNetwork,
				XCVMInstruction<XCVMNetwork, Vec<u8>, Vec<u8>, XCVMTransfer>,
			>::from(None, XCVMNetwork::PICASSO, 0)
			.spawn::<_, ProgramBuildError>(
				None,
				XCVMNetwork::ETHEREUM,
				XCVMTransfer::from(BTreeMap::from([(1, 10_000_000_000_000u128)])),
				|child| Ok(child),
			)?
			.build())
		}()
		.expect("valid program");
		let serialized = serialize_json(&program).unwrap();
		assert_eq!(program, deserialize_json(&serialized).unwrap());
	}
}
