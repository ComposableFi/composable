#![no_std]

extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use xcvm_core::XCVMInstruction;

include!("xcvm.rs");

impl<
		TNetwork: Into<u32>,
		TAbiEncoded: Into<Vec<u8>>,
		TAccount: Into<Vec<u8>>,
		TAssets: Into<BTreeMap<u32, u64>>,
	> From<XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>> for Instruction
{
	fn from(instruction: XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>) -> Self {
		Instruction {
			instruction: Some(match instruction {
				XCVMInstruction::Transfer(destination, assets) =>
					instruction::Instruction::Transfer(Transfer {
						destination: Some(Account {
							account: Some(account::Account::Addressed(destination.into())),
						}),
						assets: assets.into(),
					}),
				XCVMInstruction::Bridge(network, assets) =>
					instruction::Instruction::Bridge(Bridge {
						network: network.into(),
						assets: assets.into(),
					}),
				XCVMInstruction::Call(payload) =>
					instruction::Instruction::Call(Call { payload: payload.into() }),
			}),
		}
	}
}

impl<
		TNetwork: From<u32>,
		TAbiEncoded: From<Vec<u8>>,
		TAccount: From<Vec<u8>> + From<String>,
		TAssets: From<BTreeMap<u32, u64>>,
	> TryFrom<Instruction> for XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>
{
	type Error = ();
	fn try_from(Instruction { instruction }: Instruction) -> Result<Self, Self::Error> {
		instruction
			.map(|instruction| match instruction {
				instruction::Instruction::Transfer(Transfer {
					destination:
						Some(Account { account: Some(account::Account::Addressed(destination)) }),
					assets,
				}) => Ok(XCVMInstruction::Transfer(destination.into(), assets.into())),
				instruction::Instruction::Transfer(Transfer {
					destination:
						Some(Account { account: Some(account::Account::Named(destination)) }),
					assets,
				}) => Ok(XCVMInstruction::Transfer(destination.into(), assets.into())),
				instruction::Instruction::Bridge(Bridge { network, assets }) =>
					Ok(XCVMInstruction::Bridge(network.into(), assets.into())),
				instruction::Instruction::Call(Call { payload }) =>
					Ok(XCVMInstruction::Call(payload.into())),
				_ => Err(()),
			})
			.unwrap_or(Err(()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::vec;
	use xcvm_core::{AbiEncoded, XCVMContractBuilder, XCVMNetwork, XCVMProtocol};

	#[test]
	fn isomorphism() {
		struct DummyProtocol;
		impl XCVMProtocol<XCVMNetwork> for DummyProtocol {
			type Error = ();
			fn serialize(&self, network: XCVMNetwork) -> Result<AbiEncoded, ()> {
				match network {
					XCVMNetwork::PICASSO => Ok(AbiEncoded::from(vec![0xCA, 0xFE, 0xBA, 0xBE])),
					XCVMNetwork::ETHEREUM => Ok(AbiEncoded::from(vec![0xDE, 0xAD, 0xC0, 0xDE])),
					_ => Err(()),
				}
			}
		}

		let contract = || -> Result<_, ()> {
			Ok(XCVMContractBuilder::<
				XCVMNetwork,
				XCVMInstruction<XCVMNetwork, _, Vec<u8>, BTreeMap<u32, u64>>,
			>::from(XCVMNetwork::PICASSO)
			.call(DummyProtocol)?
			.bridge(XCVMNetwork::ETHEREUM, BTreeMap::from([(0x1337, 20_000)]))
			.call(DummyProtocol)?
			.transfer(vec![0xBE, 0xEF], BTreeMap::from([(0, 10_000)])))
		}()
		.expect("valid contract");

		// f^-1 . f = id
		assert_eq!(
			Ok(contract.instructions.clone()),
			contract
				.instructions
				.into_iter()
				.map(Into::<Instruction>::into)
				.map(TryFrom::<Instruction>::try_from)
				.collect::<Result<Vec<_>, _>>()
		);
	}
}
