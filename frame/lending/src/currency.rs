/// A const-generic currency. generic over the ID and EXPONENT.
///
/// # Examples
///
/// Intended usage:
///
/// ```
/// type A = Currency<12345, 10>;
/// type B = Currency<54321, 12>;
///
/// let 100_a_tokens = A::units(100);
/// let one_value_of_b = B::ones();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Currency<const ID: CurrencyId, const EXPONENT: u8> {}

impl<const ID: CurrencyId, const EXPONENT: u8> Currency<ID, EXPONENT> {
	/// The exponent of the currency. Specifies the precision level; can be thought of as the number
	/// of decimal points in base 10.
	///
	/// A [`Currency`] with an EXPONENT of `0` has no decimals and is the exact same as a [`u128`].
	///
	/// # NOTE
	///
	/// Although this is a [`u8`], there are some (not yet inforced) constraints/ caveats:
	/// - an exponent of `0` technically works, but is probably not what you want as there will be
	///   no decimal precision.
	/// - any value higher than `38` does not make sense (`10^39 > 2^128`) and
	/// will automatically saturate at `u128::MAX`.
	pub const EXPONENT: u8 = EXPONENT;

	/// The id of the currency. This is fairly arbitrary, and is only used to differentiate between
	/// different currencies.
	pub const ID: CurrencyId = ID;

	/// Returns the provided amount of the currency, cannonicalized to [`Self::ones()`], saturating
	/// at the numeric bounds ([`u128::MAX`]).
	///
	/// # Examples
	///
	/// ```
	/// type A = Currency<12345, 10>;
	/// assert_eq!(A::units(7), 70_000_000_000);
	///
	/// // saturates at u128::MAX
	/// assert_eq!(A::units(u128::MAX), u128::MAX);
	/// ```
	pub fn units(ones: u128) -> u128 {
		ones.saturating_mul(Self::one())
	}

	/// The `one` value of the currency, calculated with [`Self::EXPONENT`].
	///
	/// # Examples
	///
	/// ```
	/// type A = Currency<12345, 10>;
	/// assert_eq!(A::ones(), 10_000_000_000);
	/// ```
	pub const fn one() -> u128 {
		10_u128.pow(Self::EXPONENT as u32)
	}
}

pub type CurrencyId = u128;

impl<const ID: CurrencyId, const EXPONENT: u8> From<Currency<ID, EXPONENT>> for CurrencyId {
	fn from(_: Currency<ID, EXPONENT>) -> Self {
		ID
	}
}

impl<const ID: CurrencyId, const EXPONENT: u8> AsRef<CurrencyId> for Currency<ID, EXPONENT> {
	#[inline(always)]
	fn as_ref(&self) -> &CurrencyId {
		&ID
	}
}

// separate module so that the `allow` attribute isn't appllied to the entirety of the currency
// module.
pub mod defs {
	#![allow(clippy::upper_case_acronyms)]

	use super::Currency;

	pub type PICA = Currency<1, 12>;
	pub type BTC = Currency<2000, 12>;
	pub type USDT = Currency<1000, 12>;

	pub type NORMALIZED = USDT;
}

pub use defs::*;
