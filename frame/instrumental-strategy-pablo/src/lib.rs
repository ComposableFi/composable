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
	use composable_support::math::safe::{SafeDiv, SafeMul, SafeSub};
	use composable_traits::{
		dex::Amm,
		instrumental::{InstrumentalProtocolStrategy, State},
		vault::{FundsAvailability, StrategicVault, Vault},
	};
	use frame_support::{
		dispatch::{DispatchError, DispatchResult},
		pallet_prelude::*,
		storage::bounded_btree_set::BoundedBTreeSet,
		traits::fungibles::{Inspect, Mutate, MutateHold, Transfer},
		transactional, Blake2_128Concat, PalletId, RuntimeDebug,
	};
	use frame_system::pallet_prelude::OriginFor;
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Convert,
			Zero,
		},
		Permill,
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

		type ExternalOrigin: EnsureOrigin<Self::Origin>;

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
			+ Ord
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

		type Convert: Convert<Self::Balance, u128> + Convert<u128, Self::Balance>;

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
		Encode, Decode, MaxEncodedLen, Clone, Copy, Default, RuntimeDebug, PartialEq, TypeInfo,
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
		/// Add [`VaultId`](Config::VaultId) to [`AssociatedVaults`](AssociatedVaults) storage.
		///
		/// Emits [`AssociatedVault`](Event::AssociatedVault) event when successful.
		#[pallet::weight(T::WeightInfo::associate_vault())]
		pub fn associate_vault(
			origin: OriginFor<T>,
			vault_id: T::VaultId,
		) -> DispatchResultWithPostInfo {
			T::ExternalOrigin::ensure_origin(origin)?;
			<Self as InstrumentalProtocolStrategy>::associate_vault(&vault_id)?;
			Ok(().into())
		}
		/// Store a mapping of asset_id -> pool_id in the pools runtime storage object.
		///
		/// Emits [`AssociatedPoolWithAsset`](Event::AssociatedPoolWithAsset) event when successful.
		#[pallet::weight(T::WeightInfo::set_pool_id_for_asset())]
		pub fn set_pool_id_for_asset(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			pool_id: T::PoolId,
		) -> DispatchResultWithPostInfo {
			T::ExternalOrigin::ensure_origin(origin)?;
			<Self as InstrumentalProtocolStrategy>::set_pool_id_for_asset(asset_id, pool_id)?;
			Ok(().into())
		}
		/// Occur rebalance of liquidity of each vault.
		///
		/// Emits [`RebalancedVault`](Event::RebalancedVault) event when successful.
		#[pallet::weight(T::WeightInfo::liquidity_rebalance())]
		pub fn liquidity_rebalance(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			T::ExternalOrigin::ensure_origin(origin)?;
			<Self as InstrumentalProtocolStrategy>::rebalance()?;
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::transferring_funds())]
		pub fn transferring_funds(
			origin: OriginFor<T>,
			vault_id: T::VaultId,
			asset_id: T::AssetId,
			new_pool_id: T::PoolId,
			percentage_of_funds: Permill,
		) -> DispatchResultWithPostInfo {
			T::ExternalOrigin::ensure_origin(origin)?;
			<Self as InstrumentalProtocolStrategy>::transferring_funds(
				&vault_id,
				asset_id,
				new_pool_id,
				percentage_of_funds,
			)?;
			Ok(().into())
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
		fn set_pool_id_for_asset(asset_id: T::AssetId, pool_id: T::PoolId) -> DispatchResult {
			match Pools::<T>::try_get(asset_id) {
				Ok(pool) => {
					ensure!(pool.state == State::Normal, Error::<T>::TransferringInProgress);
					Pools::<T>::mutate(asset_id, |_| PoolState { pool_id, state: State::Normal });
				},
				Err(_) => Pools::<T>::insert(asset_id, PoolState { pool_id, state: State::Normal }),
			}
			Self::deposit_event(Event::AssociatedPoolWithAsset { asset_id, pool_id });
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

		fn transferring_funds(
			vault_id: &Self::VaultId,
			asset_id: Self::AssetId,
			new_pool_id: Self::PoolId,
			percentage_of_funds: Permill,
		) -> DispatchResult {
			let pool_id_and_state = Self::pools(asset_id).ok_or(Error::<T>::PoolNotFound)?;
			let pool_id_deduce = pool_id_and_state.pool_id;
			let vault_account = T::Vault::account_id(vault_id);
			let lp_token_id = T::Pablo::lp_token(pool_id_deduce)?;
			let mut balance_of_lp_token = T::Currency::balance(lp_token_id, &vault_account);
			Pools::<T>::mutate(asset_id, |pool| {
				*pool = Some(PoolState { pool_id: pool_id_deduce, state: State::Transferring });
			});
			let pertcentage_of_funds: u128 = percentage_of_funds.deconstruct().into();
			let balance_of_lp_tokens_decimal = T::Convert::convert(balance_of_lp_token);
			let balance_to_withdraw_per_transaction =
				T::Convert::convert(balance_of_lp_tokens_decimal.safe_mul(&pertcentage_of_funds)?);
			while balance_of_lp_token > balance_to_withdraw_per_transaction {
				Self::do_tranferring_funds(
					&vault_account,
					new_pool_id,
					pool_id_deduce,
					balance_to_withdraw_per_transaction,
				)?;
				balance_of_lp_token =
					balance_of_lp_token.safe_sub(&balance_to_withdraw_per_transaction)?;
			}
			if balance_of_lp_token > T::Balance::zero() {
				Self::do_tranferring_funds(
					&vault_account,
					new_pool_id,
					pool_id_deduce,
					balance_to_withdraw_per_transaction,
				)?;
			}
			Pools::<T>::mutate(asset_id, |pool| {
				*pool = Some(PoolState { pool_id: new_pool_id, state: State::Normal });
			});
			Ok(())
		}

		#[transactional]
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
		fn do_rebalance(vault_id: &T::VaultId) -> DispatchResult {
			let asset_id = T::Vault::asset_id(vault_id)?;
			let vault_account = T::Vault::account_id(vault_id);
			let pool_id_and_state = Self::pools(asset_id).ok_or(Error::<T>::PoolNotFound)?;
			let pool_id = pool_id_and_state.pool_id;
			match T::Vault::available_funds(vault_id, &Self::account_id())? {
				FundsAvailability::Withdrawable(balance) => {
					Self::withdraw(&vault_account, pool_id, balance)?;
				},
				FundsAvailability::Depositable(balance) => {
					Self::deposit(&vault_account, pool_id, balance)?;
				},
				FundsAvailability::MustLiquidate => {
					Self::liquidate(&vault_account, pool_id)?;
				},
				FundsAvailability::None => {},
			};
			Ok(())
		}

		fn withdraw(
			vault_account: &T::AccountId,
			pool_id: T::PoolId,
			balance: T::Balance,
		) -> DispatchResult {
			T::Pablo::add_liquidity(
				vault_account,
				pool_id,
				balance,
				T::Balance::zero(),
				T::Balance::zero(),
				true,
			)
		}

		fn deposit(
			vault_account: &T::AccountId,
			pool_id: T::PoolId,
			balance: T::Balance,
		) -> DispatchResult {
			let lp_price = T::Pablo::get_price_of_lp_token(pool_id)?;
			let lp_redeem = balance.safe_div(&lp_price)?;
			T::Pablo::remove_liquidity_single_asset(
				vault_account,
				pool_id,
				lp_redeem,
				T::Balance::zero(),
			)
		}

		fn liquidate(vault_account: &T::AccountId, pool_id: T::PoolId) -> DispatchResult {
			let lp_token_id = T::Pablo::lp_token(pool_id)?;
			let balance_of_lp_token = T::Currency::balance(lp_token_id, &vault_account);
			T::Pablo::remove_liquidity_single_asset(
				vault_account,
				pool_id,
				balance_of_lp_token,
				T::Balance::zero(),
			)
		}

		#[transactional]
		fn do_tranferring_funds(
			vault_account: &T::AccountId,
			new_pool_id: T::PoolId,
			pool_id_deduce: T::PoolId,
			balance: T::Balance,
		) -> DispatchResult {
			Self::deposit(vault_account, pool_id_deduce, balance)?;
			Self::withdraw(vault_account, new_pool_id, balance)?;
			Ok(())
		}
	}
}

// -------------------------------------------------------------------------------------------------
//                                            Unit Tests
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {}
