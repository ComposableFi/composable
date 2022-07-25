use crate::{Config, Error, Event, Pallet, SwapConfigOf, SwapOutputOf, VammMap, VammStateOf};
use composable_maths::labs::numbers::UnsignedMath;
use composable_traits::vamm::{AssetType, Direction, SwapOutput};
use frame_support::{pallet_prelude::*, transactional};
use sp_runtime::ArithmeticError;
use std::cmp::Ordering;

// TODO(Cardosaum): Document this struct:
// - Why is it needed
// - What's its purpose
// - What does each field represent
// - Why doing this way and not using tuples for example?
struct CalculateSwapAsset<T: Config> {
	output_amount: T::Balance,
	input_amount: T::Balance,
}

/// Stores the result of a swap operation, containing both the new values for
/// `base` and `quote` assets as well as how much the caller would receive (or
/// pay) in return for this specific swap.
pub struct ComputeSwap<T: Config> {
	/// The new amount of `base` asset that the vamm would contain after this
	/// swap.
	base_asset_reserves: T::Balance,
	/// The new amount of `quote` asset that the vamm would contain after this
	/// swap.
	quote_asset_reserves: T::Balance,
	/// The total asset amount the caller will receive/pay in terms of the the
	/// opposite asset for this swap to takes place.
	pub swap_output: SwapOutputOf<T>,
}

impl<T: Config> Pallet<T> {
	/// Performs runtime storage changes, effectively conducting the asset swap.
	/// For more information about what this function does, read
	/// [`swap`](struct.Pallet.html#method.swap) function documentation.
	///
	/// # Errors
	///
	/// * [`Error::<T>::BaseAssetReservesWouldBeCompletelyDrained`]
	/// * [`Error::<T>::FailToRetrieveVamm`]
	/// * [`Error::<T>::InsufficientFundsForTrade`]
	/// * [`Error::<T>::QuoteAssetReservesWouldBeCompletelyDrained`]
	/// * [`Error::<T>::SwappedAmountLessThanMinimumLimit`]
	/// * [`Error::<T>::TradeExtrapolatesMaximumSupportedAmount`]
	/// * [`ArithmeticError`](sp_runtime::ArithmeticError)
	#[transactional]
	pub fn do_swap(
		config: &SwapConfigOf<T>,
		vamm_state: &mut VammStateOf<T>,
	) -> Result<SwapOutputOf<T>, DispatchError> {
		// Compute new reserves of base and quote asset and swap result.
		let ComputeSwap { base_asset_reserves, quote_asset_reserves, swap_output } =
			Self::compute_swap(config, vamm_state)?;

		// Update runtime storage.
		VammMap::<T>::try_mutate(&config.vamm_id, |old_vamm_state| match old_vamm_state {
			Some(v) => {
				v.base_asset_reserves = base_asset_reserves;
				v.quote_asset_reserves = quote_asset_reserves;
				Ok(())
			},
			None => Err(Error::<T>::FailToRetrieveVamm),
		})?;

		// Deposit swap event into blockchain.
		Self::deposit_event(Event::<T>::Swapped {
			vamm_id: config.vamm_id,
			input_amount: config.input_amount,
			output_amount: swap_output,
			input_asset_type: config.asset,
			direction: config.direction,
		});

		// Return swap output.
		Ok(swap_output)
	}

	/// Performs runtime storage changes, effectively conducting the asset swap.
	/// For more information about what this function does, read
	/// [`swap`](struct.Pallet.html#method.swap) function documentation.
	///
	/// # Errors
	///
	/// * [`Error::<T>::BaseAssetReservesWouldBeCompletelyDrained`]
	/// * [`Error::<T>::FailToRetrieveVamm`]
	/// * [`Error::<T>::InsufficientFundsForTrade`]
	/// * [`Error::<T>::QuoteAssetReservesWouldBeCompletelyDrained`]
	/// * [`Error::<T>::SwappedAmountLessThanMinimumLimit`]
	/// * [`Error::<T>::TradeExtrapolatesMaximumSupportedAmount`]
	/// * [`ArithmeticError`](sp_runtime::ArithmeticError)
	pub fn compute_swap(
		config: &SwapConfigOf<T>,
		vamm_state: &VammStateOf<T>,
	) -> Result<ComputeSwap<T>, DispatchError> {
		// Check if initial swap properties are valid.
		Self::sanity_check_before_swap(config, vamm_state)?;

		// Delegate alternate computation to helper functions.
		let swap = match config.asset {
			AssetType::Quote => Self::compute_swap_quote_asset(config, vamm_state),
			AssetType::Base => Self::compute_swap_base_asset(config, vamm_state),
		}?;

		// Check if swap doesn't violate Vamm properties and swap requirements.
		Self::sanity_check_after_swap(
			&VammStateOf::<T> {
				base_asset_reserves: swap.base_asset_reserves,
				quote_asset_reserves: swap.quote_asset_reserves,
				..*vamm_state
			},
			config,
			&swap.swap_output,
		)?;

		// Return swap result.
		Ok(swap)
	}

	fn compute_swap_quote_asset(
		config: &SwapConfigOf<T>,
		vamm_state: &VammStateOf<T>,
	) -> Result<ComputeSwap<T>, DispatchError> {
		let quote_asset_reserve_amount = config.input_amount.try_div(&vamm_state.peg_multiplier)?;

		let initial_base_asset_reserve = vamm_state.base_asset_reserves;
		let swap_amount = Self::calculate_swap_asset(
			&quote_asset_reserve_amount,
			&vamm_state.quote_asset_reserves,
			&config.direction,
			vamm_state,
		)?;

		let swap_output = match initial_base_asset_reserve.cmp(&swap_amount.output_amount) {
			Ordering::Less => SwapOutput {
				output: swap_amount.output_amount.try_sub(&initial_base_asset_reserve)?,
				negative: true,
			},
			_ => SwapOutput {
				output: initial_base_asset_reserve.try_sub(&swap_amount.output_amount)?,
				negative: false,
			},
		};

		Ok(ComputeSwap {
			base_asset_reserves: swap_amount.output_amount,
			quote_asset_reserves: swap_amount.input_amount,
			swap_output,
		})
	}

	fn compute_swap_base_asset(
		config: &SwapConfigOf<T>,
		vamm_state: &VammStateOf<T>,
	) -> Result<ComputeSwap<T>, DispatchError> {
		let initial_quote_asset_reserve = vamm_state.quote_asset_reserves;
		let swap_amount = Self::calculate_swap_asset(
			&config.input_amount,
			&vamm_state.base_asset_reserves,
			&config.direction,
			vamm_state,
		)?;
		let swap_output = SwapOutput {
			output: Self::calculate_quote_asset_amount_swapped(
				&initial_quote_asset_reserve,
				&swap_amount.output_amount,
				&config.direction,
				vamm_state,
			)?,
			negative: false,
		};

		Ok(ComputeSwap {
			base_asset_reserves: swap_amount.input_amount,
			quote_asset_reserves: swap_amount.output_amount,
			swap_output,
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
