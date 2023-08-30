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

#[cfg(feature = "std")]
pub use cosmwasm_schema::{cw_serde, QueryResponses};

#[cfg(feature = "std")]
pub use schemars::JsonSchema;

use super::pb;
use crate::{shared::Displayed, Amount, Destination, Funds, NetworkId};

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

impl From<Vec<u8>> for pb::xcvm::Salt {
	fn from(value: Vec<u8>) -> Self {
		Self { salt: value }
	}
}

impl From<crate::UserId> for pb::xcvm::Account {
	fn from(crate::UserId(account): crate::UserId) -> Self {
		Self { account }
	}
}

impl From<crate::UserOrigin> for pb::xcvm::UserOrigin {
	fn from(value: crate::UserOrigin) -> Self {
		Self { network: Some(value.network_id.into()), account: Some(value.user_id.into()) }
	}
}

impl From<(crate::AssetId, Displayed<u128>)> for pb::xcvm::PacketAsset {
	fn from((asset, amount): (crate::AssetId, Displayed<u128>)) -> Self {
		Self { asset_id: Some(asset.into()), amount: Some(amount.into()) }
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
			interpreter: Some(pb::xcvm::Account { account: value.interpreter }),
			user_origin: Some(value.user_origin.into()),
			salt: Some(value.salt.into()),
			program: Some(value.program.into()),
			assets: value.assets.0.into_iter().map(pb::xcvm::PacketAsset::from).collect(),
		}
	}
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
			network_id: value.network.ok_or(())?.network_id.into(),
			user_id: value.account.ok_or(())?.account.into(),
		})
	}
}

impl TryFrom<pb::xcvm::PacketAsset> for (crate::AssetId, Displayed<u128>) {
	type Error = ();
	fn try_from(value: pb::xcvm::PacketAsset) -> Result<Self, Self::Error> {
		Ok((
			crate::AssetId::from(u128::from(value.asset_id.ok_or(())?.id.ok_or(())?)),
			value.amount.ok_or(())?.into(),
		))
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Packet>
	for XCVMPacket<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(packet: pb::xcvm::Packet) -> Result<Self, Self::Error> {
		Ok(XCVMPacket {
			interpreter: packet.interpreter.ok_or(())?.account,
			user_origin: packet.user_origin.ok_or(())?.try_into()?,
			salt: packet.salt.map(|s| s.salt).ok_or(())?,
			program: packet.program.ok_or(())?.try_into()?,
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

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Program>
	for XCVMProgram<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(program: pb::xcvm::Program) -> Result<Self, Self::Error> {
		Ok(XCVMProgram {
			tag: program.tag,
			instructions: program.instructions.ok_or(())?.try_into()?,
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Instructions>
	for VecDeque<crate::Instruction<TAbiEncoded, TAccount, TAssets>>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(instructions: pb::xcvm::Instructions) -> Result<Self, Self::Error> {
		let mut instrs = VecDeque::new();
		for inst in instructions.instructions {
			instrs.push_back(inst.try_into()?);
		}
		Ok(instrs)
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Instruction>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(instruction: pb::xcvm::Instruction) -> Result<Self, Self::Error> {
		instruction.instruction.ok_or(())?.try_into()
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::instruction::Instruction>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(instruction: pb::xcvm::instruction::Instruction) -> Result<Self, Self::Error> {
		match instruction {
			pb::xcvm::instruction::Instruction::Transfer(t) => t.try_into(),
			pb::xcvm::instruction::Instruction::Spawn(s) => s.try_into(),
			pb::xcvm::instruction::Instruction::Call(c) => c.try_into(),
			pb::xcvm::instruction::Instruction::Exchange(x) => x.try_into(),
		}
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Call>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(call: pb::xcvm::Call) -> Result<Self, Self::Error> {
		let bindings = call.bindings.ok_or(())?.try_into()?;
		Ok(crate::Instruction::Call { bindings, encoded: call.payload.try_into().map_err(|_| ())? })
	}
}

impl TryFrom<pb::xcvm::Bindings> for crate::Bindings {
	type Error = ();

	fn try_from(bindings: pb::xcvm::Bindings) -> Result<Self, Self::Error> {
		bindings
			.bindings
			.into_iter()
			.map(|binding| {
				let binding_value = binding.binding_value.ok_or(())?.try_into()?;
				Ok((binding.position, binding_value))
			})
			.collect()
	}
}

impl TryFrom<pb::xcvm::BindingValue> for crate::BindingValue {
	type Error = ();

	fn try_from(binding_value: pb::xcvm::BindingValue) -> Result<Self, Self::Error> {
		binding_value.r#type.ok_or(())?.try_into()
	}
}

impl TryFrom<pb::xcvm::binding_value::Type> for crate::BindingValue {
	type Error = ();

	fn try_from(binding_val: pb::xcvm::binding_value::Type) -> Result<Self, Self::Error> {
		use pb::xcvm::binding_value::Type;
		Ok(match binding_val {
			Type::Self_(_) => crate::BindingValue::Register(crate::Register::This),
			Type::Tip(_) => crate::BindingValue::Register(crate::Register::Tip),
			Type::Result(_) => crate::BindingValue::Register(crate::Register::Result),
			Type::IpRegister(_) => crate::BindingValue::Register(crate::Register::Ip),
			Type::AssetAmount(pb::xcvm::AssetAmount { asset_id, balance }) =>
				crate::BindingValue::AssetAmount(
					asset_id.ok_or(())?.try_into()?,
					balance.ok_or(())?.try_into()?,
				),
			Type::AssetId(asset_id) => crate::BindingValue::Asset(asset_id.try_into()?),
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Spawn>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(spawn: pb::xcvm::Spawn) -> Result<Self, Self::Error> {
		let network = spawn.network.ok_or(())?.network_id.into();
		let salt = spawn.salt.ok_or(())?.salt;
		Ok(crate::Instruction::Spawn {
			network,
			salt,
			assets: spawn
				.assets
				.into_iter()
				.map(|asset| asset.try_into())
				.collect::<Result<Vec<_>, _>>()?
				.into(),
			program: XCVMProgram {
				tag: Vec::new(),
				instructions: spawn.program.ok_or(())?.instructions.ok_or(())?.try_into()?,
			},
		})
	}
}

impl From<pb::xcvm::Network> for NetworkId {
	fn from(network: pb::xcvm::Network) -> Self {
		Self(network.network_id)
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Transfer>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(transfer: pb::xcvm::Transfer) -> Result<Self, Self::Error> {
		let account_type = transfer.account_type.ok_or(())?;
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

impl<TAbiEncoded, TAccount, TAssets> TryFrom<pb::xcvm::Exchange>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(value: pb::xcvm::Exchange) -> Result<Self, Self::Error> {
		Ok(crate::Instruction::Exchange {
			id: value.id.and_then(|x| x.id).map(TryInto::try_into).ok_or(())?.map_err(|_| ())?,
			give: value
				.give
				.into_iter()
				.map(|asset| asset.try_into())
				.collect::<Result<Vec<_>, _>>()?
				.into(),
			want: value
				.want
				.into_iter()
				.map(|asset| asset.try_into())
				.collect::<Result<Vec<_>, _>>()?
				.into(),
		})
	}
}

impl<TAccount> TryFrom<pb::xcvm::transfer::AccountType> for Destination<TAccount>
where
	TAccount: for<'a> TryFrom<&'a [u8]>,
{
	type Error = ();

	fn try_from(account_type: pb::xcvm::transfer::AccountType) -> Result<Self, Self::Error> {
		Ok(match account_type {
			pb::xcvm::transfer::AccountType::Account(acc) =>
				Destination::Account(acc.account.as_slice().try_into().map_err(|_| ())?),
			pb::xcvm::transfer::AccountType::Tip(_) => Destination::Tip,
		})
	}
}

impl TryFrom<pb::xcvm::Asset> for (crate::AssetId, crate::Balance) {
	type Error = ();

	fn try_from(asset: pb::xcvm::Asset) -> Result<Self, Self::Error> {
		let asset_id = asset.asset_id.ok_or(())?.try_into()?;
		let amount = asset.balance.ok_or(())?.try_into()?;

		Ok((asset_id, amount))
	}
}

impl TryFrom<pb::xcvm::AssetId> for crate::AssetId {
	type Error = ();

	fn try_from(asset_id: pb::xcvm::AssetId) -> Result<Self, Self::Error> {
		Ok(crate::AssetId(asset_id.id.ok_or(())?.into()))
	}
}

impl TryFrom<pb::xcvm::Balance> for crate::Balance {
	type Error = ();

	fn try_from(balance: pb::xcvm::Balance) -> Result<Self, Self::Error> {
		use pb::xcvm::balance::BalanceType;

		let balance_type = balance.balance_type.ok_or(())?;

		match balance_type {
			BalanceType::Ratio(ratio) => Ok(crate::Balance::new(ratio.try_into()?, false)),
			BalanceType::Absolute(pb::xcvm::Absolute { value }) => {
				let value = value.ok_or(())?;
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
		let integer = unit.integer.ok_or(())?;
		let ratio = unit.ratio.ok_or(())?;
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

impl From<crate::AssetId> for pb::xcvm::AssetId {
	fn from(asset_id: crate::AssetId) -> Self {
		Self { id: Some(asset_id.0 .0.into()) }
	}
}

impl From<(crate::AssetId, crate::Balance)> for pb::xcvm::Asset {
	fn from((asset_id, amount): (crate::AssetId, crate::Balance)) -> Self {
		Self { asset_id: Some(asset_id.into()), balance: Some(amount.into()) }
	}
}

impl From<crate::BindingValue> for pb::xcvm::binding_value::Type {
	fn from(binding_value: crate::BindingValue) -> Self {
		match binding_value {
			crate::BindingValue::Register(crate::Register::Ip) =>
				Self::IpRegister(pb::xcvm::IpRegister { ip: 0 }),
			crate::BindingValue::Register(crate::Register::Tip) =>
				Self::Tip(pb::xcvm::Tip { id: 0 }),
			crate::BindingValue::Register(crate::Register::Result) =>
				Self::Result(pb::xcvm::Result { result: 0 }),
			crate::BindingValue::Register(crate::Register::This) =>
				Self::Self_(pb::xcvm::Self_ { self_: 0 }),
			crate::BindingValue::Asset(asset_id) => Self::AssetId(asset_id.into()),
			crate::BindingValue::AssetAmount(asset_id, balance) =>
				Self::AssetAmount(pb::xcvm::AssetAmount {
					asset_id: Some(asset_id.into()),
					balance: Some(balance.into()),
				}),
		}
	}
}

impl From<crate::BindingValue> for pb::xcvm::BindingValue {
	fn from(binding_value: crate::BindingValue) -> Self {
		Self { r#type: Some(binding_value.into()) }
	}
}

impl From<crate::NetworkId> for pb::xcvm::Network {
	fn from(network_id: crate::NetworkId) -> Self {
		Self { network_id: network_id.0 }
	}
}

impl<TAccount> From<crate::Destination<TAccount>> for pb::xcvm::transfer::AccountType
where
	TAccount: Into<Vec<u8>>,
{
	fn from(destination: crate::Destination<TAccount>) -> Self {
		match destination {
			Destination::Account(account) =>
				Self::Account(pb::xcvm::Account { account: account.into() }),
			Destination::Tip => Self::Tip(pb::xcvm::Tip { id: 0 }),
		}
	}
}

impl From<(u32, crate::BindingValue)> for pb::xcvm::Binding {
	fn from((position, binding_value): (u32, crate::BindingValue)) -> Self {
		Self { position, binding_value: Some(binding_value.into()) }
	}
}

impl<TAbiEncoded, TAccount, TAssets> From<crate::Instruction<TAbiEncoded, TAccount, TAssets>>
	for pb::xcvm::instruction::Instruction
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Balance)>>,
{
	fn from(instruction: crate::Instruction<TAbiEncoded, TAccount, TAssets>) -> Self {
		match instruction {
			crate::Instruction::Transfer { to, assets } => Self::Transfer(pb::xcvm::Transfer {
				assets: assets.into().into_iter().map(|asset| asset.into()).collect(),
				account_type: Some(to.into()),
			}),
			crate::Instruction::Call { bindings, encoded } => Self::Call(pb::xcvm::Call {
				payload: encoded.into(),
				bindings: Some(pb::xcvm::Bindings {
					bindings: bindings.into_iter().map(|binding| binding.into()).collect(),
				}),
			}),
			crate::Instruction::Spawn { network, salt, assets, program } =>
				Self::Spawn(pb::xcvm::Spawn {
					network: Some(pb::xcvm::Network { network_id: network.into() }),
					salt: Some(pb::xcvm::Salt { salt }),
					program: Some(program.into()),
					assets: assets.into().into_iter().map(|asset| asset.into()).collect(),
				}),
			crate::Instruction::Exchange { id, give, want } => Self::Exchange(pb::xcvm::Exchange {
				id: Some(pb::xcvm::ExchangeId { id: Some(id.into()) }),
				give: give.into().into_iter().map(|asset| asset.into()).collect(),
				want: want.into().into_iter().map(|asset| asset.into()).collect(),
			}),
		}
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
		Self { instruction: Some(instruction.into()) }
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
		Self {
			tag: program.tag,
			instructions: Some(pb::xcvm::Instructions {
				instructions: program.instructions.into_iter().map(|instr| instr.into()).collect(),
			}),
		}
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
