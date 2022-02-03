#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod traits;

#[cfg(test)]
mod mocks;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod config;

#[frame_support::pallet]
pub mod pallet {
// ----------------------------------------------------------------------------------------------------
//                                       Imports and Dependencies                                      
// ----------------------------------------------------------------------------------------------------
	use crate::traits::CurrencyFactory;
	
	use codec::{Codec, FullCodec};
	use composable_traits::{
		vault::{
			Deposit as Duration, Vault, VaultConfig,
		},
		pool::{
			Bound, Deposit, ConstantMeanMarket, PoolConfig, WeightsVec,
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
	use scale_info::TypeInfo;

	use sp_runtime::{
		helpers_128bit::multiply_by_rational,
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, 
			Convert, One, Zero,
		},
		ArithmeticError, FixedPointNumber, FixedPointOperand/*, FixedU128*/, Perquintill
	};
	use fixed::{
		FixedU128,
		transcendental::pow
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

		// <Self::Vault as Vault>::AssetId
		// Generic Currency bounds. These functions are provided by the `[orml-tokens`](https://github.com/open-web3-stack/open-runtime-module-library/tree/HEAD/currencies) pallet.
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ MutateHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;
		
		// Converts the `Balance` type to `u128`, which internally is used in calculations.
		type Convert: Convert<Self::Balance, u128> + Convert<u128, Self::Balance>;

		// The asset ID used to pay for rent.
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

	pub type PoolIndex = u64;

	pub type AssetIdOf<T> =
		<<T as Config>::Currency as Inspect<<T as SystemConfig>::AccountId>>::AssetId;
	
	#[allow(missing_docs)]
	pub type AccountIdOf<T> = <T as SystemConfig>::AccountId;

	#[allow(missing_docs)]
	pub type BlockNumberOf<T> = <T as SystemConfig>::BlockNumber;

	#[allow(missing_docs)]
	pub type BalanceOf<T> = <T as Config>::Balance;

	// Type alias exists mainly since `crate::config::PoolInfo` has many generic parameters.
	pub type PoolInfo<T> =
		crate::config::PoolInfo<AccountIdOf<T>, AssetIdOf<T>>;

	pub type DepositInfo<T> = 
		Deposit<<T as Config>::AssetId, <T as Config>::Balance>;

	// ----------------------------------------------------------------------------------------------------
    //                                           Runtime  Storage                                          
	// ----------------------------------------------------------------------------------------------------

	// The number of active pools - also used to generate the next pool identifier.
	#[pallet::storage]
	#[pallet::getter(fn pool_count)]
	pub type PoolCount<T: Config> = StorageValue<
		_, 
		PoolIndex, 
		ValueQuery
	>;

	// Mapping of a Pool's index to its PoolInfo struct.
	#[pallet::storage]
	#[pallet::getter(fn pool_info_for)]
	pub type Pools<T: Config> = StorageMap<
		_, 
		Twox64Concat, 
		PoolIndex, 
		PoolInfo<T>, 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn pool_id_and_asset_id_to_weight)]
	pub type PoolIdAndAssetIdToWeight<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		PoolIndex,
		Blake2_128Concat,
		T::AssetId,
		Perquintill,
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn pool_id_and_asset_id_to_vault_id)]
	pub type PoolIdAndAssetIdToVaultId<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, 
		PoolIndex, 
		Blake2_128Concat,
		T::AssetId,
		<T::Vault as Vault>::VaultId,
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn pool_id_to_vault_ids)]
	pub type PoolIdToVaultIds<T: Config> = StorageMap<
		_, 
		Twox64Concat, 
		PoolIndex, 
		Vec<<T::Vault as Vault>::VaultId>,
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn lp_token_to_pool_id)]
	pub type LpTokenToPoolId<T: Config> = StorageMap<
		_, 
		Twox64Concat, 
		T::AssetId, 
		PoolIndex, 
		ValueQuery
	>;

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
			pool_id: PoolIndex,
			// The configuration info related to the pool just created.
			pool_info: PoolInfo<T>,
		},

		// Emitted after a user deposits assets into a pool.
		Deposited {
			// The account issuing the deposit.
			account: AccountIdOf<T>,
			// The pool deposited into.
			pool_id: PoolIndex,
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
			pool_id: PoolIndex,
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

		// Trying to create a pool (that represents N assets) when the issuer has less than 
		//     N * (CreationFee + ExistentialDeposit) native tokens results in:
		IssuerDoesNotHaveBalanceTryingToDeposit,

		// Issues that arise when transfering funds from the user to pools account results in:
		DepositingIntoPoolFailed,

		// Issues that arise when transfering funds into a vaults account results in:
		DepositingIntoVaultFailed,




		// Trying to deposit a number of assets not equivalent to the Pool's underlying assets
		//     results in:
		ThereMustBeOneDepositForEachAssetInThePool,

		// Failure to create an LP tokens during pool creation results in:
		ErrorCreatingLpTokenForPool,

		// Users issuing a request with a pool id that doesn't correspond to an active (created) 
		//     Pool results in:
		PoolDoesNotExist,

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
		// 	pool_id: PoolIndex,
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
		// 	pool_id: PoolIndex,
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
		// 	pool_id: PoolIndex,
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

		type PoolId = PoolIndex;
		type PoolInfo = PoolInfo<T>;

		fn account_id(pool_id: &Self::PoolId) -> Self::AccountId {
			T::PalletId::get().into_sub_account(pool_id)
		}

		fn pool_info(pool_id: &PoolIndex) -> Result<PoolInfo<T>, DispatchError> {
			Ok(Pools::<T>::try_get(pool_id).map_err(|_err| Error::<T>::PoolDoesNotExist)?)
		}

		fn lp_token_id(pool_id: &Self::PoolId) -> Result<Self::AssetId, DispatchError> {
			Ok(Self::pool_info(pool_id)?.lp_token_id)
		}

		fn lp_circulating_supply(pool_id: &Self::PoolId) -> Result<Self::Balance, DispatchError> {
			let lp_token_id = Self::lp_token_id(pool_id)?;

			Ok(T::Currency::total_issuance(lp_token_id))
		}

		fn balance_of(pool_id: &Self::PoolId, asset_id: &Self::AssetId) -> Result<Self::Balance, DispatchError> {
			let vault_id = &PoolIdAndAssetIdToVaultId::<T>::get(pool_id, asset_id);
			let vault_lp_token_id = T::Vault::lp_asset_id(&vault_id)?;
		
			let pool_account = &Self::account_id(pool_id);

			Ok(T::Currency::balance(vault_lp_token_id, pool_account))
		}

		fn reserves_of(pool_id: &Self::PoolId) -> Result<Vec<Deposit<Self::AssetId, Self::Balance>>, DispatchError> {
			let assets = &Self::pool_info(pool_id)?.assets;

			let mut reserves = Vec::<Deposit<Self::AssetId, Self::Balance>>::new();

			for asset in assets {
				reserves.push(Deposit {
					asset_id: *asset,
					amount:   Self::balance_of(pool_id, asset)?,
				});
			}

			Ok(reserves)
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
			// TODO:
			//  - move this check into its own function "user_has_specified_balance_for_asset"
			//  - create another function "user_has_specified_balance_for_deposit" which calls the
			//        above function for each asset in the deposit
			ensure!(
				creation_fee.amount <= T::Currency::balance(T::NativeAssetId::get(), &from), 
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

			// // Requirement 2) the user who issued the extrinsic must have the total balance trying to deposit
			// for deposit in &deposits {
			// 	ensure!(
			// 		T::Currency::balance(deposit.asset_id, &from) >= deposit.amount, 
			// 		Error::<T>::IssuerDoesNotHaveBalanceTryingToDeposit
			// 	);
			// }
			
			// let pool_info = Self::pool_info(pool_id)?;

			// // Requirement 3) There must be one deposit for each asset in the pool
			// // TODO (Nevin):
			// //  - doesn't properly check for one to one. if one asset isn't in the deposit
			// //        and one is listed twice then it still passes. need to check for 
			// //        unique occurences of each asset = assets underlying the pool
			// ensure!(
			// 	pool_info.asset_ids.len() == deposits.len(),
			// 	Error::<T>::ThereMustBeOneDepositForEachAssetInThePool
			// );

			// for deposit in &deposits {
			// 	ensure!(
			// 		PoolIdAndAssetIdToVaultId::<T>::contains_key(pool_id, &deposit.asset_id),
			// 		Error::<T>::ThereMustBeOneDepositForEachAssetInThePool
			// 	);
			// }

			// // Requirement 4) The amount of each asset being deposited must be larger than
			// //     the pools minimum deposit value and smaller than the pools maximum
			// //     deposit value
			
			// // TODO (Nevin):
			// //  - empty pools should have a minimum deposit not based on the percentage of pools assets

			// let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;
			// if lp_circulating_supply != T::Balance::zero() {
			// 	for deposit in &deposits {
			// 		let pool_balance_of_token = Self::balance_of(pool_id, &deposit.asset_id)?;
	
			// 		// Asset amount to deposit must be greater than the pools deposit minimum
			// 		let min_deposit = Self::percent_of(pool_info.deposit_min, pool_balance_of_token);
			// 		ensure!(
			// 			deposit.amount > min_deposit,
			// 			Error::<T>::AmountMustBeGreaterThanMinimumDeposit
			// 		);
	
			// 		// Asset amount to deposit must be less than the pools deposit maximum
			// 		let max_deposit = Self::percent_of(pool_info.deposit_max, pool_balance_of_token);
			// 		ensure!(
			// 			deposit.amount < max_deposit,
			// 			Error::<T>::AmountMustBeLessThanMaximumDeposit
			// 		);
			// 	}
			// }

			// // Requirement 5) Ensure that deposit amount follow the pools weighting metric with some margin of error (T::Epsilon)
			// let mut total: u128 = 0;
			// deposits.iter().for_each(|deposit| 
			// 	total += <T::Convert as Convert<T::Balance, u128>>::convert(deposit.amount)
			// );
			
			// // allow for a margin of error
			// let epsilon = T::Epsilon::get();

			// for deposit in &deposits{				
			// 	let weight = PoolIdAndAssetIdToWeight::<T>::get(&pool_id, deposit.asset_id);
			
			// 	let amount = <T::Convert as Convert<T::Balance, u128>>::convert(deposit.amount);
			// 	ensure!(
			// 		(weight * total) - (epsilon * total) <= amount && amount <= (weight * total) + (epsilon * total),
			// 		Error::<T>::DepositDoesNotMatchWeightingMetric
			// 	);
			// }

			let lp_tokens_minted = Self::do_deposit(&from, &pool_id, deposits.clone())?;
			
			Self::deposit_event(Event::Deposited { 
				account:          from.clone(), 
				pool_id:          pool_id.clone(), 
				deposited:        deposits, 
				lp_tokens_minted: lp_tokens_minted,
			});

			Ok(lp_tokens_minted)
		}

		// fn single_asset_deposit(
		// 	from: &Self::AccountId,
		// 	pool_id: &Self::PoolId,
		// 	deposit: Deposit<Self::AssetId, Self::Balance>,
		// ) -> Result<Self::Balance, DispatchError> {
		// 	let lp_tokens_minted = Self::do_single_asset_deposit(&from, &pool_id, deposit.clone())?;

		// 	Ok(lp_tokens_minted)
		// }

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
		// 		PoolIdAndAssetIdToWeight::<T>::insert(&pool_id, share.asset_id, share.weight);
		// 	}

		// 	Ok(())
		// }
	}

	// Helper functions - core functionality
	impl<T: Config> Pallet<T> {
		// Helper function for the the create extrinsic. Obtains a new LP Token Id for the pool, creates a 
		//     vault for each asset desired in the pool, and saves all important info into storage
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
		) -> Result<(PoolIndex, PoolInfo<T>), DispatchError>  {
			PoolCount::<T>::try_mutate(|id| {
				let id = {
					*id += 1;
					*id
				};

				// Requirement 1) Obtain a new asset id for this pools lp token 
				let lp_token_id =
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
					
					PoolIdAndAssetIdToVaultId::<T>::insert(id, asset_id, vault_id.clone());
				}

				// Requirement 3) Transfer native tokens from users account to pools account
				let pool_account = &Self::account_id(&id);
				T::Currency::transfer(creation_fee.asset_id, &from, pool_account, creation_fee.amount, true)
					.map_err(|_| Error::<T>::DepositingIntoPoolFailed)?;

				// Requirement 4) Transfer a portion of the cretion fee into each vault
				let vault_creation_fee = T::ExistentialDeposit::get() + T::CreationFee::get();

				for asset_id in &config.assets {
					T::Currency::transfer(
						creation_fee.asset_id,
						&pool_account,
						&<T::Vault as Vault>::account_id(&PoolIdAndAssetIdToVaultId::<T>::get(id, asset_id)),
						vault_creation_fee,
						true,
					).map_err(|_| Error::<T>::DepositingIntoVaultFailed)?;
				}
				
				// Requirement 5) Save each assets weights in global storage
				for share in &config.weights {
					PoolIdAndAssetIdToWeight::<T>::insert(&id, share.asset_id, share.weight);
				}

				// Requirement 6) Keep track of the pool's configuration
				let pool_info = PoolInfo::<T> {
					manager:		 config.manager,
					assets:			 config.assets,
					asset_bounds:	 config.asset_bounds,
					weights:		 config.weights,
					weight_bounds:	 config.weight_bounds,
					deposit_bounds:	 config.deposit_bounds,
					withdraw_bounds: config.withdraw_bounds,
					transaction_fee: config.transaction_fee,
					lp_token_id,
				};

				Pools::<T>::insert(id, pool_info.clone());
				LpTokenToPoolId::<T>::insert(lp_token_id, id);

				Ok((id, pool_info))
			})
		}

		fn do_deposit(
			from: &T::AccountId,
			pool_id: &PoolIndex,
			deposits: Vec<Deposit<T::AssetId, T::Balance>>,
		) -> Result<T::Balance, DispatchError> {
			
			let pool_info = Pools::<T>::get(&pool_id);
			let to = &Self::account_id(pool_id);

			// TODO (Nevin):
			//  - transfer all assets into the pool's account before beginning
			//     '-> should be its own function to abstract away details

			// Requirement 1) Deposit each asset into the pool's underlying vaults
			for deposit in &deposits {
				// TODO (Nevin):
				//  - check for errors in vault depositing, and if so revert all deposits so far
				//     '-> surround T::Vault::deposit call in match statement checking for Ok(lp_token_amount)
				//             and DispatchError

				let vault_id = &PoolIdAndAssetIdToVaultId::<T>::get(pool_id, deposit.asset_id);

				T::Currency::transfer(deposit.asset_id, &from, to, deposit.amount, true)
					.map_err(|_| Error::<T>::DepositingIntoPoolFailed)?;

				let _vault_lp_token_amount = T::Vault::deposit(
					vault_id,
					to,
					deposit.amount,
				)?;
			}

			// Requirement 2) Calculate the number of lp tokens to mint as a result of this deposit
			//  |-> when depositig funds to an empty pool, initialize lp token ratio to
			//  |       the weighted gemoetric mean of the amount deposited. (generalizes Uniswap V2's 
			//  |       methodology for minting lp tokens when pool is empty)
			//  '-> when depositing funds to a non-empty pool, the amount of lp tokens minted
			//          should be equivalent to the product of the current curculating supply
			//          of lp tokens for this pool and the ratio of assets deposited to the
			//          pools balance of the tokens

			let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;
			let lp_tokens_to_mint = if lp_circulating_supply == T::Balance::zero() {
				// TODO (Jesper):
				//  - accounting for MIN LIQ too like uniswap (to avoid sybil, and other issues)

				Self::weighted_geometric_mean(&pool_id)
			} else {
				Self::calculate_lp_tokens_to_mint(&pool_id, &deposits)
			}.map_err(|_| ArithmeticError::Overflow)?;

			// Requirement 3) Mint Pool's lp tokens into the issuers account and update 
			//     circulating supply of lp tokens

			T::Currency::mint_into(pool_info.lp_token_id, from, lp_tokens_to_mint)
				.map_err(|_| Error::<T>::FailedToMintLpTokens)?;

			// pool_info.lp_circulating_supply = pool_info.lp_circulating_supply + lp_tokens_to_mint;
			// Pools::<T>::insert(&pool_id, pool_info);

			Ok(lp_tokens_to_mint)
		}

		// fn do_single_asset_deposit(
		// 	from: &T::AccountId,
		// 	pool_id: &PoolIndex,
		// 	deposits: Deposit<T::AssetId, T::Balance>,
		// ) -> Result<T::Balance, DispatchError> {
		// 	let lp_circulating_supply = <Self as Pool>::lp_circulating_supply(pool_id);
			
		// 	let lp_tokens_to_mint = lp_circulating_supply;

		// 	lp_tokens_to_mint
		// }

		// fn do_withdraw(
		// 	to: &T::AccountId,
		// 	pool_id: &PoolIndex,
		// 	lp_amount: T::Balance,
		// ) -> Result<Vec<Deposit<T::AssetId, T::Balance>>, DispatchError> {
		// 	let pool_account = &Self::account_id(pool_id);

		// 	let pool_info = Pools::<T>::get(&pool_id);
		// 	let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;

		// 	// Used to keep track of the amount of each asset withdrawn from the pool's underlying vaults
		// 	let mut assets_withdrawn = Vec::<Deposit<T::AssetId, T::Balance>>::new();

		// 	// Requirement 1) Calculate and withdraw the lp tokens share of the each asset in the pool
		// 	for asset_id in &pool_info.asset_ids {
		// 		let vault_id = &PoolIdAndAssetIdToVaultId::<T>::get(pool_id, asset_id);
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


	// Helper functions - validity checks
	//     these functions are claled by the Constant Mean Market trait functions to validate the 
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
			
	}

	// Helper functions - low-level functionality
	impl<T: Config> Pallet<T> {
		// Calculates the minimum required creation deposit to open a pool with number_of_assets
		//  |  underlying assets
		//  '-> required_deposit = (number_of_assets + 1) * (existensial_deposit + creation_fee)
		//
		// # Errors
		//  - When calculating the above formula results of the asset amounts results in an overflow error.
		pub fn required_creation_deposit_for(number_of_assets: usize) -> Result<T::Balance, DispatchError> {
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

		// Calculates the geometric mean of the deposit amounts provided
		//  '-> weighted geometric mean = Π a_i ^ (w_i), where 1 ≤ i ≤ n, and a_i, w_i are the balance and weight
		//          of asset i 
		//
		// # Errors
		//  - When calculating the product of the asset amounts results in an overflow error.
		fn weighted_geometric_mean(pool_id: &PoolIndex) -> Result<T::Balance, DispatchError> {
			let pool_reserves = Self::reserves_of(pool_id)?;
		
			// let mut result = FixedU128::one();
			// let mut result = FixedU128::
			// for reserve in pool_reserves {
			// 	let weight = PoolIdAndAssetIdToWeight::<T>::get(pool_id, reserve.asset_id);

			// 	let weighted_balance = FixedU128::saturating_from_integer(reserve.amount) 
			// 		.pow(weight.into());
			// 		// FixedU128::saturating_from_rational(weight.deconstruct(), Perquintill::one().deconstruct()).to_float();

			// 	// let weight = weight.deconstruct() / Perquintill::one().deconstruct();
			// 	// let weighted_balance = reserve.amount as u128 ^ (weight.deconstruct() / Perquintill::one().deconstruct());
			// 	// let weighted_balance = pow(
			// 	// 	FixedU128::saturating_from_num(reserve.amount.into()), 
			// 	// 	FixedU128::saturating_from_num(weight.deconstruct(), Perquintill::one().deconstruct())
			// 	// );
			// 	// FixedU128::saturating_from_integer(reserve.amount).pow(
			// 	// 	// weight;
			// 	// 	FixedU128::saturating_from_rational(weight.deconstruct(), Perquintill::one().deconstruct()));
				
			// 	result.checked_mul(weighted_balance).ok_or(ArithmeticError::Overflow)?;
			// }
			
			let number_of_assets = pool_reserves.len() as u32;
			
			Ok(T::Balance::one())
		}

		// Calculates the geometric mean of the deposit amounts provided
		//  '-> geometric mean = nth-√(Π ai), where 1 ≤ i ≤ n, and ai is the ith asset balance
		//
		// # Errors
		//  - When calculating the product of the asset amounts results in an overflow error.
		fn geometric_mean(deposits: &Vec<Deposit<T::AssetId, T::Balance>>) -> Result<T::Balance, DispatchError> {
			let number_of_assets = deposits.len() as u32;
			let mut result = T::Balance::one();
		
			for deposit in deposits {
				result = result.checked_mul(&deposit.amount).ok_or(ArithmeticError::Overflow)?;
			}
					
			Ok(result.nth_root(number_of_assets))
		}

		// Calculates the number of lp tokens to mint from the deposit amounts provided
		//  '-> lp tokens to mint = lp_circulating_supply * (ai/balance_i), where 1 ≤ i ≤ n
		//
		// # Errors
		//  - When calculating the product of the asset ratio and the supply of lp tokens amounts results in an overflow error.
		fn calculate_lp_tokens_to_mint(
			pool_id: &PoolIndex,
			deposits: &Vec<Deposit<T::AssetId, T::Balance>>
		) -> Result<T::Balance, DispatchError> {
			// let lp_circulating_supply = Pools::<T>::get(pool_id).lp_circulating_supply;

			// let mut lp_tokens_to_mint = T::Balance::zero();

			// for deposit in deposits {
			// 	let weight = PoolIdAndAssetIdToWeight::<T>::get(pool_id, deposit.asset_id);
				
			// 	let vault_id = &PoolIdAndAssetIdToVaultId::<T>::get(pool_id, deposit.asset_id);
			// 	let vault_account = &<T::Vault as Vault>::account_id(vault_id);

			// 	let pool_balance_of_token = T::Currency::balance(deposit.asset_id, vault_account);

			// 	let lp_tokens_to_mint_for_asset = multiply_by_rational(
			// 		<T::Convert as Convert<T::Balance, u128>>::convert(lp_circulating_supply),
			// 		<T::Convert as Convert<T::Balance, u128>>::convert(deposit.amount),
			// 		<T::Convert as Convert<T::Balance, u128>>::convert(pool_balance_of_token),
			// 	).map_err(|_| ArithmeticError::Overflow)?;

			// 	let lp_tokens_to_mint_for_asset = weight * lp_tokens_to_mint_for_asset;

			// 	let lp_tokens_to_mint_for_asset =
			// 		<T::Convert as Convert<u128, T::Balance>>::convert(lp_tokens_to_mint_for_asset);

			// 	lp_tokens_to_mint = lp_tokens_to_mint.checked_add(&lp_tokens_to_mint_for_asset)
			// 		.ok_or(ArithmeticError::Overflow)?;

			// }

			// Ok(lp_tokens_to_mint)

			// TODO (Nevin):
			//  - generalize this formula to sum all deposit rations multiplied by their weights
			//  - check that this formula is still accurate when weights are off balance before rebalancing
			//    '-> lp_minted = lp_circulating_supply * 
			//            sum of (token i's weight * (deposit of token i / balance of token i in pool))
			let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;

			let deposit = &deposits[0];
			let vault_id = &PoolIdAndAssetIdToVaultId::<T>::get(pool_id, deposit.asset_id);
			let vault_id = &<T::Vault as Vault>::account_id(vault_id);

			let pool_balance_of_token = T::Currency::balance(deposit.asset_id, vault_id);
			let deposit_ratio = multiply_by_rational(
				<T::Convert as Convert<T::Balance, u128>>::convert(lp_circulating_supply),
				<T::Convert as Convert<T::Balance, u128>>::convert(deposit.amount),
				<T::Convert as Convert<T::Balance, u128>>::convert(pool_balance_of_token),
			).map_err(|_| ArithmeticError::Overflow)?;

			Ok(<T::Convert as Convert<u128, T::Balance>>::convert(deposit_ratio))
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