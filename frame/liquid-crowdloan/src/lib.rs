#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, PalletId, sp_runtime::traits::AccountIdConversion};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type LiquidRewardId: Get<PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	impl<T: Config> Pallet<T> {
		/// Get a unique, inaccessible account id from the `LiquidRewardId`.
		pub fn account_id() -> T::AccountId {
			T::LiquidRewardId::get().into_account()
		}
	}
}
