/// A const-generic currency. generic over the ID and EXPONENT.
///
/// # Examples
///
/// Intended usage:
///
/// ```
/// # use composable_tests_helpers::test::currency::Currency;
///
/// type ACOIN = Currency<12345, 10>;
/// type BCOIN = Currency<54321, 12>;
///
/// let one_hundred_a_tokens = ACOIN::units(100);
/// let one_value_of_b = BCOIN::ONE;
/// ```
#[derive(Debug, Clone, Copy, TypeInfo)]
pub struct Currency<const ID: u128, const EXPONENT: u8> {}

impl<const ID: u128, const EXPONENT: u8> Currency<ID, EXPONENT> {
	/// The id of the currency. This is fairly arbitrary, and is only used to differentiate between
	/// different currencies.
	pub const ID: u128 = ID;

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

	/// The `one` value of the currency, calculated with [`Self::EXPONENT`].
	///
	/// # Examples
	///
	/// ```
	/// # use composable_tests_helpers::test::currency::Currency;
	///
	/// type ACOIN = Currency<12345, 10>;
	/// assert_eq!(ACOIN::ONE, 10_000_000_000);
	/// ```
	pub const ONE: u128 = 10_u128.pow(Self::EXPONENT as u32);

	/// Returns the provided amount of the currency, cannonicalized to
	/// [`Self::ONE`](composable_tests_helpers::test::currency::Currency::ONE), saturating at the
	/// numeric bounds ([`u128::MAX`](core::u128::MAX)).
	///
	/// # Examples
	///
	/// ```
	/// # use composable_tests_helpers::test::currency::Currency;
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

	/// Creates an 'instance' of this `Currency`.
	///
	/// Sometimes a value is needed, not just a type. See [`RuntimeCurrency`] for more information.
	pub const fn instance() -> RuntimeCurrency {
		RuntimeCurrency { id: ID, exponent: EXPONENT }
	}
}

/// A 'runtime' equivalent of [`Currency`].
///
/// # Examples
///
/// Can be created from a [`Currency`]:
///
/// ```rust
/// # use composable_tests_helpers::test::currency::{Currency, RuntimeCurrency};
/// type ACOIN = Currency<12345, 10>;
/// let runtime_currency = ACOIN::instance();
/// assert_eq!(runtime_currency.one(), ACOIN::ONE);
/// ```
///
/// Useful in non-const contexts:
///
/// ```rust
/// # use composable_tests_helpers::test::currency::{Currency, RuntimeCurrency};
/// let lp_token_id = create_btc_usdt_vault();
/// let rc = RuntimeCurrency::new(lp_token_id, 12);
/// let ten_lp_tokens = rc.units(10);
///
/// fn create_btc_usdt_vault() -> u128 {
///     // do something here and return the id of an lp_token...
///     42
/// }
/// ```
///
/// Create many currencies:
///
/// ```rust
/// # use composable_tests_helpers::test::currency::{Currency, RuntimeCurrency};
/// let currencies = (0..100)
///     .zip(std::iter::repeat(12))
///     .map(|(id, exp)| RuntimeCurrency::new(id, exp));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RuntimeCurrency {
	id: u128,
	exponent: u8,
}

impl RuntimeCurrency {
	pub fn new(id: u128, exponent: u8) -> Self {
		Self { id, exponent }
	}

	/// Get the runtime currency's id.
	///
	/// See [`Currency::ID`] for more information.
	pub fn id(&self) -> u128 {
		self.id
	}

	/// Get the runtime currency's exponent.
	///
	/// See [`Currency::EXPONENT`] for more information.
	pub fn exponent(&self) -> u8 {
		self.exponent
	}

	/// The `one` value of the currency, calculated with [`Self::exponent`].
	///
	/// # Examples
	///
	/// ```
	/// # use composable_tests_helpers::test::currency::{Currency, RuntimeCurrency};
	///
	/// type ACOIN = Currency<12345, 10>;
	/// assert_eq!(ACOIN::instance().one(), 10_000_000_000);
	/// ```
	pub const fn one(&self) -> u128 {
		10_u128.pow(self.exponent as u32)
	}

	/// Returns the provided amount of the currency, cannonicalized to [`Self::one()`], saturating
	/// at the numeric bounds ([`u128::MAX`]).
	///
	/// # Examples
	///
	/// ```
	/// # use composable_tests_helpers::test::currency::{Currency, RuntimeCurrency};
	///
	/// type ACOIN = Currency<12345, 10>;
	/// assert_eq!(ACOIN::instance().units(7), 70_000_000_000);
	///
	/// // saturates at u128::MAX
	/// assert_eq!(ACOIN::instance().units(u128::MAX), u128::MAX);
	/// ```
	pub const fn units(&self, ones: u128) -> u128 {
		ones.saturating_mul(self.one())
	}
}

impl<const ID: u128, const EXPONENT: u8> codec::Encode for Currency<ID, EXPONENT> {
	fn size_hint(&self) -> usize {
		16
	}

	fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
		ID.encode_to(dest);
	}
}

// separate module so that the `allow` attribute isn't appllied to the entirety of the currency
// module or per item.
pub mod defs {
	#![allow(clippy::upper_case_acronyms)]

	use super::Currency;

	pub type PICA = Currency<1, 12>;
	pub type BTC = Currency<2000, 12>;
	pub type USDT = Currency<1000, 12>;

	pub type NORMALIZED = USDT;
}

pub use defs::*;
use scale_info::TypeInfo;

pub type CurrencyId = u128;
