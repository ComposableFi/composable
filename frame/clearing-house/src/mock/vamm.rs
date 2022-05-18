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
	use num_traits::CheckedDiv;
	use scale_info::TypeInfo;
	use sp_arithmetic::traits::Unsigned;
	use sp_runtime::{
		traits::{Saturating, Zero},
		ArithmeticError, FixedPointNumber,
	};

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
		type Decimal: FixedPointNumber<Inner = Self::Balance>
			+ FullCodec
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ Saturating
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
	#[pallet::getter(fn _price)]
	pub type Price<T: Config> = StorageValue<_, T::Decimal, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn _twap_of)]
	pub type Twaps<T: Config> = StorageMap<_, Twox64Concat, T::VammId, T::Decimal>;

	#[pallet::storage]
	#[pallet::getter(fn _price_of)]
	pub type Prices<T: Config> = StorageMap<_, Twox64Concat, T::VammId, T::Decimal>;

	// ----------------------------------------------------------------------------------------------------
	//                                           Trait Implementations
	// ----------------------------------------------------------------------------------------------------

	#[allow(unused_variables)]
	impl<T: Config> Vamm for Pallet<T> {
		type Balance = T::Balance;
		type Decimal = T::Decimal;
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
		) -> Result<Self::Decimal, DispatchError> {
			todo!()
		}

		fn get_twap(vamm: &Self::VammId) -> Result<Self::Decimal, DispatchError> {
			if let Some(twap) = Self::_twap_of(vamm) {
				Ok(twap)
			} else if let Some(twap) = Self::hardcoded_twap() {
				Ok(twap)
			} else {
				Err(Error::<T>::FailedToCalculateTwap.into())
			}
		}

		fn swap(config: &Self::SwapConfig) -> Result<Self::Balance, DispatchError> {
			if let Some(price) = Self::_price_of(&config.vamm_id) {
				Self::get_value(config.input_amount, &config.asset, price)
			} else if let Some(price) = Self::_price() {
				Self::get_value(config.input_amount, &config.asset, price)
			} else {
				Err(Error::<T>::FailedToExecuteSwap.into())
			}
		}

		fn swap_simulation(
			config: &Self::SwapSimulationConfig,
		) -> Result<Self::Balance, DispatchError> {
			let Self::SwapSimulationConfig { vamm_id, asset, input_amount, direction } =
				config.clone();
			<Self as Vamm>::swap(&Self::SwapConfig {
				vamm_id,
				asset,
				input_amount,
				direction,
				output_amount_limit: 0_u32.into(),
			})
			.map_err(|_| Error::<T>::FailedToSimulateSwap.into())
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Helper Functions
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		pub fn set_price(price: Option<T::Decimal>) {
			Price::<T>::set(price)
		}

		pub fn set_price_of(vamm_id: &T::VammId, price: Option<T::Decimal>) {
			if let Some(p) = price {
				Prices::<T>::insert(vamm_id, p);
			} else {
				Prices::<T>::remove(vamm_id);
			}
		}

		pub fn set_twap(twap: Option<T::Decimal>) {
			Twap::<T>::set(twap)
		}

		pub fn set_twap_of(vamm_id: &T::VammId, twap: Option<T::Decimal>) {
			if let Some(t) = twap {
				Twaps::<T>::insert(vamm_id, t);
			} else {
				Twaps::<T>::remove(vamm_id);
			}
		}

		pub fn get_value(
			amount: T::Balance,
			asset_type: &AssetType,
			price: T::Decimal,
		) -> Result<T::Balance, DispatchError> {
			let amount_decimal = T::Decimal::from_inner(amount);
			Ok(match asset_type {
				AssetType::Base => price.saturating_mul(amount_decimal),
				AssetType::Quote =>
					amount_decimal.checked_div(&price).ok_or(ArithmeticError::DivisionByZero)?,
			}
			.into_inner())
		}
	}
}
