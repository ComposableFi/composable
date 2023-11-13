//! Responsible for mapping CVM program as inputs on specific origin chain to on the wire
//! representation understood by each chain.
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
use crate::{shared::Displayed, Destination, Funds};

pub type CVMPacket<TAbiEncoded, TAccount, TAssets> =
	crate::Packet<CVMProgram<TAbiEncoded, TAccount, TAssets>>;

pub type CVMProgram<TAbiEncoded, TAccount, TAssets> =
	crate::Program<VecDeque<crate::Instruction<TAbiEncoded, TAccount, TAssets>>>;

impl<TAbiEncoded, TAccount, TAssets> super::Isomorphism
	for CVMPacket<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>> + Into<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>> + Into<Vec<u8>>,
	TAssets:
		From<Vec<(crate::AssetId, crate::Amount)>> + Into<Vec<(crate::AssetId, crate::Amount)>>,
{
	type Message = pb::program::Packet;
}

impl<TAbiEncoded, TAccount, TAssets> super::Isomorphism
	for CVMProgram<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>> + Into<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>> + Into<Vec<u8>>,
	TAssets:
		From<Vec<(crate::AssetId, crate::Amount)>> + Into<Vec<(crate::AssetId, crate::Amount)>>,
{
	type Message = pb::program::Program;
}

super::define_conversion! {
	(value: pb::program::UserOrigin) -> {
		Ok(crate::UserOrigin {
			network_id: value.network_id.into(),
			user_id: value.account.non_empty()?.into(),
		})
	}
	(value: crate::UserOrigin) -> {
		Self { network_id: value.network_id.into(), account: value.user_id.into() }
	}
}

super::define_conversion! {
	(value: pb::program::PacketAsset) -> {
		Ok((value.asset_id.non_empty()?.into(), value.amount.non_empty()?.into()))
	}

	(value: (crate::AssetId, Displayed<u128>)) -> {
		let (asset, amount) = value;
		Self { asset_id: Some(asset.into()), amount: Some(amount.into()) }
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::program::Packet>
	for CVMPacket<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Amount)>>,
{
	type Error = ();

	fn try_from(packet: pb::program::Packet) -> Result<Self, Self::Error> {
		Ok(CVMPacket {
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

impl<TAbiEncoded, TAccount, TAssets> From<CVMPacket<TAbiEncoded, TAccount, TAssets>>
	for pb::program::Packet
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Amount)>>,
{
	fn from(value: CVMPacket<TAbiEncoded, TAccount, TAssets>) -> Self {
		Self {
			interpreter: value.interpreter,
			user_origin: Some(value.user_origin.into()),
			salt: value.salt,
			program: Some(value.program.into()),
			assets: value.assets.0.into_iter().map(pb::program::PacketAsset::from).collect(),
		}
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::program::Program>
	for CVMProgram<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Amount)>>,
{
	type Error = ();

	fn try_from(program: pb::program::Program) -> Result<Self, Self::Error> {
		Ok(CVMProgram {
			tag: program.tag,
			instructions: super::try_from_sequence(program.instructions)?,
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> From<CVMProgram<TAbiEncoded, TAccount, TAssets>>
	for pb::program::Program
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Amount)>>,
{
	fn from(program: CVMProgram<TAbiEncoded, TAccount, TAssets>) -> Self {
		let instructions = super::from_sequence(program.instructions);
		Self { tag: program.tag, instructions }
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::program::Instruction>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Amount)>>,
{
	type Error = ();

	fn try_from(instruction: pb::program::Instruction) -> Result<Self, Self::Error> {
		let instruction = instruction.instruction.non_empty()?;
		match instruction {
			pb::program::instruction::Instruction::Transfer(t) => t.try_into(),
			pb::program::instruction::Instruction::Spawn(s) => s.try_into(),
			pb::program::instruction::Instruction::Call(c) => c.try_into(),
			pb::program::instruction::Instruction::Exchange(x) => x.try_into(),
		}
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::program::Transfer>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Amount)>>,
{
	type Error = ();

	fn try_from(transfer: pb::program::Transfer) -> Result<Self, Self::Error> {
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

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::program::Spawn>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Amount)>>,
{
	type Error = ();

	fn try_from(spawn: pb::program::Spawn) -> Result<Self, Self::Error> {
		let assets: Vec<(crate::AssetId, crate::Amount)> = super::try_from_sequence(spawn.assets)?;
		Ok(crate::Instruction::Spawn {
			network_id: spawn.network_id.into(),
			salt: spawn.salt,
			assets: assets.into(),
			program: spawn.program.non_empty()?.try_into()?,
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::program::Exchange>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Amount)>>,
{
	type Error = ();

	fn try_from(value: pb::program::Exchange) -> Result<Self, Self::Error> {
		Ok(crate::Instruction::Exchange {
			exchange_id: value.exchange_id.non_empty()?.into(),
			give: super::try_from_sequence::<Vec<_>, _, _>(value.give)?.into(),
			want: super::try_from_sequence::<Vec<_>, _, _>(value.want)?.into(),
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::program::Call>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: TryFrom<Vec<u8>>,
	TAssets: From<Vec<(crate::AssetId, crate::Amount)>>,
{
	type Error = ();

	fn try_from(call: pb::program::Call) -> Result<Self, Self::Error> {
		let bindings = super::try_from_sequence(call.bindings)?;
		let encoded = call.payload.try_into().map_err(|_| ())?;
		Ok(crate::Instruction::Call { bindings, encoded })
	}
}

impl<TAbiEncoded, TAccount, TAssets> From<crate::Instruction<TAbiEncoded, TAccount, TAssets>>
	for pb::program::Instruction
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Amount)>>,
{
	fn from(instruction: crate::Instruction<TAbiEncoded, TAccount, TAssets>) -> Self {
		use crate::Instruction;
		use pb::program::instruction::Instruction as Msg;
		let instruction = match instruction {
			Instruction::Transfer { to, assets } => Msg::Transfer(pb::program::Transfer {
				assets: assets.into().into_iter().map(|asset| asset.into()).collect(),
				account_type: Some(to.into()),
			}),
			Instruction::Call { bindings, encoded } => Msg::Call(pb::program::Call {
				payload: encoded.into(),
				bindings: super::from_sequence(bindings),
			}),
			Instruction::Spawn { network_id, salt, assets, program } =>
				Msg::Spawn(pb::program::Spawn {
					network_id: network_id.into(),
					salt,
					program: Some(program.into()),
					assets: assets.into().into_iter().map(|asset| asset.into()).collect(),
				}),
			Instruction::Exchange { exchange_id, give, want } =>
				Msg::Exchange(pb::program::Exchange {
					exchange_id: Some(exchange_id.into()),
					give: give.into().into_iter().map(|asset| asset.into()).collect(),
					want: want.into().into_iter().map(|asset| asset.into()).collect(),
				}),
		};
		Self { instruction: Some(instruction) }
	}
}

super::define_conversion! {
	(binding_value: pb::program::BindingValue) -> {
		use pb::program::binding_value::Type;
		Ok(match binding_value.r#type.non_empty()? {
			Type::Register(reg) => {
				let reg = pb::program::Register::from_i32(reg).ok_or(())?;
				Self::Register(reg.into())
			},
			Type::AssetId(asset_id) => Self::Asset(asset_id.into()),
			Type::AssetAmount(asset_amount) => Self::AssetAmount(
				asset_amount.asset_id.non_empty()?.into(),
				asset_amount.balance.non_empty()?.try_into()?,
			),
		})
	}

	(binding_value: crate::BindingValue) -> {
		use pb::program::binding_value::Type;
		let typ = match binding_value {
			crate::BindingValue::Register(reg) =>
				Type::Register(pb::program::Register::from(reg) as i32),
			crate::BindingValue::Asset(asset_id) => Type::AssetId(asset_id.into()),
			crate::BindingValue::AssetAmount(asset_id, balance) =>
				Type::AssetAmount(pb::program::AssetAmount {
					asset_id: Some(asset_id.into()),
					balance: Some(balance.into()),
				}),
		};
		Self { r#type: Some(typ) }
	}
}

super::define_conversion! {
	(binding: pb::program::Binding) -> {
		Ok((binding.position, binding.binding_value.non_empty()?.try_into()?))
	}
	(binding: (u32, crate::BindingValue)) -> {
		let (position, binding_value) = binding;
		Self { position, binding_value: Some(binding_value.into()) }
	}
}

impl From<pb::program::Register> for crate::Register {
	fn from(reg: pb::program::Register) -> Self {
		match reg {
			pb::program::Register::Ip => Self::Ip,
			pb::program::Register::Tip => Self::Tip,
			pb::program::Register::This => Self::This,
			pb::program::Register::Result => Self::Result,
			pb::program::Register::Carry => unimplemented!("need to design register with data"),
		}
	}
}

impl From<crate::Register> for pb::program::Register {
	fn from(reg: crate::Register) -> Self {
		match reg {
			crate::Register::Ip => Self::Ip,
			crate::Register::Tip => Self::Tip,
			crate::Register::This => Self::This,
			crate::Register::Result => Self::Result,
			crate::Register::Carry(_) => unimplemented!("map with data"),
		}
	}
}

impl<TAccount> TryFrom<pb::program::transfer::AccountType> for Destination<TAccount>
where
	TAccount: TryFrom<Vec<u8>>,
{
	type Error = ();

	fn try_from(account_type: pb::program::transfer::AccountType) -> Result<Self, Self::Error> {
		Ok(match account_type {
			pb::program::transfer::AccountType::Account(account) =>
				Destination::Account(account.try_into().map_err(|_| ())?),
			pb::program::transfer::AccountType::Tip(_) => Destination::Tip,
		})
	}
}

impl<TAccount> From<crate::Destination<TAccount>> for pb::program::transfer::AccountType
where
	TAccount: Into<Vec<u8>>,
{
	fn from(destination: crate::Destination<TAccount>) -> Self {
		match destination {
			Destination::Account(account) => Self::Account(account.into()),
			Destination::Tip => Self::Tip(pb::program::Tip {}),
		}
	}
}

super::define_conversion! {
	(asset: pb::program::Asset) -> {
		let asset_id = asset.asset_id.non_empty()?.into();
		let amount = asset.balance.non_empty()?.try_into()?;
		Ok((asset_id, amount))
	}
	(asset: (crate::AssetId, crate::Amount)) -> {
		let (asset_id, amount) = asset;
		Self { asset_id: Some(asset_id.into()), balance: Some(amount.into()) }
	}
}

impl TryFrom<pb::program::Balance> for crate::Amount {
	type Error = ();
	fn try_from(balance: pb::program::Balance) -> Result<Self, Self::Error> {
		let ratio = balance.ratio.map(|ratio| ratio.nominator).unwrap_or(0);
		let absolute: Displayed<u128> = balance
			.absolute
			.map(|x| x.value.unwrap_or_default())
			.map(Into::into)
			.unwrap_or_default();
		Ok(crate::Amount::new(absolute.into(), ratio))
	}
}
impl From<crate::Amount> for pb::program::Balance {
	fn from(balance: crate::Amount) -> pb::program::Balance {
		let absolute = Some(pb::program::Absolute { value: Some(balance.intercept.0.into()) });
		let ratio = Some(pb::program::Ratio { nominator: balance.slope.0 });
		Self { absolute, ratio }
	}
}
