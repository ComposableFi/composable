#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod traits;

#[cfg(test)]
mod mocks;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies                                      
	// ----------------------------------------------------------------------------------------------------
	use crate::traits::CurrencyFactory;
	use core::ops::AddAssign;

	use codec::{Codec, FullCodec};
	use composable_traits::{
		vault::{
			Deposit as Duration, Vault, VaultConfig,
		},
		pool::{
			Assets, Bound, ConstantMeanMarket, Deposit, FixedBalance, PoolConfig, PoolInfo, WeightsVec,
		},
	};

	use frame_support::{
		ensure,
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::fungibles::MutateHold,
		},
		PalletId,
	};
	use frame_system::{
		Config as SystemConfig,
	};

	use num_integer::Roots;
	use num_traits::SaturatingSub;
	use hydra_dx_math::transcendental::pow;
	use scale_info::TypeInfo;

	use sp_runtime::{
		// helpers_128bit::multiply_by_rational,
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, 
			Convert, One, Zero,
		},
		ArithmeticError, FixedPointOperand/*, FixedU128*/, Perquintill
	};

	use sp_std::fmt::Debug;
	use std::collections::BTreeSet;

	// ----------------------------------------------------------------------------------------------------
	//                                    Declaration Of The Pallet Type                                           
	// ----------------------------------------------------------------------------------------------------

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// ----------------------------------------------------------------------------------------------------
	//                                             Config Trait                                            
	// ----------------------------------------------------------------------------------------------------

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// Loosely couple the Vault trait providing local types for the vaults associated types.
		type Vault: Vault<
			AccountId = Self::AccountId,
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			BlockNumber = Self::BlockNumber
		>;

		// The Balance type used by the pallet for bookkeeping. `Config::Convert` is used for
		// conversions to `u128`, which are used in the computations.
		type Balance: Default
			+ Parameter
			+ Codec
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ One
			+ Roots
			+ SaturatingSub
			+ AtLeast32BitUnsigned
			+ Zero
			+ FixedPointOperand;

		// The pallet creates new LP tokens for every pool created. It uses `CurrencyFactory`, as
		//     `orml`'s currency traits do not provide an interface to obtain asset ids (to avoid id
		//     collisions).
		type CurrencyFactory: CurrencyFactory<Self::AssetId>;

		// The `AssetId` used by the pallet. Corresponds the the Ids used by the Currency pallet.
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo
			+ Ord;

		// // Native asset used to pay creation fees
		// type NativeCurrency: TransferNative<Self::AccountId, Balance = Self::Balance>
		// 	+ MutateNative<Self::AccountId, Balance = Self::Balance>
		// 	+ MutateHoldNative<Self::AccountId, Balance = Self::Balance>;

		// Generic Currency bounds. These functions are provided by the `[orml-tokens`](https://github.com/open-web3-stack/open-runtime-module-library/tree/HEAD/currencies) pallet.
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ MutateHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;
		
		// Converts the `Balance` type to `u128`, which internally is used in calculations.
		type Convert: Convert<Self::Balance, u128> + Convert<u128, Self::Balance>;

		// The ID used to uniquely represent Pools
		type PoolId: AddAssign
			+ FullCodec
			+ One
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ Into<u128>
			+ From<u64>
			+ TypeInfo;

		// The asset ID used to pay transaction fees.
		#[pallet::constant]
		type NativeAssetId: Get<Self::AssetId>;

		// The native asset fee needed to create a vault.
		#[pallet::constant]
		type CreationFee: Get<Self::Balance>;

		// The deposit needed for a pool to never be cleaned up.
		#[pallet::constant]
		type ExistentialDeposit: Get<Self::Balance>;

		// The margin of error when working with the Pool's weights.
		//     Pool Creation: initial weights, when normalized, must add up into the range
		//         1 - epsilon <= weights <= 1 + epsilon
		//     Pool Deposits: deposit weights, when normalized by the total deposit amount,
		//         must add up into the range 1 - epsilon <= deposit <= 1 + epsilon
		#[pallet::constant]
		type Epsilon: Get<Perquintill>;

		// The minimum allowed amount of assets a user can deposit.
		#[pallet::constant]
		type MinimumDeposit: Get<Self::Balance>;

		// The minimum allowed amount of assets a user can withdraw.
		#[pallet::constant]
		type MinimumWithdraw: Get<Self::Balance>;

		// The id used as the `AccountId` of each pool. This should be unique across all pallets to
		//     avoid name collisions with other pallets.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	// ----------------------------------------------------------------------------------------------------
    //                                             Pallet Types                                           
	// ----------------------------------------------------------------------------------------------------

	pub type AssetIdOf<T> =
		<<T as Config>::Currency as Inspect<<T as SystemConfig>::AccountId>>::AssetId;
	
	#[allow(missing_docs)]
	pub type AccountIdOf<T> = <T as SystemConfig>::AccountId;

	#[allow(missing_docs)]
	pub type BlockNumberOf<T> = <T as SystemConfig>::BlockNumber;

	#[allow(missing_docs)]
	pub type BalanceOf<T> = <T as Config>::Balance;

	// Type alias exists mainly since `PoolInfo` has two generic parameters.
	pub type PoolInfoOf<T> = PoolInfo<AccountIdOf<T>, AssetIdOf<T>>;

	pub type DepositInfo<T> = 
		Deposit<<T as Config>::AssetId, <T as Config>::Balance>;

	// ----------------------------------------------------------------------------------------------------
    //                                           Runtime  Storage                                          
	// ----------------------------------------------------------------------------------------------------

	// The number of active pools - also used to generate the next pool identifier.
	#[pallet::storage]
	#[pallet::getter(fn pool_count)]
	pub type PoolCount<T: Config> = StorageValue<_, T::PoolId, ValueQuery>;

	// Mapping of a Pool's Id to its PoolInfo struct.
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> = 
		StorageMap<_, Twox64Concat, T::PoolId, PoolInfoOf<T>, ValueQuery>;

	// Assets tracked by the pool
	#[pallet::storage]
	#[pallet::getter(fn pool_assets)]
	pub type PoolAssets<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, Assets<T::AssetId>>;

	// Weights for each asset in the pool
	#[pallet::storage]
	#[pallet::getter(fn pool_asset_weight)]
	pub type PoolAssetWeight<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::AssetId,
		Perquintill,
		ValueQuery
	>;

	// Mapping of the Pool's Id and an Assets Id to the corresponding vault
	#[pallet::storage]
	#[pallet::getter(fn pool_asset_vault)]
	pub type PoolAssetVault<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, 
		T::PoolId, 
		Blake2_128Concat,
		T::AssetId,
		<T::Vault as Vault>::VaultId,
		ValueQuery
	>;

	// Balance of asset for given pool excluding admin_fee
	#[pallet::storage]
	#[pallet::getter(fn pool_asset_balance)]
	// Absence of pool asset balance is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_type)]
	pub type PoolAssetBalance<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::AssetId,
		T::Balance,
		ValueQuery,
	>;

	// Balance of asset for given pool including admin_fee
	#[pallet::storage]
	#[pallet::getter(fn pool_asset_total_balance)]
	// Absence of pool asset balance is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_type)]
	pub type PoolAssetTotalBalance<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::AssetId,
		T::Balance,
		ValueQuery,
	>;


	// #[pallet::storage]
	// #[pallet::getter(fn lp_token_to_pool_id)]
	// pub type LpTokenToPoolId<T: Config> = 
	// 	StorageMap<_, Twox64Concat, T::AssetId, T::PoolId, ValueQuery>;

	// ----------------------------------------------------------------------------------------------------
    //                                            Runtime Events                                          
	// ----------------------------------------------------------------------------------------------------

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Emitted after a pool has been created. [pool_id]
		PoolCreated {
			// The (incremented) ID of the created pool.
			pool_id: T::PoolId,
			// The configuration info related to the pool just created.
			pool_info: PoolInfoOf<T>,
		},

		// Emitted after a user deposits assets into a pool.
		Deposited {
			// The account issuing the deposit.
			account: AccountIdOf<T>,
			// The pool deposited into.
			pool_id: T::PoolId,
			// The asset ids and corresponding amount deposited.
			deposited: Vec<DepositInfo<T>>,
			// The number of LP tokens minted for the deposit.
			lp_tokens_minted: BalanceOf<T>,
		},

		// Emitted after a user withdraws assets from a pool.
		Withdrawn {
			// The account issuing the deposit.
			account: AccountIdOf<T>,
			// The pool deposited into.
			pool_id: T::PoolId,
			// The asset ids and corresponding amount withdrawn.
			withdrawn: Vec<DepositInfo<T>>,
			// The number of LP tokens burned from the withdraw.
			lp_tokens_burned: BalanceOf<T>,
		},
	}

	// ----------------------------------------------------------------------------------------------------
    //                                           Runtime  Errors                                           
	// ----------------------------------------------------------------------------------------------------

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		// When interacting with a pool and multiple of the same asset_id were provided for the
		//     extrinsic results in:
		DuplicateAssets,

		// Creating a pool with asset bounds [min, max], where max < min results in:
		InvalidAssetBounds,

		// Creating a pool of size n with bounds [min, max], where n < min or max < n results in:
		PoolSizeIsOutsideOfAssetBounds, 

		// Creating a pool where the number of weights provided does not equal the number of assets
		//     desired results in:
		ThereMustBeOneWeightForEachAssetInThePool,

		// Creating a pool specifying weights that don't sum to one results in:
		PoolWeightsMustBeNormalized,

		// Creating a pool with weight bounds [min, max], where max < min results in:
		InvalidWeightBounds,

		// Creating a pool where weight w_i < min_weight or max_weight < w_i results in:
		PoolWeightsAreOutsideOfWeightBounds,

		// Creating a pool of size n with a creation_fee less than the required amount results in:
		CreationFeeIsInsufficient,

		// Users issuing a request with a pool id that doesn't correspond to an active (created) 
		//     Pool results in:
		PoolDoesNotExist,

		// Trying to deposit a number of assets not equivalent to the Pool's underlying assets
		//     results in:
		ThereMustBeOneDepositForEachAssetInThePool,

		// Trying to make a deposits where one or more the the asset amounts is ≤ zero results in:
		DepositsMustBeStrictlyPositive,

		// Trying to create a pool (that represents N assets) when the issuer has less than 
		//     N * (CreationFee + ExistentialDeposit) native tokens results in:
		IssuerDoesNotHaveBalanceTryingToDeposit,

		// Trying to deposit an asset amount that is either:
		//	   i.  deposit_amount < pool_deposit_minimum
		//	   ii. pool_deposit_maximum < deposit_amount
		// results in:
		DepositIsOutsideOfPoolsDepositBounds,

		// Trying to deposit a group of assets with a value distribution that does not match the Pool's 
		//     value distribution results in:
		DepositDoesNotMatchUnderlyingValueDistribution,

		// Issues that arise when transfering funds from the user to pools account results in:
		DepositingIntoPoolFailed,

		// Issues that arise when transfering funds into a vaults account results in:
		DepositingIntoVaultFailed,




		// Failure to create an LP tokens during pool creation results in:
		ErrorCreatingLpTokenForPool,

		// Users depositing assets into a pool with a ratio that doesn't match the ratio from the pools
		//     weighting metric results in:
		DepositDoesNotMatchWeightingMetric,

		// Users trying to deposit an asset amount that is smaller than the Pools configured minimum 
		//     withdraw results in:
		AmountMustBeGreaterThanMinimumDeposit,

		// Users trying to deposit an asset amount that is larger than the Pools configured maximum 
		//     deposit results in:
		AmountMustBeLessThanMaximumDeposit,

		// Users trying to deposit an asset amount that is smaller than the Pools configured maximum 
		//     withdraw results in:
		AmountMustBeLessThanMaximumWithdraw,

		// Users trying to withdraw assets while providing an amount of lp tokens that is smaller than 
		//     T::MinimumWithdraw results in:
		AmountMustBeGreaterThanMinimumWithdraw,

		// Issues that arise when a pool is trying to mint its local lp tokens into the issuers account
		//     results in:
		FailedToMintLpTokens,

		//
		IssuerDoesNotHaveLpTokensTryingToDeposit,

		// TODO (Nevin):
		//  - rename to better represent error
		// Issues that arise when transfering tokens from one address to another result in:
		TransferFromFailed,

		// Issues that arise when a pool is trying to burn its local lp tokens from the issuers account
		//     results in:
		FailedToBurnLpTokens,
	}

	// ----------------------------------------------------------------------------------------------------
    //                                              Extrinsics                                             
	// ----------------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
	
		// // # Errors
		// //  - When the extrinsic is not signed.
		// #[pallet::weight(10_000)]
		// pub fn create(
		// 	origin: OriginFor<T>,
		// 	config: PoolConfig<AccountIdOf<T>, AssetIdOf<T>>,
		// 	creation_fee: Deposit<AssetIdOf<T>, BalanceOf<T>>,
		// ) -> DispatchResultWithPostInfo {
		// 	// Requirement 0) This extrinsic must be signed 
		// 	let from = ensure_signed(origin)?;

		// 	<Self as ConstantMeanMarket>::create(from, config, creation_fee)?;
		// 	Ok(().into())
		// }
	
		// // # Errors
		// //  - When the origin is not signed.
		// #[pallet::weight(10_000)]
		// pub fn deposit(
		// 	origin: OriginFor<T>,
		// 	pool_id: T::PoolId,
		// 	deposits: Vec<Deposit<T::AssetId, T::Balance>>,
		// ) -> DispatchResultWithPostInfo {
		// 	// Requirement 0) this extrinsic must be signed 
		// 	let from = ensure_signed(origin)?;

		// 	<Self as Pool>::deposit(&from, &pool_id, deposits.clone())?;
		// 	Ok(().into())
		// }

		// // # Errors
		// //  - When the origin is not signed.
		// #[pallet::weight(10_000)]
		// pub fn single_asset_deposit(
		// 	origin: OriginFor<T>,
		// 	pool_id: T::PoolId,
		// 	deposits: Deposit<T::AssetId, T::Balance>,
		// ) ->  DispatchResultWithPostInfo {
		// 	// Requirement 0) this extrinsic must be signed 
		// 	let from = ensure_signed(origin)?;

		// 	<Self as Pool>::single_asset_deposit(&from, &pool_id, deposits.clone())?;
		// 	Ok(().into())
		// }

		// // Withdraw assets from the pool by supplying the pools native LP token.
		// // # Emits
		// //  - Event::Withdrawn
		// //
		// // # Errors
		// //  - When the origin is not signed.
		// //  - When pool_id doesn't correspond to an existing pool.
		// //  - When the issuer does not have the specified lp token amount.
		// #[pallet::weight(10_000)]
		// pub fn withdraw(
		// 	origin: OriginFor<T>,
		// 	pool_id: T::PoolId,
		// 	lp_amount: T::Balance,
		// ) -> DispatchResultWithPostInfo {
		// 	// Requirement 0) this extrinsic must be signed 
		// 	let to = ensure_signed(origin)?;

		// 	<Self as Pool>::withdraw(&to, &pool_id, lp_amount)?;
		// 	Ok(().into())
		// }
	}

	// ----------------------------------------------------------------------------------------------------
    //                              Constant Mean Market Trait Implementation                                       
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> ConstantMeanMarket for Pallet<T> {
		type AccountId = AccountIdOf<T>;
		type Balance = BalanceOf<T>;
		type AssetId = AssetIdOf<T>;
		type Weight = Perquintill;

		type PoolId = T::PoolId;
		type PoolInfo = PoolInfoOf<T>;

		fn account_id(pool_id: &Self::PoolId) -> Self::AccountId {
			T::PalletId::get().into_sub_account(pool_id)
		}
		
		fn pool_info(pool_id: &Self::PoolId) -> Result<Self::PoolInfo, DispatchError> {
			Ok(Pools::<T>::try_get(pool_id).map_err(|_err| Error::<T>::PoolDoesNotExist)?)
		}

		fn lp_token_id(pool_id: &Self::PoolId) -> Result<Self::AssetId, DispatchError> {
			Ok(Self::pool_info(pool_id)?.lp_token)
		}

		fn lp_circulating_supply(pool_id: &Self::PoolId) -> Result<Self::Balance, DispatchError> {
			let lp_token_id = Self::lp_token_id(pool_id)?;

			Ok(T::Currency::total_issuance(lp_token_id))
		}

		fn reserves_of(pool_id: &Self::PoolId) -> Result<Vec<Deposit<Self::AssetId, Self::Balance>>, DispatchError> {
			let assets = PoolAssets::<T>::try_get(pool_id).map_err(|_err| Error::<T>::PoolDoesNotExist)?;

			let mut reserves = Vec::<Deposit<Self::AssetId, Self::Balance>>::new();

			for asset in assets {
				reserves.push(Deposit {
					asset_id: asset,
					amount:   Self::balance_of(pool_id, &asset)?,
				});
			}

			Ok(reserves)
		}

		fn balance_of(pool_id: &Self::PoolId, asset_id: &Self::AssetId) -> Result<Self::Balance, DispatchError> {
			let vault_id = &PoolAssetVault::<T>::get(pool_id, asset_id);
			let vault_lp_token_id = T::Vault::lp_asset_id(&vault_id)?;
		
			let pool_account = &Self::account_id(pool_id);

			Ok(T::Currency::balance(vault_lp_token_id, pool_account))
		}

		fn weight_of(pool_id: &Self::PoolId, asset_id: &Self::AssetId) -> Result<Self::Weight, DispatchError> {
			Ok(Self::pool_asset_weight(pool_id, asset_id))
		}

		// Validates the Pool's configuration parameters, and creates a new Pool if the config is valid. 
		//      Pool creation requires a creation deposit (fee) of at least (`PoolSize` + 1) x (`ExistentialDeposit` + `CreationFee`). 
		//      A Cubic vault is created for each asset, and an equal portion of the creation fee 
		//      `ExistentialDeposit` + `CreationFee` is transfered into each vault's account. 
		// 		is transferred into the pool's account.
		//
		// # Emits
		//  - [`Event::PoolCreated`](Event::PoolCreated)
		//
		// # Requirements
		// 		i.    ∀(i, j) a_i != a_j -- no duplicate assets
		//		ii.	  min_underlying_tokens ≤ max_underlying_tokens
		// 		iii.  min_underlying_tokens ≤ n ≤ max_underlying_tokens, where n is the number
		//                  of tokens in the pool
		//      iv.   ∀ assets a_i ⇒ ∃ weight w_i
		// 		v.    Σ w_i = 1 & w_i ≥ 0
		//		vi.   min_weight ≤ max_weight
		// 		vii.  min_weight ≤ w_i ≤ max_weight	
		//      viii. user_balance ≥ creation_fee
		//      ix.   creation_fee ≥ (asset_ids.len() + 1) * (creation_deposit + existential_deposit)
		//
		// # Errors
		//  - When the number of underlying assets is less than the minimum or greater than the maximum
		//        number of allowed assets in the pools configuration.
		//  - When the issuer specifies duplicate assets to be supplied in the vault.
		//  - When the issuing user doesn't have the balance trying to deposit.
		//  - When the weights specified in the configuration do not sum to one +/- some margin of error.
		fn create(
			from: Self::AccountId,
			config: PoolConfig<Self::AccountId, AssetIdOf<T>>,
			creation_fee: Deposit<Self::AssetId, Self::Balance>,
		) -> Result<Self::PoolId, DispatchError> {			
			let number_of_assets = config.assets.len();
			
			// ---------- Asset Requirements -----------
			// Requirement 1) The pools configuration doesn't specify duplicate assets
			ensure!(
				Self::no_duplicate_assets_provided(&config.assets), 
				Error::<T>::DuplicateAssets
			);

			// Requirement 2) The minimum asset bound must be less than or equal to the maximum asset bound
			ensure!(
				config.asset_bounds.minimum <= config.asset_bounds.maximum,
				Error::<T>::InvalidAssetBounds
			);

			// Requirement 3) The number of assets in the pool's configuration must be within
			//     the required bounds set in the config's asset_bounds field
			let pool_size = number_of_assets as u8;
			ensure!(
				config.asset_bounds.minimum <= pool_size && pool_size <= config.asset_bounds.maximum,
				Error::<T>::PoolSizeIsOutsideOfAssetBounds
			);

			// ---------- Weight Requirements ----------
			// Requirement 4) There exists a corresponding weight for each asset
			ensure!(
				Self::each_asset_has_a_corresponding_weight(&config.assets, &config.weights),
				Error::<T>::ThereMustBeOneWeightForEachAssetInThePool
			);

			// Requirement 5) The weights must be normalized (sum to one) and nonnegative
			ensure!(
				Self::weights_are_normalized_and_nonnegative(&config.weights),
				Error::<T>::PoolWeightsMustBeNormalized
			);

			// Requirement 6) The minimum weight bound must be less than or equal to the maximum weight bound
			ensure!(
				config.weight_bounds.minimum <= config.weight_bounds.maximum,
				Error::<T>::InvalidWeightBounds
			);

			// Requirement 7) The provided weights are in the required weight bounds
			ensure!(
				Self::weights_are_in_weight_bounds(&config.weights, &config.weight_bounds),
				Error::<T>::PoolWeightsAreOutsideOfWeightBounds
			);

			// ---------- User Requirements ----------
			// Requirement 8) The user who issued must have the balance they are trying to deposit
			ensure!(
				Self::user_has_specified_balance_for_asset(&from, T::NativeAssetId::get(), creation_fee.amount),
				Error::<T>::IssuerDoesNotHaveBalanceTryingToDeposit
			);
			
			// Requirement 9) The specified creation fee is sufficient enough to open a Pool
			ensure!(
				Self::required_creation_deposit_for(number_of_assets)? <= creation_fee.amount, 
				Error::<T>::CreationFeeIsInsufficient
			);

			let (pool_id, pool_info) = Self::do_create_pool(from, config, creation_fee)?;
			Self::deposit_event(Event::PoolCreated { 
				pool_id,
				pool_info
			});

			Ok(pool_id)
		}

		// Deposit funds in the pool and receive LP tokens in return. Deposited funds are transferred
		//     into the respective underlying vault account. Minted LP tokens are transferred into the
		//     issuers account.
		// 
		// # Emits
		//  - Event::Deposited
		//
		// # Requirements
		// 		i.    pool id p_i ⇒ exists
		//		ii.	  ∀ asset a_i in pool : ∃ asset a_i in deposit
		//		iii.  ∀ asset a_i in deposit : dep_i > 0
		//		iv.   ∀ asset a_i in deposit : user_balance_i >= dep_i
		//		v.	  ∀ asset a_i in deposit : dep_min <= dep_i <= dep_max
		//		vi.	  ∀ asset a_i in deposit : (dep_i / Σ dep_j) = (pool_balance_i / Σ pool_balance_j)
		//
		// # Errors
		//  - When pool_id doesn't correspond to an existing pool.
		//  - When the issuer does not have the specified deposit amounts.
		//  - When there isn't a one to one correspondence of asset ids trying to deposit and asset ids
		//        in the Pool.
		//  - When one of the deposited assets is less than the required minimum or greater than the
		//        required maximum percentage of the Pools balance of the respective asset.
		//  - When the deposited assets don't follow the Pool's weights (with a margin of error).
		fn deposit(
			from: &Self::AccountId,
			pool_id: &Self::PoolId,
			deposits: Vec<Deposit<Self::AssetId, Self::Balance>>,
		) -> Result<T::Balance, DispatchError> {
			// Requirement 1) the desired pool index must exist
			ensure!(Pools::<T>::contains_key(pool_id), Error::<T>::PoolDoesNotExist);
		
			// Requirement 2) For each of the Pool's underlying assets there is a deposit of that asset
			ensure!(
				Self::there_is_one_deposit_for_each_underlying_asset(pool_id, &deposits)?,
				Error::<T>::ThereMustBeOneDepositForEachAssetInThePool
			);

			let reserve_total: BalanceOf<T> = Self::reserves_of(pool_id)?.iter()
				.fold(T::Balance::zero(), |total, reserve| total + reserve.amount);
			let deposit_total: BalanceOf<T> = deposits.iter()
				.fold(T::Balance::zero(), |total, reserve| total + reserve.amount);

			for deposit in &deposits {
				// Requirement 3) deposit amount is stictly positive
				ensure!(
					deposit.amount > T::Balance::zero(),
					Error::<T>::DepositsMustBeStrictlyPositive
				);

				// Requirement 4) the user who issued the extrinsic must have the total balance trying to deposit
				ensure!(
					Self::user_has_specified_balance_for_asset(&from, deposit.asset_id, deposit.amount),
					Error::<T>::IssuerDoesNotHaveBalanceTryingToDeposit
				);

				// Requirement 5) if there are assets, deposit is within the pools min/max deposit bounds
				ensure!(
					Self::deposit_is_within_pools_deposit_bounds(pool_id, deposit)?,
					Error::<T>::DepositIsOutsideOfPoolsDepositBounds
				);

				// Requirement 6) deposit amounts match the pools current reserve ratio
				ensure!(
					Self::deposit_matches_underlying_value_distribution(pool_id, deposit, deposit_total, reserve_total)?,
					Error::<T>::DepositDoesNotMatchUnderlyingValueDistribution
				);
			}

			let lp_tokens_minted = Self::do_deposit(&from, &pool_id, deposits.clone())?;
			
			Self::deposit_event(Event::Deposited { 
				account:          from.clone(), 
				pool_id:          pool_id.clone(), 
				deposited:        deposits, 
				lp_tokens_minted: lp_tokens_minted,
			});

			Ok(lp_tokens_minted)
		}

		// // Withdraw assets from the pool by supplying the pools native LP token. Withdrawn assets are
		// //     removed from the respective underlying vault accounts and transfered into the issuers 
		// //     account. The deposited Pools LP tokens are burned.
		// //
		// // # Emits
		// //  - Event::Withdrawn
		// //
		// // # Errors
		// //  - When pool_id doesn't correspond to an existing pool.
		// //  - When the issuer does not have the specified lp token amount.
		// //  - When one of the withdrawn assets is less than the required minimum or greater than the
		// //        required maximum percentage of the Pools balance of the respective asset.
		// fn withdraw(
		// 	to: &Self::AccountId,
		// 	pool_id: &Self::PoolId,
		// 	lp_amount: Self::Balance,
		// ) -> Result<Vec<Deposit<Self::AssetId, Self::Balance>>, DispatchError> {
		// 	// Requirement 1) the desired pool index must exist
		// 	ensure!(Pools::<T>::contains_key(pool_id), Error::<T>::PoolDoesNotExist);

		// 	// Requirement 2) the user who issued the extrinsic must have the total balance trying to deposit
		// 	ensure!(
		// 		T::Currency::balance(Self::lp_token_id(&pool_id)?, &to) >= lp_amount,
		// 		Error::<T>::IssuerDoesNotHaveLpTokensTryingToDeposit
		// 	);
			
		// 	// Requirement 3) The amount of each asset being withdrawn must be larger than
		// 	//     the pools minimum withdraw value and smaller than the pools maximum
		// 	//     withdraw value
		// 	let pool_info = Self::pool_info(pool_id)?;
		// 	let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;

		// 	for asset_id in &pool_info.asset_ids {
		// 		let pool_balance_of_token = Self::balance_of(pool_id, asset_id)?;
		// 		let lp_share_of_asset: T::Balance = Self::calculate_lps_share_of_pools_asset(
		// 			pool_balance_of_token,
		// 			lp_amount,
		// 			lp_circulating_supply
		// 		).map_err(|_| ArithmeticError::Overflow)?;

		// 		// Asset amount to withdraw must be greater than the pools withdraw minimum
		// 		let min_withdraw = Self::percent_of(pool_info.withdraw_min, pool_balance_of_token);
		// 		ensure!(
		// 			lp_share_of_asset > min_withdraw,
		// 			Error::<T>::AmountMustBeGreaterThanMinimumWithdraw
		// 		);

		// 		// Asset amount to withdraw must be less than the pools withdraw maximum
		// 		let max_withdraw = Self::percent_of(pool_info.withdraw_max, pool_balance_of_token);
		// 		ensure!(
		// 			lp_share_of_asset < max_withdraw,
		// 			Error::<T>::AmountMustBeLessThanMaximumWithdraw
		// 		);
		// 	}

		// 	let assets_withdrawn = Self::do_withdraw(&to, &pool_id, lp_amount)?;
			
		// 	Self::deposit_event(Event::Withdrawn { 
		// 		account:          to.clone(), 
		// 		pool_id:          pool_id.clone(), 
		// 		withdrawn:        assets_withdrawn.clone(), 
		// 		lp_tokens_burned: lp_amount,
		// 	});

		// 	Ok(assets_withdrawn)
		// }

		// // Calculates the weights for the pool using the weighting metric specified as a WeightingMetric variant
		// fn calculate_weights(
		// 	pool_id: &Self::PoolId,
		// 	asset_ids: &Vec<Self::AssetId>,
		// 	weighting_metric: &WeightingMetric<Self::AssetId>
		// ) -> Result<(), DispatchError> {
		// 	// Requirement 1) Calculate the pools weights dependant on its specified weighting metric
		// 	let weights: Vec<Weight<Self::AssetId>> = match weighting_metric {
		// 		WeightingMetric::Equal => {
		// 			let mut weights = Vec::<Weight<Self::AssetId>>::new();
		// 			for asset_id in asset_ids {
		// 				weights.push( Weight {
		// 					asset_id: 	*asset_id,
		// 					weight: 	Perquintill::from_float(1 as f64 / asset_ids.len() as f64),
		// 				});
		// 			}
		// 			weights
		// 		},
		// 		WeightingMetric::Fixed(weights) => {
		// 			weights.to_vec()
		// 		},
		// 	};

		// 	// Requirement 2) The weights must be normalized
		// 	// TODO (Nevin):
		// 	//  - (Maybe) enforce that weights sum to one exactly (rather than allowing margin of error)

		// 	let epsilon = T::Epsilon::get().deconstruct();
		// 	let one = Perquintill::one().deconstruct();

		// 	let sum = weights
		// 		.iter()
		// 		.map(|weight| weight.weight.deconstruct())
		// 		.sum();

		// 	ensure!(
		// 		(one - epsilon) <= sum && sum <= (one + epsilon),
		// 		Error::<T>::PoolWeightsMustBeNormalized
		// 	);

		// 	// Requirement 3) Persist PoolId and AssetId to Weight mapping in storage
		// 	for share in &weights {
		// 		PoolAssetWeight::<T>::insert(&pool_id, share.asset_id, share.weight);
		// 	}

		// 	Ok(())
		// }
	}

	// ----------------------------------------------------------------------------------------------------
	//                                 Helper Functions - Core Functionality                               
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		// Helper function for the the create trait function. Obtains a new LP Token Id for the pool, creates a 
		//     vault for each asset desired in the pool, and saves all important info into runtime storage
		//
		// # Errors
		//  - When their is an issue creating an lp token for the pool.
		//  - When there is an issue creating an underlying vault.
		//  - When any `deposit` < `CreationFee` + `ExistentialDeposit`.
		//  - When the issuer has insufficient funds to lock each deposit.
		fn do_create_pool(
			from: T::AccountId,
			config: PoolConfig<AccountIdOf<T>, AssetIdOf<T>>,
			creation_fee: Deposit<AssetIdOf<T>, BalanceOf<T>>,
		) -> Result<(T::PoolId, PoolInfoOf<T>), DispatchError>  {
			PoolCount::<T>::try_mutate(|id| {
				let id = {
					*id += One::one();
					*id
				};

				// Requirement 1) Obtain a new asset id for this pools lp token 
				let lp_token =
					{ T::CurrencyFactory::create().map_err(|_| Error::<T>::ErrorCreatingLpTokenForPool)? };

				let account_id = Self::account_id(&id);

				// TODO (Nevin):
				//  - allow strategies to be supplied during the creation of the vaults
				//     |-> each vault can have different strategies or there can be one set of
				//     |       strategies for the overall pool
				//     '-> (in PoolConfig) strategies will be a Vec<BTreeMap<AccountId, Perquintill>>
				//             and reserved will be a Vec<Perquintill>

				// Requirement 2) Create a unique vault for each underlying asset
				for asset_id in &config.assets {
					// TODO (Nevin):
					//   - if there is an issue creating the nth vault destroy the previous n-1 vaults

					let vault_id: <T::Vault as Vault>::VaultId = T::Vault::create(
						Duration::Existential,
						VaultConfig::<T::AccountId, T::AssetId> {
							asset_id: *asset_id,
							reserved: Perquintill::from_percent(100),
							manager: account_id.clone(),
							strategies: [].iter().cloned().collect(),
						},
					)?;
					
					PoolAssetVault::<T>::insert(id, asset_id, vault_id.clone());
				}

				// Requirement 3) Transfer native tokens from users account to pools account
				let pool_account = &Self::account_id(&id);
				T::Currency::transfer(T::NativeAssetId::get(), &from, pool_account, creation_fee.amount, true)
					.map_err(|_| Error::<T>::DepositingIntoPoolFailed)?;
				
				// Requirement 4) Save the pools assets in global storage
				PoolAssets::<T>::insert(&id, config.assets);

				// Requirement 5) Save each assets weight in global storage
				for share in &config.weights {
					PoolAssetWeight::<T>::insert(&id, share.asset_id, share.weight);
				}

				// Requirement 6) Keep track of the pool's configuration
				let pool_info = PoolInfoOf::<T> {
					owner:		 	 config.owner,
					lp_token:		 lp_token,
					fee: 			 config.fee,
					asset_bounds:	 config.asset_bounds,
					weight_bounds:	 config.weight_bounds,
					deposit_bounds:	 config.deposit_bounds,
					withdraw_bounds: config.withdraw_bounds,
				};
				Pools::<T>::insert(id, pool_info.clone());

				Ok((id, pool_info))
			})
		}

		// Helper function for the the deposit trait funciton. Transfers deposit into pool, mints
		//	   LP tokens for the user, and saves all important info into runtime storage
		//
		// # Errors
		//  - When their is an issue creating an lp token for the pool.
		//  - When there is an issue creating an underlying vault.
		//  - When any `deposit` < `CreationFee` + `ExistentialDeposit`.
		//  - When the issuer has insufficient funds to lock each deposit.
		fn do_deposit(
			from: &T::AccountId,
			pool_id: &T::PoolId,
			deposits: Vec<Deposit<T::AssetId, T::Balance>>,
		) -> Result<T::Balance, DispatchError> {
			
			let pool_info = Pools::<T>::get(&pool_id);
			let to = &Self::account_id(pool_id);

			// TODO (Nevin):
			//  - transfer all assets into the pool's account before beginning
			//     '-> should be its own function to abstract away details
			//  - check all deposits can happen before doing the deposits
			//     `-> T::Currency::can_deposit(asset, from, to)

			// Requirement 1) Calculate the number of LP tokens that need to be minted
			let lp_tokens_to_mint = Self::calculate_lp_tokens_to_mint(&pool_id, &deposits)?;

			// Requirement 2) Deposit each asset into the pool's underlying vaults
			for deposit in &deposits {
				// TODO (Nevin):
				//  - check for errors in vault depositing, and if so revert all deposits so far
				//     '-> surround T::Vault::deposit call in match statement checking for Ok(lp_token_amount)
				//             and DispatchError

				let vault_id = &PoolAssetVault::<T>::get(pool_id, deposit.asset_id);

				T::Currency::transfer(deposit.asset_id, &from, to, deposit.amount, true)
					.map_err(|_| Error::<T>::DepositingIntoPoolFailed)?;

				let _vault_lp_token_amount = T::Vault::deposit(
					vault_id,
					to,
					deposit.amount,
				).map_err(|_| Error::<T>::DepositingIntoPoolFailed)?;
				
				// Requirement 3) Update the pool's reserve runtime storage objects
				Self::increase_pool_asset_balance_storage(pool_id,&deposit.asset_id,&deposit.amount)?;
				Self::increase_pool_asset_total_balance_storage(pool_id,&deposit.asset_id,&deposit.amount)?;
			}

			// Requirement 4) Mint the calling user LP tokens for the deposit
			T::Currency::mint_into(pool_info.lp_token, from, lp_tokens_to_mint)
				.map_err(|_| Error::<T>::FailedToMintLpTokens)?;

			Ok(lp_tokens_to_mint)
		}

		// fn do_single_asset_deposit(
		// 	from: &T::AccountId,
		// 	pool_id: &T::PoolId,
		// 	deposits: Deposit<T::AssetId, T::Balance>,
		// ) -> Result<T::Balance, DispatchError> {
		// 	let lp_circulating_supply = <Self as Pool>::lp_circulating_supply(pool_id);
			
		// 	let lp_tokens_to_mint = lp_circulating_supply;

		// 	lp_tokens_to_mint
		// }

		// fn do_withdraw(
		// 	to: &T::AccountId,
		// 	pool_id: &T::PoolId,
		// 	lp_amount: T::Balance,
		// ) -> Result<Vec<Deposit<T::AssetId, T::Balance>>, DispatchError> {
		// 	let pool_account = &Self::account_id(pool_id);

		// 	let pool_info = Pools::<T>::get(&pool_id);
		// 	let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;

		// 	// Used to keep track of the amount of each asset withdrawn from the pool's underlying vaults
		// 	let mut assets_withdrawn = Vec::<Deposit<T::AssetId, T::Balance>>::new();

		// 	// Requirement 1) Calculate and withdraw the lp tokens share of the each asset in the pool
		// 	for asset_id in &pool_info.asset_ids {
		// 		let vault_id = &PoolAssetVault::<T>::get(pool_id, asset_id);
		// 		let pool_balance_of_token = Self::balance_of(pool_id, asset_id)?;

		// 		// Calculate the percentage of the pool's assets that correspond to the deposited lp tokens
		// 		let lp_share_of_asset: T::Balance = Self::calculate_lps_share_of_pools_asset(
		// 			pool_balance_of_token,
		// 			lp_amount,
		// 			lp_circulating_supply
		// 		).map_err(|_| ArithmeticError::Overflow)?;

		// 		// Withdraw the vaults assets into the pools account
		// 		let vault_balance_withdrawn = T::Vault::withdraw(
		// 			vault_id,
		// 			&pool_account,
		// 			lp_share_of_asset
		// 		)?;
		// 		// Withdraw the assets now in the pools account into the issuers account
		// 		T::Currency::transfer(*asset_id, pool_account, to, vault_balance_withdrawn, true)
		// 			.map_err(|_| Error::<T>::TransferFromFailed)?;

		// 		assets_withdrawn.push(
		// 			Deposit {
		// 				asset_id: *asset_id,
		// 				amount: vault_balance_withdrawn,
		// 			}
		// 		);
		// 	}

		// 	// Requirement 2) burn the lp tokens that were deposited during this withdraw
		// 	T::Currency::burn_from(pool_info.lp_token_id, to, lp_amount)
		// 		.map_err(|_| Error::<T>::FailedToBurnLpTokens)?;
			
		// 	// Update the pools counter of the circulating supply of lp tokens to subtract the amount burned
		// 	// pool_info.lp_circulating_supply = pool_info.lp_circulating_supply - lp_amount;
		// 	// Pools::<T>::insert(&pool_id, pool_info);

		// 	Ok(assets_withdrawn)
		// }
	}

	// ----------------------------------------------------------------------------------------------------
    //                                  Helper functions - Validity Checks                                 
	// ----------------------------------------------------------------------------------------------------

	// These functions are caled by the Constant Mean Market trait functions to validate the 
	//     input parameters and state space
	impl<T: Config> Pallet<T> {

		// Checks if the input vector has duplicate enteries and returns true if it doesn't, false otherwise
		//  '-> Conditions:
		//        i.  Σ(i, j) a_i != a_j
		fn no_duplicate_assets_provided(assets: &Vec<AssetIdOf<T>>) -> bool {
			let unique_assets = BTreeSet::<T::AssetId>::from_iter(assets.iter().copied());

			// Condition i
			unique_assets.len() == assets.len()
		}

		// Checks that there is a one-to-one correspondence of weights to assets
		//  '-> Conditions:
		//        i.  ∀ assets a_i ⇒ ∃ weight w_i
		pub fn each_asset_has_a_corresponding_weight(
			assets: &Vec<AssetIdOf<T>>, 
			weights: &WeightsVec<AssetIdOf<T>>
		) -> bool {
			if weights.len() != assets.len() {
				return false;
			}

			let assets = BTreeSet::<AssetIdOf<T>>::from_iter(
				assets.iter()
				.copied()
			);

			let weights = BTreeSet::<AssetIdOf<T>>::from_iter(
				weights.iter()
				.map(|weight| weight.asset_id)
			);

			// Condition i
			assets.is_subset(&weights) && assets.is_superset(&weights)
		}

		// Checks the provided weights are all strictly nonnegative (greater than or equal to one) and
		//  |  they sum to one plus or minus a margin of error
		//  '-> Conditions:
		//        i.  w_i ≥ 0
		//        ii. Σ w_i ≈ 1
		pub fn weights_are_normalized_and_nonnegative(weights: &WeightsVec<T::AssetId>) -> bool {
			let zero = Perquintill::zero();

			// Condition i
			for weight in weights {
				if weight.weight < zero {
					return false;
				}
			}
			
			// Condition ii
			let epsilon = T::Epsilon::get().deconstruct();
			let one = Perquintill::one().deconstruct();

			let sum: u64 = weights.iter()
				.map(|weight| weight.weight.deconstruct())
				.sum();

			(one - epsilon) <= sum && sum <= (one + epsilon)
		}

		// Checks the provided weights are all strictly in between (inclusive) the required weight bounds
		//  '-> Conditions:
		//        i. min_weight ≤ w_i ≤ max_weight
		pub fn weights_are_in_weight_bounds(
			weights: &WeightsVec<AssetIdOf<T>>, 
			weight_bounds: &Bound<Perquintill>
		) -> bool {
			// Condition i
			for weight_struct in weights {
				if weight_struct.weight < weight_bounds.minimum || weight_bounds.maximum < weight_struct.weight {
					return false;
				}
			}

			true
		}

		// Checks that, for an all-asset deposit, there is actually one deposit for each asset
		pub fn there_is_one_deposit_for_each_underlying_asset(
			pool_id: &T::PoolId, 
			deposits: &Vec<Deposit<AssetIdOf<T>, BalanceOf<T>>>,
		) -> Result<bool, DispatchError> {
			let underlying_assets: Assets<AssetIdOf<T>> = PoolAssets::<T>::get(&pool_id)
				.ok_or(Error::<T>::PoolDoesNotExist)?;

			let underlying_assets: BTreeSet::<AssetIdOf<T>> = 
				BTreeSet::<AssetIdOf<T>>::from_iter(underlying_assets.iter().copied());

			let deposit_assets: BTreeSet::<AssetIdOf<T>> = 
				BTreeSet::<AssetIdOf<T>>::from_iter(deposits.iter().map(|deposit| deposit.asset_id));

			Ok(underlying_assets == deposit_assets)
		}

		// Checks that the specified user has at least *amount* of *asset* for each asset in the deposit
		//  '-> Conditions:
		//        i. ∀ assets a_i : amount ≤ user_balance
		pub fn user_has_specified_balance_for_deposits(
			user: &T::AccountId,
            deposits: &Vec<Deposit<AssetIdOf<T>, BalanceOf<T>>>                                                                                                                                            
		) -> bool {
			for deposit in deposits {
				if !Self::user_has_specified_balance_for_asset(user, deposit.asset_id, deposit.amount) {
					return false;
				}
			}

			return true;
		}

		// Checks that the specified user has at least *amount* of *asset*
		//  '-> Conditions:
		//        i. amount ≤ user_balance
		pub fn user_has_specified_balance_for_asset(
			user: &T::AccountId,
			asset: T::AssetId,
			amount: T::Balance
		) -> bool {
			amount <= T::Currency::balance(asset, user)
		}

		// Redirects to the correct validity check method depending on if the pool is empty or not as
		//     initial deposits have different requirements for deposit bounds.
		pub fn deposit_is_within_pools_deposit_bounds(
			pool_id: &T::PoolId, 
			deposit: &Deposit<AssetIdOf<T>, BalanceOf<T>>
		) -> Result<bool, DispatchError> {
			let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;
			
			if lp_circulating_supply == (BalanceOf::<T>::zero()) {
				Self::deposit_is_within_empty_pools_deposit_bounds(pool_id, deposit)
			} else {
				Self::deposit_is_within_nonempty_pools_deposit_bounds(pool_id, deposit)
			}
		}

		// Checks that the specified deposit is within initial static deposit minimum and maximum bounds
		pub fn deposit_is_within_empty_pools_deposit_bounds(
			_pool_id: &T::PoolId, 
			_deposit: &Deposit<AssetIdOf<T>, BalanceOf<T>>
		) -> Result<bool, DispatchError> {
			// TODO (Nevin):
			//  - allow pool configurations to have initial deposit limits or have a default initial deposit limit

			Ok(true)
		}

		// Checks that the specified deposit is greater than or equal to the pools reserves of that asset times 
		//     the pools deposit-minimum percentage and less than or equal to the pools reserves of that asset
		//     times the pools deposit-maximum percentage
		pub fn deposit_is_within_nonempty_pools_deposit_bounds(
			pool_id: &T::PoolId, 
			deposit: &Deposit<AssetIdOf<T>, BalanceOf<T>>
		) -> Result<bool, DispatchError> {
			let asset: AssetIdOf<T> = deposit.asset_id;

			// Version 1: ~219 seconds for 10_000 runs of 
			// depositing_into_a_non_empty_pool_with_duplicate_deposits_correctly_mints_lp_tokens
			{
			// let deposit = <T::Convert as Convert<T::Balance, u128>>::convert(deposit.amount);

			// let reserve = Self::balance_of(pool_id, &asset)?;
			// let reserve = <T::Convert as Convert<T::Balance, u128>>::convert(reserve);

			// let deposit_bounds: Bound<Perquintill> = Self::pool_info(pool_id)?.deposit_bounds;
			// let lower_bound = deposit_bounds.minimum * reserve;
			// let upper_bound = deposit_bounds.maximum * reserve;

			// Ok(lower_bound <= deposit && deposit <= upper_bound)
			}
			
			// Version 2: ~217 seconds for 10_000 runs of 
			// depositing_into_a_non_empty_pool_with_duplicate_deposits_correctly_mints_lp_tokens
			
			// TODO: (Kevin)
			//  - there is a lot of boiler plate code here - maybe make a new type or set of functions
			//    that can remove some of this repetitiveness
			
			let deposit: u128 = 
				<T::Convert as Convert<BalanceOf<T>, u128>>::convert(deposit.amount);
			let deposit: FixedBalance = FixedBalance::saturating_from_num(deposit);

			let reserve: BalanceOf<T> = Self::balance_of(pool_id, &asset)?;
			let reserve: u128 = 
				<T::Convert as Convert<BalanceOf<T>, u128>>::convert(reserve);
			let reserve: FixedBalance = FixedBalance::saturating_from_num(reserve);

			let deposit_bounds: Bound<Perquintill> = Self::pool_info(pool_id)?.deposit_bounds;
			let lower_bound: FixedBalance = FixedBalance::from_num(
				deposit_bounds.minimum.deconstruct() as f64 / Perquintill::one().deconstruct() as f64
			);
			let lower_bound: FixedBalance = lower_bound.checked_mul(reserve).ok_or(ArithmeticError::Overflow)?;

			let upper_bound: FixedBalance = FixedBalance::from_num(
				deposit_bounds.maximum.deconstruct() as f64 / Perquintill::one().deconstruct() as f64
			);
			let upper_bound: FixedBalance = upper_bound.checked_mul(reserve).ok_or(ArithmeticError::Overflow)?;

			Ok(lower_bound <= deposit && deposit <= upper_bound)
		}

		// Checks that the value distribution of the deposited asset (deposit.amount / deposit_total) matches
		//     that of the underlying pool's reserves.
		pub fn deposit_matches_underlying_value_distribution(
			pool_id: &T::PoolId, 
			deposit: &Deposit<AssetIdOf<T>, BalanceOf<T>>,
			deposit_total: BalanceOf<T>,
			reserve_total: BalanceOf<T>
		) -> Result<bool, DispatchError> {
			let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;
			
			if lp_circulating_supply == (BalanceOf::<T>::zero()) { 
				return Ok(true);
			}

			// Version 1: ~220 seconds for 10_000 runs of 
			// depositing_into_a_non_empty_pool_with_duplicate_deposits_correctly_mints_lp_tokens
			{
			// let asset: AssetIdOf<T> = deposit.asset_id;
			
			// let epsilon = T::Epsilon::get();

			// let deposit = <T::Convert as Convert<T::Balance, u128>>::convert(deposit.amount);
			// let deposit_total = <T::Convert as Convert<T::Balance, u128>>::convert(deposit_total);
			// let deposit_value_distribution = multiply_by_rational(1, deposit, deposit_total)
			// 	.map_err(|_| ArithmeticError::Overflow)?;

			// let reserve: BalanceOf<T> = Self::balance_of(pool_id, &asset)?;
			// let reserve: u128 = <T::Convert as Convert<BalanceOf<T>, u128>>::convert(reserve);
			// let reserve_total = <T::Convert as Convert<T::Balance, u128>>::convert(reserve_total);
			// let reserve_value_distribution = multiply_by_rational(1, reserve, reserve_total)
			// 	.map_err(|_| ArithmeticError::Overflow)?;

			// let margin_of_error = epsilon * reserve_value_distribution;
			// let lower_bound = reserve_value_distribution - margin_of_error;
			// let upper_bound = reserve_value_distribution + margin_of_error;

			// Ok(lower_bound <= deposit_value_distribution && deposit_value_distribution <= upper_bound)
			}

			// Version 2: ~217 seconds for 10_000 runs of
			//depositing_into_a_non_empty_pool_with_duplicate_deposits_correctly_mints_lp_tokens
			let asset: AssetIdOf<T> = deposit.asset_id;
			
			let deposit: u128 =
				<T::Convert as Convert<T::Balance, u128>>::convert(deposit.amount);
			let deposit: FixedBalance = FixedBalance::saturating_from_num(deposit);

			let deposit_total: u128 =
				<T::Convert as Convert<T::Balance, u128>>::convert(deposit_total);
			let deposit_total: FixedBalance = FixedBalance::saturating_from_num(deposit_total);

			let deposit_value_distribution: FixedBalance = deposit.checked_div(deposit_total)
				.ok_or(ArithmeticError::Overflow)?;

			let reserve: T::Balance = Self::balance_of(pool_id, &asset)?;
			let reserve: u128 =
				<T::Convert as Convert<T::Balance, u128>>::convert(reserve);
			let reserve: FixedBalance = FixedBalance::saturating_from_num(reserve);

			let reserve_total: u128 =
				<T::Convert as Convert<T::Balance, u128>>::convert(reserve_total);
			let reserve_total: FixedBalance = FixedBalance::saturating_from_num(reserve_total);

			let reserve_value_distribution: FixedBalance = reserve.checked_div(reserve_total)
				.ok_or(ArithmeticError::Overflow)?;

			let margin_of_error: Perquintill = T::Epsilon::get();
			let margin_of_error: FixedBalance = FixedBalance::from_num(
				margin_of_error.deconstruct() as f64 / Perquintill::one().deconstruct() as f64
			);

			let lower_bound = reserve_value_distribution - margin_of_error;
			let upper_bound = reserve_value_distribution + margin_of_error;

			Ok(lower_bound <= deposit_value_distribution && deposit_value_distribution <= upper_bound)
		}
	}

	// ----------------------------------------------------------------------------------------------------
    //                              Helper functions - Low-Level Functionality                             
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		// Adds the asset amount to the PoolAssetTotalBalance runtime storage object for the sepcified asset
		pub fn increase_pool_asset_total_balance_storage(
			pool_id: &T::PoolId,
			asset_id: &T::AssetId,
			amount: &T::Balance
		) -> Result<(), DispatchError> {
			PoolAssetTotalBalance::<T>::mutate(
				pool_id,
				asset_id,
				|balance| -> DispatchResult {
					*balance = balance.checked_add(amount)
						.ok_or(ArithmeticError::Overflow)?;
					Ok(())
				},
			)?;

			Ok(())
		}
		
		// Adds the asset amount to the PoolAssetBalance runtime storage object for the sepcified asset
		pub fn increase_pool_asset_balance_storage(
			pool_id: &T::PoolId,
			asset_id: &T::AssetId,
			amount: &T::Balance
		) -> Result<(), DispatchError> {
			PoolAssetBalance::<T>::mutate(
				pool_id,
				asset_id,
				|balance| -> DispatchResult {
					*balance = balance.checked_add(amount)
						.ok_or(ArithmeticError::Overflow)?;
					Ok(())
				},
			)?;

			Ok(())
		}
		
		// Calculates the minimum required creation deposit to open a pool with number_of_assets
		//  |  underlying assets
		//  '-> required_deposit = (number_of_assets + 1) * (existensial_deposit + creation_fee)
		//
		// # Errors
		//  - When calculating the above formula results of the asset amounts results in an overflow error.
		pub fn required_creation_deposit_for(number_of_assets: usize) -> Result<BalanceOf<T>, DispatchError> {
			let number_of_assets = number_of_assets as u128 + 1;
			let existential_deposit = T::ExistentialDeposit::get();
			let creation_fee = T::CreationFee::get();

			let number_of_assets = 
				<T::Convert as Convert<u128, T::Balance>>::convert(number_of_assets);
				
			let result = creation_fee
				.checked_add(&existential_deposit).ok_or(ArithmeticError::Overflow)?
				.checked_mul(&number_of_assets).ok_or(ArithmeticError::Overflow)?;
					
			Ok(result)
		}

		// Calculate the number of lp tokens to mint as a result of this deposit
		//  |-> when depositig funds to an empty pool, initialize lp token ratio to
		//  |       the weighted gemoetric mean of the amount deposited.
		//  '-> when depositing funds to a non-empty pool, the amount of lp tokens minted
		//          should be calculated relative to the increase in the invariant value
		fn calculate_lp_tokens_to_mint(
			pool_id: &T::PoolId,
			deposits: &Vec<Deposit<T::AssetId, T::Balance>>
		) -> Result<T::Balance, DispatchError> {
			let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;

			if lp_circulating_supply == T::Balance::zero() {
				// TODO (Jesper):
				//  - accounting for MIN LIQ too like uniswap (to avoid sybil, and other issues)
				Self::weighted_geometric_mean(&pool_id, &deposits)
			} else {
				Self::increase_in_weighted_geometric_mean(&pool_id, &deposits)
			}
		}

		// Calculates the geometric mean of the deposit amounts provided
		//  '-> weighted geometric mean = Π a_i ^ (w_i), where 1 ≤ i ≤ n, and a_i, w_i are the balance and weight
		//          of asset i 
		//
		// # Errors
		//  - When erros propogate up from obtaining the reserves of the specified pool.
		//  - When calculating an assets balance raised to its weight results in an overflow error.
		//  - When calculating the product of weighted balances results in an overflow error.
		fn weighted_geometric_mean(
			pool_id: &T::PoolId,
			deposits: &Vec<Deposit<T::AssetId, T::Balance>>
		) -> Result<BalanceOf<T>, DispatchError> {
			let mut result: FixedBalance = FixedBalance::from_num(1u8);

			for deposit in deposits {
				let balance: u128 = 
					<T::Convert as Convert<T::Balance, u128>>::convert(deposit.amount);
				let balance: FixedBalance = FixedBalance::saturating_from_num(balance);

				let weight: Perquintill  = PoolAssetWeight::<T>::get(pool_id, deposit.asset_id);
				let weight: FixedBalance = FixedBalance::from_num(
					weight.deconstruct() as f64 / Perquintill::one().deconstruct() as f64
				);

				result = result.checked_mul(
					pow(balance, weight).map_err(|_| ArithmeticError::Overflow)?
				).ok_or(ArithmeticError::Overflow)?;

			}

			let result: u128 = FixedBalance::saturating_to_num::<u128>(result);
			let result: BalanceOf<T> = 
				<T::Convert as Convert<u128, T::Balance>>::convert(result);

			Ok(result)
		}

		// Calculates the number of lp tokens to mint from the deposit amounts provided
		//  '-> lp_minted = lp_circulating_supply * 
		//            sum of (token i's weight * (deposit of token i / balance of token i in pool))
		//
		// # Errors
		//  - When errors propagate up from retrieving the pool's reserves
		//  - When errors propagate up from retrieving the pool's assets
		//  - When errors propagate up from retrieving the amount of LP tokens circulating for a pool
		fn increase_in_weighted_geometric_mean(
			pool_id: &T::PoolId,
			deposits: &Vec<Deposit<T::AssetId, T::Balance>>
		) -> Result<T::Balance, DispatchError> {
			let mut deposit_ratio = FixedBalance::from_num(0 as u8);
			
			for deposit in deposits {
				let deposit_amount: u128 = 
					<T::Convert as Convert<BalanceOf<T>, u128>>::convert(deposit.amount);
				let deposit_amount: FixedBalance = FixedBalance::saturating_from_num(deposit_amount);

				let reserve: BalanceOf<T> = Self::balance_of(pool_id, &deposit.asset_id)?;
				let reserve: u128 = 
					<T::Convert as Convert<BalanceOf<T>, u128>>::convert(reserve);
				let reserve: FixedBalance = FixedBalance::saturating_from_num(reserve);

				let weight: Perquintill  = Self::weight_of(pool_id, &deposit.asset_id)?;
				let weight: FixedBalance = FixedBalance::from_num(
					weight.deconstruct() as f64 / Perquintill::one().deconstruct() as f64
				);

				let asset_ratio = weight * (deposit_amount / reserve);
				deposit_ratio += asset_ratio;
			}

			let lp_circulating_supply: BalanceOf<T> = Self::lp_circulating_supply(pool_id)?;
			let lp_circulating_supply: u128 = 
				<T::Convert as Convert<BalanceOf<T>, u128>>::convert(lp_circulating_supply);
			let lp_circulating_supply: FixedBalance = FixedBalance::saturating_from_num(lp_circulating_supply);

			let lp_tokens_to_mint: FixedBalance = lp_circulating_supply * deposit_ratio;
			let lp_tokens_to_mint: u128 = FixedBalance::saturating_to_num::<u128>(lp_tokens_to_mint);
			let lp_tokens_to_mint: BalanceOf<T> = 
				<T::Convert as Convert<u128, T::Balance>>::convert(lp_tokens_to_mint);

			Ok(lp_tokens_to_mint)
		}

	// 	// Calculates the exact balance of assets that the provided lp tokens (shares) correspond to
	// 	//  '-> LP Share = pool_balance_of_token * (1 - (lp_circulating_supply-lp_amount)/lp_circulating_supply)
	// 	//
	// 	// # Errors
	// 	//  - When calculating the LP Share amount results in an overflow error.
	// 	fn calculate_lps_share_of_pools_asset(
	// 		pool_balance_of_token: T::Balance,
	// 		lp_amount: T::Balance,
	// 		lp_circulating_supply: T::Balance,
	// 	) -> Result<T::Balance, DispatchError> {
	// 		// Convert all three arguments to u128
	// 		let pool_balance_of_token: u128 = 
	// 			<T::Convert as Convert<T::Balance, u128>>::convert(pool_balance_of_token);

	// 		let lp_amount: u128 = 
	// 			<T::Convert as Convert<T::Balance, u128>>::convert(lp_amount);

	// 		let lp_circulating_supply: u128 = 
	// 			<T::Convert as Convert<T::Balance, u128>>::convert(lp_circulating_supply);

	// 		let lp_circulating_minus_amount = lp_circulating_supply
	// 			.checked_sub(lp_amount).ok_or(ArithmeticError::Overflow)?;

	// 		// Calculate the LP Share amount
	// 		let lp_share_of_asset = multiply_by_rational(
	// 			pool_balance_of_token,
	// 			lp_circulating_minus_amount,
	// 			lp_circulating_supply
	// 		).map_err(|_| ArithmeticError::Overflow)?;

	// 		let lp_share_of_asset = pool_balance_of_token
	// 			.checked_sub(lp_share_of_asset).ok_or(ArithmeticError::Overflow)?;

	// 		// Convert back to Balance type
	// 		Ok(<T::Convert as Convert<u128, T::Balance>>::convert(lp_share_of_asset))
	// 	} 
	
	// 	// Calculates the percentage of the given balance and returns it as Balance type
	// 	fn percent_of(
	// 		percentage: Perquintill,
	// 		balance: T::Balance
	// 	) -> T::Balance {
	// 		let balance: u128 = 
	// 			<T::Convert as Convert<T::Balance, u128>>::convert(balance);
	
	// 		<T::Convert as Convert<u128, T::Balance>>::convert(percentage * balance)
	// 	}
	}

}