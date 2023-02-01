#![no_std]

extern crate alloc;

use core::fmt::Display;

use alloc::{collections::VecDeque, format, vec::Vec};
use fixed::{types::extra::U16, FixedU128};
use prost::{DecodeError, Message};
use xcvm_core::{Amount, Destination, Displayed, Funds, NetworkId, MAX_PARTS};

include!(concat!(env!("OUT_DIR"), "/interpreter.rs"));

pub type XCVMPacket<TNetwork, TAbiEncoded, TAccount, TAssets> =
	xcvm_core::Packet<XCVMProgram<TNetwork, TAbiEncoded, TAccount, TAssets>>;

pub type XCVMProgram<TNetwork, TAbiEncoded, TAccount, TAssets> =
	xcvm_core::Program<VecDeque<xcvm_core::Instruction<TNetwork, TAbiEncoded, TAccount, TAssets>>>;

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

pub fn decode_packet<TNetwork, TAbiEncoded, TAccount, TAssets>(
	buffer: &[u8],
) -> core::result::Result<
	xcvm_core::Packet<XCVMProgram<TNetwork, TAbiEncoded, TAccount, TAssets>>,
	DecodingFailure,
>
where
	TNetwork: From<u32>,
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	Packet::decode(buffer)
		.map_err(DecodingFailure::Protobuf)
		.and_then(|x| TryInto::try_into(x).map_err(|_| DecodingFailure::Isomorphism))
}

pub fn decode<TNetwork, TAbiEncoded, TAccount, TAssets>(
	buffer: &[u8],
) -> core::result::Result<XCVMProgram<TNetwork, TAbiEncoded, TAccount, TAssets>, DecodingFailure>
where
	TNetwork: From<u32>,
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	Program::decode(buffer)
		.map_err(DecodingFailure::Protobuf)
		.and_then(|x| TryInto::try_into(x).map_err(|_| DecodingFailure::Isomorphism))
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets> Encodable
	for XCVMPacket<TNetwork, TAbiEncoded, TAccount, TAssets>
where
	TNetwork: Into<u32>,
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
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

impl From<xcvm_core::UserId> for Account {
	fn from(xcvm_core::UserId(account): xcvm_core::UserId) -> Self {
		Account { account }
	}
}

impl From<xcvm_core::UserOrigin> for UserOrigin {
	fn from(value: xcvm_core::UserOrigin) -> Self {
		UserOrigin { network: Some(value.network_id.into()), account: Some(value.user_id.into()) }
	}
}

impl From<(xcvm_core::AssetId, Displayed<u128>)> for PacketAsset {
	fn from((asset, Displayed(amount)): (xcvm_core::AssetId, Displayed<u128>)) -> Self {
		PacketAsset { asset_id: Some(asset.into()), amount: Some(amount.into()) }
	}
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets>
	From<XCVMPacket<TNetwork, TAbiEncoded, TAccount, TAssets>> for Packet
where
	TNetwork: Into<u32>,
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	fn from(value: XCVMPacket<TNetwork, TAbiEncoded, TAccount, TAssets>) -> Self {
		Packet {
			interpreter: Some(Account { account: value.interpreter.into() }),
			user_origin: Some(value.user_origin.into()),
			salt: Some(value.salt.into()),
			program: Some(value.program.into()),
			assets: value.assets.0.into_iter().map(PacketAsset::from).collect::<Vec<_>>(),
		}
	}
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets> Encodable
	for XCVMProgram<TNetwork, TAbiEncoded, TAccount, TAssets>
where
	TNetwork: Into<u32>,
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
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

impl TryFrom<UserOrigin> for xcvm_core::UserOrigin {
	type Error = ();
	fn try_from(value: UserOrigin) -> core::result::Result<Self, Self::Error> {
		Ok(xcvm_core::UserOrigin {
			network_id: value.network.ok_or(())?.network_id.into(),
			user_id: value.account.ok_or(())?.account.into(),
		})
	}
}

impl TryFrom<PacketAsset> for (xcvm_core::AssetId, Displayed<u128>) {
	type Error = ();
	fn try_from(value: PacketAsset) -> core::result::Result<Self, Self::Error> {
		Ok((
			xcvm_core::AssetId::from(u128::from(value.asset_id.ok_or(())?.asset_id.ok_or(())?)),
			Displayed(value.amount.ok_or(())?.into()),
		))
	}
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets> TryFrom<Packet>
	for XCVMPacket<TNetwork, TAbiEncoded, TAccount, TAssets>
where
	TNetwork: From<u32>,
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	type Error = ();

	fn try_from(packet: Packet) -> core::result::Result<Self, Self::Error> {
		Ok(XCVMPacket {
			interpreter: packet.interpreter.ok_or(())?.account.into(),
			user_origin: packet.user_origin.ok_or(())?.try_into()?,
			salt: packet.salt.map(|s| s.salt).ok_or(())?,
			program: packet.program.ok_or(())?.try_into()?,
			assets: Funds(
				packet
					.assets
					.into_iter()
					.map(|asset| <(xcvm_core::AssetId, Displayed<u128>)>::try_from(asset))
					.collect::<core::result::Result<Vec<_>, _>>()?,
			),
		})
	}
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets> TryFrom<Program>
	for XCVMProgram<TNetwork, TAbiEncoded, TAccount, TAssets>
where
	TNetwork: From<u32>,
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	type Error = ();

	fn try_from(program: Program) -> core::result::Result<Self, Self::Error> {
		Ok(XCVMProgram {
			tag: program.tag,
			instructions: program.instructions.ok_or(())?.try_into()?,
		})
	}
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets> TryFrom<Instructions>
	for VecDeque<xcvm_core::Instruction<TNetwork, TAbiEncoded, TAccount, TAssets>>
where
	TNetwork: From<u32>,
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
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

impl<TNetwork, TAbiEncoded, TAccount, TAssets> TryFrom<Instruction>
	for xcvm_core::Instruction<TNetwork, TAbiEncoded, TAccount, TAssets>
where
	TNetwork: From<u32>,
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	type Error = ();

	fn try_from(instruction: Instruction) -> core::result::Result<Self, Self::Error> {
		instruction.instruction.ok_or(())?.try_into()
	}
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets> TryFrom<instruction::Instruction>
	for xcvm_core::Instruction<TNetwork, TAbiEncoded, TAccount, TAssets>
where
	TNetwork: From<u32>,
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	type Error = ();

	fn try_from(instruction: instruction::Instruction) -> core::result::Result<Self, Self::Error> {
		match instruction {
			instruction::Instruction::Transfer(t) => t.try_into(),
			instruction::Instruction::Spawn(s) => s.try_into(),
			instruction::Instruction::Call(c) => c.try_into(),
			// TODO(aeryz): Query needs to be implemented
			// instruction::Instruction::Query(q) => q.try_into(),
			_ => Err(()),
		}
	}
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets> TryFrom<Call>
	for xcvm_core::Instruction<TNetwork, TAbiEncoded, TAccount, TAssets>
where
	TNetwork: From<u32>,
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	type Error = ();

	fn try_from(call: Call) -> core::result::Result<Self, Self::Error> {
		let bindings = call.bindings.ok_or(())?.try_into()?;
		Ok(xcvm_core::Instruction::Call {
			bindings,
			encoded: call.payload.try_into().map_err(|_| ())?,
		})
	}
}

impl TryFrom<Bindings> for xcvm_core::Bindings {
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

impl TryFrom<BindingValue> for xcvm_core::BindingValue {
	type Error = ();

	fn try_from(binding_value: BindingValue) -> core::result::Result<Self, Self::Error> {
		binding_value.r#type.ok_or(())?.try_into()
	}
}

impl TryFrom<binding_value::Type> for xcvm_core::BindingValue {
	type Error = ();

	fn try_from(binding_val: binding_value::Type) -> core::result::Result<Self, Self::Error> {
		Ok(match binding_val {
			binding_value::Type::Self_(_) =>
				xcvm_core::BindingValue::Register(xcvm_core::Register::This),
			binding_value::Type::Relayer(_) =>
				xcvm_core::BindingValue::Register(xcvm_core::Register::Relayer),
			binding_value::Type::Result(_) =>
				xcvm_core::BindingValue::Register(xcvm_core::Register::Result),
			binding_value::Type::IpRegister(_) =>
				xcvm_core::BindingValue::Register(xcvm_core::Register::Ip),
			binding_value::Type::AssetAmount(AssetAmount { asset_id, balance }) =>
				xcvm_core::BindingValue::AssetAmount(
					asset_id.ok_or(())?.try_into()?,
					balance.ok_or(())?.try_into()?,
				),
			binding_value::Type::AssetId(asset_id) =>
				xcvm_core::BindingValue::Asset(asset_id.try_into()?),
		})
	}
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets> TryFrom<Spawn>
	for xcvm_core::Instruction<TNetwork, TAbiEncoded, TAccount, TAssets>
where
	TNetwork: From<u32>,
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	type Error = ();

	fn try_from(spawn: Spawn) -> core::result::Result<Self, Self::Error> {
		let network = spawn.network.ok_or(())?.network_id.into();
		let salt = spawn.salt.ok_or(())?.salt;
		Ok(xcvm_core::Instruction::Spawn {
			network,
			salt,
			bridge_security: match spawn.security {
				0 => xcvm_core::BridgeSecurity::Insecure,
				1 => xcvm_core::BridgeSecurity::Optimistic,
				2 => xcvm_core::BridgeSecurity::Probabilistic,
				3 => xcvm_core::BridgeSecurity::Deterministic,
				_ => return Err(()),
			},
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
		network.network_id.into()
	}
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets> TryFrom<Transfer>
	for xcvm_core::Instruction<TNetwork, TAbiEncoded, TAccount, TAssets>
where
	TNetwork: From<u32>,
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	type Error = ();

	fn try_from(transfer: Transfer) -> core::result::Result<Self, Self::Error> {
		let account_type = transfer.account_type.ok_or(())?;
		Ok(xcvm_core::Instruction::Transfer {
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

impl<TAccount> TryFrom<transfer::AccountType> for Destination<TAccount>
where
	TAccount: for<'a> TryFrom<&'a [u8]>,
{
	type Error = ();

	fn try_from(account_type: transfer::AccountType) -> core::result::Result<Self, Self::Error> {
		Ok(match account_type {
			transfer::AccountType::Account(Account { account }) =>
				Destination::Account(account.as_slice().try_into().map_err(|_| ())?),
			transfer::AccountType::Relayer(_) => Destination::Relayer,
		})
	}
}

impl TryFrom<Asset> for (xcvm_core::AssetId, xcvm_core::Balance) {
	type Error = ();

	fn try_from(asset: Asset) -> core::result::Result<Self, Self::Error> {
		let asset_id = asset.asset_id.ok_or(())?.try_into()?;
		let amount = asset.balance.ok_or(())?.try_into()?;

		Ok((asset_id, amount))
	}
}

impl TryFrom<AssetId> for xcvm_core::AssetId {
	type Error = ();

	fn try_from(asset_id: AssetId) -> core::result::Result<Self, Self::Error> {
		Ok(xcvm_core::AssetId(Displayed(asset_id.asset_id.ok_or(())?.into())))
	}
}

impl TryFrom<Balance> for xcvm_core::Balance {
	type Error = ();

	fn try_from(balance: Balance) -> core::result::Result<Self, Self::Error> {
		let balance_type = balance.balance_type.ok_or(())?;

		match balance_type {
			balance::BalanceType::Ratio(ratio) =>
				Ok(xcvm_core::Balance::new(ratio.try_into()?, false)),
			balance::BalanceType::Absolute(Absolute { value }) => {
				let value = value.ok_or(())?;
				Ok(xcvm_core::Balance::new(Amount::absolute(value.into()), false))
			},
			balance::BalanceType::Unit(unit) => unit.try_into(),
		}
	}
}

impl TryFrom<Ratio> for Amount {
	type Error = ();

	fn try_from(ratio: Ratio) -> core::result::Result<Self, Self::Error> {
		let nominator = ratio.nominator.ok_or(())?;
		let denominator = ratio.denominator.ok_or(())?;
		Ok(Amount::ratio(calc_nom(nominator.into(), denominator.into(), MAX_PARTS)))
	}
}

impl TryFrom<Unit> for xcvm_core::Balance {
	type Error = ();

	fn try_from(unit: Unit) -> core::result::Result<Self, Self::Error> {
		let integer = unit.integer.ok_or(())?;
		let ratio = unit.ratio.ok_or(())?;
		Ok(xcvm_core::Balance::new(
			Amount::new(
				integer.into(),
				calc_nom(
					ratio.nominator.ok_or(())?.into(),
					ratio.denominator.ok_or(())?.into(),
					MAX_PARTS,
				),
			),
			true,
		))
	}
}
// XCVM types to Protobuf conversion

impl From<xcvm_core::Balance> for Balance {
	fn from(balance: xcvm_core::Balance) -> Self {
		// Note that although functionally nothing changes, there is no guarantee of getting the
		// same protobuf when you convert protobuf to XCVM types and convert back again. Because
		// `intercept = 0 & ratio = 0` is always converted to `Absolute`. But this can be also
		// expressed with `Ratio` and `Unit` as well. Also, since the ratio is expanded to use
		// denominator `MAX_PARTS`, it also won't be the same.

		let balance_type = if balance.is_unit {
			balance::BalanceType::Unit(Unit {
				integer: Some(balance.amount.intercept.0.into()),
				ratio: Some(Ratio {
					nominator: Some(balance.amount.slope.0.into()),
					denominator: Some(MAX_PARTS.into()),
				}),
			})
		} else if balance.amount.is_absolute() {
			balance::BalanceType::Absolute(Absolute {
				value: Some(balance.amount.intercept.0.into()),
			})
		} else {
			balance::BalanceType::Ratio(Ratio {
				nominator: Some(balance.amount.slope.0.into()),
				denominator: Some(MAX_PARTS.into()),
			})
		};
		Balance { balance_type: Some(balance_type) }
	}
}

impl From<xcvm_core::AssetId> for AssetId {
	fn from(asset_id: xcvm_core::AssetId) -> Self {
		AssetId { asset_id: Some(asset_id.0 .0.into()) }
	}
}

impl From<(xcvm_core::AssetId, xcvm_core::Balance)> for Asset {
	fn from((asset_id, amount): (xcvm_core::AssetId, xcvm_core::Balance)) -> Self {
		Asset { asset_id: Some(asset_id.into()), balance: Some(amount.into()) }
	}
}

impl From<xcvm_core::BindingValue> for binding_value::Type {
	fn from(binding_value: xcvm_core::BindingValue) -> Self {
		match binding_value {
			xcvm_core::BindingValue::Register(xcvm_core::Register::Ip) =>
				binding_value::Type::IpRegister(IpRegister { ip: 0 }),
			xcvm_core::BindingValue::Register(xcvm_core::Register::Relayer) =>
				binding_value::Type::Relayer(Relayer { id: 0 }),
			xcvm_core::BindingValue::Register(xcvm_core::Register::Result) =>
				binding_value::Type::Result(Result { result: 0 }),
			xcvm_core::BindingValue::Register(xcvm_core::Register::This) =>
				binding_value::Type::Self_(Self_ { self_: 0 }),
			xcvm_core::BindingValue::Asset(asset_id) =>
				binding_value::Type::AssetId(asset_id.into()),
			xcvm_core::BindingValue::AssetAmount(asset_id, balance) =>
				binding_value::Type::AssetAmount(AssetAmount {
					asset_id: Some(asset_id.into()),
					balance: Some(balance.into()),
				}),
		}
	}
}

impl From<xcvm_core::BindingValue> for BindingValue {
	fn from(binding_value: xcvm_core::BindingValue) -> Self {
		BindingValue { r#type: Some(binding_value.into()) }
	}
}

impl From<xcvm_core::NetworkId> for Network {
	fn from(network_id: xcvm_core::NetworkId) -> Self {
		Network { network_id: network_id.0 as u32 }
	}
}

impl<TAccount> From<xcvm_core::Destination<TAccount>> for transfer::AccountType
where
	TAccount: Into<Vec<u8>>,
{
	fn from(destination: xcvm_core::Destination<TAccount>) -> Self {
		match destination {
			Destination::Account(account) =>
				transfer::AccountType::Account(Account { account: account.into() }),
			Destination::Relayer => transfer::AccountType::Relayer(Relayer { id: 0 }),
		}
	}
}

impl From<(u32, xcvm_core::BindingValue)> for Binding {
	fn from((position, binding_value): (u32, xcvm_core::BindingValue)) -> Self {
		Binding { position, binding_value: Some(binding_value.into()) }
	}
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets>
	From<xcvm_core::Instruction<TNetwork, TAbiEncoded, TAccount, TAssets>> for instruction::Instruction
where
	TNetwork: Into<u32>,
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	fn from(instruction: xcvm_core::Instruction<TNetwork, TAbiEncoded, TAccount, TAssets>) -> Self {
		match instruction {
			xcvm_core::Instruction::Transfer { to, assets } =>
				instruction::Instruction::Transfer(Transfer {
					assets: assets.into().into_iter().map(|asset| asset.into()).collect(),
					account_type: Some(to.into()),
				}),
			xcvm_core::Instruction::Call { bindings, encoded } =>
				instruction::Instruction::Call(Call {
					payload: encoded.into(),
					bindings: Some(Bindings {
						bindings: bindings.into_iter().map(|binding| binding.into()).collect(),
					}),
				}),
			xcvm_core::Instruction::Spawn { network, bridge_security, salt, assets, program } =>
				instruction::Instruction::Spawn(Spawn {
					network: Some(Network { network_id: network.into() }),
					security: bridge_security as i32,
					salt: Some(Salt { salt }),
					program: Some(program.into()),
					assets: assets.into().into_iter().map(|asset| asset.into()).collect(),
				}),
			xcvm_core::Instruction::Query { network, salt } =>
				instruction::Instruction::Query(Query {
					network: Some(Network { network_id: network.into() }),
					salt: Some(Salt { salt }),
				}),
		}
	}
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets>
	From<xcvm_core::Instruction<TNetwork, TAbiEncoded, TAccount, TAssets>> for Instruction
where
	TNetwork: Into<u32>,
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	fn from(instruction: xcvm_core::Instruction<TNetwork, TAbiEncoded, TAccount, TAssets>) -> Self {
		Instruction { instruction: Some(instruction.into()) }
	}
}

impl<TNetwork, TAbiEncoded, TAccount, TAssets>
	From<XCVMProgram<TNetwork, TAbiEncoded, TAccount, TAssets>> for Program
where
	TNetwork: Into<u32>,
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: Into<Vec<u8>>,
	TAssets: Into<Vec<(xcvm_core::AssetId, xcvm_core::Balance)>>,
{
	fn from(program: XCVMProgram<TNetwork, TAbiEncoded, TAccount, TAssets>) -> Self {
		Program {
			tag: program.tag,
			instructions: Some(Instructions {
				instructions: program.instructions.into_iter().map(|instr| instr.into()).collect(),
			}),
		}
	}
}

// TODO(aeryz): This can be a helper function in SDK so that users won't
// necessarily need to know how the ratio is handled in our SDK
// Calculates `x` in the following equation: nom / denom = x / max
fn calc_nom(nom: u128, denom: u128, max: u128) -> u128 {
	let wrap = |num: u128| -> FixedU128<U16> { FixedU128::wrapping_from_num(num) };
	wrap(nom)
		.saturating_div(wrap(denom))
		.saturating_mul(wrap(max))
		.wrapping_to_num::<u128>()
}

#[cfg(test)]
mod tests {
	use super::*;
	use xcvm_core::Displayed;

	#[test]
	fn balance_to_amount_works() {
		let balance = Balance {
			balance_type: Some(balance::BalanceType::Ratio(Ratio {
				nominator: Some(3u128.into()),
				denominator: Some(5u128.into()),
			})),
		};
		let xcvm_balance: xcvm_core::Balance = balance.try_into().unwrap();
		assert_eq!(xcvm_balance.amount.intercept, Displayed(0));

		let wrap = |num: u128| -> FixedU128<U16> { FixedU128::wrapping_from_num(num) };
		assert_eq!(
			wrap(3).saturating_div(wrap(5)),
			wrap(xcvm_balance.amount.slope.0).saturating_div(wrap(MAX_PARTS))
		)
	}
	#[test]
	fn u128_from_uint128_works() {
		let real_value = 1231231231231231233123123123123123_u128;
		let high_bits = u64::from_be_bytes(real_value.to_be_bytes()[0..8].try_into().unwrap());
		let low_bits = u64::from_be_bytes(real_value.to_be_bytes()[8..].try_into().unwrap());
		let uint128 = Uint128 { high_bits, low_bits };
		assert_eq!(Into::<u128>::into(uint128.clone()), real_value);
		assert_eq!(Into::<Uint128>::into(real_value), uint128)
	}
}
