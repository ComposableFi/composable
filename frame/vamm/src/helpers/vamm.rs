use crate::{Config, Error, Pallet, VammMap, VammStateOf};
use frame_support::{pallet_prelude::*, traits::UnixTime};

impl<T: Config> Pallet<T> {
	/// Retrieve the [`VammState`](../types/struct.VammState.html) of the desired vamm.
	///
	/// # Errors
	///
	/// * [`Error::<T>::VammDoesNotExist`]
	pub fn get_vamm_state(vamm_id: &T::VammId) -> Result<VammStateOf<T>, DispatchError> {
		// Requested vamm must exist and be retrievable.
		VammMap::<T>::get(vamm_id).ok_or_else(|| Error::<T>::VammDoesNotExist.into())
	}

	/// Returns a boolean informing if the vamm is closed or not.
	pub fn is_vamm_closed(vamm_state: &VammStateOf<T>, now: &Option<T::Moment>) -> bool {
		let now = Self::now(now);
		match vamm_state.closed {
			Some(timestamp) => now >= timestamp,
			None => false,
		}
	}

	/// Returns the current timestamp.
	pub fn now(now: &Option<T::Moment>) -> T::Moment {
		match now {
			Some(now) => *now,
			None => T::TimeProvider::now().as_secs().into(),
		}
	}
}
