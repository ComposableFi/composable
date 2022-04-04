pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies
	// ----------------------------------------------------------------------------------------------------

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
		type Balance: From<u64>;
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
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			SupportsAssets::<T>::set(self.supports_assets)
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Errors
	// ----------------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		CantCheckAssetSupport,
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Storage
	// ----------------------------------------------------------------------------------------------------

	#[pallet::storage]
	#[pallet::getter(fn supports_assets)]
	pub type SupportsAssets<T: Config> = StorageValue<_, bool, OptionQuery>;

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
			unimplemented!()
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
}
