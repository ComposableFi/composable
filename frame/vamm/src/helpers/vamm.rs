use crate::{Config, Error, Pallet, VammMap, VammStateOf};
use frame_support::{pallet_prelude::*, traits::UnixTime};

impl<T: Config> Pallet<T> {
	/// Retrieve the [`VammState`](../types/struct.VammState.html) of the desired vamm.
	///
	/// # Errors
	///
	/// * [`Error::<T>::VammDoesNotExist`]
	/// * [`Error::<T>::FailToRetrieveVamm`]
	pub fn get_vamm_state(vamm_id: &T::VammId) -> Result<VammStateOf<T>, DispatchError> {
		// Requested vamm must exists and be retrievable.
		ensure!(VammMap::<T>::contains_key(vamm_id), Error::<T>::VammDoesNotExist);
		let vamm_state = VammMap::<T>::get(vamm_id).ok_or(Error::<T>::FailToRetrieveVamm)?;
		Ok(vamm_state)
	}

	/// Returns a boolean informing if the vamm is closed or not.
	pub fn is_vamm_closed(vamm_state: &VammStateOf<T>, now: &Option<T::Moment>) -> bool {
		let now = Self::now(now);
		match vamm_state.closed {
			Some(timestamp) => now >= timestamp,
			None => false,
		}
	}

	/// Returns a boolean informing if the vamm is closing or not.
	pub fn is_vamm_closing(vamm_state: &VammStateOf<T>, now: &Option<T::Moment>) -> bool {
		let now = Self::now(now);
		match vamm_state.closed {
			Some(closing_time) => now < closing_time,
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
