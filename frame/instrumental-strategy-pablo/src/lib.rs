#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// -------------------------------------------------------------------------------------------
	//                                   Imports and Dependencies
	// -------------------------------------------------------------------------------------------
	use codec::{Codec, FullCodec};
	use composable_traits::{instrumental::InstrumentalProtocolStrategy, vault::StrategicVault};
	use frame_support::{
		dispatch::DispatchResult, pallet_prelude::*, storage::bounded_btree_set::BoundedBTreeSet,
		transactional, PalletId,
	};
	use sp_runtime::traits::{
		AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Zero,
	};
	use sp_std::fmt::Debug;

	use crate::weights::WeightInfo;

	// -------------------------------------------------------------------------------------------
	//                                Declaration Of The Pallet Type
	// -------------------------------------------------------------------------------------------

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// -------------------------------------------------------------------------------------------
	//                                         Config Trait
	// -------------------------------------------------------------------------------------------

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

		/// The maximum number of vaults that can be associated with this strategy.
		#[pallet::constant]
		type MaxAssociatedVaults: Get<u32>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	// -------------------------------------------------------------------------------------------
	//                                         Pallet Types
	// -------------------------------------------------------------------------------------------

	// -------------------------------------------------------------------------------------------
	//                                       Runtime  Storage
	// -------------------------------------------------------------------------------------------

	#[pallet::storage]
	#[pallet::getter(fn associated_vaults)]
	pub type AssociatedVaults<T: Config> =
		StorageValue<_, BoundedBTreeSet<T::VaultId, T::MaxAssociatedVaults>, ValueQuery>;

	// -------------------------------------------------------------------------------------------
	//                                        Runtime Events
	// -------------------------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AssociatedVault { vault_id: T::VaultId },

		RebalancedVault { vault_id: T::VaultId },

		UnableToRebalanceVault { vault_id: T::VaultId },
	}

	// -------------------------------------------------------------------------------------------
	//                                        Runtime Errors
	// -------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		VaultAlreadyAssociated,

		TooManyAssociatedStrategies,
	}

	// -------------------------------------------------------------------------------------------
	//                                            Hooks
	// -------------------------------------------------------------------------------------------

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	// -------------------------------------------------------------------------------------------
	//                                          Extrinsics
	// -------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	// -------------------------------------------------------------------------------------------
	//                                      Protocol Strategy
	// -------------------------------------------------------------------------------------------

	impl<T: Config> InstrumentalProtocolStrategy for Pallet<T> {
		type AccountId = T::AccountId;
		type AssetId = T::AssetId;
		type VaultId = T::VaultId;

		fn account_id() -> Self::AccountId {
			T::PalletId::get().into_account()
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

	// -------------------------------------------------------------------------------------------
	//                                   Low Level Functionality
	// -------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		#[transactional]
		fn do_rebalance(_vault_id: &T::VaultId) -> DispatchResult {
			Ok(())
		}
	}
}

// -----------------------------------------------------------------------------------------------
//                                             Unit Tests
// -----------------------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {}
