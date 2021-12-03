#![cfg_attr(not(feature = "std"), no_std)]

pub mod blockchain;
mod mock;
mod operation_params;
mod pallet_api;
mod tests;

pub use operation_params::*;
pub use pallet::*;
pub use pallet_api::*;

#[frame_support::pallet]
pub mod pallet {
	use crate::{PalletApi, TransferParams};
	use frame_support::traits::Hooks;
	use xcm::v2::ExecuteXcm;

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type XcmExecutor: ExecuteXcm<Self::Call>;
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	impl<T> PalletApi for Pallet<T> {
		fn transfer(_params: TransferParams) {
			todo!()
		}
	}
}
