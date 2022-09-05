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
		clippy::panic,
		clippy::unwrap_used,
		clippy::indexing_slicing,
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
		instrumental::{InstrumentalDynamicStrategy, InstrumentalProtocolStrategy},
		vault::StrategicVault,
	};
	use frame_support::{
		pallet_prelude::*, storage::bounded_btree_set::BoundedBTreeSet, transactional, PalletId,
	};
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Zero,
		},
		Percent,
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

		// TODO: (Nevin)
		//  - try to make the connection to substrategies a vec of InstrumentalProtocolStrategy
		//  - ideally something like: type WhitelistedStrategies: Get<[dyn
		//    InstrumentalProtocolStrategy]>;

		type PabloStrategy: InstrumentalProtocolStrategy<
			AccountId = Self::AccountId,
			AssetId = Self::AssetId,
			VaultId = Self::VaultId,
		>;

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

	// ---------------------------------------------------------------------------------------------
	//                                          Runtime Storage
	// ---------------------------------------------------------------------------------------------

	// TODO: (Nevin)
	//  - we need to store all vaults that are associated with this strategy

	#[pallet::storage]
	#[pallet::getter(fn associated_vaults)]
	#[allow(clippy::disallowed_types)]
	pub type AssociatedVaults<T: Config> =
		StorageValue<_, BoundedBTreeSet<T::VaultId, T::MaxAssociatedVaults>, ValueQuery>;

	// TODO: (Nevin)
	//  - we need a way of mapping a vault_id to its associated strategy

	// ---------------------------------------------------------------------------------------------
	//                                          Runtime Events
	// ---------------------------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AssociatedVault { vault_id: T::VaultId },
	}

	// ---------------------------------------------------------------------------------------------
	//                                          Runtime Errors
	// ---------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		VaultAlreadyAssociated,

		TooManyAssociatedStrategies,
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
	impl<T: Config> Pallet<T> {}

	// ---------------------------------------------------------------------------------------------
	//                                   Instrumental Dynamic Strategy
	// ---------------------------------------------------------------------------------------------

	// TODO: (Nevin)
	//  - create InstrumentalStrategy trait

	impl<T: Config> InstrumentalDynamicStrategy for Pallet<T> {
		type AccountId = T::AccountId;
		type AssetId = T::AssetId;

		// TODO: (Nevin)
		//  - we need a way to store a vector of all strategies that are whitelisted

		// fn get_strategies() -> [dyn InstrumentalProtocolStrategy<
		// 	AssetId = T::AssetId,
		// 	VaultId = T::VaultId
		// >] {
		// 	vec![&T::PabloStrategy]
		// }

		fn get_optimum_strategy_for(_asset: T::AssetId) -> Result<T::AccountId, DispatchError> {
			Ok(T::PabloStrategy::account_id())
		}
	}

	// ---------------------------------------------------------------------------------------------
	//                                         Protocol Strategy
	// ---------------------------------------------------------------------------------------------

	impl<T: Config> InstrumentalProtocolStrategy for Pallet<T> {
		type AccountId = T::AccountId;
		type AssetId = T::AssetId;
		type VaultId = T::VaultId;
		type PoolId = T::PoolId;

		fn account_id() -> Self::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		#[transactional]
		fn associate_vault(vault_id: &Self::VaultId) -> DispatchResult {
			// TODO: (Nevin)
			//  - cycle through all whitelisted strategies and associate the vault with the strategy
			//    with the highest earning apy

			// let asset_id = T::Vault::asset_id(vault_id)?;

			// let optimum_strategy = Self::get_strategies().iter()
			// 	.max_by_key(|strategy| strategy.get_apy(asset_id)?);

			// optimum_strategy.associate_vault(vault_id)?;

			AssociatedVaults::<T>::try_mutate(|vaults| -> DispatchResult {
				ensure!(!vaults.contains(vault_id), Error::<T>::VaultAlreadyAssociated);

				vaults
					.try_insert(*vault_id)
					.map_err(|_| Error::<T>::TooManyAssociatedStrategies)?;

				T::PabloStrategy::associate_vault(vault_id)?;

				Self::deposit_event(Event::AssociatedVault { vault_id: *vault_id });

				Ok(())
			})
		}

		fn rebalance() -> DispatchResult {
			Ok(())
		}

		fn set_pool_id_for_asset(_asset_id: T::AssetId, _pool_id: T::PoolId) -> DispatchResult {
			// TODO: (belousm)
			// The same functionality like in `instrumental-strategy-pablo`
			Ok(())
		}

		fn transferring_funds(
			_vault_id: &Self::VaultId,
			_asset_id: Self::AssetId,
			_new_pool_id: Self::PoolId,
			_percentage_of_funds: Percent,
		) -> DispatchResult {
			// TODO: (belousm)
			// implement transferring funds
			Ok(())
		}

		fn get_apy(asset: Self::AssetId) -> Result<u128, DispatchError> {
			// TODO: (Nevin)
			//  - cycle through all whitelisted strategies and return highest available apy

			// let optimum_apy = Self::get_strategies()
			// 	.iter()
			// 	.map(|strategy| strategy.get_apy(asset))
			// 	.max();

			// Ok(optimum_apy)

			T::PabloStrategy::get_apy(asset)
		}
	}
}

// -------------------------------------------------------------------------------------------------
//                                            Unit Tests
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {}
