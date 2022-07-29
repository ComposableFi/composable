#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod abstraction;
mod asset;
mod instruction;
mod network;
mod program;
mod protocol;

use core::marker::PhantomData;

pub use crate::{asset::*, instruction::*, network::*, program::*, protocol::*};
use alloc::{collections::VecDeque, vec::Vec};

#[inline]
pub fn serialize_json<T: serde::Serialize>(
	program: &Program<T>,
) -> Result<Vec<u8>, serde_json::Error> {
	serde_json::to_vec(program)
}

#[inline]
pub fn deserialize_json<T: serde::de::DeserializeOwned>(
	buffer: &[u8],
) -> Result<Program<T>, serde_json::Error> {
	serde_json::from_slice(buffer)
}

#[derive(Clone)]
pub struct ProgramBuilder<CurrentNetwork: Network, Account, Assets> {
	pub tag: Option<Vec<u8>>,
	pub instructions: VecDeque<Instruction<NetworkID, Vec<u8>, Account, Assets>>,
	pub _marker: PhantomData<CurrentNetwork>,
}

impl<CurrentNetwork, Account, Assets> ProgramBuilder<CurrentNetwork, Account, Assets>
where
	CurrentNetwork: Network,
	CurrentNetwork::EncodedCall: Into<Vec<u8>>,
{
	#[inline]
	pub fn new(tag: Option<Vec<u8>>) -> Self {
		ProgramBuilder { tag, instructions: VecDeque::new(), _marker: PhantomData }
	}

	#[inline]
	pub fn transfer(mut self, to: Account, assets: Assets) -> Self {
		self.instructions.push_back(Instruction::Transfer { to, assets });
		self
	}

	#[inline]
	pub fn spawn<SpawningNetwork, FinalNetwork, E, F>(
		self,
		tag: Option<Vec<u8>>,
		salt: Vec<u8>,
		assets: Assets,
		f: F,
	) -> Result<ProgramBuilder<FinalNetwork, Account, Assets>, E>
	where
		F: FnOnce(
			ProgramBuilder<SpawningNetwork, Account, Assets>,
		) -> Result<ProgramBuilder<FinalNetwork, Account, Assets>, E>,
		SpawningNetwork: Network,
		SpawningNetwork::EncodedCall: Into<Vec<u8>>,
		FinalNetwork: Network,
		FinalNetwork::EncodedCall: Into<Vec<u8>>,
	{
		// We need to recreate the builder to mutate the phantom marker.
		let mut builder =
			ProgramBuilder { tag: self.tag, instructions: self.instructions, _marker: PhantomData };
		builder.instructions.push_back(Instruction::Spawn {
			salt,
			assets,
			network: SpawningNetwork::ID,
			program: f(ProgramBuilder::<SpawningNetwork, Account, Assets>::new(tag))?.build(),
		});
		Ok(builder)
	}

	#[inline]
	pub fn call_raw(mut self, encoded: CurrentNetwork::EncodedCall) -> Self {
		self.instructions.push_back(Instruction::Call { encoded: encoded.into() });
		self
	}

	#[inline]
	pub fn call<T>(self, protocol: T) -> Result<Self, T::Error>
	where
		T: Protocol<CurrentNetwork>,
	{
		protocol.serialize().map(|encoded_call| self.call_raw(encoded_call))
	}

	#[inline]
	pub fn build(self) -> Program<VecDeque<Instruction<NetworkID, Vec<u8>, Account, Assets>>> {
		Program { tag: self.tag, instructions: self.instructions }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::{collections::BTreeMap, vec};

	struct DummyProtocol1;
	#[derive(Debug)]
	struct DummyProtocol1Error;
	impl Protocol<Picasso> for DummyProtocol1 {
		type Error = DummyProtocol1Error;
		fn serialize(&self) -> Result<<Picasso as Network>::EncodedCall, Self::Error> {
			Ok(vec![0xCA, 0xFE, 0xBE, 0xEF])
		}
	}
	impl Protocol<Ethereum> for DummyProtocol1 {
		type Error = DummyProtocol1Error;
		fn serialize(&self) -> Result<<Ethereum as Network>::EncodedCall, Self::Error> {
			Ok(vec![0xC0, 0xDE, 0xC0, 0xDE])
		}
	}

	struct DummyProtocol2;
	#[derive(Debug)]
	struct DummyProtocol2Error;
	impl Protocol<Picasso> for DummyProtocol2 {
		type Error = DummyProtocol2Error;
		fn serialize(&self) -> Result<<Picasso as Network>::EncodedCall, Self::Error> {
			Ok(vec![0xCA, 0xFE, 0xBA, 0xBE])
		}
	}
	impl Protocol<Ethereum> for DummyProtocol2 {
		type Error = DummyProtocol2Error;
		fn serialize(&self) -> Result<<Ethereum as Network>::EncodedCall, Self::Error> {
			Ok(vec![0xDE, 0xAD, 0xC0, 0xDE])
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
			Ok(ProgramBuilder::<Picasso, (), Funds>::new(Some("Main program".as_bytes().to_vec()))
				.call(DummyProtocol1)?
				.spawn::<Ethereum, _, ProgramBuildError, _>(
					None,
					Default::default(),
					Funds::empty(),
					|child| {
						Ok(child
							.call(DummyProtocol2)?
							.call(DummyProtocol1)?
							.transfer((), Funds::from([(PICA::ID, u128::MAX)])))
					},
				)?
				.build())
		}()
		.expect("valid program");

		assert_eq!(
			program,
			Program {
				tag: Some("Main program".as_bytes().to_vec()),
				instructions: VecDeque::from([
					// Protocol 1 on picasso
					Instruction::Call { encoded: vec![202, 254, 190, 239] },
					// Move to ethereum
					Instruction::Spawn {
						network: Ethereum::ID,
						salt: Vec::new(),
						assets: Funds::empty(),
						program: Program {
							tag: None,
							instructions: VecDeque::from([
								// Protocol 2 on eth
								Instruction::Call { encoded: vec![222, 173, 192, 222] },
								// Protocol 1 on eth, different encoding than on previous network
								Instruction::Call { encoded: vec![192, 222, 192, 222] },
								Instruction::Transfer {
									to: (),
									assets: Funds::from(BTreeMap::from([(PICA::ID, u128::MAX)]))
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
			Ok(ProgramBuilder::<Picasso, Vec<u8>, Funds>::new(None)
				.spawn::<Ethereum, _, ProgramBuildError, _>(
					None,
					Vec::new(),
					Funds::from(BTreeMap::from([(1, 10_000_000_000_000u128)])),
					|child| Ok(child),
				)?
				.build())
		}()
		.expect("valid program");
		let serialized = serialize_json(&program).unwrap();
		assert_eq!(program, deserialize_json(&serialized).unwrap());
	}
}
