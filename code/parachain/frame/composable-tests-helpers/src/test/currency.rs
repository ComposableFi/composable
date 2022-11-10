pub type CurrencyId = u128;

use core::marker::PhantomData;

use sp_std::ops::Deref;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

// https://github.com/paritytech/substrate/pull/12638
// const XCM_RESERVE_LOCATION: Option<xcm::latest::MultiLocation> = None,
#[derive(RuntimeDebug, Clone, Copy, TypeInfo, PartialEq, Eq)]
pub struct ComposableCurrency<
	Consensus,
	const ID: CurrencyId,
	const EXPONENT: u8 = 12,
	const PROPOSED_RESERVE_MINIMAL_FEE: u64 = 0,
> {
	_marker: PhantomData<Consensus>,
}

impl<
		Consensus,
		const ID: CurrencyId,
		const EXPONENT: u8,
		const PROPOSED_RESERVE_MINIMAL_FEE: u64,
	> Deref for ComposableCurrency<Consensus, ID, EXPONENT, PROPOSED_RESERVE_MINIMAL_FEE>
{
	type Target = CurrencyId;

	fn deref(&self) -> &Self::Target {
		&Self::ID
	}
}

/// A const-generic currency. generic over the ID and EXPONENT.
///
/// # Examples
///
/// Intended usage:
///
/// ```
/// # use composable_tests_helpers::test::currency::*;
///
/// type ACOIN = Currency<12345, 10>;
/// type BCOIN = Currency<54321, 12>;
///
/// let one_hundred_a_tokens = ACOIN::units(100);
/// let one_value_of_b = BCOIN::ONE;
/// ```
pub type Currency<
	const ID: CurrencyId,
	const EXPONENT: u8 = 12,
	const PROPOSED_RESERVE_MINIMAL_FEE: u64 = 0,
> = ComposableCurrency<(), ID, EXPONENT, PROPOSED_RESERVE_MINIMAL_FEE>;

impl<
		Consensus,
		const ID: CurrencyId,
		const EXPONENT: u8,
		const PROPOSED_RESERVE_MINIMAL_FEE: u64,
	> ComposableCurrency<Consensus, ID, EXPONENT, PROPOSED_RESERVE_MINIMAL_FEE>
{
	/// The id of the currency. This is fairly arbitrary, and is only used to differentiate between
	/// different currencies.
	pub const ID: CurrencyId = ID;

	/// The exponent of the currency. Specifies the precision level; can be thought of as the number
	/// of decimal points in base 10.
	///
	/// A [`Currency`] with an EXPONENT of `0` has no decimals and is the exact same as a
	/// [`CurrencyId`].
	///
	/// # NOTE
	///
	/// Although this is a [`u8`], there are some (not yet enforced) constraints/ caveats:
	/// - an exponent of `0` technically works, but is probably not what you want as there will be
	///   no decimal precision.
	/// - any value higher than `38` does not make sense (`10^39 > 2^128`) and will automatically
	///   saturate at [`CurrencyId::MAX`].
	pub const EXPONENT: u8 = EXPONENT;

	/// The `one` value of the currency, calculated with [`Self::EXPONENT`].
	///
	/// # Examples
	///
	/// ```
	/// # use composable_tests_helpers::test::currency::*;
	///
	/// type ACOIN = Currency<12345, 10>;
	/// assert_eq!(ACOIN::ONE, 10_000_000_000);
	/// ```
	pub const ONE: CurrencyId = (10 as CurrencyId).pow(Self::EXPONENT as u32);

	/// Returns the provided amount of the currency, canonicalized to
	/// [`Self::ONE`](composable_tests_helpers::test::currency::Currency::ONE), saturating at the
	/// numeric bounds ([`CurrencyId::MAX`](core::CurrencyId::MAX)).
	///
	/// # Examples
	///
	/// ```
	/// # use composable_tests_helpers::test::currency::*;
	///
	/// type ACOIN = Currency<12345, 10>;
	/// assert_eq!(ACOIN::units(7), 70_000_000_000);
	///
	/// // saturates at CurrencyId::MAX
	/// assert_eq!(ACOIN::units(CurrencyId::MAX), CurrencyId::MAX);
	/// ```
	pub const fn units(ones: CurrencyId) -> CurrencyId {
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
/// # use composable_tests_helpers::test::currency::*;
/// type ACOIN = Currency<12345, 10>;
/// let runtime_currency = ACOIN::instance();
/// assert_eq!(runtime_currency.one(), ACOIN::ONE);
/// ```
///
/// Useful in non-const contexts:
///
/// ```rust
/// # use composable_tests_helpers::test::currency::*;
/// let lp_token_id = create_btc_usdt_vault();
/// let rc = RuntimeCurrency::new(lp_token_id, 12);
/// let ten_lp_tokens = rc.units(10);
///
/// fn create_btc_usdt_vault() -> CurrencyId {
///     // do something here and return the id of an lp_token...
///     42
/// }
/// ```
///
/// Create many currencies:
///
/// ```rust
/// # use composable_tests_helpers::test::currency::*;
/// let currencies = (0..100)
///     .zip(std::iter::repeat(12))
///     .map(|(id, exp)| RuntimeCurrency::new(id, exp));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RuntimeCurrency {
	id: CurrencyId,
	exponent: u8,
}

impl RuntimeCurrency {
	pub fn new(id: CurrencyId, exponent: u8) -> Self {
		Self { id, exponent }
	}

	/// Get the runtime currency's id.
	///
	/// See [`Currency::ID`] for more information.
	pub fn id(&self) -> CurrencyId {
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
	/// # use composable_tests_helpers::test::currency::*;
	///
	/// type ACOIN = Currency<12345, 10>;
	/// assert_eq!(ACOIN::instance().one(), 10_000_000_000);
	/// ```
	pub const fn one(&self) -> CurrencyId {
		(10 as CurrencyId).pow(self.exponent as u32)
	}

	/// Returns the provided amount of the currency, canonicalized to [`Self::one()`], saturating
	/// at the numeric bounds ([`CurrencyId::MAX`]).
	///
	/// # Examples
	///
	/// ```
	/// # use composable_tests_helpers::test::currency::*;
	///
	/// type ACOIN = Currency<12345, 10>;
	/// assert_eq!(ACOIN::instance().units(7), 70_000_000_000);
	///
	/// // saturates at CurrencyId::MAX
	/// assert_eq!(ACOIN::instance().units(CurrencyId::MAX), CurrencyId::MAX);
	/// ```
	pub const fn units(&self, ones: CurrencyId) -> CurrencyId {
		ones.saturating_mul(self.one())
	}
}

impl<Consensus, const ID: CurrencyId, const EXPONENT: u8> codec::Encode
	for ComposableCurrency<Consensus, ID, EXPONENT>
{
	fn size_hint(&self) -> usize {
		sp_std::mem::size_of::<CurrencyId>()
	}

	fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
		ID.encode_to(dest);
	}
}

impl<Consensus, const ID: CurrencyId, const EXPONENT: u8> MaxEncodedLen
	for ComposableCurrency<Consensus, ID, EXPONENT>
{
	fn max_encoded_len() -> usize {
		sp_std::mem::size_of::<CurrencyId>()
	}
}

// separate module so that the `allow` attribute isn't applied to the entirety of the currency
// module or per item.
// TODO: make macro which takes `type` name as `symbol` and produce static `map` from id to
// metadata.
pub mod defs {
	#![allow(clippy::upper_case_acronyms)]

	use super::*;

	pub type PICA = Currency<1>;
	pub type XPICA = Currency<2>;
	pub type BTC = Currency<2000>;
	pub type USDT = Currency<130>;

	pub type NORMALIZED = USDT;
}

pub use defs::*;

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, PartialEq, Eq, Copy, RuntimeDebug)]
pub struct CurrencyAmount<
	Consensus,
	const ID: CurrencyId,
	const EXPONENT: u8,
	const PROPOSED_RESERVE_MINIMAL_FEE: u64,
	PositiveBalance,
> {
	pub currency: ComposableCurrency<Consensus, ID, EXPONENT, PROPOSED_RESERVE_MINIMAL_FEE>,
	pub amount: PositiveBalance,
}
