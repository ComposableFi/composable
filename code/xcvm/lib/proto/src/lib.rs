#![no_std]

extern crate alloc;

use alloc::{collections::VecDeque, vec::Vec};
use fixed::{types::extra::U16, FixedU128};
use xcvm_core::{Amount, Destination, Funds, NetworkId, MAX_PARTS};

include!(concat!(env!("OUT_DIR"), "/interpreter.rs"));

pub type XCVMInstruction<Account> = xcvm_core::Instruction<NetworkId, Vec<u8>, Account, Funds>;
pub type XCVMProgram<Account> = xcvm_core::Program<VecDeque<XCVMInstruction<Account>>>;

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

impl<Account> TryFrom<Program> for XCVMProgram<Account>
where
	Account: From<Vec<u8>>,
{
	type Error = ();

	fn try_from(program: Program) -> core::result::Result<Self, Self::Error> {
		Ok(XCVMProgram {
			tag: program.tag,
			instructions: program.instructions.ok_or(())?.try_into()?,
		})
	}
}

impl<Account> TryFrom<Instructions> for VecDeque<XCVMInstruction<Account>>
where
	Account: From<Vec<u8>>,
{
	type Error = ();

	fn try_from(instructions: Instructions) -> core::result::Result<Self, Self::Error> {
		instructions
			.instructions
			.into_iter()
			.map(|instruction| instruction.try_into())
			.collect()
	}
}

impl<Account> TryFrom<Instruction> for XCVMInstruction<Account>
where
	Account: From<Vec<u8>>,
{
	type Error = ();

	fn try_from(instruction: Instruction) -> core::result::Result<Self, Self::Error> {
		instruction.instruction.ok_or(())?.try_into()
	}
}

impl<Account> TryFrom<instruction::Instruction> for XCVMInstruction<Account>
where
	Account: From<Vec<u8>>,
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

impl<Account> TryFrom<Call> for XCVMInstruction<Account>
where
	Account: From<Vec<u8>>,
{
	type Error = ();

	fn try_from(call: Call) -> core::result::Result<Self, Self::Error> {
		let bindings = call.bindings.ok_or(())?.try_into()?;
		Ok(XCVMInstruction::Call { bindings, encoded: call.payload })
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
			binding_value::Type::Ip(_) =>
				xcvm_core::BindingValue::Register(xcvm_core::Register::Ip),
			binding_value::Type::AssetAmount(_) => todo!(),
			binding_value::Type::AssetId(asset_id) =>
				xcvm_core::BindingValue::Asset(asset_id.try_into()?),
		})
	}
}

impl<Account> TryFrom<Spawn> for XCVMInstruction<Account>
where
	Account: From<Vec<u8>>,
{
	type Error = ();

	fn try_from(spawn: Spawn) -> core::result::Result<Self, Self::Error> {
		let network = spawn.network.ok_or(())?.into();
		let salt = spawn.salt.ok_or(())?.salt;
		// let program = spawn.program.ok_or(())?;
		// TODO(aeryz): convert program
		// TODO(aeryz): assets conversion can be a function
		Ok(XCVMInstruction::Spawn {
			network,
			salt,
			bridge_security: match spawn.security {
				0 => xcvm_core::BridgeSecurity::None,
				1 => xcvm_core::BridgeSecurity::Deterministic,
				2 => xcvm_core::BridgeSecurity::Probabilistic,
				3 => xcvm_core::BridgeSecurity::Optimistic,
				_ => return Err(()),
			},
			assets: Funds(
				spawn
					.assets
					.into_iter()
					.map(|asset| asset.try_into())
					.collect::<core::result::Result<Vec<_>, _>>()?,
			),
			program: XCVMProgram { tag: Vec::new(), instructions: VecDeque::new() },
		})
	}
}

impl From<Network> for NetworkId {
	fn from(network: Network) -> Self {
		(network.network_id as u8).into()
	}
}

impl<Account> TryFrom<Transfer> for XCVMInstruction<Account>
where
	Account: From<Vec<u8>>,
{
	type Error = ();

	fn try_from(transfer: Transfer) -> core::result::Result<Self, Self::Error> {
		let account_type = transfer.account_type.ok_or(())?;
		Ok(XCVMInstruction::Transfer {
			to: account_type.into(),
			assets: Funds(
				transfer
					.assets
					.into_iter()
					.map(|asset| asset.try_into())
					.collect::<core::result::Result<Vec<_>, _>>()?,
			),
		})
	}
}

impl<Acc> From<transfer::AccountType> for Destination<Acc>
where
	Acc: From<Vec<u8>>,
{
	fn from(account_type: transfer::AccountType) -> Self {
		match account_type {
			transfer::AccountType::Account(Account { account }) =>
				Destination::Account(account.into()),
			transfer::AccountType::Relayer(_) => Destination::Relayer,
		}
	}
}

impl TryFrom<Asset> for (xcvm_core::AssetId, Amount) {
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
		Ok(xcvm_core::AssetId(asset_id.asset_id.ok_or(())?.into()))
	}
}

impl TryFrom<Balance> for Amount {
	type Error = ();

	fn try_from(balance: Balance) -> core::result::Result<Self, Self::Error> {
		let balance_type = balance.balance_type.ok_or(())?;
		let wrap = |num: u128| -> FixedU128<U16> { FixedU128::wrapping_from_num(num) };
		// TODO(aeryz): This can be a helper function in SDK so that users won't
		// necesarilly need to know how the ratio is handled in our SDK
		// Calculates `x` in the following equation: nom / denom = x / max
		let calc_nom = |nom: u128, denom: u128, max: u128| -> u128 {
			wrap(nom)
				.saturating_div(wrap(denom))
				.saturating_mul(wrap(max))
				.wrapping_to_num::<u128>()
		};
		match balance_type {
			balance::BalanceType::Ratio(Ratio { nominator, denominator }) => {
				let nominator = nominator.ok_or(())?;
				let denominator = denominator.ok_or(())?;
				Ok(Amount::ratio(calc_nom(nominator.into(), denominator.into(), MAX_PARTS)))
			},
			balance::BalanceType::Absolute(Absolute { value }) => {
				let value = value.ok_or(())?;
				Ok(Amount::absolute(value.into()))
			},
			balance::BalanceType::Unit(Unit { integer, ratio }) => {
				let integer = integer.ok_or(())?;
				let ratio = ratio.ok_or(())?;
				Ok(Amount::new(
					integer.into(),
					calc_nom(
						ratio.nominator.ok_or(())?.into(),
						ratio.denominator.ok_or(())?.into(),
						MAX_PARTS,
					),
				))
			},
		}
	}
}

// XCVM types to Protobuf conversion

impl From<Amount> for Balance {
	fn from(amount: Amount) -> Self {
		// Note that although functionally nothing changes, there is no guarantee of getting the
		// same protobuf when you convert protobuf to XCVM types and convert back again. Because
		// `intercept = 0 & ratio = 0` is always converted to `Absolute`. But this can be also
		// expressed with `Ratio` and `Unit` as well. Also, since the ratio is expanded to use
		// denomitor `MAX_PARTS`, it also won't be the same.
		let balance_type = if amount.is_absolute() {
			balance::BalanceType::Absolute(Absolute { value: Some(amount.intercept.0.into()) })
		} else if amount.is_ratio() {
			balance::BalanceType::Ratio(Ratio {
				nominator: Some(amount.slope.0.into()),
				denominator: Some(MAX_PARTS.into()),
			})
		} else {
			balance::BalanceType::Unit(Unit {
				integer: Some(amount.intercept.0.into()),
				ratio: Some(Ratio {
					nominator: Some(amount.slope.0.into()),
					denominator: Some(MAX_PARTS.into()),
				}),
			})
		};
		Balance { balance_type: Some(balance_type) }
	}
}

impl From<xcvm_core::AssetId> for AssetId {
	fn from(asset_id: xcvm_core::AssetId) -> Self {
		AssetId { asset_id: Some(asset_id.0.into()) }
	}
}

impl From<(xcvm_core::AssetId, xcvm_core::Amount)> for Asset {
	fn from((asset_id, amount): (xcvm_core::AssetId, xcvm_core::Amount)) -> Self {
		Asset { asset_id: Some(asset_id.into()), balance: Some(amount.into()) }
	}
}

impl From<xcvm_core::BindingValue> for binding_value::Type {
	fn from(binding_value: xcvm_core::BindingValue) -> Self {
		match binding_value {
			xcvm_core::BindingValue::Register(xcvm_core::Register::Ip) =>
				binding_value::Type::Ip(Ip { id: 0 }),
			xcvm_core::BindingValue::Register(xcvm_core::Register::Relayer) =>
				binding_value::Type::Relayer(Relayer { id: 0 }),
			xcvm_core::BindingValue::Register(xcvm_core::Register::Result) =>
				binding_value::Type::Result(Result { result: 0 }),
			xcvm_core::BindingValue::Register(xcvm_core::Register::This) =>
				binding_value::Type::Self_(Self_ { self_: 0 }),
			xcvm_core::BindingValue::Asset(asset_id) =>
				binding_value::Type::AssetId(asset_id.into()),
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

impl<Account_> From<xcvm_core::Destination<Account_>> for transfer::AccountType
where
	Vec<u8>: From<Account_>,
{
	fn from(destination: xcvm_core::Destination<Account_>) -> Self {
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

impl<Account> From<XCVMInstruction<Account>> for instruction::Instruction
where
	Vec<u8>: From<Account>,
{
	fn from(instruction: XCVMInstruction<Account>) -> Self {
		match instruction {
			xcvm_core::Instruction::Transfer { to, assets } =>
				instruction::Instruction::Transfer(Transfer {
					assets: assets.0.into_iter().map(|asset| asset.into()).collect(),
					account_type: Some(to.into()),
				}),
			xcvm_core::Instruction::Call { bindings, encoded } =>
				instruction::Instruction::Call(Call {
					payload: encoded,
					bindings: Some(Bindings {
						bindings: bindings.into_iter().map(|binding| binding.into()).collect(),
					}),
				}),
			xcvm_core::Instruction::Spawn { network, bridge_security, salt, assets, program } =>
				instruction::Instruction::Spawn(Spawn {
					network: Some(network.into()),
					security: bridge_security as i32,
					salt: Some(Salt { salt }),
					program: Some(program.into()),
					assets: assets.0.into_iter().map(|asset| asset.into()).collect(),
				}),
			xcvm_core::Instruction::Query { network, salt } =>
				instruction::Instruction::Query(Query {
					network: Some(network.into()),
					salt: Some(Salt { salt }),
				}),
		}
	}
}

impl<Account> From<XCVMInstruction<Account>> for Instruction
where
	Vec<u8>: From<Account>,
{
	fn from(instruction: XCVMInstruction<Account>) -> Self {
		Instruction { instruction: Some(instruction.into()) }
	}
}

impl<Account> From<XCVMProgram<Account>> for Program
where
	Vec<u8>: From<Account>,
{
	fn from(program: XCVMProgram<Account>) -> Self {
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
	use xcvm_core::Displayed;

	#[test]
	fn balance_to_amount_works() {
		let balance = Balance {
			balance_type: Some(balance::BalanceType::Ratio(Ratio {
				nominator: Some(3u128.into()),
				denominator: Some(5u128.into()),
			})),
		};
		let amount: Amount = balance.try_into().unwrap();
		assert_eq!(amount.intercept, Displayed(0));

		let wrap = |num: u128| -> FixedU128<U16> { FixedU128::wrapping_from_num(num) };
		assert_eq!(
			wrap(3).saturating_div(wrap(5)),
			wrap(amount.slope.0).saturating_div(wrap(MAX_PARTS))
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
