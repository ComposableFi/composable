use crate::{Config, Pallet, VammStateOf};
use composable_maths::labs::numbers::IntoU256;
use composable_traits::vamm::AssetType;
use frame_support::pallet_prelude::*;
use sp_runtime::{ArithmeticError, FixedPointNumber};

impl<T: Config> Pallet<T> {
	/// Computes the current price for the desired asset, returning it.
	///
	/// # Errors
	///
	/// * [`ArithmeticError`](sp_runtime::ArithmeticError)
	pub fn do_get_price(
		vamm_state: &VammStateOf<T>,
		asset_type: AssetType,
	) -> Result<T::Decimal, DispatchError> {
		let precision = T::Decimal::DIV.into_u256();
		let base_u256 = vamm_state.base_asset_reserves.into_u256();
		let quote_u256 = vamm_state.quote_asset_reserves.into_u256();
		let peg_u256 = vamm_state.peg_multiplier.into_u256();

		let price_u256 = match asset_type {
			AssetType::Base => quote_u256
				.checked_mul(peg_u256)
				.ok_or(ArithmeticError::Overflow)?
				.checked_mul(precision)
				.ok_or(ArithmeticError::Overflow)?
				.checked_div(base_u256)
				.ok_or(ArithmeticError::DivisionByZero)?,

			AssetType::Quote => base_u256
				.checked_mul(precision)
				.ok_or(ArithmeticError::Overflow)?
				.checked_div(peg_u256.checked_mul(quote_u256).ok_or(ArithmeticError::Overflow)?)
				.ok_or(ArithmeticError::DivisionByZero)?,
		};

		let price_u128: u128 = price_u256.try_into()?;
		Ok(T::Decimal::from_inner(price_u128.into()))
	}
}
