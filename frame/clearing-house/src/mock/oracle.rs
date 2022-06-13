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
		type Balance: Clone
			+ From<u64>
			+ FullCodec
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ TypeInfo;
		type Timestamp: Default;
		type LocalAssets: LocalAssets<Self::AssetId>;
		type MaxAnswerBound: Get<u32>;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                            Genesis Configuration
	// ----------------------------------------------------------------------------------------------------

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub price: Option<T::Balance>,
		pub supports_assets: Option<bool>,
		pub twap: Option<T::Balance>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				price: Some(100_u64.into()),
				supports_assets: Some(true),
				twap: Some(100_u64.into()),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(price) = self.price.clone() {
				MockPrice::<T>::set(Some(price));
			}

			SupportsAssets::<T>::set(self.supports_assets);

			if let Some(twap) = self.twap.clone() {
				Twap::<T>::set(Some(twap));
			}
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Errors
	// ----------------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		CantCheckAssetSupport,
		CantComputePrice,
		CantComputeTwap,
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Storage
	// ----------------------------------------------------------------------------------------------------

	#[pallet::storage]
	#[pallet::getter(fn _price)]
	pub type MockPrice<T: Config> = StorageValue<_, T::Balance, OptionQuery>;

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
			Ok(Price::<Self::Balance, Self::Timestamp> {
				price: Self::_price().ok_or(Error::<T>::CantComputePrice)?,
				block: Self::Timestamp::default(),
			})
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
		pub fn set_price(price: Option<T::Balance>) {
			MockPrice::<T>::set(price);
		}

		pub fn set_twap(twap: Option<T::Balance>) {
			Twap::<T>::set(twap);
		}
	}
}
