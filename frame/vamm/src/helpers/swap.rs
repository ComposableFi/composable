use crate::{Config, Pallet, SwapConfigOf, SwapOutputOf, VammStateOf};
use composable_maths::labs::numbers::UnsignedMath;
use composable_traits::vamm::{AssetType, Direction, SwapOutput};
use frame_support::pallet_prelude::*;
use sp_runtime::ArithmeticError;
use std::cmp::Ordering;

struct CalculateSwapAsset<T: Config> {
	pub output_amount: T::Balance,
	pub input_amount: T::Balance,
}

impl<T: Config> Pallet<T> {
	pub fn do_swap(
		config: &SwapConfigOf<T>,
		vamm_state: &mut VammStateOf<T>,
	) -> Result<SwapOutputOf<T>, DispatchError> {
		// Delegate swap to helper functions.
		let amount_swapped = match config.asset {
			AssetType::Quote => Self::swap_quote_asset(config, vamm_state),
			AssetType::Base => Self::swap_base_asset(config, vamm_state),
		}?;

		// Check if swap doesn't violate Vamm properties and swap requirements.
		Self::sanity_check_after_swap(vamm_state, config, &amount_swapped)?;

		// TODO(Cardosaum): Write one more `ensure!` block regarding
		// amount_swapped negative or positive?

		Ok(amount_swapped)
	}

	fn swap_quote_asset(
		config: &SwapConfigOf<T>,
		vamm_state: &mut VammStateOf<T>,
	) -> Result<SwapOutputOf<T>, DispatchError> {
		let quote_asset_reserve_amount = config.input_amount.try_div(&vamm_state.peg_multiplier)?;

		let initial_base_asset_reserve = vamm_state.base_asset_reserves;
		let swap_amount = Self::calculate_swap_asset(
			&quote_asset_reserve_amount,
			&vamm_state.quote_asset_reserves,
			&config.direction,
			vamm_state,
		)?;

		vamm_state.base_asset_reserves = swap_amount.output_amount;
		vamm_state.quote_asset_reserves = swap_amount.input_amount;

		match initial_base_asset_reserve.cmp(&swap_amount.output_amount) {
			Ordering::Less => Ok(SwapOutput {
				output: swap_amount.output_amount.try_sub(&initial_base_asset_reserve)?,
				negative: true,
			}),
			_ => Ok(SwapOutput {
				output: initial_base_asset_reserve.try_sub(&swap_amount.output_amount)?,
				negative: false,
			}),
		}
	}

	fn swap_base_asset(
		config: &SwapConfigOf<T>,
		vamm_state: &mut VammStateOf<T>,
	) -> Result<SwapOutputOf<T>, DispatchError> {
		let initial_quote_asset_reserve = vamm_state.quote_asset_reserves;
		let swap_amount = Self::calculate_swap_asset(
			&config.input_amount,
			&vamm_state.base_asset_reserves,
			&config.direction,
			vamm_state,
		)?;

		vamm_state.base_asset_reserves = swap_amount.input_amount;
		vamm_state.quote_asset_reserves = swap_amount.output_amount;

		Ok(SwapOutput {
			output: Self::calculate_quote_asset_amount_swapped(
				&initial_quote_asset_reserve,
				&swap_amount.output_amount,
				&config.direction,
				vamm_state,
			)?,
			negative: false,
		})
	}

	fn calculate_swap_asset(
		swap_amount: &T::Balance,
		input_asset_amount: &T::Balance,
		direction: &Direction,
		vamm_state: &VammStateOf<T>,
	) -> Result<CalculateSwapAsset<T>, DispatchError> {
		let new_input_amount = match direction {
			Direction::Add => input_asset_amount.try_add(swap_amount)?,
			Direction::Remove => input_asset_amount.try_sub(swap_amount)?,
		};
		let new_input_amount_u256 = Self::balance_to_u256(new_input_amount)?;

		// TODO(Cardosaum): Maybe it would be worth to create another sanity
		// check in the helper function tracking the inputs and verify if
		// they would result in a division by zero? (Doing this we could
		// present a better error message for the caller).
		let new_output_amount_u256 = vamm_state
			.invariant
			.checked_div(new_input_amount_u256)
			.ok_or(ArithmeticError::DivisionByZero)?;
		let new_output_amount = Self::u256_to_balance(new_output_amount_u256)?;

		Ok(CalculateSwapAsset { input_amount: new_input_amount, output_amount: new_output_amount })
	}

	fn calculate_quote_asset_amount_swapped(
		quote_asset_reserve_before: &T::Balance,
		quote_asset_reserve_after: &T::Balance,
		direction: &Direction,
		vamm_state: &VammStateOf<T>,
	) -> Result<T::Balance, DispatchError> {
		let quote_asset_reserve_change = match direction {
			Direction::Add => quote_asset_reserve_before.try_sub(quote_asset_reserve_after)?,
			Direction::Remove => quote_asset_reserve_after.try_sub(quote_asset_reserve_before)?,
		};

		let quote_asset_amount = quote_asset_reserve_change.try_mul(&vamm_state.peg_multiplier)?;

		Ok(quote_asset_amount)
	}
}
