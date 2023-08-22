use core::fmt::Display;

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

use crate::{shared::Displayed, Amount, Destination, Funds, NetworkId};
use alloc::format;
use prost::{DecodeError, Message};

include!(concat!(env!("OUT_DIR"), "/xc.rs"));

pub type XCVMPacket<TAbiEncoded, TAccount, TAssets> =
	crate::Packet<XCVMProgram<TAbiEncoded, TAccount, TAssets>>;

pub type XCVMProgram<TAbiEncoded, TAccount, TAssets> =
	crate::Program<VecDeque<crate::Instruction<TAbiEncoded, TAccount, TAssets>>>;

pub trait Encodable {
	fn encode(self) -> Vec<u8>;
}

#[derive(Clone, Debug)]
pub enum DecodingFailure {
	Protobuf(DecodeError),
	Isomorphism,
}

impl Display for DecodingFailure {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		<str as Display>::fmt(&format!("{:?}", self), f)
	}
}

pub fn decode_packet<TAbiEncoded, TAccount, TAssets>(
	buffer: &[u8],
) -> core::result::Result<crate::Packet<XCVMProgram<TAbiEncoded, TAccount, TAssets>>, DecodingFailure>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	Packet::decode(buffer)
		.map_err(DecodingFailure::Protobuf)
		.and_then(|x| TryInto::try_into(x).map_err(|_| DecodingFailure::Isomorphism))
}

pub fn decode<TAbiEncoded, TAccount, TAssets>(
	buffer: &[u8],
) -> core::result::Result<XCVMProgram<TAbiEncoded, TAccount, TAssets>, DecodingFailure>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	Program::decode(buffer)
		.map_err(DecodingFailure::Protobuf)
		.and_then(|x| TryInto::try_into(x).map_err(|_| DecodingFailure::Isomorphism))
}

impl<TAbiEncoded, TAccount, TAssets> Encodable for XCVMPacket<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Balance)>>,
{
	fn encode(self) -> Vec<u8> {
		Packet::encode_to_vec(&self.into())
	}
}

impl From<Vec<u8>> for Salt {
	fn from(value: Vec<u8>) -> Self {
		Salt { salt: value }
	}
}

impl From<crate::UserId> for Account {
	fn from(crate::UserId(account): crate::UserId) -> Self {
		Account { account }
	}
}

impl From<crate::UserOrigin> for UserOrigin {
	fn from(value: crate::UserOrigin) -> Self {
		UserOrigin { network: Some(value.network_id.into()), account: Some(value.user_id.into()) }
	}
}

impl From<(crate::AssetId, Displayed<u128>)> for PacketAsset {
	fn from((asset, amount): (crate::AssetId, Displayed<u128>)) -> Self {
		PacketAsset { asset_id: Some(asset.into()), amount: Some(amount.into()) }
	}
}

impl<TAbiEncoded, TAccount, TAssets> From<XCVMPacket<TAbiEncoded, TAccount, TAssets>> for Packet
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Balance)>>,
{
	fn from(value: XCVMPacket<TAbiEncoded, TAccount, TAssets>) -> Self {
		Packet {
			interpreter: Some(Account { account: value.interpreter }),
			user_origin: Some(value.user_origin.into()),
			salt: Some(value.salt.into()),
			program: Some(value.program.into()),
			assets: value.assets.0.into_iter().map(PacketAsset::from).collect::<Vec<_>>(),
		}
	}
}

impl<TAbiEncoded, TAccount, TAssets> Encodable for XCVMProgram<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Balance)>>,
{
	fn encode(self) -> Vec<u8> {
		Program::encode_to_vec(&self.into())
	}
}

impl From<Uint128> for u128 {
	fn from(value: Uint128) -> Self {
		((value.high_bits as u128) << 64) + value.low_bits as u128
	}
}

impl From<u128> for Uint128 {
	fn from(value: u128) -> Self {
		Uint128 { high_bits: (value >> 64) as u64, low_bits: (value & 0xFFFFFFFFFFFFFFFF) as u64 }
	}
}

impl TryFrom<UserOrigin> for crate::UserOrigin {
	type Error = ();
	fn try_from(value: UserOrigin) -> core::result::Result<Self, Self::Error> {
		Ok(crate::UserOrigin {
			network_id: value.network.ok_or(())?.network_id.into(),
			user_id: value.account.ok_or(())?.account.into(),
		})
	}
}

impl TryFrom<PacketAsset> for (crate::AssetId, Displayed<u128>) {
	type Error = ();
	fn try_from(value: PacketAsset) -> core::result::Result<Self, Self::Error> {
		Ok((
			crate::AssetId::from(u128::from(value.asset_id.ok_or(())?.id.ok_or(())?)),
			value.amount.ok_or(())?.into(),
		))
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<Packet> for XCVMPacket<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(packet: Packet) -> core::result::Result<Self, Self::Error> {
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
					.collect::<core::result::Result<Vec<_>, _>>()?,
			),
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<Program>
	for XCVMProgram<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(program: Program) -> core::result::Result<Self, Self::Error> {
		Ok(XCVMProgram {
			tag: program.tag,
			instructions: program.instructions.ok_or(())?.try_into()?,
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<Instructions>
	for VecDeque<crate::Instruction<TAbiEncoded, TAccount, TAssets>>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(instructions: Instructions) -> core::result::Result<Self, Self::Error> {
		let mut instrs = VecDeque::new();
		for inst in instructions.instructions {
			instrs.push_back(inst.try_into()?);
		}
		Ok(instrs)
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<Instruction>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(instruction: Instruction) -> core::result::Result<Self, Self::Error> {
		instruction.instruction.ok_or(())?.try_into()
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<instruction::Instruction>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(instruction: instruction::Instruction) -> core::result::Result<Self, Self::Error> {
		match instruction {
			instruction::Instruction::Transfer(t) => t.try_into(),
			instruction::Instruction::Spawn(s) => s.try_into(),
			instruction::Instruction::Call(c) => c.try_into(),
			instruction::Instruction::Exchange(x) => x.try_into(),
		}
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<Call>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(call: Call) -> core::result::Result<Self, Self::Error> {
		let bindings = call.bindings.ok_or(())?.try_into()?;
		Ok(crate::Instruction::Call { bindings, encoded: call.payload.try_into().map_err(|_| ())? })
	}
}

impl TryFrom<Bindings> for crate::Bindings {
	type Error = ();

	fn try_from(bindings: Bindings) -> core::result::Result<Self, Self::Error> {
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

impl TryFrom<BindingValue> for crate::BindingValue {
	type Error = ();

	fn try_from(binding_value: BindingValue) -> core::result::Result<Self, Self::Error> {
		binding_value.r#type.ok_or(())?.try_into()
	}
}

impl TryFrom<binding_value::Type> for crate::BindingValue {
	type Error = ();

	fn try_from(binding_val: binding_value::Type) -> core::result::Result<Self, Self::Error> {
		Ok(match binding_val {
			binding_value::Type::Self_(_) => crate::BindingValue::Register(crate::Register::This),
			binding_value::Type::Tip(_) => crate::BindingValue::Register(crate::Register::Tip),
			binding_value::Type::Result(_) =>
				crate::BindingValue::Register(crate::Register::Result),
			binding_value::Type::IpRegister(_) =>
				crate::BindingValue::Register(crate::Register::Ip),
			binding_value::Type::AssetAmount(AssetAmount { asset_id, balance }) =>
				crate::BindingValue::AssetAmount(
					asset_id.ok_or(())?.try_into()?,
					balance.ok_or(())?.try_into()?,
				),
			binding_value::Type::AssetId(asset_id) =>
				crate::BindingValue::Asset(asset_id.try_into()?),
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<Spawn>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(spawn: Spawn) -> core::result::Result<Self, Self::Error> {
		let network = spawn.network.ok_or(())?.network_id.into();
		let salt = spawn.salt.ok_or(())?.salt;
		Ok(crate::Instruction::Spawn {
			network,
			salt,
			assets: spawn
				.assets
				.into_iter()
				.map(|asset| asset.try_into())
				.collect::<core::result::Result<Vec<_>, _>>()?
				.into(),
			program: XCVMProgram {
				tag: Vec::new(),
				instructions: spawn.program.ok_or(())?.instructions.ok_or(())?.try_into()?,
			},
		})
	}
}

impl From<Network> for NetworkId {
	fn from(network: Network) -> Self {
		Self(network.network_id)
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<Transfer>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(transfer: Transfer) -> core::result::Result<Self, Self::Error> {
		let account_type = transfer.account_type.ok_or(())?;
		Ok(crate::Instruction::Transfer {
			to: account_type.try_into()?,
			assets: transfer
				.assets
				.into_iter()
				.map(|asset| asset.try_into())
				.collect::<core::result::Result<Vec<_>, _>>()?
				.into(),
		})
	}
}

impl<TAbiEncoded, TAccount, TAssets> TryFrom<Exchange>
	for crate::Instruction<TAbiEncoded, TAccount, TAssets>
where
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(crate::AssetId, crate::Balance)>>,
{
	type Error = ();

	fn try_from(value: Exchange) -> core::result::Result<Self, Self::Error> {
		Ok(crate::Instruction::Exchange {
			id: value.id.and_then(|x| x.id).map(TryInto::try_into).ok_or(())?.map_err(|_| ())?,
			give: value
				.give
				.into_iter()
				.map(|asset| asset.try_into())
				.collect::<core::result::Result<Vec<_>, _>>()?
				.into(),
			want: value
				.want
				.into_iter()
				.map(|asset| asset.try_into())
				.collect::<core::result::Result<Vec<_>, _>>()?
				.into(),
		})
	}
}

impl<TAccount> TryFrom<transfer::AccountType> for Destination<TAccount>
where
	TAccount: for<'a> TryFrom<&'a [u8]>,
{
	type Error = ();

	fn try_from(account_type: transfer::AccountType) -> core::result::Result<Self, Self::Error> {
		Ok(match account_type {
			transfer::AccountType::Account(Account { account }) =>
				Destination::Account(account.as_slice().try_into().map_err(|_| ())?),
			transfer::AccountType::Tip(_) => Destination::Tip,
		})
	}
}

impl TryFrom<Asset> for (crate::AssetId, crate::Balance) {
	type Error = ();

	fn try_from(asset: Asset) -> core::result::Result<Self, Self::Error> {
		let asset_id = asset.asset_id.ok_or(())?.try_into()?;
		let amount = asset.balance.ok_or(())?.try_into()?;

		Ok((asset_id, amount))
	}
}

impl TryFrom<AssetId> for crate::AssetId {
	type Error = ();

	fn try_from(asset_id: AssetId) -> core::result::Result<Self, Self::Error> {
		Ok(crate::AssetId(asset_id.id.ok_or(())?.into()))
	}
}

impl TryFrom<Balance> for crate::Balance {
	type Error = ();

	fn try_from(balance: Balance) -> core::result::Result<Self, Self::Error> {
		let balance_type = balance.balance_type.ok_or(())?;

		match balance_type {
			balance::BalanceType::Ratio(ratio) => Ok(crate::Balance::new(ratio.try_into()?, false)),
			balance::BalanceType::Absolute(Absolute { value }) => {
				let value = value.ok_or(())?;
				Ok(crate::Balance::new(Amount::absolute(value.into()), false))
			},
			balance::BalanceType::Unit(unit) => unit.try_into(),
		}
	}
}

impl TryFrom<Ratio> for Amount {
	type Error = ();

	fn try_from(ratio: Ratio) -> core::result::Result<Self, Self::Error> {
		let nominator = ratio.nominator;
		let denominator = ratio.denominator;
		Ok(Amount::from((nominator.into(), denominator.into())))
	}
}

impl TryFrom<Unit> for crate::Balance {
	type Error = ();

	fn try_from(unit: Unit) -> core::result::Result<Self, Self::Error> {
		let integer = unit.integer.ok_or(())?;
		let ratio = unit.ratio.ok_or(())?;
		Ok(crate::Balance::new(
			Amount::new(
				integer.into(),
				Amount::from((ratio.nominator.into(), ratio.denominator.into())).slope.into(),
			),
			true,
		))
	}
}
// XCVM types to Protobuf conversion

impl From<crate::Balance> for Balance {
	fn from(balance: crate::Balance) -> Self {
		// Note that although functionally nothing changes, there is no guarantee of getting the
		// same protobuf when you convert protobuf to XCVM types and convert back again. Because
		// `intercept = 0 & ratio = 0` is always converted to `Absolute`. But this can be also
		// expressed with `Ratio` and `Unit` as well. Also, since the ratio is expanded to use
		// denominator `MAX_PARTS`, it also won't be the same.

		let balance_type = if balance.is_unit {
			balance::BalanceType::Unit(Unit {
				integer: Some(balance.amount.intercept.0.into()),
				ratio: Some(Ratio {
					nominator: balance.amount.slope.0.into(),
					denominator: Amount::MAX_PARTS.into(),
				}),
			})
		} else if balance.amount.is_absolute() {
			balance::BalanceType::Absolute(Absolute {
				value: Some(balance.amount.intercept.0.into()),
			})
		} else {
			balance::BalanceType::Ratio(Ratio {
				nominator: balance.amount.slope.0.into(),
				denominator: Amount::MAX_PARTS.into(),
			})
		};
		Balance { balance_type: Some(balance_type) }
	}
}

impl From<crate::AssetId> for AssetId {
	fn from(asset_id: crate::AssetId) -> Self {
		AssetId { id: Some(asset_id.0 .0.into()) }
	}
}

impl From<(crate::AssetId, crate::Balance)> for Asset {
	fn from((asset_id, amount): (crate::AssetId, crate::Balance)) -> Self {
		Asset { asset_id: Some(asset_id.into()), balance: Some(amount.into()) }
	}
}

impl From<crate::BindingValue> for binding_value::Type {
	fn from(binding_value: crate::BindingValue) -> Self {
		match binding_value {
			crate::BindingValue::Register(crate::Register::Ip) =>
				binding_value::Type::IpRegister(IpRegister { ip: 0 }),
			crate::BindingValue::Register(crate::Register::Tip) =>
				binding_value::Type::Tip(Tip { id: 0 }),
			crate::BindingValue::Register(crate::Register::Result) =>
				binding_value::Type::Result(Result { result: 0 }),
			crate::BindingValue::Register(crate::Register::This) =>
				binding_value::Type::Self_(Self_ { self_: 0 }),
			crate::BindingValue::Asset(asset_id) => binding_value::Type::AssetId(asset_id.into()),
			crate::BindingValue::AssetAmount(asset_id, balance) =>
				binding_value::Type::AssetAmount(AssetAmount {
					asset_id: Some(asset_id.into()),
					balance: Some(balance.into()),
				}),
		}
	}
}

impl From<crate::BindingValue> for BindingValue {
	fn from(binding_value: crate::BindingValue) -> Self {
		BindingValue { r#type: Some(binding_value.into()) }
	}
}

impl From<crate::NetworkId> for Network {
	fn from(network_id: crate::NetworkId) -> Self {
		Network { network_id: network_id.0 }
	}
}

impl<TAccount> From<crate::Destination<TAccount>> for transfer::AccountType
where
	TAccount: Into<Vec<u8>>,
{
	fn from(destination: crate::Destination<TAccount>) -> Self {
		match destination {
			Destination::Account(account) =>
				transfer::AccountType::Account(Account { account: account.into() }),
			Destination::Tip => transfer::AccountType::Tip(Tip { id: 0 }),
		}
	}
}

impl From<(u32, crate::BindingValue)> for Binding {
	fn from((position, binding_value): (u32, crate::BindingValue)) -> Self {
		Binding { position, binding_value: Some(binding_value.into()) }
	}
}

impl<TAbiEncoded, TAccount, TAssets> From<crate::Instruction<TAbiEncoded, TAccount, TAssets>>
	for instruction::Instruction
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Balance)>>,
{
	fn from(instruction: crate::Instruction<TAbiEncoded, TAccount, TAssets>) -> Self {
		match instruction {
			crate::Instruction::Transfer { to, assets } =>
				instruction::Instruction::Transfer(Transfer {
					assets: assets.into().into_iter().map(|asset| asset.into()).collect(),
					account_type: Some(to.into()),
				}),
			crate::Instruction::Call { bindings, encoded } =>
				instruction::Instruction::Call(Call {
					payload: encoded.into(),
					bindings: Some(Bindings {
						bindings: bindings.into_iter().map(|binding| binding.into()).collect(),
					}),
				}),
			crate::Instruction::Spawn { network, salt, assets, program } =>
				instruction::Instruction::Spawn(Spawn {
					network: Some(Network { network_id: network.into() }),
					salt: Some(Salt { salt }),
					program: Some(program.into()),
					assets: assets.into().into_iter().map(|asset| asset.into()).collect(),
				}),
			crate::Instruction::Exchange { id, give, want } =>
				instruction::Instruction::Exchange(Exchange {
					id: Some(ExchangeId { id: Some(id.into()) }),
					give: give.into().into_iter().map(|asset| asset.into()).collect(),
					want: want.into().into_iter().map(|asset| asset.into()).collect(),
				}),
		}
	}
}

impl<TAbiEncoded, TAccount, TAssets> From<crate::Instruction<TAbiEncoded, TAccount, TAssets>>
	for Instruction
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Balance)>>,
{
	fn from(instruction: crate::Instruction<TAbiEncoded, TAccount, TAssets>) -> Self {
		Instruction { instruction: Some(instruction.into()) }
	}
}

impl<TAbiEncoded, TAccount, TAssets> From<XCVMProgram<TAbiEncoded, TAccount, TAssets>> for Program
where
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(crate::AssetId, crate::Balance)>>,
{
	fn from(program: XCVMProgram<TAbiEncoded, TAccount, TAssets>) -> Self {
		Program {
			tag: program.tag,
			instructions: Some(Instructions {
				instructions: program.instructions.into_iter().map(|instr| instr.into()).collect(),
			}),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn balance_to_amount_works() {
		let balance = Balance {
			balance_type: Some(balance::BalanceType::Ratio(Ratio {
				nominator: 3u64.into(),
				denominator: 5u64.into(),
			})),
		};
		let xcvm_balance: crate::Balance = balance.try_into().unwrap();
		assert_eq!(xcvm_balance.amount.intercept, Displayed(0));
	}
	#[test]
	fn u128_from_uint128_works() {
		let real_value = 1231231231231231233123123123123123_u128;
		let high_bits = u64::from_be_bytes(real_value.to_be_bytes()[0..8].try_into().unwrap());
		let low_bits = u64::from_be_bytes(real_value.to_be_bytes()[8..].try_into().unwrap());
		let uint128 = Uint128 { high_bits, low_bits };
		assert_eq!(u128::from(uint128.clone()), real_value);
		assert_eq!(Uint128::from(real_value), uint128)
	}
}
