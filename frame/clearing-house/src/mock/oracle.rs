pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies
	// ----------------------------------------------------------------------------------------------------

	use codec::FullCodec;
	use composable_traits::{
		currency::LocalAssets,
		defi::{CurrencyPair, Ratio},
		oracle::{Oracle, Price},
	};
	use frame_support::pallet_prelude::*;

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
		type AssetId: Copy;
		type Balance: From<u64> + FullCodec + MaxEncodedLen + TypeInfo;
		type Timestamp;
		type LocalAssets: LocalAssets<Self::AssetId>;
		type MaxAnswerBound: Get<u32>;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                            Genesis Configuration
	// ----------------------------------------------------------------------------------------------------

	#[pallet::genesis_config]
	#[derive(Default)]
	pub struct GenesisConfig {
		pub supports_assets: Option<bool>,
		pub twap: Option<u64>,
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			SupportsAssets::<T>::set(self.supports_assets);
			if let Some(twap) = self.twap {
				Twap::<T>::set(Some(twap.into()));
			}
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Errors
	// ----------------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		CantCheckAssetSupport,
		CantComputeTwap,
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Storage
	// ----------------------------------------------------------------------------------------------------

	#[pallet::storage]
	#[pallet::getter(fn supports_assets)]
	pub type SupportsAssets<T: Config> = StorageValue<_, bool, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn hardcoded_twap)]
	pub type Twap<T: Config> = StorageValue<_, T::Balance, OptionQuery>;

	// ----------------------------------------------------------------------------------------------------
	//                                           Trait Implementations
	// ----------------------------------------------------------------------------------------------------

	#[allow(unused_variables)]
	impl<T: Config> Oracle for Pallet<T> {
		type AssetId = T::AssetId;
		type Balance = T::Balance;
		type Timestamp = T::Timestamp;
		type LocalAssets = T::LocalAssets;
		type MaxAnswerBound = T::MaxAnswerBound;

		fn is_supported(asset: Self::AssetId) -> Result<bool, DispatchError> {
			if let Some(support) = Self::supports_assets() {
				Ok(support)
			} else {
				Err(Error::<T>::CantCheckAssetSupport.into())
			}
		}

		fn get_price(
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Price<Self::Balance, Self::Timestamp>, DispatchError> {
			unimplemented!()
		}

		fn get_twap(
			of: Self::AssetId,
			weighting: Vec<Self::Balance>,
		) -> Result<Self::Balance, DispatchError> {
			if let Some(twap) = Self::hardcoded_twap() {
				Ok(twap)
			} else {
				Err(Error::<T>::CantComputeTwap.into())
			}
		}

		fn get_ratio(pair: CurrencyPair<Self::AssetId>) -> Result<Ratio, DispatchError> {
			unimplemented!()
		}

		fn get_price_inverse(
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			unimplemented!()
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Helper Functions
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		pub fn set_twap(twap: Option<T::Balance>) {
			Twap::<T>::set(twap);
		}
	}
}
