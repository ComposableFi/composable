pub use alloc::{
	boxed::Box,
	collections::VecDeque,
	string::{String, ToString},
	vec,
	vec::Vec,
};
pub use core::str::FromStr;
pub use cosmwasm_std::{Addr, Binary, Coin};
pub use serde::{Deserialize, Serialize};

pub use parity_scale_codec::{Decode, Encode};

use super::{pb, NonEmptyExt};
use crate::{shared::Displayed, Amount, Destination, Funds};

pub type XCVMPacket<TAbiEncoded, TAccount, TAssets> =
	crate::Packet<XCVMProgram<TAbiEncoded, TAccount, TAssets>>;

pub type XCVMProgram<TAbiEncoded, TAccount, TAssets> =
	crate::Program<VecDeque<crate::Instruction<TAbiEncoded, TAccount, TAssets>>>;

impl<TAbiEncoded, TAccount, TAssets> super::Isomorphism
	for XCVMPacket<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Message = pb::xcvm::Packet;
}

impl<TAbiEncoded, TAccount, TAssets> super::Isomorphism
	for XCVMProgram<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Message = pb::xcvm::Program;
}

impl TryFrom<pb::xcvm::UserOrigin> for crate::UserOrigin {
	type Error = ();
	fn try_from(value: pb::xcvm::UserOrigin) -> Result<Self, Self::Error> {
		Ok(crate::UserOrigin {
			network_id: value.network_id.into(),
			user_id: value.account.non_empty()?.into(),
		})
	}
}

impl From<crate::UserOrigin> for pb::xcvm::UserOrigin {
	fn from(value: crate::UserOrigin) -> Self {
		Self { network_id: value.network_id.into(), account: value.user_id.into() }
	}
}

impl TryFrom<pb::xcvm::PacketAsset> for (crate::AssetId, Displayed<u128>) {
	type Error = ();
	fn try_from(value: pb::xcvm::PacketAsset) -> Result<Self, Self::Error> {
		Ok((value.asset_id.non_empty()?.into(), value.amount.non_empty()?.into()))
	}
}

impl From<(crate::AssetId, Displayed<u128>)> for pb::xcvm::PacketAsset {
	fn from((asset, amount): (crate::AssetId, Displayed<u128>)) -> Self {
		Self { asset_id: Some(asset.into()), amount: Some(amount.into()) }
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Packet>
	for XCVMPacket<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(packet: pb::xcvm::Packet) -> Result<Self, Self::Error> {
		Ok(XCVMPacket {
			interpreter: packet.interpreter.non_empty()?,
			user_origin: packet.user_origin.non_empty()?.try_into()?,
			salt: packet.salt,
			program: packet.program.non_empty()?.try_into()?,
			assets: Funds(
				packet
					.assets
					.into_iter()
					.map(TryFrom::try_from)
					.collect::<Result<Vec<_>, _>>()?,
			),
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> From<XCVMPacket<TAbiEncoded, TAccount, TAssets>>
	for pb::xcvm::Packet
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Balance)>>,
{
	fn from(value: XCVMPacket<TAbiEncoded, TAccount, TAssets>) -> Self {
		Self {
			interpreter: value.interpreter,
			user_origin: Some(value.user_origin.into()),
			salt: value.salt,
			program: Some(value.program.into()),
			assets: value.assets.0.into_iter().map(pb::xcvm::PacketAsset::from).collect(),
		}
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Program>
	for XCVMProgram<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(program: pb::xcvm::Program) -> Result<Self, Self::Error> {
		Ok(XCVMProgram {
			tag: program.tag,
			instructions: super::try_from_sequence(program.instructions)?,
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> From<XCVMProgram<TAbiEncoded, TAccount, TAssets>>
	for pb::xcvm::Program
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Balance)>>,
{
	fn from(program: XCVMProgram<TAbiEncoded, TAccount, TAssets>) -> Self {
		let instructions = super::from_sequence(program.instructions);
		Self { tag: program.tag, instructions }
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Instruction>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(instruction: pb::xcvm::Instruction) -> Result<Self, Self::Error> {
		let instruction = instruction.instruction.non_empty()?;
		match instruction {
			pb::xcvm::instruction::Instruction::Transfer(t) => t.try_into(),
			pb::xcvm::instruction::Instruction::Spawn(s) => s.try_into(),
			pb::xcvm::instruction::Instruction::Call(c) => c.try_into(),
			pb::xcvm::instruction::Instruction::Exchange(x) => x.try_into(),
		}
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Transfer>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(transfer: pb::xcvm::Transfer) -> Result<Self, Self::Error> {
		let account_type = transfer.account_type.non_empty()?;
		Ok(crate::Instruction::Transfer {
			to: account_type.try_into()?,
			assets: transfer
				.assets
				.into_iter()
				.map(|asset| asset.try_into())
				.collect::<Result<Vec<_>, _>>()?
				.into(),
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Spawn>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(spawn: pb::xcvm::Spawn) -> Result<Self, Self::Error> {
		let assets: Vec<(crate::AssetId, crate::Balance)> = super::try_from_sequence(spawn.assets)?;
		Ok(crate::Instruction::Spawn {
			network_id: spawn.network_id.into(),
			salt: spawn.salt,
			assets: assets.into(),
			program: spawn.program.non_empty()?.try_into()?,
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Exchange>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(value: pb::xcvm::Exchange) -> Result<Self, Self::Error> {
		Ok(crate::Instruction::Exchange {
			exchange_id: value.exchange_id.non_empty()?.into(),
			give: super::try_from_sequence::<Vec<_>, _, _>(value.give)?.into(),
			want: super::try_from_sequence::<Vec<_>, _, _>(value.want)?.into(),
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Call>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(call: pb::xcvm::Call) -> Result<Self, Self::Error> {
		let bindings = super::try_from_sequence(call.bindings)?;
		let encoded = call.payload.try_into().map_err(|_| ())?;
		Ok(crate::Instruction::Call { bindings, encoded })
	}
}

impl<TAbiEncoded, TAccount, TAssets> From<crate::Instruction<TAbiEncoded, TAccount, TAssets>>
	for pb::xcvm::Instruction
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Balance)>>,
{
	fn from(instruction: crate::Instruction<TAbiEncoded, TAccount, TAssets>) -> Self {
		use crate::Instruction;
		use pb::xcvm::instruction::Instruction as Msg;
		let instruction = match instruction {
			Instruction::Transfer { to, assets } => Msg::Transfer(pb::xcvm::Transfer {
				assets: assets.into().into_iter().map(|asset| asset.into()).collect(),
				account_type: Some(to.into()),
			}),
			Instruction::Call { bindings, encoded } => Msg::Call(pb::xcvm::Call {
				payload: encoded.into(),
				bindings: super::from_sequence(bindings),
			}),
			Instruction::Spawn { network_id, salt, assets, program } =>
				Msg::Spawn(pb::xcvm::Spawn {
					network_id: network_id.into(),
					salt,
					program: Some(program.into()),
					assets: assets.into().into_iter().map(|asset| asset.into()).collect(),
				}),
			Instruction::Exchange { exchange_id, give, want } =>
				Msg::Exchange(pb::xcvm::Exchange {
					exchange_id: Some(exchange_id.into()),
					give: give.into().into_iter().map(|asset| asset.into()).collect(),
					want: want.into().into_iter().map(|asset| asset.into()).collect(),
				}),
		};
		Self { instruction: Some(instruction) }
	}
}

impl TryFrom<pb::xcvm::BindingValue> for crate::BindingValue {
	type Error = ();

	fn try_from(binding_value: pb::xcvm::BindingValue) -> Result<Self, Self::Error> {
		use pb::xcvm::binding_value::Type;
		Ok(match binding_value.r#type.non_empty()? {
			Type::Register(reg) => {
				let reg = pb::xcvm::Register::from_i32(reg).ok_or(())?;
				Self::Register(reg.into())
			},
			Type::AssetId(asset_id) => Self::Asset(asset_id.into()),
			Type::AssetAmount(asset_amount) => Self::AssetAmount(
				asset_amount.asset_id.non_empty()?.into(),
				asset_amount.balance.non_empty()?.try_into()?,
			),
		})
	}
}

impl From<crate::BindingValue> for pb::xcvm::BindingValue {
	fn from(binding_value: crate::BindingValue) -> Self {
		use pb::xcvm::binding_value::Type;
		let typ = match binding_value {
			crate::BindingValue::Register(reg) =>
				Type::Register(pb::xcvm::Register::from(reg) as i32),
			crate::BindingValue::Asset(asset_id) => Type::AssetId(asset_id.into()),
			crate::BindingValue::AssetAmount(asset_id, balance) =>
				Type::AssetAmount(pb::xcvm::AssetAmount {
					asset_id: Some(asset_id.into()),
					balance: Some(balance.into()),
				}),
		};
		Self { r#type: Some(typ) }
	}
}

impl TryFrom<pb::xcvm::Binding> for (u32, crate::BindingValue) {
	type Error = ();
	fn try_from(binding: pb::xcvm::Binding) -> Result<Self, Self::Error> {
		Ok((binding.position, binding.binding_value.non_empty()?.try_into()?))
	}
}

impl From<(u32, crate::BindingValue)> for pb::xcvm::Binding {
	fn from((position, binding_value): (u32, crate::BindingValue)) -> Self {
		Self { position, binding_value: Some(binding_value.into()) }
	}
}

impl From<pb::xcvm::Register> for crate::Register {
	fn from(reg: pb::xcvm::Register) -> Self {
		match reg {
			pb::xcvm::Register::Ip => Self::Ip,
			pb::xcvm::Register::Tip => Self::Tip,
			pb::xcvm::Register::This => Self::This,
			pb::xcvm::Register::Result => Self::Result,
		}
	}
}

impl From<crate::Register> for pb::xcvm::Register {
	fn from(reg: crate::Register) -> Self {
		match reg {
			crate::Register::Ip => Self::Ip,
			crate::Register::Tip => Self::Tip,
			crate::Register::This => Self::This,
			crate::Register::Result => Self::Result,
		}
	}
}

impl<TAccount> TryFrom<pb::xcvm::transfer::AccountType> for Destination<TAccount>
where
	TAccount: TryFrom<Vec<u8>>,
{
	type Error = ();

	fn try_from(account_type: pb::xcvm::transfer::AccountType) -> Result<Self, Self::Error> {
		Ok(match account_type {
			pb::xcvm::transfer::AccountType::Account(account) =>
				Destination::Account(account.try_into().map_err(|_| ())?),
			pb::xcvm::transfer::AccountType::Tip(_) => Destination::Tip,
		})
	}
}

impl<TAccount> From<crate::Destination<TAccount>> for pb::xcvm::transfer::AccountType
where
	TAccount: Into<Vec<u8>>,
{
	fn from(destination: crate::Destination<TAccount>) -> Self {
		match destination {
			Destination::Account(account) => Self::Account(account.into()),
			Destination::Tip => Self::Tip(pb::xcvm::Tip {}),
		}
	}
}

impl TryFrom<pb::xcvm::Asset> for (crate::AssetId, crate::Balance) {
	type Error = ();

	fn try_from(asset: pb::xcvm::Asset) -> Result<Self, Self::Error> {
		let asset_id = asset.asset_id.non_empty()?.into();
		let amount = asset.balance.non_empty()?.try_into()?;

		Ok((asset_id, amount))
	}
}

impl From<(crate::AssetId, crate::Balance)> for pb::xcvm::Asset {
	fn from((asset_id, amount): (crate::AssetId, crate::Balance)) -> Self {
		Self { asset_id: Some(asset_id.into()), balance: Some(amount.into()) }
	}
}

impl TryFrom<pb::xcvm::Balance> for crate::Balance {
	type Error = ();

	fn try_from(balance: pb::xcvm::Balance) -> Result<Self, Self::Error> {
		use pb::xcvm::balance::BalanceType;

		let balance_type = balance.balance_type.non_empty()?;

		match balance_type {
			BalanceType::Ratio(ratio) => Ok(crate::Balance::new(ratio.try_into()?, false)),
			BalanceType::Absolute(pb::xcvm::Absolute { value }) => {
				let value = value.non_empty()?;
				Ok(crate::Balance::new(Amount::absolute(value.into()), false))
			},
			BalanceType::Unit(unit) => unit.try_into(),
		}
	}
}

impl TryFrom<pb::xcvm::Ratio> for crate::Amount {
	type Error = ();

	fn try_from(ratio: pb::xcvm::Ratio) -> Result<Self, Self::Error> {
		let nominator = ratio.nominator;
		let denominator = ratio.denominator;
		Ok(Self::from((nominator, denominator)))
	}
}

impl TryFrom<pb::xcvm::Unit> for crate::Balance {
	type Error = ();

	fn try_from(unit: pb::xcvm::Unit) -> Result<Self, Self::Error> {
		let integer = unit.integer.non_empty()?;
		let ratio = unit.ratio.non_empty()?;
		Ok(crate::Balance::new(
			Amount::new(
				integer.into(),
				Amount::from((ratio.nominator, ratio.denominator)).slope.into(),
			),
			true,
		))
	}
}

impl From<crate::Balance> for pb::xcvm::Balance {
	fn from(balance: crate::Balance) -> Self {
		// Note that although functionally nothing changes, there is no guarantee of getting the
		// same protobuf when you convert protobuf to XCVM types and convert back again. Because
		// `intercept = 0 & ratio = 0` is always converted to `Absolute`. But this can be also
		// expressed with `Ratio` and `Unit` as well. Also, since the ratio is expanded to use
		// denominator `MAX_PARTS`, it also won't be the same.

		let balance_type = if balance.is_unit {
			pb::xcvm::balance::BalanceType::Unit(pb::xcvm::Unit {
				integer: Some(balance.amount.intercept.0.into()),
				ratio: Some(pb::xcvm::Ratio {
					nominator: balance.amount.slope.0,
					denominator: Amount::MAX_PARTS,
				}),
			})
		} else if balance.amount.is_absolute() {
			pb::xcvm::balance::BalanceType::Absolute(pb::xcvm::Absolute {
				value: Some(balance.amount.intercept.0.into()),
			})
		} else {
			pb::xcvm::balance::BalanceType::Ratio(pb::xcvm::Ratio {
				nominator: balance.amount.slope.0,
				denominator: Amount::MAX_PARTS,
			})
		};
		Self { balance_type: Some(balance_type) }
	}
}

#[test]
fn test_balance_to_amount_works() {
	let ratio = pb::xcvm::Ratio { nominator: 3u64.into(), denominator: 5u64.into() };
	let balance =
		pb::xcvm::Balance { balance_type: Some(pb::xcvm::balance::BalanceType::Ratio(ratio)) };
	let xcvm_balance: crate::Balance = balance.try_into().unwrap();
	assert_eq!(xcvm_balance.amount.intercept, Displayed(0));
}
