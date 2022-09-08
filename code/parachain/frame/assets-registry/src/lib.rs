#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![deny(clippy::unseparated_literal_suffix, unused_imports, dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub use pallet::*;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
#[cfg(test)]
mod runtime;

#[cfg(test)]
mod tests;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use codec::FullCodec;
	use composable_traits::{
		currency::{BalanceLike, CurrencyFactory, Exponent, RangeId},
		defi::Ratio,
		xcm::assets::{
			AssetRatioInspect, ForeignMetadata, RemoteAssetRegistryInspect,
			RemoteAssetRegistryMutate,
		},
	};
	use cumulus_primitives_core::ParaId;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::EnsureOrigin,
	};

	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_std::{fmt::Debug, str};

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Identifier for the class of local asset.
		type LocalAssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ From<u128>
			+ Into<u128>
			+ Debug
			+ Default
			+ TypeInfo;

		/// Identifier for the class of foreign asset.
		type ForeignAssetId: FullCodec
			+ Eq
			+ PartialEq
			+ MaybeSerializeDeserialize
			+ Debug
			+ Clone
			+ Default
			+ TypeInfo;

		/// The origin which may set local and foreign admins.
		type UpdateAssetRegistryOrigin: EnsureOrigin<Self::Origin>;
		/// really can be governance of this chain or remote parachain origin
		type ParachainOrGovernanceOrigin: EnsureOrigin<Self::Origin>;
		type WeightInfo: WeightInfo;
		type Balance: BalanceLike;
		type CurrencyFactory: CurrencyFactory<Self::LocalAssetId, Self::Balance>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Mapping local asset to foreign asset.
	#[pallet::storage]
	#[pallet::getter(fn from_local_asset)]
	pub type LocalToForeign<T: Config> =
		StorageMap<_, Twox128, T::LocalAssetId, ForeignMetadata<T::ForeignAssetId>, OptionQuery>;

	/// Mapping foreign asset to local asset.
	#[pallet::storage]
	#[pallet::getter(fn from_foreign_asset)]
	pub type ForeignToLocal<T: Config> =
		StorageMap<_, Blake2_128Concat, T::ForeignAssetId, T::LocalAssetId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn minimal_amount)]
	pub type MinFeeAmounts<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		ParaId,
		Blake2_128Concat,
		T::ForeignAssetId,
		T::Balance,
		OptionQuery,
	>;

	/// How much of asset amount is needed to pay for one unit of native token.
	#[pallet::storage]
	#[pallet::getter(fn asset_ratio)]
	pub type AssetRatio<T: Config> = StorageMap<_, Twox128, T::LocalAssetId, Ratio, OptionQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config>(sp_std::marker::PhantomData<T>);

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self(<_>::default())
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T>
	where
		composable_traits::xcm::assets::XcmAssetLocation:
			codec::EncodeLike<<T as Config>::ForeignAssetId>,
	{
		fn build(&self) {}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AssetRegistered {
			asset_id: T::LocalAssetId,
			location: T::ForeignAssetId,
		},
		AssetUpdated {
			asset_id: T::LocalAssetId,
			location: T::ForeignAssetId,
		},
		MinFeeUpdated {
			target_parachain_id: ParaId,
			foreign_asset_id: T::ForeignAssetId,
			amount: Option<T::Balance>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		AssetNotFound,
		ForeignAssetAlreadyRegistered,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates asset using `CurrencyFactory`.
		/// Raises `AssetRegistered` event
		///
		/// Sets only required fields by `CurrencyFactory`, to upsert metadata use referenced
		/// pallet.
		///
		/// # Parameters:
		///
		/// `ratio` -  allows `bring you own gas` fees.
		/// Set to `None` to prevent payment in this asset, only transferring.
		/// Setting to some will NOT start minting tokens with specified ratio.
		/// Foreign assets will be put into parachain treasury as is.
		///
		/// ```python
		/// # if cross chain message wants to pay tx fee with non native token
		/// # then amount of native token would be:
		/// amount_of_native_token = amount_of_foreign_token * ratio
		/// ```
		///
		/// Examples:
		///  
		/// - One to one conversion is 10^18 integer.
		///
		/// - 10*10^18 will tell that for 1 foreign asset can `buy` 10 local native.
		///
		/// `decimals` - remote number of decimals on other(remote) chain
		///
		/// `ed` - same meaning as in `CurrencyFactory`
		#[pallet::weight(<T as Config>::WeightInfo::register_asset())]
		pub fn register_asset(
			origin: OriginFor<T>,
			location: T::ForeignAssetId,
			ed: T::Balance,
			ratio: Option<Ratio>,
			decimals: Option<Exponent>,
		) -> DispatchResultWithPostInfo {
			T::UpdateAssetRegistryOrigin::ensure_origin(origin)?;
			ensure!(
				!ForeignToLocal::<T>::contains_key(&location),
				Error::<T>::ForeignAssetAlreadyRegistered
			);
			let asset_id = T::CurrencyFactory::create(RangeId::FOREIGN_ASSETS, ed)?;
			Self::set_reserve_location(asset_id, location.clone(), ratio, decimals)?;
			Self::deposit_event(Event::<T>::AssetRegistered { asset_id, location });
			Ok(().into())
		}

		/// Given well existing asset, update its remote information.
		/// Use with caution as it allow reroute assets location.
		/// See `register_asset` for parameters meaning.
		#[pallet::weight(<T as Config>::WeightInfo::update_asset())]
		pub fn update_asset(
			origin: OriginFor<T>,
			asset_id: T::LocalAssetId,
			location: T::ForeignAssetId,
			ratio: Option<Ratio>,
			decimals: Option<Exponent>,
		) -> DispatchResultWithPostInfo {
			T::UpdateAssetRegistryOrigin::ensure_origin(origin)?;
			// note: does not validates if assets exists, not clear what is expected in this case
			// TODO: after compile time well known assets allow to check existence, add ensure
			// clause for that
			Self::set_reserve_location(asset_id, location.clone(), ratio, decimals)?;
			Self::deposit_event(Event::<T>::AssetUpdated { asset_id, location });
			Ok(().into())
		}

		/// Minimal amount of asset_id required to send message to other network.
		/// Target network may or may not accept payment.
		/// Assumed this is maintained up to date by technical team.
		/// Mostly UI hint and fail fast solution.
		/// In theory can be updated by parachain sovereign account too.
		/// If None, than it is well known cannot pay with that asset on target_parachain_id.
		/// If Some(0), than price can be anything greater or equal to zero.
		/// If Some(MAX), than actually it forbids transfers.
		#[pallet::weight(<T as Config>::WeightInfo::set_min_fee())]
		pub fn set_min_fee(
			origin: OriginFor<T>,
			target_parachain_id: ParaId,
			foreign_asset_id: T::ForeignAssetId,
			amount: Option<T::Balance>,
		) -> DispatchResultWithPostInfo {
			T::ParachainOrGovernanceOrigin::ensure_origin(origin)?;
			// TODO: in case it is set to parachain, check that chain can target only its origin
			MinFeeAmounts::<T>::mutate_exists(target_parachain_id, foreign_asset_id.clone(), |x| {
				*x = amount
			});
			Self::deposit_event(Event::<T>::MinFeeUpdated {
				target_parachain_id,
				foreign_asset_id,
				amount,
			});
			Ok(().into())
		}
	}

	impl<T: Config> RemoteAssetRegistryMutate for Pallet<T> {
		type AssetId = T::LocalAssetId;

		type AssetNativeLocation = T::ForeignAssetId;

		type Balance = T::Balance;

		fn set_reserve_location(
			asset_id: Self::AssetId,
			location: Self::AssetNativeLocation,
			ratio: Option<Ratio>,
			decimals: Option<Exponent>,
		) -> DispatchResult {
			ForeignToLocal::<T>::insert(&location, asset_id);
			LocalToForeign::<T>::insert(asset_id, ForeignMetadata { decimals, location });
			AssetRatio::<T>::mutate_exists(asset_id, |x| *x = ratio);
			Ok(())
		}

		fn update_ratio(
			location: Self::AssetNativeLocation,
			ratio: Option<Ratio>,
		) -> DispatchResult {
			let asset_id =
				ForeignToLocal::<T>::try_get(location).map_err(|_| Error::<T>::AssetNotFound)?;
			AssetRatio::<T>::mutate_exists(asset_id, |x| *x = ratio);
			Ok(())
		}
	}

	impl<T: Config> RemoteAssetRegistryInspect for Pallet<T> {
		type AssetId = T::LocalAssetId;
		type AssetNativeLocation = T::ForeignAssetId;
		type Balance = T::Balance;

		fn asset_to_remote(
			asset_id: Self::AssetId,
		) -> Option<composable_traits::xcm::assets::ForeignMetadata<Self::AssetNativeLocation>> {
			LocalToForeign::<T>::get(asset_id)
		}

		fn location_to_asset(location: Self::AssetNativeLocation) -> Option<Self::AssetId> {
			ForeignToLocal::<T>::get(location)
		}

		fn min_xcm_fee(
			parachain_id: ParaId,
			remote_asset_id: Self::AssetNativeLocation,
		) -> Option<Self::Balance> {
			<MinFeeAmounts<T>>::get(parachain_id, remote_asset_id)
		}
	}

	impl<T: Config> AssetRatioInspect for Pallet<T> {
		type AssetId = T::LocalAssetId;
		fn get_ratio(asset_id: Self::AssetId) -> Option<composable_traits::defi::Ratio> {
			AssetRatio::<T>::get(asset_id)
		}
	}
}
