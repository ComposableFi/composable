#![allow(clippy::upper_case_acronyms)]
<<<<<<< HEAD
use sp_std::ops::Deref;
=======
use std::ops::Deref;
>>>>>>> lending into runtime

pub type CurrencyId = u128;

#[derive(Copy, Debug, Clone)]
pub struct Currency<const ID: u128, const EXPONENT: u8> {}

impl<const ID: u128, const EXPONENT: u8> Currency<ID, EXPONENT> {
	pub const EXPONENT: u8 = EXPONENT;
	pub const ID: u128 = ID;

	pub fn units(ones: u128) -> u128 {
		ones.saturating_mul(Self::one())
	}
	pub const fn one() -> u128 {
		10_u128.pow(Self::EXPONENT as u32)
	}
}

impl<const ID: u128, const EXPONENT: u8> From<Currency<ID, EXPONENT>> for CurrencyId {
	fn from(_val: Currency<ID, EXPONENT>) -> Self {
		ID
	}
}
impl<const ID: u128, const EXPONENT: u8> Deref for Currency<ID, EXPONENT> {
	type Target = CurrencyId;

	fn deref(&self) -> &Self::Target {
		&ID
	}
}

impl<const ID: u128, const EXPONENT: u8> AsRef<CurrencyId> for Currency<ID, EXPONENT> {
	#[inline(always)]
	fn as_ref(&self) -> &CurrencyId {
		&ID
	}
}

#[allow(dead_code)]
pub type PICA = Currency<1, 12>;
pub type BTC = Currency<2000, 12>;
pub type USDT = Currency<1000, 12>;
#[allow(dead_code)]
pub type NORAMLIZED = USDT;
