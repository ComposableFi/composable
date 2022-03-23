//! Pallet for allowing to map assets from this and other parachain.
//!
//! It works as next:
//! 1. Each mapping is bidirectional.
//! 2. Assets map added as candidate and waits for approval.
//! 3. After approval map return mapped value.
//! 4. Map of native token to this chain(here) is added unconditionally.

#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_method,
		clippy::disallowed_type,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use codec::{EncodeLike, FullCodec};
	use composable_traits::assets::{RemoteAssetRegistry, XcmAssetLocation};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::EnsureOrigin,
	};

	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_std::{fmt::Debug, marker::PhantomData, str};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
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
		type Location: FullCodec
			+ Eq
			+ PartialEq
			// we wrap non serde type, so until written custom serde, cannot handle that
			// + MaybeSerializeDeserialize
			+ Debug
			+ Clone
			+ Default
			+ TypeInfo;
		type UpdateAdminOrigin: EnsureOrigin<Self::Origin>;
		type LocalAdminOrigin: EnsureOrigin<Self::Origin>;
		type ForeignAdminOrigin: EnsureOrigin<Self::Origin>;
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub enum CandidateStatus {
		LocalAdminApproved,
		ForeignAdminApproved,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct ForeignMetadata {
		pub decimals: u8,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn local_admin)]
	/// Local admin account
	pub type LocalAdmin<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn foreign_admin)]
	/// Foreign admin account
	pub type ForeignAdmin<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn from_local_asset)]
	/// Mapping local asset to foreign asset.
	pub type LocalToForeign<T: Config> =
		StorageMap<_, Blake2_128Concat, T::LocalAssetId, T::ForeignAssetId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn from_foreign_asset)]
	/// Mapping foreign asset to local asset.
	pub type ForeignToLocal<T: Config> =
		StorageMap<_, Blake2_128Concat, T::ForeignAssetId, T::LocalAssetId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn assets_mapping_candidates)]
	/// Mapping (local asset, foreign asset) to candidate status.
	pub type AssetsMappingCandidates<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::LocalAssetId, T::ForeignAssetId),
		CandidateStatus,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn foreign_asset_location)]
	/// Mapping foreign asset to foreign location.
	pub type ForeignAssetLocation<T: Config> =
		StorageMap<_, Blake2_128Concat, T::LocalAssetId, T::Location, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn from_foreign_asset_location)]
	/// Mapping foreign location to foreign asset.
	pub type FromForeignAssetLocation<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Location, T::LocalAssetId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn foreign_asset_metadata)]
	/// Mapping local asset to foreign asset metadata.
	pub type ForeignAssetMetadata<T: Config> =
		StorageMap<_, Blake2_128Concat, T::LocalAssetId, ForeignMetadata, OptionQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		local_admin: Option<T::AccountId>,
		foreign_admin: Option<T::AccountId>,
		// TODO: split this into 2 pairs
		// 1. (xcm location -> local asset id as used in our tuntime), so that when others send our
		// id to our chain we can trust them 2. (local qasset id - > remote location -> remote
		// asset id) so then when we send our local asset to remote chain we know what id we should
		// envode.
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
		LocalAdminUpdated(T::AccountId),
		ForeignAdminUpdated(T::AccountId),
		AssetsMappingCandidateUpdated {
			local_asset_id: T::LocalAssetId,
			foreign_asset_id: T::ForeignAssetId,
		},
		AssetMetadataUpdated(T::LocalAssetId),
	}

	#[pallet::error]
	pub enum Error<T> {
		OnlyAllowedForAdmins,
		LocalAssetIdAlreadyUsed,
		ForeignAssetIdAlreadyUsed,
		LocalAssetIdNotFound,
		ForeignAssetIdNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// TODO: bench this
		#[pallet::weight(10_000)]
		pub fn set_local_admin(
			origin: OriginFor<T>,
			local_admin: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::UpdateAdminOrigin::ensure_origin(origin)?;
			<LocalAdmin<T>>::put(local_admin.clone());
			Self::deposit_event(Event::LocalAdminUpdated(local_admin));
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn set_foreign_admin(
			origin: OriginFor<T>,
			foreign_admin: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::UpdateAdminOrigin::ensure_origin(origin)?;
			<ForeignAdmin<T>>::put(foreign_admin.clone());
			Self::deposit_event(Event::ForeignAdminUpdated(foreign_admin));
			Ok(().into())
		}

		#[pallet::weight(10_000)]
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

		#[pallet::weight(10_000)]
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
	}

	impl<T: Config> RemoteAssetRegistry for Pallet<T> {
		type AssetId = T::LocalAssetId;

		type AssetNativeLocation = T::Location;

		fn set_location(
			local_asset_id: Self::AssetId,
			location: Self::AssetNativeLocation,
		) -> DispatchResult {
			<ForeignAssetLocation<T>>::insert(local_asset_id, location.clone());
			<FromForeignAssetLocation<T>>::insert(location, local_asset_id);
			Ok(())
		}

		fn asset_to_location(local_asset_id: Self::AssetId) -> Option<Self::AssetNativeLocation> {
			<ForeignAssetLocation<T>>::get(local_asset_id)
		}

		fn location_to_asset(location: Self::AssetNativeLocation) -> Option<Self::AssetId> {
			<FromForeignAssetLocation<T>>::get(location)
		}
	}

	impl<T: Config> Pallet<T> {
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
