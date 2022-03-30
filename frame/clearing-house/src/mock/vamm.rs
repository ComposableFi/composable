pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies
	// ----------------------------------------------------------------------------------------------------

	use codec::{Decode, Encode, FullCodec, MaxEncodedLen};
	use composable_traits::vamm::VirtualAMM;
	use frame_support::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_runtime::FixedPointNumber;

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
		type Decimal: FixedPointNumber;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                            Genesis Configuration
	// ----------------------------------------------------------------------------------------------------

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub vamm_id: Option<T::VammId>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { vamm_id: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			NextVammId::<T>::set(self.vamm_id.clone())
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
	pub struct VammParams;

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Storage
	// ----------------------------------------------------------------------------------------------------

	#[pallet::storage]
	#[pallet::getter(fn vamm_id)]
	pub type NextVammId<T: Config> = StorageValue<_, T::VammId, OptionQuery>;

	// ----------------------------------------------------------------------------------------------------
	//                                           Trait Implementations
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> VirtualAMM for Pallet<T> {
		type VammId = T::VammId;
		type VammParams = VammParams;
		type Decimal = T::Decimal;

		#[allow(unused_variables)]
		fn create(info: Self::VammParams) -> Result<Self::VammId, DispatchError> {
			if let Some(id) = Self::vamm_id() {
				Ok(id)
			} else {
				Err(Error::<T>::FailedToCreateVamm.into())
			}
		}

		fn get_twap(vamm: Self::VammId) -> Result<Self::Decimal, DispatchError> {
			Ok(Self::Decimal::checked_from_rational(10000u32, 100u32)
				.ok_or(Error::<T>::FailedToCalculateTwap)?)
		}
	}
}
