pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies
	// ----------------------------------------------------------------------------------------------------

	use codec::{Decode, Encode, FullCodec, MaxEncodedLen};
	use composable_traits::{
		defi::DeFiComposableConfig,
		vamm::{AssetType, SwapConfig, SwapSimulationConfig, Vamm},
	};
	use frame_support::pallet_prelude::*;
	use num_integer::Integer;
	use scale_info::TypeInfo;
	use sp_arithmetic::traits::Unsigned;
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
	pub trait Config: DeFiComposableConfig + frame_system::Config {
		type VammId: Clone
			+ FullCodec
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ TypeInfo
			+ Unsigned;
		type Decimal: FixedPointNumber
			+ FullCodec
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ TypeInfo;
		type Integer: From<i128> + FullCodec + Integer + MaxEncodedLen + TypeInfo;
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
		FailedToExecuteSwap,
		FailedToSimulateSwap,
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

	#[pallet::storage]
	#[pallet::getter(fn _swap_output)]
	pub type SwapOutput<T: Config> = StorageValue<_, T::Integer, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn _swap_simulation_output)]
	pub type SwapSimulationOutput<T: Config> = StorageValue<_, T::Integer, OptionQuery>;

	// ----------------------------------------------------------------------------------------------------
	//                                           Trait Implementations
	// ----------------------------------------------------------------------------------------------------

	#[allow(unused_variables)]
	impl<T: Config> Vamm for Pallet<T> {
		type Balance = T::Balance;
		type Decimal = T::Decimal;
		type Integer = T::Integer;
		type SwapConfig = SwapConfig<Self::VammId, Self::Balance>;
		type SwapSimulationConfig = SwapSimulationConfig<Self::VammId, Self::Balance>;
		type VammConfig = VammConfig;
		type VammId = T::VammId;

		fn create(config: &Self::VammConfig) -> Result<Self::VammId, DispatchError> {
			if let Some(id) = Self::vamm_id() {
				Ok(id)
			} else {
				Err(Error::<T>::FailedToCreateVamm.into())
			}
		}

		fn get_price(
			vamm_id: Self::VammId,
			asset_type: AssetType,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn get_twap(vamm: &Self::VammId) -> Result<Self::Decimal, DispatchError> {
			if let Some(twap) = Self::hardcoded_twap() {
				Ok(twap)
			} else {
				Err(Error::<T>::FailedToCalculateTwap.into())
			}
		}

		fn swap(config: &Self::SwapConfig) -> Result<Self::Integer, DispatchError> {
			match Self::_swap_output() {
				Some(integer) => Ok(integer),
				None => Err(Error::<T>::FailedToExecuteSwap.into()),
			}
		}

		fn swap_simulation(
			config: &Self::SwapSimulationConfig,
		) -> Result<Self::Integer, DispatchError> {
			match Self::_swap_simulation_output() {
				Some(integer) => Ok(integer),
				None => Err(Error::<T>::FailedToSimulateSwap.into()),
			}
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Helper Functions
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		pub fn set_swap_output(integer: Option<T::Integer>) {
			SwapOutput::<T>::set(integer);
		}

		pub fn set_swap_simulation_output(integer: Option<T::Integer>) {
			SwapSimulationOutput::<T>::set(integer);
		}
	}
}
