use crate::{types::VammState, Config, Error, Pallet, VammMap};
use frame_support::pallet_prelude::*;

impl<T: Config> Pallet<T> {
	pub fn get_vamm_state(
		vamm_id: &T::VammId,
	) -> Result<VammState<T::Balance, T::Moment, T::Decimal>, DispatchError> {
		// Requested vamm must exists and be retrievable.
		ensure!(VammMap::<T>::contains_key(vamm_id), Error::<T>::VammDoesNotExist);
		let vamm_state = VammMap::<T>::get(vamm_id).ok_or(Error::<T>::FailToRetrieveVamm)?;
		Ok(vamm_state)
	}

	pub fn is_vamm_closed(
		vamm_state: &VammState<T::Balance, T::Moment, T::Decimal>,
		now: &Option<T::Moment>,
	) -> bool {
		let now = Self::now(now);
		match vamm_state.closed {
			Some(timestamp) => now >= timestamp,
			None => false,
		}
	}
}
