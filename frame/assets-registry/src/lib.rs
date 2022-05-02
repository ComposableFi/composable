#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub use pallet::*;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use codec::{EncodeLike, FullCodec};
	use composable_traits::assets::{RemoteAssetRegistry, XcmAssetLocation};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::EnsureOrigin,
	};

	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_std::{fmt::Debug, marker::PhantomData, str};

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
			+ Copy
			+ MaybeSerializeDeserialize
			+ From<u128>
			+ Into<u128>
			+ Debug
			+ Default
			+ TypeInfo;

		/// Location of foreign asset.
		type Location: FullCodec
			+ Eq
			+ PartialEq
			// we wrap non serde type, so until written custom serde, cannot handle that
			// + MaybeSerializeDeserialize
			+ Debug
			+ Clone
			+ Default
			+ TypeInfo;

		/// The origin which may set local and foreign admins.
		type UpdateAdminOrigin: EnsureOrigin<Self::Origin>;

		/// The origin of local admin.
		type LocalAdminOrigin: EnsureOrigin<Self::Origin>;

		/// The origin of foreign admin.
		type ForeignAdminOrigin: EnsureOrigin<Self::Origin>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	/// Statuses of assets mapping candidate.
	#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub enum CandidateStatus {
		LocalAdminApproved,
		ForeignAdminApproved,
	}

	/// A metadata of foreign asset.
	#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct ForeignMetadata {
		pub decimals: u8,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Local admin account.
	#[pallet::storage]
	#[pallet::getter(fn local_admin)]
	pub type LocalAdmin<T: Config> = StorageValue<_, T::AccountId>;

	/// Foreign admin account.
	#[pallet::storage]
	#[pallet::getter(fn foreign_admin)]
	pub type ForeignAdmin<T: Config> = StorageValue<_, T::AccountId>;

	/// Mapping local asset to foreign asset.
	#[pallet::storage]
	#[pallet::getter(fn from_local_asset)]
	pub type LocalToForeign<T: Config> =
		StorageMap<_, Blake2_128Concat, T::LocalAssetId, T::ForeignAssetId, OptionQuery>;

	/// Mapping foreign asset to local asset.
	#[pallet::storage]
	#[pallet::getter(fn from_foreign_asset)]
	pub type ForeignToLocal<T: Config> =
		StorageMap<_, Blake2_128Concat, T::ForeignAssetId, T::LocalAssetId, OptionQuery>;

	/// Mapping (local asset, foreign asset) to a candidate status.
	#[pallet::storage]
	#[pallet::getter(fn assets_mapping_candidates)]
	pub type AssetsMappingCandidates<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::LocalAssetId, T::ForeignAssetId),
		CandidateStatus,
		OptionQuery,
	>;

	/// Mapping foreign asset to foreign location.
	#[pallet::storage]
	#[pallet::getter(fn foreign_asset_location)]
	pub type ForeignAssetLocation<T: Config> =
		StorageMap<_, Blake2_128Concat, T::LocalAssetId, T::Location, OptionQuery>;

	/// Mapping foreign location to foreign asset.
	#[pallet::storage]
	#[pallet::getter(fn from_foreign_asset_location)]
	pub type FromForeignAssetLocation<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Location, T::LocalAssetId, OptionQuery>;

	/// Mapping local asset to foreign asset metadata.
	#[pallet::storage]
	#[pallet::getter(fn foreign_asset_metadata)]
	pub type ForeignAssetMetadata<T: Config> =
		StorageMap<_, Blake2_128Concat, T::LocalAssetId, ForeignMetadata, OptionQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		local_admin: Option<T::AccountId>,
		foreign_admin: Option<T::AccountId>,
		// TODO: split this into 2 pairs
		// 1. (xcm location -> local asset id as used in our runtime), so that when others send our
		// id to our chain we can trust them
		// 2. (local asset id -> remote location -> remote asset id), so then when we send our
		// local asset to remote chain we know what id we should envode.
		asset_pairs: Vec<(T::LocalAssetId, XcmAssetLocation)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				local_admin: Default::default(),
				foreign_admin: Default::default(),
				asset_pairs: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T>
	where
		XcmAssetLocation: EncodeLike<<T as Config>::ForeignAssetId>,
	{
		fn build(&self) {
			if let Some(local_admin) = &self.local_admin {
				<LocalAdmin<T>>::put(local_admin)
			}
			if let Some(foreign_admin) = &self.foreign_admin {
				<ForeignAdmin<T>>::put(foreign_admin)
			}
			for p in &self.asset_pairs {
				<LocalToForeign<T>>::insert(p.0, p.1.to_owned());
				<ForeignToLocal<T>>::insert(p.1.to_owned(), p.0);
			}
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// `Config::UpdateAdminOrigin` set a new local admin.
		LocalAdminUpdated(T::AccountId),
		/// `Config::UpdateAdminOrigin` set a new foreign admin.
		ForeignAdminUpdated(T::AccountId),
		/// Local admin or foreign admin approved an assets mapping candidate.
		AssetsMappingCandidateUpdated {
			local_asset_id: T::LocalAssetId,
			foreign_asset_id: T::ForeignAssetId,
		},
		/// Local admin or foreign admin updated a metadata of foreign asset.
		AssetMetadataUpdated(T::LocalAssetId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Only local admin or foreign admin can do this.
		OnlyAllowedForAdmins,
		/// The local asset Id already used.
		LocalAssetIdAlreadyUsed,
		/// The foreign asset Id already used.
		ForeignAssetIdAlreadyUsed,
		/// The local asset Id not found.
		LocalAssetIdNotFound,
		/// The foreign asset Id not found.
		ForeignAssetIdNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set the given account as local admin.
		///
		/// The dispatch origin for this call is `Config::UpdateAdminOrigin`.
		///
		/// # Emits
		///  - [`Event::LocalAdminUpdated`](Event::LocalAdminUpdated)
		///
		/// # Errors
		///  - `BadOrigin` when the origin isn't `Config::UpdateAdminOrigin`.
		#[pallet::weight(<T as Config>::WeightInfo::set_local_admin())]
		pub fn set_local_admin(
			origin: OriginFor<T>,
			local_admin: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::UpdateAdminOrigin::ensure_origin(origin)?;
			<LocalAdmin<T>>::put(local_admin.clone());
			Self::deposit_event(Event::LocalAdminUpdated(local_admin));
			Ok(().into())
		}

		/// Set the given account as foreign admin.
		///
		/// The dispatch origin for this call is `Config::UpdateAdminOrigin`.
		///
		/// # Emits
		///  - [`Event::ForeignAdminUpdated`](Event::ForeignAdminUpdated)
		///
		/// # Errors
		///  - `BadOrigin` when the origin isn't `Config::UpdateAdminOrigin`.
		#[pallet::weight(<T as Config>::WeightInfo::set_foreign_admin())]
		pub fn set_foreign_admin(
			origin: OriginFor<T>,
			foreign_admin: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::UpdateAdminOrigin::ensure_origin(origin)?;
			<ForeignAdmin<T>>::put(foreign_admin.clone());
			Self::deposit_event(Event::ForeignAdminUpdated(foreign_admin));
			Ok(().into())
		}

		/// Approve an assets mapping candidate.
		///
		/// The dispatch origin for this call is `Config::LocalAdminOrigin` or
		/// `Config::ForeignAdminOrigin`.
		///
		/// # Emits
		///  - [`Event::AssetsMappingCandidateUpdated`](Event::AssetsMappingCandidateUpdated)
		///
		/// # Errors
		/// - `OnlyAllowedForAdmins` when the origin isn't `Config::LocalAdminOrigin` or
		///   `Config::ForeignAdminOrigin`.
		/// - `LocalAssetIdAlreadyUsed` when local_asset_id is already used.
		/// - `ForeignAssetIdAlreadyUsed` when foreign_asset_id is already used.
		#[pallet::weight(<T as Config>::WeightInfo::approve_assets_mapping_candidate())]
		pub fn approve_assets_mapping_candidate(
			origin: OriginFor<T>,
			local_asset_id: T::LocalAssetId,
			foreign_asset_id: T::ForeignAssetId,
			location: T::Location,
			decimals: u8,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin.clone())?;
			Self::ensure_admins_only(origin)?;
			ensure!(
				!<LocalToForeign<T>>::contains_key(local_asset_id),
				Error::<T>::LocalAssetIdAlreadyUsed
			);
			ensure!(
				!<ForeignToLocal<T>>::contains_key(foreign_asset_id),
				Error::<T>::ForeignAssetIdAlreadyUsed
			);
			Self::approve_candidate(who, local_asset_id, foreign_asset_id, location, decimals)?;
			Self::deposit_event(Event::AssetsMappingCandidateUpdated {
				local_asset_id,
				foreign_asset_id,
			});
			Ok(().into())
		}

		/// Update a foreign metadata for an assets mapping.
		///
		/// The dispatch origin for this call is `Config::LocalAdminOrigin` or
		/// `Config::ForeignAdminOrigin`.
		///
		/// # Emits
		///  - [`Event::AssetMetadataUpdated`](Event::AssetMetadataUpdated)
		///
		/// # Errors
		/// - `OnlyAllowedForAdmins` when the origin isn't `Config::LocalAdminOrigin` or
		///   `Config::ForeignAdminOrigin`.
		/// - `LocalAssetIdNotFound` when local_asset_id not found.
		#[pallet::weight(<T as Config>::WeightInfo::set_metadata())]
		pub fn set_metadata(
			origin: OriginFor<T>,
			local_asset_id: T::LocalAssetId,
			metadata: ForeignMetadata,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_signed(origin.clone())?;
			Self::ensure_admins_only(origin)?;

			ensure!(
				<LocalToForeign<T>>::contains_key(local_asset_id),
				Error::<T>::LocalAssetIdNotFound
			);

			<ForeignAssetMetadata<T>>::insert(local_asset_id, metadata);
			Self::deposit_event(Event::AssetMetadataUpdated(local_asset_id));
			Ok(().into())
		}

		/// Minimal amount of asset_id required to send message to other network.
		/// Target network may or may not accept payment.
		/// Assumed this is maintained up to date by technical team.
		/// Mostly UI hint and fail fast solution. In theory can be updated by 
		#[pallet::weight(<T as Config>::WeightInfo::set_metadata())]
		pub fn minimal_amount(asset_id: ) -> DispatchResultWithPostInfo {

		}
	}

	impl<T: Config> RemoteAssetRegistry for Pallet<T> {
		type AssetId = T::LocalAssetId;

		type AssetNativeLocation = T::Location;

		/// Update the location of local_asset_id.
		fn set_location(
			local_asset_id: Self::AssetId,
			location: Self::AssetNativeLocation,
		) -> DispatchResult {
			<ForeignAssetLocation<T>>::insert(local_asset_id, location.clone());
			<FromForeignAssetLocation<T>>::insert(location, local_asset_id);
			Ok(())
		}

		/// Get the location of local_asset_id.
		fn asset_to_location(local_asset_id: Self::AssetId) -> Option<Self::AssetNativeLocation> {
			<ForeignAssetLocation<T>>::get(local_asset_id)
		}

		/// Get the foreign_asset_id of location.
		fn location_to_asset(location: Self::AssetNativeLocation) -> Option<Self::AssetId> {
			//panic!("{:?}", location);
			<FromForeignAssetLocation<T>>::get(location)
		}
	}

	impl<T: Config> Pallet<T> {
		/// Ensure that an account matches with `Config::LocalAdminOrigin` or
		/// `Config::ForeignAdminOrigin`.
		fn ensure_admins_only(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			if let (Err(_), Err(_)) = (
				T::LocalAdminOrigin::ensure_origin(origin.clone()),
				T::ForeignAdminOrigin::ensure_origin(origin),
			) {
				Err(Error::<T>::OnlyAllowedForAdmins.into())
			} else {
				Ok(().into())
			}
		}

		/// Approve an assets mapping candidate.
		fn approve_candidate(
			who: T::AccountId,
			local_asset_id: T::LocalAssetId,
			foreign_asset_id: T::ForeignAssetId,
			location: T::Location,
			decimals: u8,
		) -> DispatchResultWithPostInfo {
			let current_candidate_status =
				<AssetsMappingCandidates<T>>::get((local_asset_id, foreign_asset_id));
			let local_admin = <LocalAdmin<T>>::get();
			let foreign_admin = <ForeignAdmin<T>>::get();
			match current_candidate_status {
				None =>
					if Some(who) == local_admin {
						<AssetsMappingCandidates<T>>::insert(
							(local_asset_id, foreign_asset_id),
							CandidateStatus::LocalAdminApproved,
						);
					} else {
						<AssetsMappingCandidates<T>>::insert(
							(local_asset_id, foreign_asset_id),
							CandidateStatus::ForeignAdminApproved,
						);
					},
				Some(CandidateStatus::LocalAdminApproved) =>
					if Some(who) == foreign_admin {
						Self::promote_candidate(
							local_asset_id,
							foreign_asset_id,
							location,
							decimals,
						)?;
					},
				Some(CandidateStatus::ForeignAdminApproved) =>
					if Some(who) == local_admin {
						Self::promote_candidate(
							local_asset_id,
							foreign_asset_id,
							location,
							decimals,
						)?;
					},
			};
			Ok(().into())
		}

		/// Promote an assets mapping candidate to assets mapping.
		fn promote_candidate(
			local_asset_id: T::LocalAssetId,
			foreign_asset_id: T::ForeignAssetId,
			location: T::Location,
			decimals: u8,
		) -> DispatchResult {
			Self::set_location(local_asset_id, location)?;
			<LocalToForeign<T>>::insert(local_asset_id, foreign_asset_id);
			<ForeignToLocal<T>>::insert(foreign_asset_id, local_asset_id);
			<ForeignAssetMetadata<T>>::insert(local_asset_id, ForeignMetadata { decimals });
			<AssetsMappingCandidates<T>>::remove((local_asset_id, foreign_asset_id));
			Ok(())
		}
	}

	pub struct EnsureLocalAdmin<T>(PhantomData<T>);
	impl<T: Config> EnsureOrigin<T::Origin> for EnsureLocalAdmin<T> {
		type Success = T::AccountId;
		fn try_origin(o: T::Origin) -> Result<Self::Success, T::Origin> {
			o.into().and_then(|o| match (o, LocalAdmin::<T>::try_get()) {
				(frame_system::RawOrigin::Signed(ref who), Ok(ref f)) if who == f =>
					Ok(who.clone()),
				(r, _) => Err(T::Origin::from(r)),
			})
		}

		#[cfg(feature = "runtime-benchmarks")]
		fn successful_origin() -> T::Origin {
			let local_admin = LocalAdmin::<T>::try_get().expect("local admin should exist");
			T::Origin::from(frame_system::RawOrigin::Signed(local_admin))
		}
	}

	pub struct EnsureForeignAdmin<T>(PhantomData<T>);
	impl<T: Config> EnsureOrigin<T::Origin> for EnsureForeignAdmin<T> {
		type Success = T::AccountId;
		fn try_origin(o: T::Origin) -> Result<Self::Success, T::Origin> {
			o.into().and_then(|o| match (o, ForeignAdmin::<T>::try_get()) {
				(frame_system::RawOrigin::Signed(ref who), Ok(ref f)) if who == f =>
					Ok(who.clone()),
				(r, _) => Err(T::Origin::from(r)),
			})
		}

		#[cfg(feature = "runtime-benchmarks")]
		fn successful_origin() -> T::Origin {
			let foreign_admin = ForeignAdmin::<T>::try_get().expect("foreign admin should exist");
			T::Origin::from(frame_system::RawOrigin::Signed(foreign_admin))
		}
	}
}
