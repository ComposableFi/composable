use crate::{Config, Error, Pallet};
use frame_support::pallet_prelude::*;
use sp_core::U256;
use sp_runtime::{traits::Zero, ArithmeticError, FixedPointNumber};

impl<T: Config> Pallet<T> {
	/// Returns the vamm invariant (aka. `K`), given `base` and `quote` asset
	/// amounts, where `K` can be derived as `base * quote`.
	///
	/// # Assumptions or Requirements
	/// In order to compute the invariant, both `base` and `quote` amounts must
	/// be greater than zero, as it's strictly forbidden for a vamm to have a
	/// invariant equal zero.
	///
	/// # Errors
	///
	/// * [`Error::<T>::BaseAssetReserveIsZero`]
	/// * [`Error::<T>::QuoteAssetReserveIsZero`]
	/// * [`Error::<T>::FailedToDeriveInvariantFromBaseAndQuoteAsset`]
	/// * [`Error::<T>::InvariantIsZero`]
	/// * [`ArithmeticError`](sp_runtime::ArithmeticError)
	pub fn compute_invariant(base: T::Balance, quote: T::Balance) -> Result<U256, DispatchError> {
		// Neither base nor quote asset are allowed to be zero since it
		// would mean the invariant would also be zero.
		ensure!(!base.is_zero(), Error::<T>::BaseAssetReserveIsZero);
		ensure!(!quote.is_zero(), Error::<T>::QuoteAssetReserveIsZero);

		let base_u256 = Self::balance_to_u256(base)?;
		let quote_u256 = Self::balance_to_u256(quote)?;
		let invariant = base_u256
			.checked_mul(quote_u256)
			.ok_or(Error::<T>::FailedToDeriveInvariantFromBaseAndQuoteAsset)?;

		ensure!(!invariant.is_zero(), Error::<T>::InvariantIsZero);

		Ok(invariant)
	}

	/// Performs an exponential moving average (EMA) calculation following the formula:
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
	pub fn calculate_exponential_moving_average(
		x1: T::Decimal,
		w1: T::Moment,
		x2: T::Decimal,
		w2: T::Moment,
	) -> Result<T::Decimal, DispatchError> {
		let w1_u256 = U256::from(w1.into());
		let w2_u256 = U256::from(w2.into());
		let denominator = w1_u256.checked_add(w2_u256).ok_or(ArithmeticError::Overflow)?;
		let xw1 = Self::balance_to_u256(x1.into_inner())?
			.checked_mul(w1_u256)
			.ok_or(ArithmeticError::Overflow)?;
		let xw2 = Self::balance_to_u256(x2.into_inner())?
			.checked_mul(w2_u256)
			.ok_or(ArithmeticError::Overflow)?;

		let twap_u256 = xw1
			.checked_add(xw2)
			.ok_or(ArithmeticError::Overflow)?
			.checked_div(denominator)
			.ok_or(ArithmeticError::DivisionByZero)?;
		let twap = Self::balance_to_decimal(Self::u256_to_balance(twap_u256)?);
		Ok(twap)
	}

	// TODO(Cardosaum): Create trait for U256 in `labs::numbers` implementing conversion between
	// balance, decimal and U256.
	/// Converts a [`Balance`](Config::Balance) into a [`Decimal`](Config::Decimal)
	/// value.
	pub fn balance_to_decimal(value: T::Balance) -> T::Decimal {
		T::Decimal::from_inner(value)
	}

	/// Converts a [`Balance`](Config::Balance) into a [`u128`] value.
	///
	/// # Errors
	///
	/// * [`ArithmeticError`](sp_runtime::ArithmeticError)
	pub fn balance_to_u128(value: T::Balance) -> Result<u128, DispatchError> {
		Ok(TryInto::<u128>::try_into(value).ok().ok_or(ArithmeticError::Overflow)?)
	}

	/// Converts a [`Balance`](Config::Balance) into a [`U256`] value.
	///
	/// # Errors
	///
	/// * [`ArithmeticError`](sp_runtime::ArithmeticError)
	pub fn balance_to_u256(value: T::Balance) -> Result<U256, DispatchError> {
		Ok(U256::from(Self::balance_to_u128(value)?))
	}

	/// Converts a [`U256`] into a [`u128`] value.
	///
	/// # Errors
	///
	/// * [`ArithmeticError`](sp_runtime::ArithmeticError)
	pub fn u256_to_u128(value: U256) -> Result<u128, DispatchError> {
		Ok(TryInto::<u128>::try_into(value).ok().ok_or(ArithmeticError::Overflow)?)
	}

	/// Converts a [`U256`] into a [`Balance`](Config::Balance) value.
	///
	/// # Errors
	///
	/// * [`ArithmeticError`](sp_runtime::ArithmeticError)
	pub fn u256_to_balance(value: U256) -> Result<T::Balance, DispatchError> {
		Ok(Self::u256_to_u128(value)?.try_into().ok().ok_or(ArithmeticError::Overflow)?)
	}
}
