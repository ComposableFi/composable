/// A const-generic currency. generic over the ID and EXPONENT.
///
/// # Examples
///
/// Intended usage:
///
/// ```
/// # use pallet_lending::currency::Currency;
///
/// type ACOIN = Currency<12345, 10>;
/// type BCOIN = Currency<54321, 12>;
///
/// let one_hundred_a_tokens = ACOIN::units(100);
/// let one_value_of_b = BCOIN::ONE;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Currency<const ID: u128, const EXPONENT: u8> {}

impl<const ID: u128, const EXPONENT: u8> Currency<ID, EXPONENT> {
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
	/// - any value higher than `38` does not make sense (`10^39 > 2^128`) and will automatically
	///   saturate at [`u128::MAX`].
	pub const EXPONENT: u8 = EXPONENT;

	/// The id of the currency. This is fairly arbitrary, and is only used to differentiate between
	/// different currencies.
	pub const ID: u128 = ID;

	/// The `one` value of the currency, calculated with [`Self::EXPONENT`].
	///
	/// # Examples
	///
	/// ```
	/// # use pallet_lending::currency::Currency;
	///
	/// type ACOIN = Currency<12345, 10>;
	/// assert_eq!(ACOIN::ONE, 10_000_000_000);
	/// ```
	pub const ONE: u128 = 10_u128.pow(Self::EXPONENT as u32);

	/// Returns the provided amount of the currency, cannonicalized to [`Self::ONE`], saturating
	/// at the numeric bounds ([`u128::MAX`]).
	///
	/// # Examples
	///
	/// ```
	/// # use pallet_lending::currency::Currency;
	///
	/// type ACOIN = Currency<12345, 10>;
	/// assert_eq!(ACOIN::units(7), 70_000_000_000);
	///
	/// // saturates at u128::MAX
	/// assert_eq!(ACOIN::units(u128::MAX), u128::MAX);
	/// ```
	pub const fn units(ones: u128) -> u128 {
		ones.saturating_mul(Self::ONE)
	}

	// Runtime methods

	/// Creates an instance of this `Currency`.
	///
	/// Sometimes a value is needed, not just a type. This could be called `new`, but since there
	/// aren't any runtime values associated with this type, `instance` is less confusing.
	const fn instance() -> Self {
		Self {}
	}

	/// Runtime version of [`Self::ID`].
	const fn id(&self) -> u128 {
		Self::ID
	}
}

// pub trait RuntimeCurrency {
// 	const fn instance() -> Self;

// 	const fn id(&self) -> u128;
// }

// impl<const ID: u128, const EXPONENT: u8> RuntimeCurrency for Currency<ID, EXPONENT> {
// 	/// Creates an instance of this `Currency`.
// 	///
// 	/// Sometimes a value is needed, not just a type. This could be called `new`, but since there
// 	/// aren't any runtime values associated with this type, `instance` is less confusing.
// 	const fn instance() -> Self {
// 		Self {}
// 	}

// 	/// Runtime version of [`Self::ID`].
//     const fn id(&self) -> u128 {
//         Self::ID
//     }
// }

fn t234() {
	let t = BTC::instance();
	let t22 = t.id();
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

pub type CurrencyId = u128;
