#![no_std]

extern crate alloc;

mod xcvm;

use alloc::{
	collections::{BTreeMap, VecDeque},
	vec::Vec,
};
pub use prost::{DecodeError, EncodeError, Message};
use xcvm::*;
pub use xcvm_core::*;

#[derive(Clone, Debug)]
pub enum DecodingFailure {
	Protobuf(DecodeError),
	Isomorphism,
}

pub fn decode<
	TNetwork: From<u32>,
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<BTreeMap<u32, u128>>,
>(
	buffer: &[u8],
) -> Result<
	XCVMProgram<VecDeque<XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>>>,
	DecodingFailure,
> {
	Program::decode(buffer)
		.map_err(DecodingFailure::Protobuf)
		.and_then(|x| TryInto::try_into(x).map_err(|_| DecodingFailure::Isomorphism))
}

pub fn encode<
	TNetwork: Into<u32>,
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: AsRef<[u8]>,
	TAssets: Into<BTreeMap<u32, u128>>,
>(
	program: XCVMProgram<VecDeque<XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>>>,
) -> Vec<u8> {
	Program::encode_to_vec(&program.into())
}

impl From<u128> for U128 {
	fn from(x: u128) -> Self {
		U128 { encoded: x.to_le_bytes().to_vec() }
	}
}

impl TryInto<u128> for U128 {
	type Error = ();
	fn try_into(self) -> Result<u128, Self::Error> {
		Ok(u128::from_le_bytes(TryInto::<[u8; 16]>::try_into(self.encoded).map_err(|_| ())?))
	}
}

impl<
		TNetwork: Into<u32>,
		TAbiEncoded: Into<Vec<u8>>,
		TAccount: AsRef<[u8]>,
		TAssets: Into<BTreeMap<u32, u128>>,
	> From<XCVMProgram<VecDeque<XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>>>>
	for Program
{
	fn from(
		XCVMProgram { tag, instructions, nonce }: XCVMProgram<
			VecDeque<XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>>,
		>,
	) -> Self {
		Program {
			tag: tag.unwrap_or_default(),
			instructions: Some(Instructions {
				instructions: instructions.into_iter().map(Into::<Instruction>::into).collect(),
			}),
			nonce,
		}
	}
}

impl<
		TNetwork: From<u32>,
		TAbiEncoded: TryFrom<Vec<u8>>,
		TAccount: for<'a> TryFrom<&'a [u8]>,
		TAssets: From<BTreeMap<u32, u128>>,
	> TryFrom<Program>
	for XCVMProgram<VecDeque<XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>>>
{
	type Error = ();
	fn try_from(program: Program) -> Result<Self, Self::Error> {
		match program {
			Program { instructions: Some(Instructions { instructions }), nonce, tag } => {
				let tag = if tag.is_empty() { None } else { Some(tag) };

				Ok(XCVMProgram {
					tag,
					instructions: instructions
						.into_iter()
						.map(TryInto::<XCVMInstruction<_, _, _, _>>::try_into)
						.collect::<Result<VecDeque<_>, _>>()?,
					nonce,
				})
			},
			_ => Err(()),
		}
	}
}

impl<
		TNetwork: Into<u32>,
		TAbiEncoded: Into<Vec<u8>>,
		TAccount: AsRef<[u8]>,
		TAssets: Into<BTreeMap<u32, u128>>,
	> From<XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>> for Instruction
{
	fn from(instruction: XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>) -> Self {
		Instruction {
			instruction: Some(match instruction {
				XCVMInstruction::Transfer { to: destination, assets } => {
					instruction::Instruction::Transfer(Transfer {
						destination: Some(Account { encoded: destination.as_ref().to_vec() }),
						assets: assets
							.into()
							.into_iter()
							.map(|(asset, amount)| (asset, amount.into()))
							.collect(),
					})
				},
				XCVMInstruction::Call { encoded } => {
					instruction::Instruction::Call(Call { encoded: encoded.into() })
				},
				XCVMInstruction::Spawn { network, assets, program } => {
					instruction::Instruction::Spawn(Spawn {
						network: network.into(),
						assets: assets
							.into()
							.into_iter()
							.map(|(asset, amount)| (asset, amount.into()))
							.collect(),
						program: Some(program.into()),
					})
				},
			}),
		}
	}
}

impl<
		TNetwork: From<u32>,
		TAbiEncoded: TryFrom<Vec<u8>>,
		TAccount: for<'a> TryFrom<&'a [u8]>,
		TAssets: From<BTreeMap<u32, u128>>,
	> TryFrom<Instruction> for XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>
{
	type Error = ();
	fn try_from(Instruction { instruction }: Instruction) -> Result<Self, Self::Error> {
		instruction
			.map(|instruction| match instruction {
				instruction::Instruction::Transfer(Transfer {
					destination: Some(Account { encoded }),
					assets,
				}) => Ok(XCVMInstruction::Transfer {
					to: (&encoded[..]).try_into().map_err(|_| ())?,
					assets: assets
						.into_iter()
						.map(|(asset, amount)| Ok((asset, amount.try_into()?)))
						.collect::<Result<BTreeMap<u32, u128>, ()>>()?
						.into(),
				}),
				instruction::Instruction::Call(Call { encoded }) => {
					Ok(XCVMInstruction::Call { encoded: encoded.try_into().map_err(|_| ())? })
				},
				instruction::Instruction::Spawn(Spawn {
					network,
					assets,
					program: Some(program),
				}) => Ok(XCVMInstruction::Spawn {
					network: network.into(),
					assets: assets
						.into_iter()
						.map(|(asset, amount)| Ok((asset, amount.try_into()?)))
						.collect::<Result<BTreeMap<u32, u128>, ()>>()?
						.into(),
					program: program.try_into().map_err(|_| ())?,
				}),
				instruction::Instruction::Transfer(Transfer { destination: None, .. }) => Err(()),
				instruction::Instruction::Spawn(Spawn { program: None, .. }) => Err(()),
			})
			.unwrap_or(Err(()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::vec;
	use xcvm_core::{XCVMNetwork, XCVMProgramBuilder, XCVMProtocol};

	#[test]
	fn type_isomorphism() {
		struct DummyProtocol;
		impl XCVMProtocol<XCVMNetwork> for DummyProtocol {
			type Error = ();
			fn serialize(&self, network: XCVMNetwork) -> Result<Vec<u8>, ()> {
				match network {
					XCVMNetwork::PICASSO => Ok(vec![0xCA, 0xFE, 0xBA, 0xBE]),
					XCVMNetwork::ETHEREUM => Ok(vec![0xDE, 0xAD, 0xC0, 0xDE]),
					_ => Err(()),
				}
			}
		}

		let program = || -> Result<_, ()> {
			XCVMProgramBuilder::<
				XCVMNetwork,
				XCVMInstruction<XCVMNetwork, _, Vec<u8>, BTreeMap<u32, u128>>,
			>::from(None, XCVMNetwork::PICASSO, 0)
			.call(DummyProtocol)?
			.spawn(None, XCVMNetwork::ETHEREUM, BTreeMap::from([(0x1337, 20_000)]), |child| {
				Ok(child
					.call(DummyProtocol)?
					.transfer(vec![0xBE, 0xEF], BTreeMap::from([(0, 10_000)])))
			})
		}()
		.expect("valid program");

		// f^-1 . f = id
		assert_eq!(
			Ok(program.instructions.clone()),
			program
				.instructions
				.into_iter()
				.map(Into::<Instruction>::into)
				.map(TryFrom::<Instruction>::try_from)
				.collect::<Result<VecDeque<_>, _>>()
		);
	}

	#[test]
	fn encoding_isomorphism() {
		let program = || -> Result<_, ()> {
			Ok(XCVMProgramBuilder::<
				XCVMNetwork,
				XCVMInstruction<XCVMNetwork, _, Vec<u8>, BTreeMap<u32, u128>>,
			>::from(Some("test tag".as_bytes().to_vec()), XCVMNetwork::PICASSO, 0)
			.spawn::<_, ()>(
				Some("test tag 2".as_bytes().to_vec()),
				XCVMNetwork::ETHEREUM,
				BTreeMap::from([(0x1337, 20_000)]),
				|child| Ok(child.transfer(vec![0xBE, 0xEF], BTreeMap::from([(0, 10_000)]))),
			)?
			.build())
		}()
		.expect("valid program");
		assert_eq!(program.clone(), decode(&encode(program)).expect("must decode"));
	}

	#[test]
	fn test_program() {
		// Transfer to Alice/Bob and redispatch the same program from itself, without the redispatch
		let program = || -> Result<_, ()> {
			Ok(XCVMProgramBuilder::<
				XCVMNetwork,
				XCVMInstruction<XCVMNetwork, _, Vec<u8>, BTreeMap<u32, u128>>,
			>::from(Some("test tag".as_bytes().to_vec()), XCVMNetwork::PICASSO, 0)
			.transfer(
				hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d")
					.expect("valid"),
				BTreeMap::from([(XCVMAsset::PICA.into(), 1337000000000000)]),
			)
			.transfer(
				hex::decode("8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48")
					.expect("valid"),
				BTreeMap::from([(XCVMAsset::PICA.into(), 1336000000000000)]),
			)
			.build())
		}()
		.expect("valid program");
		assert_eq!(
      "0a80010a3e0a3c0a220a20d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d1216080112120a1000901092febf040000000000000000000a3e0a3c0a220a208eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a481216080112120a1000806bbd15bf04000000000000000000",
      hex::encode(encode(program))
    );
	}

	#[test]
	fn test_cross_chain_program() {
		// Transfer to Alice/Bob and redispatch
		let program = || -> Result<_, ()> {
			Ok(XCVMProgramBuilder::<
				XCVMNetwork,
				XCVMInstruction<XCVMNetwork, _, Vec<u8>, BTreeMap<u32, u128>>,
			>::from(Some("test tag".as_bytes().to_vec()), XCVMNetwork::PICASSO, 0)
			.transfer(
				hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d")
					.expect("valid"),
				BTreeMap::from([(XCVMAsset::PICA.into(), 1337000000000000)]),
			)
			.transfer(
				hex::decode("8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48")
					.expect("valid"),
				BTreeMap::from([(XCVMAsset::PICA.into(), 1336000000000000)]),
			)
			.spawn::<_, ()>(
				Some("test tag 2".as_bytes().to_vec()),
				XCVMNetwork::ETHEREUM,
				BTreeMap::from([(XCVMAsset::PICA.into(), 1000000000000)]),
				|child| {
					Ok(child.transfer(
						vec![1u8; 20],
						BTreeMap::from([(XCVMAsset::PICA.into(), 1000000000000)]),
					))
				},
			)?
			.build())
		}()
		.expect("valid program");
		assert_eq!(
      "0ad2010a3e0a3c0a220a20d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d1216080112120a1000901092febf040000000000000000000a3e0a3c0a220a208eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a481216080112120a1000806bbd15bf040000000000000000000a501a4e08021216080112120a100010a5d4e800000000000000000000001a320a300a160a1401010101010101010101010101010101010101011216080112120a100010a5d4e80000000000000000000000",
      hex::encode(encode(program))
    );
	}
}
