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
)]
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]
#![allow(dead_code)] // TODO: remove when most of the work is completed.

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Codec, FullCodec};
	use composable_traits::{
		currency::LocalAssets,
		defi::CurrencyPair,
		dex::StableSwapPoolInfo,
		math::{SafeAdd, SafeSub},
	};
	use core::fmt::Debug;
	use frame_support::{
		pallet_prelude::*,
		traits::fungibles::{Inspect, Mutate, Transfer},
		PalletId, RuntimeDebug,
	};
	use frame_system::pallet_prelude::OriginFor;
	use sp_runtime::traits::{AccountIdConversion, Convert, One, Zero};

	#[derive(RuntimeDebug, Encode, Decode, MaxEncodedLen, Clone, PartialEq, Eq, TypeInfo)]
	pub enum PoolConfiguration<AccountId, AssetId> {
		StableSwap(StableSwapPoolInfo<AccountId, AssetId>),
	}

	type AssetIdOf<T> = <T as Config>::AssetId;
	type BalanceOf<T> = <T as Config>::Balance;
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type PoolIdOf<T> = <T as Config>::PoolId;
	type PoolOf<T> =
		PoolConfiguration<<T as frame_system::Config>::AccountId, <T as Config>::AssetId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Pool with specified id `T::PoolId` was created successfully by `T::AccountId`.
		PoolCreated {
			/// Id of newly created pool.
			pool_id: T::PoolId,
			/// Owner of the pool.
			owner: T::AccountId,
		},
		/// The sale ended, the funds repatriated and the pool deleted.
		PoolDeleted {
			/// Pool that was removed.
			pool_id: T::PoolId,
			/// Amount of base asset repatriated.
			base_amount: T::Balance,
			/// Amount of quote asset repatriated.
			quote_amount: T::Balance,
		},
		/// Liquidity added into the pool `T::PoolId`.
		LiquidityAdded {
			/// Account id who added liquidity.
			who: T::AccountId,
			/// Pool id to which liquidity added.
			pool_id: T::PoolId,
			/// Amount of base asset deposited.
			base_amount: T::Balance,
			/// Amount of quote asset deposited.
			quote_amount: T::Balance,
			/// Amount of minted lp tokens.
			mint_amount: T::Balance,
		},
		/// Liquidity removed from pool `T::PoolId` by `T::AccountId` in balanced way.
		LiquidityRemoved {
			/// Account id who removed liquidity.
			who: T::AccountId,
			/// Pool id to which liquidity added.
			pool_id: T::PoolId,
			/// Amount of base asset removed from pool.
			base_amount: T::Balance,
			/// Amount of quote asset removed from pool.
			quote_amount: T::Balance,
			/// Updated lp token supply.
			total_issuance: T::Balance,
		},
		/// Token exchange happened.
		Swapped {
			/// Pool id on which exchange done.
			pool_id: T::PoolId,
			/// Account id who exchanged token.
			who: T::AccountId,
			/// Id of asset used as input.
			base_asset: T::AssetId,
			/// Id of asset used as output.
			quote_asset: T::AssetId,
			/// Amount of base asset received.
			base_amount: T::Balance,
			/// Amount of quote asset provided.
			quote_amount: T::Balance,
			/// Charged fees.
			fee: T::Balance,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		PoolNotFound,
		PoolConfigurationNotSupported,
		PairMismatch,
		MustBeOwner,
		InvalidSaleState,
		InvalidAmount,
		CannotRespectMinimumRequested,
		AssetAmountMustBePositiveNumber,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Type representing the unique ID of an asset.
		type AssetId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ Clone
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo
			+ Ord;

		/// Type representing the Balance of an account.
		type Balance: Default
			+ Parameter
			+ Codec
			+ MaxEncodedLen
			+ Copy
			+ Ord
			+ Zero
			+ SafeAdd
			+ SafeSub;

		/// An isomorphism: Balance<->u128
		type Convert: Convert<u128, BalanceOf<Self>> + Convert<BalanceOf<Self>, u128>;

		/// Dependency allowing this pallet to transfer funds from one account to another.
		type Assets: Transfer<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ Mutate<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ Inspect<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>;

		/// Type representing the unique ID of a pool.
		type PoolId: FullCodec
			+ MaxEncodedLen
			+ Default
			+ Debug
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy
			+ Zero
			+ One
			+ SafeAdd
			+ SafeSub;

		type LocalAssets: LocalAssets<AssetIdOf<Self>>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// The origin allowed to create new pools.
		type AdminOrigin: EnsureOrigin<Self::Origin>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::type_value]
	pub fn PoolCountOnEmpty<T: Config>() -> T::PoolId {
		Zero::zero()
	}

	#[pallet::storage]
	#[pallet::getter(fn pool_count)]
	#[allow(clippy::disallowed_type)]
	pub type PoolCount<T: Config> = StorageValue<_, T::PoolId, ValueQuery, PoolCountOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> = StorageMap<_, Blake2_128Concat, T::PoolId, PoolOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new pool.
		///
		/// Emits `PoolCreated` event when successful.
		// TODO: enable weight
		#[pallet::weight(10_000)]
		pub fn create(
			_origin: OriginFor<T>,
			_pool: PoolConfiguration<T::AccountId, T::AssetId>,
		) -> DispatchResult {
			Ok(())
		}

		/// Execute a buy order on pool.
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(10_000)]
		pub fn buy(
			_origin: OriginFor<T>,
			_pool_id: T::PoolId,
			_asset_id: T::AssetId,
			_amount: T::Balance,
			_keep_alive: bool,
		) -> DispatchResult {
			Ok(())
		}

		/// Execute a sell order on pool.
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(10_000)]
		pub fn sell(
			_origin: OriginFor<T>,
			_pool_id: T::PoolId,
			_asset_id: T::AssetId,
			_amount: T::Balance,
			_keep_alive: bool,
		) -> DispatchResult {
			Ok(())
		}

		/// Execute a specific swap operation.
		///
		/// The `quote_amount` is always the quote asset amount (A/B => B), (B/A => A).
		///
		/// Emits `Swapped` event when successful.
		#[pallet::weight(10_000)]
		pub fn swap(
			_origin: OriginFor<T>,
			_pool_id: T::PoolId,
			_pair: CurrencyPair<T::AssetId>,
			_quote_amount: T::Balance,
			_min_receive: T::Balance,
			_keep_alive: bool,
		) -> DispatchResult {
			Ok(())
		}

		/// Add liquidity to a stable-swap pool.
		///
		/// Emits `LiquidityAdded` event when successful.
		#[pallet::weight(10_000)]
		pub fn add_liquidity(
			_origin: OriginFor<T>,
			_pool_id: T::PoolId,
			_base_amount: T::Balance,
			_quote_amount: T::Balance,
			_min_mint_amount: T::Balance,
			_keep_alive: bool,
		) -> DispatchResult {
			Ok(())
		}

		/// Remove liquidity from stable-swap pool.
		///
		/// Emits `LiquidityRemoved` event when successful.
		#[pallet::weight(10_000)]
		pub fn remove_liquidity(
			_origin: OriginFor<T>,
			_pool_id: T::PoolId,
			_lp_amount: T::Balance,
			_min_base_amount: T::Balance,
			_min_quote_amount: T::Balance,
		) -> DispatchResult {
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn do_create_pool(pool: PoolOf<T>) -> Result<T::PoolId, DispatchError> {
			let pool_id =
				PoolCount::<T>::try_mutate(|pool_count| -> Result<T::PoolId, DispatchError> {
					let pool_id = *pool_count;
					Pools::<T>::insert(pool_id, pool.clone());
					*pool_count = pool_id.safe_add(&T::PoolId::one())?;
					Ok(pool_id)
				})?;
			match pool {
				PoolConfiguration::StableSwap(stable_swap_pool_info) => {
					let pool = stable_swap_pool_info;
					Self::deposit_event(Event::PoolCreated { pool_id, owner: pool.owner.clone() });
					Ok(pool_id)
				},
			}
		}

		pub(crate) fn get_pool(pool_id: T::PoolId) -> Result<PoolOf<T>, DispatchError> {
			Pools::<T>::get(pool_id).ok_or_else(|| Error::<T>::PoolNotFound.into())
		}

		pub(crate) fn account_id(pool_id: &T::PoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account(pool_id)
		}
	}
}
