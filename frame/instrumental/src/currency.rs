pub type CurrencyId = u128;

/// # Note: 
///
/// I was having issues add pallet_lending as a dev-dependency, so this
/// is a copy of pallet_lending::currency::Currency.

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
/// let one_value_of_b = BCOIN::one();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Currency<const ID: CurrencyId, const EXPONENT: u8> {}

impl<const ID: u128, const EXPONENT: u8> Currency<ID, EXPONENT> {
	#![allow(unused)]

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

	/// Returns the provided amount of the currency, cannonicalized to [`Self::one()`], saturating
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
    pub fn units(ones: u128) -> u128 {
		ones.saturating_mul(Self::one())
	}

	/// The `one` value of the currency, calculated with [`Self::EXPONENT`].
	///
	/// # Examples
	///
	/// ```
	/// # use pallet_lending::currency::Currency;
	///
	/// type ACOIN = Currency<12345, 10>;
	/// assert_eq!(ACOIN::one(), 10_000_000_000);
	/// ```
    pub const fn one() -> u128 {
		10_u128.pow(Self::EXPONENT as u32)
	}
}

// separate module so that the `allow` attribute isn't appllied to the entirety of the currency
// module.
pub mod defs {
	#![allow(clippy::upper_case_acronyms)]
    #![allow(unused)]

	use super::Currency;

	pub type PICA = Currency<1, 12>;
	pub type USDC = Currency<1000, 12>;
	pub type BTC = Currency<2000, 12>;
	pub type LAYR = Currency<3000, 12>;
	pub type CROWDLOAN = Currency<4000, 12>;
	pub type KSM = Currency<5000, 12>;

	pub type NORMALIZED = USDC;
}

pub use defs::*;

use proptest::{prop_oneof, strategy::{Just, Strategy}};

#[allow(dead_code)]
pub fn pick_currency() -> impl Strategy<Value = CurrencyId> {
	prop_oneof![
		Just(PICA::ID),
		Just(BTC::ID),
		Just(USDC::ID),
	]
}
