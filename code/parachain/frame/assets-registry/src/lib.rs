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
	use composable_support::math::safe::safe_multiply_by_rational;
	use composable_traits::{
		assets::{
			Asset, AssetInfo, AssetInfoUpdate, AssetType, AssetTypeInspect, BiBoundedAssetName,
			BiBoundedAssetSymbol, GenerateAssetId, InspectRegistryMetadata, MutateRegistryMetadata,
		},
		currency::{AssetExistentialDepositInspect, BalanceLike, ForeignByNative},
		storage::UpdateValue,
		xcm::assets::{RemoteAssetRegistryInspect, RemoteAssetRegistryMutate},
	};
	use cumulus_primitives_core::ParaId;
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{tokens::BalanceConversion, EnsureOrigin},
		Twox128,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_runtime::traits::Convert;
	use sp_std::{borrow::ToOwned, fmt::Debug, str, vec::Vec};

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

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

		type UpdateAssetRegistryOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// really can be governance of this chain or remote parachain origin
		type ParachainOrGovernanceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		type WeightInfo: WeightInfo;

		type Balance: BalanceLike;

		/// An isomorphism: Balance<->u128
		type Convert: Convert<u128, Self::Balance> + Convert<Self::Balance, u128>;
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

	/// The minimum balance of an asset required for the balance to be stored on chain
	#[pallet::storage]
	#[pallet::getter(fn existential_deposit)]
	pub type ExistentialDeposit<T: Config> =
		StorageMap<_, Twox128, T::LocalAssetId, T::Balance, OptionQuery>;

	/// Name of an asset
	#[pallet::storage]
	#[pallet::getter(fn asset_name)]
	pub type AssetName<T: Config> =
		StorageMap<_, Twox128, T::LocalAssetId, BiBoundedAssetName, OptionQuery>;

	/// Symbol of an asset
	#[pallet::storage]
	#[pallet::getter(fn asset_symbol)]
	pub type AssetSymbol<T: Config> =
		StorageMap<_, Twox128, T::LocalAssetId, BiBoundedAssetSymbol, OptionQuery>;

	/// Decimals of an asset
	#[pallet::storage]
	#[pallet::getter(fn asset_decimals)]
	pub type AssetDecimals<T: Config> =
		StorageMap<_, Twox128, T::LocalAssetId, Exponent, OptionQuery>;

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
			asset_info: AssetInfo<T::Balance>,
		},
		AssetUpdated {
			asset_id: T::LocalAssetId,
			asset_info: AssetInfoUpdate<T::Balance>,
		},
		AssetLocationUpdated {
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
		AssetAlreadyRegistered,
		StringExceedsMaxLength,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates an asset.
		///
		/// # Parameters:
		///
		/// * `local_or_foreign` - Foreign asset location or unused local asset ID
		///
		/// * `asset_info` - Information to register the asset with, see [`AssetInfo`]
		///
		/// # Emits
		/// * `AssetRegistered`
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::register_asset())]
		pub fn register_asset(
			origin: OriginFor<T>,
			protocol_id: [u8; 8],
			nonce: u64,
			location: Option<T::ForeignAssetId>,
			asset_info: AssetInfo<T::Balance>,
		) -> DispatchResult {
			T::UpdateAssetRegistryOrigin::ensure_origin(origin)?;

			let asset_id = Self::generate_asset_id(protocol_id, nonce);

			ensure!(
				!ExistentialDeposit::<T>::contains_key(asset_id),
				Error::<T>::AssetAlreadyRegistered
			);

			if let Some(location) = location.clone() {
				ensure!(
					!ForeignToLocal::<T>::contains_key(&location),
					Error::<T>::AssetAlreadyRegistered
				);
			}

			<Self as RemoteAssetRegistryMutate>::register_asset(asset_id, location, asset_info)?;
			Ok(())
		}

		/// Update the location of a foreign asset.
		///
		/// Emits:
		/// * `AssetLocationUpdated`
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::update_asset_location())]
		pub fn update_asset_location(
			origin: OriginFor<T>,
			asset_id: T::LocalAssetId,
			location: T::ForeignAssetId,
		) -> DispatchResultWithPostInfo {
			T::UpdateAssetRegistryOrigin::ensure_origin(origin)?;
			Self::set_reserve_location(asset_id, location)?;
			Ok(().into())
		}

		/// Update stored asset information.
		///
		/// Emits:
		/// * `AssetUpdated`
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::update_asset())]
		pub fn update_asset(
			origin: OriginFor<T>,
			asset_id: T::LocalAssetId,
			asset_info: AssetInfoUpdate<T::Balance>,
		) -> DispatchResult {
			T::UpdateAssetRegistryOrigin::ensure_origin(origin)?;
			<Self as RemoteAssetRegistryMutate>::update_asset(asset_id, asset_info)
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
		#[pallet::call_index(4)]
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
			asset_info: AssetInfo<Self::Balance>,
		) -> DispatchResult {
			if ExistentialDeposit::<T>::contains_key(asset_id) {
				return Err(Error::<T>::AssetAlreadyRegistered.into())
			}

			if let Some(location) = location.clone() {
				Self::set_reserve_location(asset_id, location)?;
			}

			AssetRatio::<T>::set(asset_id, asset_info.ratio);
			<Self as MutateRegistryMetadata>::set_metadata(
				&asset_id,
				asset_info.name.clone(),
				asset_info.symbol.clone(),
				asset_info.decimals,
			)?;
			ExistentialDeposit::<T>::set(asset_id, Some(asset_info.existential_deposit));

			Self::deposit_event(Event::<T>::AssetRegistered { asset_id, location, asset_info });
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

		fn update_asset(
			asset_id: Self::AssetId,
			asset_info: AssetInfoUpdate<Self::Balance>,
		) -> DispatchResult {
			Self::update_metadata(
				&asset_id,
				asset_info.name.clone(),
				asset_info.symbol.clone(),
				asset_info.decimals.clone(),
			)?;

			if let UpdateValue::Set(ed) = asset_info.existential_deposit {
				ExistentialDeposit::<T>::set(asset_id, Some(ed));
			}

			if let UpdateValue::Set(ratio) = asset_info.ratio {
				AssetRatio::<T>::set(asset_id, ratio);
			}

			Self::deposit_event(Event::AssetUpdated { asset_id, asset_info });

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
					let existential_deposit =
						ExistentialDeposit::<T>::get(asset_id).unwrap_or_default();

					Asset {
						name: None,
						id: asset_id.into(),
						decimals,
						ratio,
						foreign_id,
						existential_deposit,
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

	impl<T: Config> AssetExistentialDepositInspect for Pallet<T> {
		type AssetId = T::LocalAssetId;
		type Balance = T::Balance;

		fn existential_deposit(asset_id: Self::AssetId) -> Result<Self::Balance, DispatchError> {
			ExistentialDeposit::<T>::get(asset_id).ok_or_else(|| Error::<T>::AssetNotFound.into())
		}
	}

	impl<T: Config> InspectRegistryMetadata for Pallet<T> {
		type AssetId = T::LocalAssetId;

		fn asset_name(asset_id: &Self::AssetId) -> Option<Vec<u8>> {
			AssetName::<T>::get(asset_id).map(|name| name.as_vec().to_owned())
		}

		fn symbol(asset_id: &Self::AssetId) -> Option<Vec<u8>> {
			AssetSymbol::<T>::get(asset_id).map(|symbol| symbol.as_vec().to_owned())
		}

		fn decimals(asset_id: &Self::AssetId) -> Option<u8> {
			AssetDecimals::<T>::get(asset_id)
		}
	}

	impl<T: Config> MutateRegistryMetadata for Pallet<T> {
		type AssetId = T::LocalAssetId;

		fn set_metadata(
			asset_id: &Self::AssetId,
			name: Option<BiBoundedAssetName>,
			symbol: Option<BiBoundedAssetSymbol>,
			decimals: Option<u8>,
		) -> DispatchResult {
			AssetName::<T>::set(asset_id, name);
			AssetSymbol::<T>::set(asset_id, symbol);
			AssetDecimals::<T>::set(asset_id, decimals);
			Ok(())
		}

		fn update_metadata(
			asset_id: &Self::AssetId,
			name: UpdateValue<Option<BiBoundedAssetName>>,
			symbol: UpdateValue<Option<BiBoundedAssetSymbol>>,
			decimals: UpdateValue<Option<u8>>,
		) -> DispatchResult {
			if let UpdateValue::Set(name) = name {
				AssetName::<T>::set(asset_id, name);
			}
			if let UpdateValue::Set(symbol) = symbol {
				AssetSymbol::<T>::set(asset_id, symbol);
			}
			if let UpdateValue::Set(decimals) = decimals {
				AssetDecimals::<T>::set(asset_id, decimals);
			}

			Ok(())
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

	impl<T: Config> BalanceConversion<T::Balance, T::LocalAssetId, T::Balance> for Pallet<T> {
		type Error = DispatchError;

		fn to_asset_balance(
			native_amount: T::Balance,
			asset_id: T::LocalAssetId,
		) -> Result<T::Balance, Self::Error> {
			let native_amount = T::Convert::convert(native_amount);
			if let Some(ratio) = Self::get_ratio(asset_id) {
				Ok(safe_multiply_by_rational(native_amount, ratio.n().into(), ratio.d().into())
					.map(T::Convert::convert)?)
			} else {
				Err(Error::<T>::AssetNotFound.into())
			}
		}
	}

	impl<T: Config> GenerateAssetId for Pallet<T> {
		type AssetId = T::LocalAssetId;

		fn generate_asset_id(protocol_id: [u8; 8], nonce: u64) -> Self::AssetId {
			// NOTE: Asset ID generation rational found here:
			// https://github.com/PoisonPhang/composable-poison-fork/blob/INIT-13/rfcs/0013-redesign-assets-id-system.md#local-asset-id-generation
			let bytes = protocol_id
				.into_iter()
				.chain(nonce.to_le_bytes())
				.collect::<Vec<u8>>()
				.try_into()
				.expect("[u8; 8] + bytes(u64) = [u8; 16]");

			Self::AssetId::from(u128::from_le_bytes(bytes))
		}
	}
}
