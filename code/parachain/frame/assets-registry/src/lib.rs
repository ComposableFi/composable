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

mod prelude;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use crate::prelude::*;
	pub use crate::weights::WeightInfo;
	use codec::FullCodec;
	use composable_traits::{
		assets::{
			Asset, AssetMetadata, AssetType, AssetTypeInspect, InspectRegistryMetadata,
			LocalOrForeignAssetId, MutateRegistryMetadata,
		},
		currency::{BalanceLike, CurrencyFactory, Exponent, ForeignByNative},
		xcm::assets::{RemoteAssetRegistryInspect, RemoteAssetRegistryMutate},
	};
	use cumulus_primitives_core::ParaId;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::EnsureOrigin, BoundedVec,
		Twox128,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_std::{fmt::Debug, str, vec::Vec};

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
			+ Ord
			+ TypeInfo
			+ MaxEncodedLen;

		/// Identifier for the class of foreign asset.
		type ForeignAssetId: FullCodec
			+ Eq
			+ PartialEq
			+ MaybeSerializeDeserialize
			+ Debug
			+ Clone
			+ Default
			+ TypeInfo
			+ MaxEncodedLen;

		/// The origin which may set local and foreign admins.
		type UpdateAssetRegistryOrigin: EnsureOrigin<Self::Origin>;

		/// really can be governance of this chain or remote parachain origin
		type ParachainOrGovernanceOrigin: EnsureOrigin<Self::Origin>;

		type WeightInfo: WeightInfo;

		type Balance: BalanceLike;

		type CurrencyFactory: CurrencyFactory<AssetId = Self::LocalAssetId, Balance = Self::Balance>;

		/// Maximum number of characters allowed in an asset symbol
		#[pallet::constant]
		type AssetSymbolMaxChars: Get<u32>;

		/// Maximum number of characters allowed in an asset name
		#[pallet::constant]
		type AssetNameMaxChars: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Mapping local asset to foreign asset.
	#[pallet::storage]
	#[pallet::getter(fn from_local_asset)]
	pub type LocalToForeign<T: Config> =
		StorageMap<_, Twox128, T::LocalAssetId, T::ForeignAssetId, OptionQuery>;

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
	pub type AssetRatio<T: Config> = StorageMap<_, Twox128, T::LocalAssetId, Rational, OptionQuery>;

	/// Metadata of an asset
	#[pallet::storage]
	#[pallet::getter(fn decimals)]
	pub type Metadata<T: Config> = StorageMap<
		_,
		Twox128,
		T::LocalAssetId,
		AssetMetadata<BoundedVec<u8, T::AssetNameMaxChars>, BoundedVec<u8, T::AssetSymbolMaxChars>>,
		OptionQuery,
	>;

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
			location: Option<T::ForeignAssetId>,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: u8,
			ratio: Rational,
		},
		AssetLocationUpdated {
			asset_id: T::LocalAssetId,
			location: T::ForeignAssetId,
		},
		AssetRatioUpdated {
			asset_id: T::LocalAssetId,
			ratio: Option<Rational>,
		},
		AssetMetadataUpdated {
			asset_id: T::LocalAssetId,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: u8,
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
		StringExceedsMaxLength,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates an asset.
		///
		/// # Parameters:
		///
		/// * `location` - Foreign asset location, use `None` when registering local assets
		///
		/// * `ratio` -
		/// Allows `bring you own gas` fees.
		/// Set to `None` to prevent payment in this asset, only transferring.
		/// Setting to some will NOT start minting tokens with specified ratio.
		///
		/// ```python
		///  ratio = foreign_token / native_token
		///  amount_of_foreign_asset = amount_of_native_asset * ratio
		/// ```
		///
		/// * `name` - Name of the asset
		///
		/// * symbol - Symbol of the asset
		///
		/// * decimals - The number of decimals this asset uses to represent one unit
		///
		/// # Emmits
		/// * `AssetRegistered`
		#[pallet::weight(<T as Config>::WeightInfo::register_asset())]
		pub fn register_asset(
			origin: OriginFor<T>,
			local_or_foreign: LocalOrForeignAssetId<T::LocalAssetId, T::ForeignAssetId>,
			ratio: Rational,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: Exponent,
		) -> DispatchResult {
			T::UpdateAssetRegistryOrigin::ensure_origin(origin)?;

			let (asset_id, location) = match local_or_foreign {
				LocalOrForeignAssetId::Local(asset_id) => (asset_id, None),
				LocalOrForeignAssetId::Foreign(location) => (
					T::LocalAssetId::from(u128::from_be_bytes(sp_core::blake2_128(
						&location.encode(),
					))),
					Some(location),
				),
			};

			if let Some(location) = location.clone() {
				ensure!(
					!ForeignToLocal::<T>::contains_key(&location),
					Error::<T>::ForeignAssetAlreadyRegistered
				);
			}

			<Self as RemoteAssetRegistryMutate>::register_asset(
				asset_id, location, ratio, name, symbol, decimals,
			)?;
			Ok(())
		}

		/// Update the location of a foreign asset.
		///
		/// Emmits:
		/// * `AssetLocationUpdated`
		#[pallet::weight(<T as Config>::WeightInfo::update_asset())]
		pub fn update_asset_location(
			origin: OriginFor<T>,
			asset_id: T::LocalAssetId,
			location: T::ForeignAssetId,
		) -> DispatchResultWithPostInfo {
			T::UpdateAssetRegistryOrigin::ensure_origin(origin)?;
			Self::set_reserve_location(asset_id, location.clone())?;
			Ok(().into())
		}

		/// Update the ratio of an asset.
		///
		/// Emmits:
		/// * `AssetRatioUpdated`
		#[pallet::weight(10_000)]
		pub fn update_asset_ratio(
			origin: OriginFor<T>,
			asset_id: T::LocalAssetId,
			ratio: Option<Rational>,
		) -> DispatchResult {
			T::UpdateAssetRegistryOrigin::ensure_origin(origin)?;
			Self::update_ratio(asset_id, ratio)?;
			Ok(())
		}

		/// Update the metadata of an asset.
		///
		/// All metadata feilds are optional, only those provided as `Some` will be updated, the
		/// rest will be unchanged.
		///
		/// # Parameters:
		/// * `name` - Name of the asset
		/// * symbol - Symbol of the asset
		/// * decimals - The number of decimals this asset uses to represent one unit
		///
		/// Emmits:
		/// * `AssetMetadataUpdated`
		#[pallet::weight(10_000)]
		pub fn update_asset_metadata(
			origin: OriginFor<T>,
			asset_id: T::LocalAssetId,
			name: Option<Vec<u8>>,
			symbol: Option<Vec<u8>>,
			decimals: Option<u8>,
		) -> DispatchResult {
			T::UpdateAssetRegistryOrigin::ensure_origin(origin)?;
			<Self as MutateRegistryMetadata>::update_metadata(&asset_id, name, symbol, decimals)?;
			Ok(())
		}

		/// Minimal amount of `foreign_asset_id` required to send message to other network.
		/// Target network may or may not accept payment `amount`.
		/// Assumed this is maintained up to date by technical team.
		/// Mostly UI hint and fail fast solution.
		/// Messages sending smaller fee will not be sent.
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

		fn register_asset(
			asset_id: Self::AssetId,
			location: Option<Self::AssetNativeLocation>,
			ratio: Rational,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: u8,
		) -> DispatchResult {
			if let Some(location) = location.clone() {
				Self::set_reserve_location(asset_id, location)?;
			}

			Self::update_ratio(asset_id, Some(ratio))?;
			<Self as MutateRegistryMetadata>::set_metadata(
				&asset_id,
				name.clone(),
				symbol.clone(),
				decimals,
			)?;
			Self::deposit_event(Event::<T>::AssetRegistered {
				asset_id,
				location,
				name,
				symbol,
				decimals,
				ratio,
			});
			Ok(())
		}

		fn set_reserve_location(
			asset_id: Self::AssetId,
			location: Self::AssetNativeLocation,
		) -> DispatchResult {
			ForeignToLocal::<T>::insert(&location, asset_id);
			LocalToForeign::<T>::insert(asset_id, location.clone());
			Self::deposit_event(Event::AssetLocationUpdated { asset_id, location });
			Ok(())
		}

		fn update_ratio(asset_id: Self::AssetId, ratio: Option<Rational>) -> DispatchResult {
			AssetRatio::<T>::mutate_exists(asset_id, |x| *x = ratio);
			Self::deposit_event(Event::AssetRatioUpdated { asset_id, ratio });
			Ok(())
		}
	}

	impl<T: Config> RemoteAssetRegistryInspect for Pallet<T> {
		type AssetId = T::LocalAssetId;
		type AssetNativeLocation = T::ForeignAssetId;
		type Balance = T::Balance;

		fn asset_to_remote(asset_id: Self::AssetId) -> Option<Self::AssetNativeLocation> {
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

		fn get_foreign_assets_list() -> Vec<Asset<T::Balance, Self::AssetNativeLocation>> {
			ForeignToLocal::<T>::iter()
				.map(|(_, asset_id)| {
					let foreign_id = LocalToForeign::<T>::get(asset_id);
					let decimals =
						<Pallet<T> as InspectRegistryMetadata>::decimals(&asset_id).unwrap_or(12);
					let ratio = AssetRatio::<T>::get(asset_id);

					Asset {
						name: None,
						id: asset_id.into(),
						decimals,
						ratio,
						foreign_id,
						existential_deposit: T::Balance::default(),
					}
				})
				.collect::<Vec<_>>()
		}
	}

	impl<T: Config> AssetRatioInspect for Pallet<T> {
		type AssetId = T::LocalAssetId;
		fn get_ratio(asset_id: Self::AssetId) -> Option<ForeignByNative> {
			AssetRatio::<T>::get(asset_id).map(Into::into)
		}
	}

	impl<T: Config> InspectRegistryMetadata for Pallet<T> {
		type AssetId = T::LocalAssetId;

		fn asset_name(asset_id: &Self::AssetId) -> Option<Vec<u8>> {
			Metadata::<T>::get(asset_id).map(|metadata| metadata.name.to_vec())
		}

		fn symbol(asset_id: &Self::AssetId) -> Option<Vec<u8>> {
			Metadata::<T>::get(asset_id).map(|metadata| metadata.symbol.to_vec())
		}

		fn decimals(asset_id: &Self::AssetId) -> Option<u8> {
			Metadata::<T>::get(asset_id).map(|metadata| metadata.decimals)
		}
	}

	impl<T: Config> MutateRegistryMetadata for Pallet<T> {
		type AssetId = T::LocalAssetId;
		type BoundedName = BoundedVec<u8, T::AssetNameMaxChars>;
		type BoundedSymbol = BoundedVec<u8, T::AssetSymbolMaxChars>;

		fn set_metadata(
			asset_id: &Self::AssetId,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: u8,
		) -> DispatchResult {
			let name = Self::BoundedName::try_from(name)
				.map_err(|_| Error::<T>::StringExceedsMaxLength)?;
			let symbol = Self::BoundedSymbol::try_from(symbol)
				.map_err(|_| Error::<T>::StringExceedsMaxLength)?;
			Metadata::<T>::insert(asset_id, AssetMetadata { name, symbol, decimals });
			Ok(())
		}

		fn update_metadata(
			asset_id: &Self::AssetId,
			name: Option<Vec<u8>>,
			symbol: Option<Vec<u8>>,
			decimals: Option<u8>,
		) -> DispatchResult {
			let name = name
				.map(BoundedVec::<u8, T::AssetNameMaxChars>::try_from)
				.transpose()
				.map_err(|_| Error::<T>::StringExceedsMaxLength)?;
			let symbol = symbol
				.map(BoundedVec::<u8, T::AssetSymbolMaxChars>::try_from)
				.transpose()
				.map_err(|_| Error::<T>::StringExceedsMaxLength)?;
			Metadata::<T>::mutate_exists(asset_id, |metadata| {
				if let Some(metadata) = metadata {
					let name = name.unwrap_or(metadata.clone().name);
					let symbol = symbol.unwrap_or(metadata.clone().symbol);
					let decimals = decimals.unwrap_or(metadata.decimals);

					*metadata = AssetMetadata { name, symbol, decimals };
					Self::deposit_event(Event::AssetMetadataUpdated {
						asset_id: *asset_id,
						name: metadata.name.to_vec(),
						symbol: metadata.symbol.to_vec(),
						decimals: metadata.decimals,
					});
				}
				None
			})
			.ok_or_else(|| Error::<T>::AssetNotFound.into())
		}
	}

	impl<T: Config> AssetTypeInspect for Pallet<T> {
		type AssetId = T::LocalAssetId;

		fn inspect(asset: &Self::AssetId) -> AssetType {
			if LocalToForeign::<T>::contains_key(asset) {
				AssetType::Foreign
			} else {
				AssetType::Local
			}
		}
	}
}
