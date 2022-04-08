pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies
	// ----------------------------------------------------------------------------------------------------

	use codec::{Decode, Encode, FullCodec, MaxEncodedLen};
	use composable_traits::vamm::Vamm;
	use frame_support::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_runtime::{traits::Zero, FixedPointNumber};

	// ----------------------------------------------------------------------------------------------------
	//                                    Declaration Of The Pallet Type
	// ----------------------------------------------------------------------------------------------------

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// ----------------------------------------------------------------------------------------------------
	//                                             Config Trait
	// ----------------------------------------------------------------------------------------------------

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type VammId: FullCodec + MaxEncodedLen + MaybeSerializeDeserialize + TypeInfo + Clone;
		type Decimal: FixedPointNumber
			+ FullCodec
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ TypeInfo;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                            Genesis Configuration
	// ----------------------------------------------------------------------------------------------------

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub vamm_id: Option<T::VammId>,
		pub twap: Option<T::Decimal>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { vamm_id: None, twap: Some(T::Decimal::zero()) }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			NextVammId::<T>::set(self.vamm_id.clone());
			Twap::<T>::set(self.twap);
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Errors
	// ----------------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		FailedToCreateVamm,
		FailedToCalculateTwap,
	}

	// ----------------------------------------------------------------------------------------------------
	//                                             Pallet Types
	// ----------------------------------------------------------------------------------------------------

	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug, Clone, PartialEq)]
	pub struct VammConfig;

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Storage
	// ----------------------------------------------------------------------------------------------------

	#[pallet::storage]
	#[pallet::getter(fn vamm_id)]
	pub type NextVammId<T: Config> = StorageValue<_, T::VammId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn hardcoded_twap)]
	pub type Twap<T: Config> = StorageValue<_, T::Decimal, OptionQuery>;

	// ----------------------------------------------------------------------------------------------------
	//                                           Trait Implementations
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Vamm for Pallet<T> {
		type VammId = T::VammId;
		type VammConfig = VammConfig;
		type Decimal = T::Decimal;

		#[allow(unused_variables)]
		fn create(info: &Self::VammConfig) -> Result<Self::VammId, DispatchError> {
			if let Some(id) = Self::vamm_id() {
				Ok(id)
			} else {
				Err(Error::<T>::FailedToCreateVamm.into())
			}
		}

		#[allow(unused_variables)]
		fn get_twap(vamm: &Self::VammId) -> Result<Self::Decimal, DispatchError> {
			if let Some(twap) = Self::hardcoded_twap() {
				Ok(twap)
			} else {
				Err(Error::<T>::FailedToCalculateTwap.into())
			}
		}
	}
}
