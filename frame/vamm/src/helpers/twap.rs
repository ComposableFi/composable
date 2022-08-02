use crate::{Config, Pallet, VammMap, VammStateOf};
use composable_traits::vamm::AssetType;
use frame_support::{pallet_prelude::*, transactional};
use sp_runtime::traits::Saturating;

impl<T: Config> Pallet<T> {
	/// Performs runtime storage changes, effectively updating the asset twap.
	/// This `update_twap` variation can't accept any error to occur, expecting
	/// all properties described in
	/// [`update_twap`](struct.Pallet.html#method.update_twap) to hold.
	///
	/// # Errors
	///
	/// * All errors returned by
	/// [`sanity_check_before_update_twap`](
	/// struct.Pallet.html#method.sanity_check_before_update_twap).
	#[transactional]
	pub fn do_update_twap(
		vamm_id: T::VammId,
		vamm_state: &mut VammStateOf<T>,
		base_twap: Option<T::Decimal>,
		now: &Option<T::Moment>,
	) -> Result<T::Decimal, DispatchError> {
		Self::internal_update_twap(vamm_id, vamm_state, base_twap, now, false)
	}

	/// *Tries to* perform runtime storage changes, effectively updating the
	/// asset twap.  Diferently than [`do_update_twap`](Self::do_update_twap) though, this variation
	/// accepts one specific error to occur, therefore not honoring all
	/// properties described in
	/// [`update_twap`](../pallet/struct.Pallet.html#method.update_twap). The
	/// property that can be violated using this variation is that the current
	/// twap timestamp can in fact be more recent than the current time.
	///
	/// # Errors
	///
	/// * All errors returned by
	/// [`sanity_check_before_update_twap`](
	/// struct.Pallet.html#method.sanity_check_before_update_twap),
	/// except
	/// [`Error::<T>::AssetTwapTimestampIsMoreRecent`](
	/// ../pallet/enum.Error.html#variant.AssetTwapTimestampIsMoreRecent).
	#[transactional]
	pub fn try_update_twap(
		vamm_id: T::VammId,
		vamm_state: &mut VammStateOf<T>,
		base_twap: Option<T::Decimal>,
		now: &Option<T::Moment>,
	) -> Result<T::Decimal, DispatchError> {
		Self::internal_update_twap(vamm_id, vamm_state, base_twap, now, true)
	}

	/// Handles the optional value for `base_twap` parameter in function
	/// [`update_twap`](struct.Pallet.html#method.update_twap), computing a new
	/// twap value if necessary.
	///
	/// # Errors
	///
	/// * [`ArithmeticError`](sp_runtime::ArithmeticError)
	pub fn handle_base_twap(
		base_twap: Option<T::Decimal>,
		vamm_state: &VammStateOf<T>,
	) -> Result<T::Decimal, DispatchError> {
		match base_twap {
			Some(base_twap) => Ok(base_twap),
			None => Self::calculate_twap(
				&None,
				vamm_state.twap_timestamp,
				vamm_state.twap_period,
				Self::do_get_price(vamm_state, AssetType::Base)?,
				vamm_state.base_asset_twap,
			),
		}
	}

	/// Efectively mutate runtime storage and
	/// [`VammState`](../types/struct.VammState.html#structfield.base_asset_reserves).
	fn internal_update_twap(
		vamm_id: T::VammId,
		vamm_state: &mut VammStateOf<T>,
		base_twap: Option<T::Decimal>,
		now: &Option<T::Moment>,
		try_update: bool,
	) -> Result<T::Decimal, DispatchError> {
		// Handle optional value.
		let base_twap = Self::handle_base_twap(base_twap, vamm_state)?;

		// Sanity checks must pass before updating runtime storage.
		Self::sanity_check_before_update_twap(vamm_state, base_twap, now, try_update)?;

		// Update VammState.
		let now = Self::now(now);
		vamm_state.base_asset_twap = base_twap;
		vamm_state.twap_timestamp = now;

		// Update runtime storage.
		VammMap::<T>::insert(&vamm_id, vamm_state);

		Ok(base_twap)
	}

	fn calculate_twap(
		now: &Option<T::Moment>,
		last_twap_timestamp: T::Moment,
		twap_period: T::Moment,
		new_price: T::Decimal,
		old_price: T::Decimal,
	) -> Result<T::Decimal, DispatchError> {
		let now = Self::now(now);
		let weight_now: T::Moment = now.saturating_sub(last_twap_timestamp).max(1_u64.into());
		let weight_last_twap: T::Moment = twap_period.saturating_sub(weight_now).max(1_u64.into());

		Self::calculate_exponential_moving_average(
			new_price,
			weight_now,
			old_price,
			weight_last_twap,
		)
	}
}
