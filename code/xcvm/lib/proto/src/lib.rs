#![no_std]

extern crate alloc;

use fixed::{types::extra::U16, FixedU128};
use xcvm_core::{Amount, MAX_PARTS};

include!(concat!(env!("OUT_DIR"), "/interpreter.rs"));

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
