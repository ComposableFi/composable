#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
	clippy::indexing_slicing,
	clippy::panic,
	clippy::todo,
	clippy::unseparated_literal_suffix,
	clippy::unwrap_used
)]
#![cfg_attr(
	test,
	allow(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::panic,
		clippy::unwrap_used,
	)
)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// ---------------------------------------------------------------------------------------------
	//                                     Imports and Dependencies
	// ---------------------------------------------------------------------------------------------

	use codec::{Codec, FullCodec};
	use composable_traits::{
		dex::Amm,
		instrumental::{AccessRights, InstrumentalProtocolStrategy, State},
		vault::StrategicVault,
	};
	use frame_support::{
		dispatch::{DispatchError, DispatchResult},
		pallet_prelude::*,
		storage::bounded_btree_set::BoundedBTreeSet,
		traits::fungibles::{Mutate, MutateHold, Transfer},
		transactional, Blake2_128Concat, PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_runtime::traits::{
		AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Zero,
	};
	use sp_std::fmt::Debug;

	use crate::weights::WeightInfo;

	// ---------------------------------------------------------------------------------------------
	//                                  Declaration Of The Pallet Type
	// ---------------------------------------------------------------------------------------------

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// ---------------------------------------------------------------------------------------------
	//                                           Config Trait
	// ---------------------------------------------------------------------------------------------

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type WeightInfo: WeightInfo;

		/// The [`Balance`](Config::Balance) type used by the pallet for bookkeeping.
		type Balance: Default
			+ Parameter
			+ Codec
			+ MaxEncodedLen
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ AtLeast32BitUnsigned
			+ Zero;

		/// The [`AssetId`](Config::AssetId) used by the pallet. Corresponds to the Ids used by the
		/// Currency pallet.
		type AssetId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo;

		/// The [`VaultId`](Config::VaultId) used by the pallet. Corresponds to the Ids used by the
		/// Vault pallet.
		type VaultId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ Ord
			+ TypeInfo
			+ Into<u128>;

		type Vault: StrategicVault<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			VaultId = Self::VaultId,
		>;

		/// The [`Currency`](Config::Currency).
		///
		/// Currency is used for the assets managed by the vaults.
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ MutateHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;

		type Pablo: Amm<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			PoolId = Self::PoolId,
		>;

		/// Type representing the unique ID of a pool.
		type PoolId: FullCodec
			+ MaxEncodedLen
			+ Default
			+ Debug
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy;

		/// The maximum number of vaults that can be associated with this strategy.
		#[pallet::constant]
		type MaxAssociatedVaults: Get<u32>;

		/// The id used as the
		/// [`AccountId`](composable_traits::instrumental::Instrumental::AccountId) of the vault.
		/// This should be unique across all pallets to avoid name collisions with other pallets and
		/// vaults.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	// ---------------------------------------------------------------------------------------------
	//                                           Pallet Types
	// ---------------------------------------------------------------------------------------------

	#[derive(
		Encode, Decode, MaxEncodedLen, Clone, Copy, Default, Debug, PartialEq, Eq, TypeInfo,
	)]
	pub struct PoolState<PoolId, State> {
		pub pool_id: PoolId,
		pub state: State,
	}

	// ---------------------------------------------------------------------------------------------
	//                                          Runtime Storage
	// ---------------------------------------------------------------------------------------------

	#[pallet::storage]
	#[pallet::getter(fn associated_vaults)]
	#[allow(clippy::disallowed_types)]
	pub type AssociatedVaults<T: Config> =
		StorageValue<_, BoundedBTreeSet<T::VaultId, T::MaxAssociatedVaults>, ValueQuery>;

	/// An asset whitelisted by Instrumental.
	///
	/// The corresponding Pool to invest the whitelisted asset into.
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AssetId, PoolState<T::PoolId, State>>;

	#[pallet::storage]
	#[pallet::getter(fn admin_accounts)]
	pub type AdminAccountIds<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, AccessRights>;

	// ---------------------------------------------------------------------------------------------
	//                                          Runtime Events
	// ---------------------------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AssociatedVault { vault_id: T::VaultId },

		RebalancedVault { vault_id: T::VaultId },

		UnableToRebalanceVault { vault_id: T::VaultId },

		AssociatedPoolWithAsset { asset_id: T::AssetId, pool_id: T::PoolId },
	}

	// ---------------------------------------------------------------------------------------------
	//                                          Runtime Errors
	// ---------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		VaultAlreadyAssociated,

		TooManyAssociatedStrategies,
		// TODO(belousm): only for MVP version we can assume the `pool_id` is already known and
		// exist. We should remove it in V1.
		PoolNotFound,
		// Occurs when we try to set a new pool_id, during a transferring from or to an old one
		TransferringInProgress,
		// Occurs when an attempt is made to initialize a function not from an admin account
		NotAdminAccount,
		// Occurs when an admin account doesn't have enough access rights to initialize a function
		NotEnoughAccessRights,
	}

	// ---------------------------------------------------------------------------------------------
	//                                               Hooks
	// ---------------------------------------------------------------------------------------------

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	// ---------------------------------------------------------------------------------------------
	//                                            Extrinsics
	// ---------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Store a mapping of asset_id -> pool_id in the pools runtime storage object.
		///
		/// Emits [`AssociatedPoolWithAsset`](Event::AssociatedPoolWithAsset) event when successful.
		#[pallet::weight(T::WeightInfo::set_pool_id_for_asset())]
		pub fn set_pool_id_for_asset(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			pool_id: T::PoolId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let access_right = Self::admin_accounts(who).ok_or(Error::<T>::NotAdminAccount)?;
			ensure!(
				access_right == AccessRights::Full || access_right == AccessRights::SetPoolId,
				Error::<T>::NotEnoughAccessRights
			);
			<Self as InstrumentalProtocolStrategy>::set_pool_id_for_asset(asset_id, pool_id)?;
			Self::deposit_event(Event::AssociatedPoolWithAsset { asset_id, pool_id });
			Ok(().into())
		}
		/// Occur rebalance of liquidity of each vault.
		///
		/// Emits [`RebalancedVault`](Event::RebalancedVault) event when successful.
		#[pallet::weight(T::WeightInfo::liquidity_rebalance())]
		pub fn liquidity_rebalance(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let access_right = Self::admin_accounts(who).ok_or(Error::<T>::NotAdminAccount)?;
			ensure!(
				access_right == AccessRights::Full || access_right == AccessRights::Rebalance,
				Error::<T>::NotEnoughAccessRights
			);
			<Self as InstrumentalProtocolStrategy>::rebalance()?;
			Ok(().into())
		}
	}

	// ---------------------------------------------------------------------------------------------
	//                                         Protocol Strategy
	// ---------------------------------------------------------------------------------------------

	impl<T: Config> InstrumentalProtocolStrategy for Pallet<T> {
		type AccountId = T::AccountId;
		type VaultId = T::VaultId;
		type AssetId = T::AssetId;
		type PoolId = T::PoolId;

		fn account_id() -> Self::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		#[transactional]
		fn set_pool_id_for_asset(asset_id: T::AssetId, pool_id: T::PoolId) -> DispatchResult {
			match Pools::<T>::try_get(asset_id) {
				Ok(pool) => {
					ensure!(pool.state == State::Normal, Error::<T>::TransferringInProgress);
					Pools::<T>::mutate(asset_id, |_| PoolState { pool_id, state: State::Normal });
				},
				Err(_) => Pools::<T>::insert(asset_id, PoolState { pool_id, state: State::Normal }),
			}
			Ok(())
		}

		#[transactional]
		fn associate_vault(vault_id: &Self::VaultId) -> DispatchResult {
			AssociatedVaults::<T>::try_mutate(|vaults| -> DispatchResult {
				ensure!(!vaults.contains(vault_id), Error::<T>::VaultAlreadyAssociated);

				vaults
					.try_insert(*vault_id)
					.map_err(|_| Error::<T>::TooManyAssociatedStrategies)?;

				Self::deposit_event(Event::AssociatedVault { vault_id: *vault_id });

				Ok(())
			})
		}

		fn rebalance() -> DispatchResult {
			AssociatedVaults::<T>::try_mutate(|vaults| -> DispatchResult {
				vaults.iter().for_each(|vault_id| {
					if Self::do_rebalance(vault_id).is_ok() {
						Self::deposit_event(Event::RebalancedVault { vault_id: *vault_id });
					} else {
						Self::deposit_event(Event::UnableToRebalanceVault { vault_id: *vault_id });
					}
				});

				Ok(())
			})
		}

		fn get_apy(_asset: Self::AssetId) -> Result<u128, DispatchError> {
			Ok(0)
		}
	}

	// ---------------------------------------------------------------------------------------------
	//                                      Low Level Functionality
	// ---------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		#[transactional]
		fn do_rebalance(_vault_id: &T::VaultId) -> DispatchResult {
			// TODO(saruman9): reimplement
			Ok(())
		}
	}
}

// -------------------------------------------------------------------------------------------------
//                                            Unit Tests
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {}
