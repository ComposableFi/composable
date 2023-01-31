//! # Assets Transactor Router Pallet
//!
//! The Transactor Router provides implementations of common currency traits
//! (e.g. from [`orml`](https://docs.rs/orml-traits) and `frame_support`)
//! and functionality for handling transfers and minting.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The Assets pallet provides functions for:
//!
//! - Transferring balances of native and other assets between accounts.
//! - Minting and burn new assets by per asset governance.
//! - Crediting and debiting of created asset balances.
//! - By design similar to [orml_currencies](https://docs.rs/orml-currencies/latest/orml_currencies/)
//!   and [substrate_assets](https://github.com/paritytech/substrate/tree/master/frame/assets)
//! Functions requiring authorization are checked via asset's governance registry origin. Example,
//! minting.
//!
//! ### Implementations
//!
//! The Assets pallet provides implementations for the following traits:
//!
//! - [`Currency`](frame_support::traits::Currency):
//! - [`LockableCurrency`](frame_support::traits::tokens::currency::LockableCurrency)
//! - [`ReservableCurrency`](frame_support::traits::ReservableCurrency):
//! - [`MultiCurrency`](orml_traits::MultiCurrency):
//! - [`MultiLockableCurrency`](orml_traits::MultiLockableCurrency):
//! - [`MultiReservableCurrency`](orml_traits::MultiReservableCurrency):
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `transfer`
//! - `transfer_native`
//! - `force_transfer`
//! - `force_transfer_native`
//! - `transfer_all`
//! - `transfer_all_native`
//! - `mint_initialize`
//! - `mint_initialize_with_governance`
//! - `mint_into`
//! - `burn_from`

// we start lag behind useful traits:
// TODO: implement fungibles::Balanced like orml Tokens do
// TODO: implement tokens::NamedReservableCurrency like orml Tokens do

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

pub use pallet::*;

mod currency;
mod fungible;
mod fungibles;
mod orml;

#[cfg(test)]
mod mocks;

#[cfg(test)]
mod tests;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
pub mod weights;

macro_rules! route {
	(
		fn $fn:ident($asset:ident: $asset_ty:ty $(, $arg:ident: $ty:ty)* $(,)?) $(-> $ret:ty)?;
	) => {
		fn $fn($asset: $asset_ty, $($arg:$ty),*) $(-> $ret)? {
			if T::AssetId::from($asset.into()) == <T::NativeAssetId as frame_support::traits::Get<_>>::get() {
				<<T as Config>::NativeTransactor>::$fn($($arg),*)
			} else {
				crate::route_asset_type! { $fn($asset, $($arg),*) }
			}
		}
	};
}

macro_rules! route_asset_type {
	(
		$fn:ident($asset:ident $(, $arg:ident)* $(,)?)
	) => {
		match <T::AssetsRegistry as composable_traits::assets::AssetTypeInspect>::inspect(&$asset) {
			composable_traits::assets::AssetType::Foreign => {
				<<T as Config>::ForeignTransactor>::$fn($asset, $($arg),*)
			}
			composable_traits::assets::AssetType::Local => {
				<<T as Config>::LocalTransactor>::$fn($asset, $($arg),*)
			}
		}
	};
}

pub(crate) use route;
pub(crate) use route_asset_type;

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;
	use codec::FullCodec;
	use composable_traits::{
		assets::{
			AssetInfo, AssetTypeInspect, CreateAsset, InspectRegistryMetadata,
			MutateRegistryMetadata,
		},
		currency::{AssetIdLike, BalanceLike},
		governance::{GovernanceRegistry, SignedRawOrigin},
		xcm::assets::RemoteAssetRegistryMutate,
	};
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		sp_runtime::traits::StaticLookup,
		traits::{
			fungible, fungibles, Currency, EnsureOrigin, LockableCurrency, ReservableCurrency,
		},
	};
	use frame_system::{ensure_root, ensure_signed, pallet_prelude::OriginFor};
	use orml_traits::{GetByKey, MultiCurrency, MultiLockableCurrency, MultiReservableCurrency};
	use sp_runtime::{DispatchError, FixedPointOperand};
	use sp_std::{fmt::Debug, str, vec::Vec};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// currency id
		type AssetId: AssetIdLike + From<u128> + Into<u128> + MaybeSerializeDeserialize;
		type AssetLocation: FullCodec
			+ Eq
			+ PartialEq
			+ MaybeSerializeDeserialize
			+ Debug
			+ Clone
			+ Default
			+ TypeInfo
			+ MaxEncodedLen;
		type Balance: BalanceLike + FixedPointOperand;

		#[pallet::constant]
		type NativeAssetId: Get<Self::AssetId>;

		type ForeignTransactor: fungibles::Inspect<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ fungibles::Transfer<Self::AccountId>
			+ fungibles::Mutate<Self::AccountId>
			+ fungibles::Unbalanced<Self::AccountId>
			+ fungibles::InspectHold<Self::AccountId>
			+ fungibles::MutateHold<Self::AccountId>
			+ MultiCurrency<Self::AccountId, Balance = Self::Balance, CurrencyId = Self::AssetId>
			+ MultiLockableCurrency<
				Self::AccountId,
				Balance = Self::Balance,
				CurrencyId = Self::AssetId,
			> + MultiReservableCurrency<
				Self::AccountId,
				Balance = Self::Balance,
				CurrencyId = Self::AssetId,
			>;

		type LocalTransactor: fungibles::Inspect<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ fungibles::Transfer<Self::AccountId>
			+ fungibles::Mutate<Self::AccountId>
			+ fungibles::Unbalanced<Self::AccountId>
			+ fungibles::InspectHold<Self::AccountId>
			+ fungibles::MutateHold<Self::AccountId>
			+ MultiCurrency<Self::AccountId, Balance = Self::Balance, CurrencyId = Self::AssetId>
			+ MultiLockableCurrency<
				Self::AccountId,
				Balance = Self::Balance,
				CurrencyId = Self::AssetId,
			> + MultiReservableCurrency<
				Self::AccountId,
				Balance = Self::Balance,
				CurrencyId = Self::AssetId,
			>;

		// TODO(benluelo): Move trait bounds here, rename to NativeTransactor
		type NativeTransactor: fungible::Inspect<Self::AccountId, Balance = Self::Balance>
			+ fungible::Transfer<Self::AccountId>
			+ fungible::Mutate<Self::AccountId>
			+ fungible::Unbalanced<Self::AccountId>
			+ fungible::InspectHold<Self::AccountId>
			+ fungible::Transfer<Self::AccountId>
			+ fungible::MutateHold<Self::AccountId>
			+ Currency<Self::AccountId, Balance = Self::Balance>
			+ LockableCurrency<Self::AccountId, Balance = Self::Balance>
			+ ReservableCurrency<Self::AccountId, Balance = Self::Balance>;

		type GovernanceRegistry: GetByKey<Self::AssetId, Result<SignedRawOrigin<Self::AccountId>, DispatchError>>
			+ GovernanceRegistry<Self::AssetId, Self::AccountId>;

		/// origin of admin of this pallet
		type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Assets registry
		/// Maintains general info about any given asset
		type AssetsRegistry: AssetTypeInspect<AssetId = Self::AssetId>
			+ RemoteAssetRegistryMutate<
				AssetId = Self::AssetId,
				AssetNativeLocation = Self::AssetLocation,
				Balance = Self::Balance,
			> + InspectRegistryMetadata<AssetId = Self::AssetId>
			+ MutateRegistryMetadata<AssetId = Self::AssetId>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		CannotSetNewCurrencyToRegistry,
		InvalidCurrency,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Transfer `amount` of `asset` from `origin` to `dest`.
		///
		/// # Errors
		///  - When `origin` is not signed.
		///  - If the account has insufficient free balance to make the transfer, or if `keep_alive`
		///    cannot be respected.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			asset: T::AssetId,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let source = ensure_signed(origin)?;
			let dest = T::Lookup::lookup(dest)?;

			<Pallet<T> as fungibles::Transfer<_>>::transfer(
				asset, &source, &dest, amount, keep_alive,
			)?;
			Ok(())
		}

		/// Transfer `amount` of the native asset from `origin` to `dest`. This is slightly
		/// cheaper to call, as it avoids an asset lookup.
		///
		/// # Errors
		///  - When `origin` is not signed.
		///  - If the account has insufficient free balance to make the transfer, or if `keep_alive`
		///    cannot be respected.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::transfer_native())]
		pub fn transfer_native(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] value: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let source = ensure_signed(origin)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungible::Transfer<_>>::transfer(&source, &dest, value, keep_alive)?;
			Ok(())
		}

		/// Transfer `amount` of the `asset` from `origin` to `dest`. This requires root.
		///
		/// # Errors
		///  - When `origin` is not root.
		///  - If the account has insufficient free balance to make the transfer, or if `keep_alive`
		///    cannot be respected.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::force_transfer())]
		pub fn force_transfer(
			origin: OriginFor<T>,
			asset: T::AssetId,
			source: <T::Lookup as StaticLookup>::Source,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] value: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			ensure_root(origin)?;
			let source = T::Lookup::lookup(source)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungibles::Transfer<_>>::transfer(asset, &source, &dest, value, keep_alive)?;
			Ok(())
		}

		/// Transfer `amount` of the the native asset from `origin` to `dest`. This requires root.
		///
		/// # Errors
		///  - When `origin` is not root.
		///  - If the account has insufficient free balance to make the transfer, or if `keep_alive`
		///    cannot be respected.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::force_transfer_native())]
		pub fn force_transfer_native(
			origin: OriginFor<T>,
			source: <T::Lookup as StaticLookup>::Source,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] value: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			ensure_root(origin)?;
			let source = T::Lookup::lookup(source)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungible::Transfer<_>>::transfer(&source, &dest, value, keep_alive)?;
			Ok(())
		}

		/// Transfer all free balance of the `asset` from `origin` to `dest`.
		///
		/// # Errors
		///  - When `origin` is not signed.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::transfer_all())]
		pub fn transfer_all(
			origin: OriginFor<T>,
			asset: T::AssetId,
			dest: <T::Lookup as StaticLookup>::Source,
			keep_alive: bool,
		) -> DispatchResult {
			let transactor = ensure_signed(origin)?;
			let reducible_balance = <Self as fungibles::Inspect<T::AccountId>>::reducible_balance(
				asset,
				&transactor,
				keep_alive,
			);
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungibles::Transfer<T::AccountId>>::transfer(
				asset,
				&transactor,
				&dest,
				reducible_balance,
				keep_alive,
			)?;
			Ok(())
		}

		/// Transfer all free balance of the native asset from `origin` to `dest`.
		///
		/// # Errors
		///  - When `origin` is not signed.
		///  - If the `dest` cannot be looked up.
		#[pallet::weight(T::WeightInfo::transfer_all_native())]
		pub fn transfer_all_native(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			keep_alive: bool,
		) -> DispatchResult {
			let transactor = ensure_signed(origin)?;
			let reducible_balance =
				<Self as fungible::Inspect<_>>::reducible_balance(&transactor, keep_alive);
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungible::Transfer<_>>::transfer(
				&transactor,
				&dest,
				reducible_balance,
				keep_alive,
			)?;
			Ok(())
		}

		/// Creates a new asset, minting `amount` of funds into the `dest` account.
		///
		/// Intended to be used for creating wrapped assets, not associated with any project.
		#[pallet::weight(T::WeightInfo::mint_initialize())]
		pub fn mint_initialize(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			asset_info: AssetInfo<T::Balance>,
			#[pallet::compact] amount: T::Balance,
			dest: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			ensure_root(origin)?;

			T::AssetsRegistry::register_asset(asset_id, None, asset_info)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungibles::Mutate<T::AccountId>>::mint_into(asset_id, &dest, amount)?;
			Ok(())
		}

		/// Creates a new local asset, minting `amount` of funds into the `dest` account.
		///
		/// The `dest` account can use the democracy pallet to mint further assets, or if the
		/// governance_origin is set to an owned account, using signed transactions. In general the
		/// `governance_origin` should be generated from the pallet id.
		#[pallet::weight(T::WeightInfo::mint_initialize())]
		pub fn mint_initialize_with_governance(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			asset_info: AssetInfo<T::Balance>,
			#[pallet::compact] amount: T::Balance,
			governance_origin: <T::Lookup as StaticLookup>::Source,
			dest: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			ensure_root(origin)?;

			T::AssetsRegistry::register_asset(asset_id, None, asset_info)?;
			let governance_origin = T::Lookup::lookup(governance_origin)?;
			T::GovernanceRegistry::set(asset_id, SignedRawOrigin::Signed(governance_origin));
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungibles::Mutate<T::AccountId>>::mint_into(asset_id, &dest, amount)?;
			Ok(())
		}

		/// Mints `amount` of `asset_id` into the `dest` account.
		#[pallet::weight(T::WeightInfo::mint_into())]
		pub fn mint_into(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] amount: T::Balance,
		) -> DispatchResult {
			Pallet::<T>::ensure_admin_or_governance(origin, &asset_id)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungibles::Mutate<T::AccountId>>::mint_into(asset_id, &dest, amount)?;
			Ok(())
		}

		/// Burns `amount` of `asset_id` into the `dest` account.
		#[pallet::weight(T::WeightInfo::burn_from())]
		pub fn burn_from(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			dest: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] amount: T::Balance,
		) -> DispatchResult {
			Pallet::<T>::ensure_admin_or_governance(origin, &asset_id)?;
			let dest = T::Lookup::lookup(dest)?;
			<Self as fungibles::Mutate<T::AccountId>>::burn_from(asset_id, &dest, amount)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Returns `Ok(())` if origin is root or asset is signed by root or by origin
		pub(crate) fn ensure_admin_or_governance(
			origin: OriginFor<T>,
			asset_id: &T::AssetId,
		) -> Result<(), DispatchError> {
			// TODO: that must be ensure_asset_origin(origin, asset_id))
			if T::AdminOrigin::ensure_origin(origin.clone()).is_ok() {
				return Ok(())
			}

			match origin.into() {
				Ok(frame_system::RawOrigin::Signed(account)) => {
					match T::GovernanceRegistry::get(asset_id) {
						Ok(SignedRawOrigin::Root) => Ok(()), /* ISSUE: it says if */
						// (call_origin.is_signed &&
						// asst_owner.is_root) then allow
						// mint/burn -> anybody can mint and
						// burn PICA?
						// TODO: https://app.clickup.com/t/37h4edu
						Ok(SignedRawOrigin::Signed(acc)) if acc == account => Ok(()),
						_ => Err(DispatchError::BadOrigin),
					}
				},
				Ok(frame_system::RawOrigin::Root) => Ok(()),
				_ => Err(DispatchError::BadOrigin), /* ISSUE: likely will not support collective
													* origin which is reasonable to have for
													* governance
													https://app.clickup.com/t/37h4edu
													*/
			}
		}
	}

	impl<T: Config> CreateAsset for Pallet<T> {
		type LocalAssetId = T::AssetId;
		type ForeignAssetId = T::AssetLocation;
		type Balance = T::Balance;

		fn create_local_asset(
			protocol_id: [u8; 8],
			nonce: u64,
			asset_info: AssetInfo<T::Balance>,
		) -> Result<Self::LocalAssetId, DispatchError> {
			let bytes = protocol_id
				.into_iter()
				.chain(nonce.to_le_bytes())
				.collect::<Vec<u8>>()
				.try_into()
				.expect("[u8; 8] + bytes(u64) = [u8; 16]");
			let asset_id = Self::LocalAssetId::from(u128::from_le_bytes(bytes));

			T::AssetsRegistry::register_asset(asset_id, None, asset_info)?;

			Ok(asset_id)
		}

		fn create_foreign_asset(
			foreign_asset_id: Self::ForeignAssetId,
			asset_info: AssetInfo<T::Balance>,
		) -> Result<Self::LocalAssetId, DispatchError> {
			let asset_id = Self::LocalAssetId::from(u128::from_be_bytes(
				sp_io::hashing::blake2_128(&foreign_asset_id.encode()),
			));

			T::AssetsRegistry::register_asset(asset_id, Some(foreign_asset_id), asset_info)?;

			Ok(asset_id)
		}
	}
}
