use crate::{Config, Error, Pallet};
use composable_maths::labs::numbers::IntoU256;
use frame_support::pallet_prelude::*;
use sp_core::U256;
use sp_runtime::{
	traits::Zero,
	ArithmeticError::{DivisionByZero, Overflow},
	FixedPointNumber,
};

impl<T: Config> Pallet<T> {
	/// Returns the vamm invariant (aka. `K`), given `base` and `quote` asset
	/// amounts, where `K` can be derived as `base * quote`.
	///
	/// # Assumptions or Requirements
	/// In order to compute the invariant, both `base` and `quote` amounts must
	/// be greater than zero, as it's strictly forbidden for a vamm to have a
	/// invariant equal to zero.
	///
	/// # Errors
	///
	/// * [`Error::<T>::BaseAssetReserveIsZero`]
	/// * [`Error::<T>::QuoteAssetReserveIsZero`]
	/// * [`Error::<T>::InvariantIsZero`]
	/// * [`ArithmeticError`](sp_runtime::ArithmeticError)
	pub fn compute_invariant(base: T::Balance, quote: T::Balance) -> Result<U256, DispatchError> {
		// Neither base nor quote asset are allowed to be zero since it
		// would mean the invariant would also be zero.
		ensure!(!base.is_zero(), Error::<T>::BaseAssetReserveIsZero);
		ensure!(!quote.is_zero(), Error::<T>::QuoteAssetReserveIsZero);

		let base_u256 = base.into_u256();
		let quote_u256 = quote.into_u256();
		let invariant = base_u256.checked_mul(quote_u256).ok_or(Overflow)?;

		ensure!(!invariant.is_zero(), Error::<T>::InvariantIsZero);

		Ok(invariant)
	}

	/// Calculates the exponential moving average (EMA) following the formula:
	///
	/// - `ema = ((x1 * w1) + (x2 * w2)) / (w1 + w2)`
	///
	/// Where:
	///
	/// - `x1`: New point value,
	/// - `x2`: Previous EMA value,
	/// - `w1`: Weight for `x1`,
	/// - `w2`: Weight for `x2`.
	///
	///
	/// # Errors
	///
	/// * [`ArithmeticError`](sp_runtime::ArithmeticError)
	pub fn calculate_weighted_average(
		x1: T::Decimal,
		w1: T::Moment,
		x2: T::Decimal,
		w2: T::Moment,
	) -> Result<T::Decimal, DispatchError> {
		let w1_u256 = U256::from(w1.into());
		let w2_u256 = U256::from(w2.into());
		let denominator = w1_u256.checked_add(w2_u256).ok_or(Overflow)?;
		let xw1 = x1.into_inner().into_u256().checked_mul(w1_u256).ok_or(Overflow)?;
		let xw2 = x2.into_inner().into_u256().checked_mul(w2_u256).ok_or(Overflow)?;

		let twap_u256 = xw1
			.checked_add(xw2)
			.ok_or(Overflow)?
			.checked_div(denominator)
			.ok_or(DivisionByZero)?;

		let twap_u128: u128 = twap_u256.try_into()?;

		Ok(T::Decimal::from_inner(twap_u128.into()))
	}
}
