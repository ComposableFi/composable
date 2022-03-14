//! <!-- Original author of documentation: @kevin | Composable -->

//! # Pool Pallet
//! 
//! The Pool pallet provides the functioanlity behind [`Balancer`](https://balancer.fi/whitepaper.pdf)-like Constant Mean Markets.
//! 
//! ## Overview
//! 
//! The Pool pallet ale can be executed. The trading funciton is said to be invariant as a trade is only
//! considered valid if itlows users to create and interact with weighted portfolios (Pools) of assets. 
//! Liquidity providers (LPs) provide assets to these pools in return for LP tokens that represent their 
//! pro-rata share of the Pool’s reserves. These LP tokens can then be deposited back into the Pool to 
//! collect the equivalent share of Pool reserves and any fees that were earned through trades. Similarly, 
//! users are able to trade assets with the Pool at a price determined intrinsically. The API provided by 
//! the Pool pallet includes:
//! 
//! - Pool Creation
//! - Deposits
//! - Withdraws
//! - Trades
//! - Price Querying
//! 
//! To use the Pool pallet in your runtime, you need to implement the Pools [`Config`].
//! 
//! ### Terminology
//! 
//! - **Constant Function Market Maker**: A family of automated market makers that is defined by an invariant
//!     trading function parameratized by the markets reserves. The trading function determines which actions (deposits/withdraws/trades)
//!     are valid and therefor keeps the trading function constant.
//! 
//! - **Constant Mean Market**: A generalization of Constant Prouct Market Makers that utilize the weighted 
//!     geometric mean as its invariant function, allowing for more than two assets.
//! 
//! - **Pool**: An active instance of a Constant Mean Market that represents a weighted portfolio of assets that 
//!     users can interact with.
//! 
//! - **Weight**: A value identifying the Pool's distribution of value between its underlying assets.
//!     Pool weights must be nonnegative and sum to one.
//! 
//! - **Join or Creation**: The process of a liquidity provider depositing funds into a Pool and receiving an amount of
//!     LP tokens in return.
//! 
//! - **LP Token**: An asset that represents a liquidity providers pro-rata share of the Pool's total assets and 
//!     any rewards earned through trades within the Pool.
//! 
//! - **Leave or Redemption**: The process of a liquidity provider withdrawing funds from the Pool by supplying an amount of
//!     LP tokens to the Pool.
//! 
//! - **Spot Price**: The Pool's current price of a given asset at which it can be bought or sold for
//!     immediate delivery. 
//! 
//! - **Numeraire**: An asset that acts as a measure of value for a currency exchange.
//! 
//! ### Goals
//! 
//! The Pool pallet is designed to make the following possible:
//! 
//! - Create completely configurable n-asset weighted portfolios.
//! - Ability to join and leave Pools.
//! - Trade assets with Pools.
//! - Ability to query the spot price of an asset in terms of a numeraire.
//! 
//! ### Actors
//!
//! There are many external entities who will interact with the Pool pallet's public API. Below are the common
//! actors for the pallet:
//! - **Trader**: A user that exchanges one asset in a Pool for another.
//! 
//! - **Arbitraguer**: A type of trader who faciliatates trades bewteen the Pool and an external market to
//!     obtain a profit when the intrinsic prices bewteen the markets differentiate.
//! 
//! - **Liquidity Provider**: A user who funds a Pool by depositing assets, thus allowing trades to occur within
//!     the pool.
//! 
//! - **Other Pallets**: The Pool pallet is designed to be utilized as an underlying layer of other pallets, 
//!     not as a standalone pallet.
//! 
//! ### Implementations
//! - [`ConstantMeanMarket`](composable_traits::pool::ConstantMeanMarket): Functions for providing the defualt 
//!     Constant Mean Market implementation.
//! 
//! ## Interface
//! 
//! ### Extrinsics
//! 
//! At this time, the Pool pallet does not provide any native extrinsic functions. This pallet is meant to be 
//!     utilized by other pallets and thus does not expect any calls from outside the runtime. For this reason
//!     the [`Call`] enum is left empty.
//! 
//! ### Implmented Functions
//! 
//! #### [ConstantMeanMarket](composable_traits::pool::ConstantMeanMarket)
//! 
//! The Pool pallet implements the following functions for the ConstantMeanMarket trait:
//! 
//! - [`create`](composable_traits::pool::ConstantMeanMarket::create): Creates a unique Pool, taking a 
//!     required creation fee.
//! 
//! - [`all_asset_deposit`](composable_traits::pool::ConstantMeanMarket::all_asset_deposit): Transfers assets 
//!     into the Pool and mints LP tokens for the issuer.
//! 
//! - [`all_asset_withdraw`](composable_traits::pool::ConstantMeanMarket::all_asset_withdraw): Transfers assets
//!     out of the Pool and burns the deposited LP tokens.  
//! 
//! - [`spot_price`](composable_traits::pool::ConstantMeanMarket::spot_price): Queries the price of an asset in 
//!     terms of a numeraire asset.
//! 
//! ### Public Functions
//! 
//! - [`account_id`](Pallet::account_id): Returns the account associated with a given Pool.
//! 
//! - [`pool_info`](Pallet::pool_info): Returns the `PoolInfo` struct for a given Pool.
//! 
//! - [`lp_token_id`](Pallet::lp_token_id): Returns the LP token associated with a given Pool.
//! 
//! - [`lp_circulating_supply`](Pallet::lp_circulating_supply): Returns the total number of minted LP tokens
//!     for a Pool.
//! 
//! - [`reserves_of`](Pallet::reserves_of): Returns the Pool's total reserves.
//! 
//! - [`balance_of`](Pallet::balance_of): Returns the Pool's reserve of an asset.
//! 
//! - [`weight_of`](Pallet::weight_of): Returns the Pool's weight associated with an asset.
//! 
//! Please refer to the [`Pallet`] struct for details on all publicly available functions.
//! 
//! ### Runtime Storage Objects:
//! 
//! - [`PoolCount`](PoolCount): A Counter for the number of Pools that have been created. Might not
//!     correspond to the number of active Pools if a Pool has been deleted.
//! 
//! - [`Pools`](Pools): Mapping of a Pool's `pool_id` to its [`PoolInfo`](composable_traits::pool::PoolInfo) struct.
//! 
//! - [`PoolAssets`](PoolAssets): Mapping of a Pool's `pool_id` to a vector of assets underlying the Pool.
//! 
//! - [`PoolAssetWeight`](PoolAssetWeight): Mapping of a Pool's `pool_id` and an assets `asset_id` to the weight
//!     that asset maintains in the Pool.
//! 
//! - [`PoolAssetVault`](PoolAssetVault): Mapping of a Pool's `pool_id` and an assets `asset_id` to the
//!     underlying Cubic Vault's `vault_id` that holds the asset for the Pool.
//! 
//! - [`PoolAssetBalance`](PoolAssetBalance): Mapping of a Pool's `pool_id` and an assets `asset_id` to the Pool's
//!     reserves of the asset. (Doesn't include the assets reserves that were taken as transaction fees).
//! 
//! - [`PoolAssetTotalBalance`](PoolAssetTotalBalance): Mapping of a Pool's `pool_id` and an assets `asset_id` to the Pool's
//!     total reserves of the asset. (Includes the assets reserves that were taken as transaction fees).
//! 
//! ## Usage
//! 
//! The following examples show how to use the Pool pallet in your custom pallet.
//! 
//! ### Example
//! 
//! ### Example
//! 
//! ## Related Modules
//! - [`Ensemble Pallet`]
//! - [`Index Pallet`]
//! - [`Vaults Pallet`](../pallet_vault/index.html)

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
			Assets, Bound, ConstantMeanMarket, FixedBalance, Reserve, PoolConfig, 
			PoolInfo, WeightsVec,
		},
	};
	
	use frame_support::{
		ensure,
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::{fungibles::MutateHold, WithdrawConsequence},
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
		helpers_128bit::multiply_by_rational,
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, 
			Convert, One, Zero,
		},
		ArithmeticError, FixedPointOperand, Perquintill, PerThing
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
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Loosely couple the Vault trait providing local types for the vaults associated types.
		type Vault: Vault<
			AccountId = Self::AccountId,
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			BlockNumber = Self::BlockNumber
		>;

		/// The Balance type used by the pallet for bookkeeping. `Config::Convert` is used for
		/// conversions to `u128`, which are used in the computations.
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

		/// The Weight type used by the pallet to represent asset weights.
		type Weight: PerThing
			+ FullCodec
			+ Default
			+ Encode
			+ Decode
			+ TypeInfo;

		/// The pallet creates new LP tokens for every pool created. It uses `CurrencyFactory`, as
		/// 	`orml`'s currency traits do not provide an interface to obtain asset ids (to avoid id
		/// 	collisions).
		type CurrencyFactory: CurrencyFactory<Self::AssetId>;

		/// The `AssetId` used by the pallet. Corresponds the the Ids used by the Currency pallet.
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo
			+ Ord;

		/// Generic Currency bounds. These functions are provided by the `[orml-tokens`](https://github.com/open-web3-stack/open-runtime-module-library/tree/HEAD/currencies) pallet.
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ MutateHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;
		
		/// Converts the `Balance` type to `u128`, which internally is used in calculations.
		type Convert: Convert<Self::Balance, u128> + Convert<u128, Self::Balance>;

		/// The ID used to uniquely represent Pools
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

		/// The asset ID used to pay transaction fees.
		#[pallet::constant]
		type NativeAssetId: Get<Self::AssetId>;

		/// The native asset fee needed to create a vault.
		#[pallet::constant]
		type CreationFee: Get<Self::Balance>;

		/// The deposit needed for a pool to never be cleaned up.
		#[pallet::constant]
		type ExistentialDeposit: Get<Self::Balance>;

		/// The margin of error when calculating the Pool's math behind the scenes.
		///     Pool Creation: initial weights, when normalized, must add up into the range
		///         1 - epsilon <= weights <= 1 + epsilon
		///     Pool Deposits: deposit weights, when normalized by the total deposit amount,
		///         must add up into the range 1 - epsilon <= deposit <= 1 + epsilon
		#[pallet::constant]
		type Epsilon: Get<Self::Weight>;

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

	#[allow(missing_docs)]
	pub type WeightOf<T> = <T as Config>::Weight;

	// Type alias exists mainly since `PoolInfo` has two generic parameters.
	pub type PoolInfoOf<T> = PoolInfo<AccountIdOf<T>, AssetIdOf<T>, WeightOf<T>>;

	// type synonym to better represent the `Reserve` type in deposits
	pub type Deposit<T> = Reserve<AssetIdOf<T>, BalanceOf<T>>;

	// type synonym to better represent the `Reserve` type in withdrawals
	pub type Withdraw<T> = Reserve<AssetIdOf<T>, BalanceOf<T>>;
	// ----------------------------------------------------------------------------------------------------
    //                                           Runtime  Storage                                          
	// ----------------------------------------------------------------------------------------------------

	/// The number of active pools - also used to generate the next pool identifier.
	#[pallet::storage]
	#[pallet::getter(fn pool_count)]
	pub type PoolCount<T: Config> = StorageValue<_, T::PoolId, ValueQuery>;

	/// Mapping of a Pool's Id to its PoolInfo struct.
	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> = 
		StorageMap<_, Twox64Concat, T::PoolId, PoolInfoOf<T>, ValueQuery>;

	/// Assets tracked by the pool
	#[pallet::storage]
	#[pallet::getter(fn pool_assets)]
	pub type PoolAssets<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PoolId, Assets<T::AssetId>>;

	/// Weights for each asset in the pool
	#[pallet::storage]
	#[pallet::getter(fn pool_asset_weight)]
	pub type PoolAssetWeight<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::AssetId,
		T::Weight,
		ValueQuery
	>;

	/// Mapping of the Pool's Id and an Assets Id to the corresponding vault
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

	/// Balance of asset for given pool excluding admin_fee
	#[pallet::storage]
	#[pallet::getter(fn pool_asset_balance)]
	// Absence of pool asset balance is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_types)]
	pub type PoolAssetBalance<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PoolId,
		Blake2_128Concat,
		T::AssetId,
		T::Balance,
		ValueQuery,
	>;

	/// Balance of asset for given pool including admin_fee
	#[pallet::storage]
	#[pallet::getter(fn pool_asset_total_balance)]
	// Absence of pool asset balance is equivalent to 0, so ValueQuery is allowed.
	#[allow(clippy::disallowed_types)]
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
		/// Emitted after a Pool has been successfully created.
		PoolCreated {
			/// The Id of the created Pool.
			pool_id: T::PoolId,
			/// The configuration info related to the Pool that was created.
			pool_info: PoolInfoOf<T>,
		},

		/// Emitted after a user deposits an all-asset deposit into a pool.
		AllAssetDeposit {
			/// The account id of the user issuing the deposit.
			account: AccountIdOf<T>,
			/// The pool id of the Pool deposited into.
			pool_id: T::PoolId,
			/// A vector of asset ids and corresponding balance deposited.
			deposited: Vec<Deposit<T>>,
			/// The number of LP tokens minted for the deposit.
			lp_tokens_minted: BalanceOf<T>,
		},

		/// Emitted after a user withdraws assets from a pool.
		AllAssetWithdraw {
			/// The account issuing the deposit.
			account: AccountIdOf<T>,
			/// The pool deposited into.
			pool_id: T::PoolId,
			/// The asset ids and corresponding amount withdrawn.
			withdrawn: Vec<Withdraw<T>>,
			/// The number of LP tokens burned from the withdraw.
			lp_tokens_burned: BalanceOf<T>,
		},
	}

	// ----------------------------------------------------------------------------------------------------
    //                                           Runtime  Errors                                           
	// ----------------------------------------------------------------------------------------------------

	/// Custom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors/) of this pallet.
	/// The following errors are very verbose and try to give as much context to why they arise.
	#[pallet::error]
	pub enum Error<T> {
		/// A Pool config, or deposit, containing an asset more than once was provided.
		///
		/// # Related Functions:
		/// - [`create`](composable_traits::pool::ConstantMeanMarket::create)
		/// - [`all_asset_deposit`](composable_traits::pool::ConstantMeanMarket::all_asset_deposit)
		DuplicateAssets,

		/// A Pool config with asset bounds where the maximum amount of assets is less than
		/// 	the minimum amount of assets was provided.
		///
		/// # Related Functions:
		/// - [`create`](composable_traits::pool::ConstantMeanMarket::create)
		InvalidAssetBounds,

		/// A Pool config that contains n assets where n is less than the minimum asset bound or greater
		/// 	than the maximum asset bound was provided.
		///
		/// # Related Functions:
		/// - [`create`](composable_traits::pool::ConstantMeanMarket::create)
		PoolSizeIsOutsideOfAssetBounds, 

		/// A Pool config that doesn't contain a one to one correspondance of weights to assets w
		/// 	as provided.
		///
		/// # Related Functions:
		/// - [`create`](composable_traits::pool::ConstantMeanMarket::create)
		ThereMustBeOneWeightForEachAssetInThePool,
		
		/// A Pool config that contains a set of asset weights where at least one weight is less
		///     than zero was provided.
		///
		/// # Related Functions:
		/// - [`create`](composable_traits::pool::ConstantMeanMarket::create)
		PoolWeightsMustBeNonnegative,

		/// A Pool config that contains a set of asset weights that do not sum to one was provided.
		///
		/// # Related Functions:
		/// - [`create`](composable_traits::pool::ConstantMeanMarket::create)
		PoolWeightsMustBeNormalized,

		/// A Pool config with weight bounds where the maximum weight bound is less than
		/// 	the minimum weight bound was provided.
		///
		/// # Related Functions:
		/// - [`create`](composable_traits::pool::ConstantMeanMarket::create)
		InvalidWeightBounds,

		/// A Pool config that contains a set of weights where at least one of the weights is less 
		/// 	than the minimum weight bound or greater than the maximum weight bound was provided.
		///
		/// # Related Functions:
		/// - [`create`](composable_traits::pool::ConstantMeanMarket::create)
		PoolWeightsAreOutsideOfWeightBounds,

		/// The issuer of the `create` function provided an amount of native tokens that is insufficient
		/// 	to create a Pool of the specified size.
		///
		/// # Related Functions:
		/// - [`create`](composable_traits::pool::ConstantMeanMarket::create)
		CreationFeeIsInsufficient,

		// Users issuing a request with a pool id that doesn't correspond to an active (created) 
		//     Pool results in:
		/// A [`pool_id`] was provided that does not correspond to an active Pool.
		///
		/// # Related Functions:
		/// - [`create`](composable_traits::pool::ConstantMeanMarket::create)
		/// - [`all_asset_deposit`](composable_traits::pool::ConstantMeanMarket::all_asset_deposit)
		/// - [`all_asset_withdraw`](composable_traits::pool::ConstantMeanMarket::all_asset_withdraw)
		/// - [`spot_price`](composable_traits::pool::ConstantMeanMarket::spot_price)
		PoolDoesNotExist,

		/// An all-asset-deposit that does not contain one entry for each of the Pool's underlying 
		/// 	assets was provided.
		///
		/// # Related Functions:
		/// - [`all_asset_deposit`](composable_traits::pool::ConstantMeanMarket::all_asset_deposit)
		ThereMustBeOneDepositForEachAssetInThePool,

		/// An all-asset-deposit containing a balance of zero for one of its deposits was provided.
		///
		/// # Related Functions:
		/// - [`all_asset_deposit`](composable_traits::pool::ConstantMeanMarket::all_asset_deposit)
		DepositsMustBeStrictlyPositive,

		/// The issuer of the function specified a balance of assets greater than they own.
		///
		/// # Related Functions:
		/// - [`create`](composable_traits::pool::ConstantMeanMarket::create)
		/// - [`all_asset_deposit`](composable_traits::pool::ConstantMeanMarket::all_asset_deposit)
		/// - [`all_asset_withdraw`](composable_traits::pool::ConstantMeanMarket::all_asset_withdraw)
		IssuerDoesNotHaveBalanceTryingToDeposit,

		/// A deposit that would increase the Pool's constant outside the bounds specified by
		/// 	the Pool's deposit bounds was provided.
		///
		/// # Related Functions:
		/// - [`all_asset_deposit`](composable_traits::pool::ConstantMeanMarket::all_asset_deposit)
		DepositIsOutsideOfPoolsDepositBounds,

		/// An all-asset-deposit that contains a value distribution not matching that of the Pool's 
		///		was provided.
		/// 
		/// # Related Functions:
		/// - [`all_asset_deposit`](composable_traits::pool::ConstantMeanMarket::all_asset_deposit)
		DepositDoesNotMatchUnderlyingValueDistribution,

		/// An issue arised when trying to reserve an LP token for the Pool.
		///
		/// # Related Functions:
		/// - [`do_all_asset_deposit`](Pallet::do_all_asset_deposit)
		ErrorCreatingLpTokenForPool,

		/// An issue arised when transfering funds from the users account into the Pool's account.
		///
		/// # Related Functions:
		/// - [`do_all_asset_deposit`](Pallet::do_all_asset_deposit)
		DepositingIntoPoolFailed,

		/// An issue arised when transfering funds from the users account into the Pool's underlying Vault
		/// 	accounts.
		///
		/// # Related Functions:
		/// - [`do_all_asset_deposit`](Pallet::do_all_asset_deposit)
		DepositingIntoVaultFailed,

		/// An issue arised when trying to minting the Pool's LP token into the issuers account.
		///
		/// # Related Functions:
		/// - [`do_all_asset_deposit`](Pallet::do_all_asset_deposit)
		FailedToMintLpTokens,

		
		/// The issuer of the function specified an `asset` or `numeraire` that is not currently 
		/// 	in the Pool.
		///
		/// # Related Functions:
		/// - [`spot_price`](composable_traits::pool::ConstantMeanMarket::spot_price)
		AssetIsNotTrackedByPool,

		/// The number of LP tokens provided would decrease the Pool's constant outside the bounds 
		/// 	specified by the Pool's withdraw bounds.
		/// 
		/// # Related Functions:
		/// - [`all_asset_withdraw`](composable_traits::pool::ConstantMeanMarket::all_asset_withdraw)
		WithdrawIsOutsideOfPoolsWithdrawBounds,

		/// The number of LP tokens provided would result in a withdraw that does not match the Pool's
		/// 	value distribution. Note: currently only useful for tests.
		/// 
		/// # Related Functions:
		/// - [`all_asset_withdraw`](composable_traits::pool::ConstantMeanMarket::all_asset_withdraw)
		WithdrawDoesNotMatchUnderlyingValueDistribution,




		// Users trying to deposit an asset amount that is smaller than the Pools configured maximum 
		//     withdraw results in:
		AmountMustBeLessThanMaximumWithdraw,

		// Users trying to withdraw assets while providing an amount of lp tokens that is smaller than 
		//     T::MinimumWithdraw results in:
		AmountMustBeGreaterThanMinimumWithdraw,

		// TODO (Nevin):
		//  - rename to better represent error
		// Issues that arise when transfering tokens from one address to another result in:
		TransferFromFailed,

		// Issues that arise when a pool is trying to burn its local lp tokens from the issuers account
		//     results in:
		FailedToBurnLpTokens,
	}

	// ----------------------------------------------------------------------------------------------------
    //                                                Hooks                                                
	// ----------------------------------------------------------------------------------------------------

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	// ----------------------------------------------------------------------------------------------------
    //                                              Extrinsics                                             
	// ----------------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// // # Errors
		// //  - When the extrinsic is not signed.
		// #[pallet::weight(10_000)]
		// pub fn dcreate(
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
		/// Corresponds to the Ids used by the pallet to uniquely identify accounts.
		type AccountId = AccountIdOf<T>;
		/// The Balance type used by the pallet for bookkeeping.
		type Balance = BalanceOf<T>;
		/// Corresponds to the Ids used by the pallet to uniquely identify assets.
		type AssetId = AssetIdOf<T>;
		/// The type used by the pallet to deal with asset weights.
		type Weight = WeightOf<T>;

		/// Key type for Pool that uniquely identifieds a Pool.
		type PoolId = T::PoolId;
		/// Represents the PoolInfo struct that is used to save information about each Pool.
		type PoolInfo = PoolInfoOf<T>;

		/// Queries the spot price between two of the Pool's underlying assets.
		///
		/// # Overview 
		/// 
		/// ## Parameters
		/// - `pool_id`: The Pools identifier. This must correspond to an existing Pool.
		/// - `asset`: The identifier of the asset wanting to obtain the price of. This asset must be
		///		tracked by the specified Pool.
		/// - `numeraire`: The identifier of the base asset wanting to obtain the price of `asset` in. 
		/// 		This asset must be tracked by the specified Pool.
		/// 
		/// ## Requirements
		/// 1. `pool_id` P exists
		///	2. `asset` a ∈ P
		///	3. `numeraire` n ∈ P
		///
		/// ## Errors
		/// - `PoolDoesNotExist`: When `pool_id` doesn't correspond to an existing pool.
		/// - `AssetIsNotTrackedByPool`: When `asset` or `numeraire` aren't being tracked by the specified pool.
		/// - `ArithmeticError::Overflow`: When calculating the spot_price results in an overflow or underflow 
		///		behind the scenes.
		/// 
		/// # Examples
		/// ```
		/// use composable_traits::pool::{Bound, ConstantMeanMarket, Deposit, PoolConfig, Weight};
		/// use sp_runtime::Perquintill;
		/// use pallet_pool::Pallet;
		/// 
		/// let user = 1u128;
		/// let asset_1 = 1u128;
		/// let asset_2 = 2u128;
		/// 
		/// let config = PoolConfig {
		/// 	owner: user,
		/// 	fee: Perquintill::zero(),
		/// 
		/// 	assets: vec![
		/// 		asset_1,
		/// 		asset_2
		/// 	],
		/// 	asset_bounds: Bound {
		/// 		minimum: 0, 
		/// 		maximum: 26
		/// 	},
		/// 
		/// 	weights: vec![
		/// 		Weight {
		/// 			asset_id: asset_1,
		/// 			weight: Perquintill::from_percent(50)
		/// 		},
		/// 		Weight {
		/// 			asset_id: asset_2,
		/// 			weight: Perquintill::from_percent(50)
		/// 		}
		/// 	],
		/// 	weight_bounds: Bound {
		/// 		minimum: Perquintill::zero(), 
		/// 		maximum: Perquintill::one()
		/// 	},
		/// 
		/// 	deposit_bounds: Bound {
		/// 		minimum: Perquintill::zero(), 
		/// 		maximum: Perquintill::one()
		/// 	},
		/// 	withdraw_bounds: Bound {
		/// 		minimum: Perquintill::zero(), 
		/// 		maximum: Perquintill::one()
		/// 	},
		/// };
		/// 
		/// let creation_fee = Deposit {
		/// 	asset_id: asset_1,
		/// 	amount: 10000u128, 
		/// };
		/// 
		/// let pool_id = <Pallet as ConstantMeanMarket>::create(user, config, creation_fee);
		/// ```
		/// 
		/// # Weight: O(1)
		fn spot_price(
			pool_id: &Self::PoolId,
			asset: &Self::AssetId,
			numeraire: &Self::AssetId
		) -> Result<FixedBalance, DispatchError> {
			// Requirement 1) the desired pool index must exist
			ensure!(Self::pool_exists(pool_id), Error::<T>::PoolDoesNotExist);

			// Requirement 2) asset must be tracked by the pool
			ensure!(
				Self::asset_is_tracked_by_pool(pool_id, asset),
				Error::<T>::AssetIsNotTrackedByPool
			);
			// Requirement 2) numeraire must be tracked by the pool
			ensure!(
				Self::asset_is_tracked_by_pool(pool_id, numeraire),
				Error::<T>::AssetIsNotTrackedByPool
			);

			let price: FixedBalance = Self::do_spot_price(pool_id, asset, numeraire)?;

			Ok(price)
		}

		/// Validates the Pool's configuration parameters, and creates a new Pool if the config is valid. 
		/// 	Pool creation requires a creation deposit (fee) of at least (`PoolSize` + 1) x 
		/// 	(`ExistentialDeposit` + `CreationFee`). A Cubic vault is created for each asset.
		///
		/// # Overview 
		/// 
		/// ## Parameters
		/// - `from`: The `account_id` of the issuing user.
		/// - `config`: A [`PoolConfig`](composable_traits::pool::PoolConfig) struct containing the 
		/// 	parameter values to instantiate a new Pool with.
		/// - `creation_fee`: The blance, in the runtimes native asset, that the issuer is supplying 
		/// 	for the creation fee.
		/// 
		/// ## Requirements
		/// 1.  ∀ assets (i, j) ∈ `config.assets`: a_i != a_j -- no duplicate assets
		/// 2.  min_underlying_tokens ≤ max_underlying_tokens
		/// 3.  min_underlying_tokens ≤ n ≤ max_underlying_tokens, where n is the number
		///		    of tokens in the pool
		///	4.  ∀ assets a_i ⇒ ∃ weight w_i
		/// 5.  ∀ weights w_i ∈ `config.weights` : w_i ≥ 0
		/// 6.  Σ w_i = 1 & w_i ≥ 0
		///	7.  min_weight ≤ max_weight
		/// 8.  min_weight ≤ w_i ≤ max_weight	
		///	9.  user_balance ≥ creation_fee
		///	10. creation_fee ≥ (asset_ids.len() + 1) * (creation_deposit + existential_deposit)
		/// 
		/// ## Emits 
		/// - [`Event::PoolCreated`](Event::PoolCreated)
		///
		/// ## State Changes
		/// - [`PoolCount`](PoolCount): Increases by one to account for the newly created Pool.
		/// - [`Pools`](Pools): Stores the [`PoolInfo`](composable_traits::pool::PoolInfo) struct of the
		/// 	created Pool.
		/// - [`PoolAssets`](PoolAssets): Stores the assets of the created Pool.
		/// - [`PoolAssetWeight`](PoolAssetWeight): Stores a mapping of the created Pool's `pool_id` and 
		/// 	underlying asset id's to their corresponding weights.
		/// - [`PoolAssetVault`](PoolAssetVault): Stores a mapping of the created Pool's `pool_id` and 
		/// 	underlying asset id's to their corresponding Cubic Vault `vault_id`.
		/// - [`PoolAssetBalance`](PoolAssetBalance): Sets the mapping of the created Pool's `pool_id` 
		/// 	and underlying asset id's to zero.
		/// - [`PoolAssetTotalBalance`](PoolAssetTotalBalance): Sets the mapping of the created Pool's 
		/// 	`pool_id` and underlying asset id's to zero.
		/// 
		/// ## Errors
		/// - `DuplicateAssets`: `config.assets` contains the same asset more than once.
		/// - `InvalidAssetBounds`: `config.asset_bounds.maximum` < `config.asset_bounds.minimum`
		/// - `PoolSizeIsOutsideOfAssetBounds`: `config.assets.len()` < `config.asset_bounds.minimum` or
		/// 	`config.asset_bounds.maximum` < `config.assets.len()`.
		/// - `ThereMustBeOneWeightForEachAssetInThePool`: Each of the Pool's assets must have an accompanying
		/// 	weight.
        /// - `PoolWeightsMustBeNonnegative`: One of the weights in `config.weights` is less than zero.
		/// - `PoolWeightsMustBeNormalized`: The sum of `config.weights` does not equal one.
		/// - `InvalidWeightBounds`: `config.weight_bounds.maximum` < `config.weight_bounds.minimum`.
		/// - `PoolWeightsAreOutsideOfWeightBounds`: One of the weights is less than the minimum allowed 
		/// 	weight or greater than the maximum allowed weight.
		/// - `IssuerDoesNotHaveBalanceTryingToDeposit`: The issuer specified a native asset `creation_fee`
		/// 	larger than they own. 
		/// - `CreationFeeIsInsufficient`: `creation_fee` is not larger enough to create a Pool of the 
		/// 	desired size.
		/// - `ErrorCreatingLpTokenForPool`: An issue arised assigning an LP token to the Pool.
		/// - `DepositingIntoPoolFailed`: There was an issue transfering assets from the issuer's account 
		/// 	into the Pool's account.
		/// 
		/// # Examples
		/// ```
		/// ```
		/// 
		/// # Weight: O()
		fn create(
			from: Self::AccountId,
			config: PoolConfig<Self::AccountId, Self::AssetId, Self::Weight>,
			creation_fee: Deposit<T>,
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
				Self::bounds_are_valid(&config.asset_bounds),
				Error::<T>::InvalidAssetBounds
			);

			// Requirement 3) The number of assets in the pool's configuration must be within
			//     the required bounds set in the config's asset_bounds field
			let pool_size = number_of_assets as u8;
			ensure!(
				Self::pool_size_is_within_asset_bounds(pool_size, &config.asset_bounds),
				Error::<T>::PoolSizeIsOutsideOfAssetBounds
			);

			// ---------- Weight Requirements ----------
			// Requirement 4) There exists a corresponding weight for each asset
			ensure!(
				Self::each_asset_has_exactly_one_corresponding_weight(&config.assets, &config.weights),
				Error::<T>::ThereMustBeOneWeightForEachAssetInThePool
			);

			// Requirement 5) The weights must be greater than zero
			ensure!(
				Self::weights_are_nonnegative(&config.weights),
				Error::<T>::PoolWeightsMustBeNonnegative
			);

			// Requirement 6) The weights must be normalized (sum to one)
			ensure!(
				Self::weights_are_normalized(&config.weights),
				Error::<T>::PoolWeightsMustBeNormalized
			);

			// Requirement 7) The minimum weight bound must be less than or equal to the maximum weight bound
			ensure!(
				Self::bounds_are_valid(&config.weight_bounds),
				Error::<T>::InvalidWeightBounds
			);

			// Requirement 8) The provided weights are in the required weight bounds
			ensure!(
				Self::weights_are_in_weight_bounds(&config.weights, &config.weight_bounds),
				Error::<T>::PoolWeightsAreOutsideOfWeightBounds
			);

			// ---------- User Requirements ----------
			// Requirement 9) The user who issued must have the balance they are trying to deposit
			ensure!(
				Self::user_has_specified_balance_for_asset(&from, T::NativeAssetId::get(), creation_fee.amount),
				Error::<T>::IssuerDoesNotHaveBalanceTryingToDeposit
			);
			
			// Requirement 10) The specified creation fee is sufficient enough to open a Pool
			ensure!(
				Self::required_creation_deposit_for(number_of_assets)? <= creation_fee.amount, 
				Error::<T>::CreationFeeIsInsufficient
			);

			let (pool_id, pool_info) = Self::do_create(from, config, creation_fee)?;
			Self::deposit_event(Event::PoolCreated { 
				pool_id,
				pool_info
			});

			Ok(pool_id)
		}

		/// Validates an all-asset deposit and deposit the funds into the pool if it is valid. Deposited 
		/// 	funds are transferred into the underlying vault accounts associated with the respective 
		/// 	assets. LP tokens are minted for the issuer in return. 
		/// 
		/// # Overview 
		/// 
		/// ## Parameters
		/// - `from`: The `account_id` of the issuing user.
		/// - `pool_id`: A unique identifier specifying the Pool to interact with.
		/// - `deposits`: A vector of [`Deposit`](composable_traits::pool::Deposit) structs specifying
		/// 	the balance of each asset to deposit
		/// 
		/// ## Requirements
		/// 1. pool id p_i ⇒ exists
		///	2. ∀ asset a_i in pool : ∃ asset a_i in deposit
		///	3. ∀ asset a_i in deposit : dep_i > 0
		///	4. ∀ asset a_i in deposit : user_balance_i >= dep_i
		///	5. ∀ asset a_i in deposit : dep_min ≤ dep_i ≤ dep_max
		///	6. ∀ asset a_i in deposit : (dep_i / Σ dep_j) = (pool_balance_i / Σ pool_balance_j)
		/// 		
		/// ## Emits 
		/// - [`Event::AllAssetDeposit`](Event::AllAssetDeposit)
		///
		/// ## State Changes
		/// - [`PoolAssetBalance`](PoolAssetBalance): Updates the mapping of the Pool's `pool_id` 
		/// 	and underlying asset id's to include the deposited balances.
		/// - [`PoolAssetTotalBalance`](PoolAssetTotalBalance): Updates the mapping of the Pool's 
		/// 	`pool_id` and underlying asset id's to include the deposited balances.
		/// 
		/// ## Errors
		/// - `PoolDoesNotExist`: `pool_id` doesn't correspond with an active Pool.
		/// - `ThereMustBeOneDepositForEachAssetInThePool`: All-asset deposits must have a balance of
		/// 	each of the Pool's underlying assets.
		/// - `DepositsMustBeStrictlyPositive`: Pool's don't allow deposits of zero balance.
		/// - `IssuerDoesNotHaveBalanceTryingToDeposit`: The issuer specified an asset balance larger
		/// 	than they possess.
		/// - `DepositIsOutsideOfPoolsDepositBounds`: The deposit will increase the Pool's value by a 
		/// 	factor outside of its `deposit_bounds`.
		/// - `DepositDoesNotMatchUnderlyingValueDistribution`: The value distribution of All-asset deposits
		/// 	must closely resemble the value distribution of the Pool.
		/// - `DepositingIntoPoolFailed`: An issue arised transfering the deposit from the users account 
		/// 	and into the Pool's account.
		/// - `DepositingIntoVaultFailed`: An issue arised transfering the deposit from the Pool's account 
		/// 	and into the underlying vault accounts.
		/// - `FailedToMintLpTokens`: An issue arised minting LP token's for the issuer.
		/// - `ArithmeticError::Overflow`: When updating the runtime storage objects results in an overflow.
		///
		/// # Examples
		/// 
		/// # Weight: O()
		fn all_asset_deposit(
			from: &AccountIdOf<T>,
			pool_id: &T::PoolId,
			deposits: Vec<Deposit<T>>,
		) -> Result<Self::Balance, DispatchError> {
			// Requirement 1) the desired pool index must exist
			ensure!(Self::pool_exists(pool_id), Error::<T>::PoolDoesNotExist);
		
			// Requirement 2) For each of the Pool's underlying assets there is a deposit of that asset
			ensure!(
				Self::there_is_one_deposit_for_each_underlying_asset(pool_id, &deposits)?,
				Error::<T>::ThereMustBeOneDepositForEachAssetInThePool
			);

			let reserve_total: BalanceOf<T> = Self::reserves_of(pool_id)?.iter()
				.fold(BalanceOf::<T>::zero(), |total, reserve| total + reserve.amount);
			let deposit_total: BalanceOf<T> = deposits.iter()
				.fold(BalanceOf::<T>::zero(), |total, reserve| total + reserve.amount);

			for deposit in &deposits {
				// Requirement 3) deposit amount is stictly positive
				ensure!(
					deposit.amount > BalanceOf::<T>::zero(),
					Error::<T>::DepositsMustBeStrictlyPositive
				);

				// Requirement 4) the user who issued the extrinsic must have the total balance trying to deposit
				ensure!(
					Self::user_has_specified_balance_for_asset(from, deposit.asset_id, deposit.amount),
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

			let lp_tokens_minted = Self::do_all_asset_deposit(from, pool_id, deposits.clone())?;
			
			Self::deposit_event(Event::AllAssetDeposit { 
				account:          from.clone(), 
				pool_id:          *pool_id,
				deposited:        deposits, 
				lp_tokens_minted,
			});

			Ok(lp_tokens_minted)
		}

		/// Withdraw assets from the Pool by supplying the Pool's LP token. Withdrawn assets are
		/// 	removed from the Pool's underlying vault accounts and transfered into the issuers 
		/// 	account. The supplied LP tokens are then burned.
		/// 
		/// # Overview 
		/// 
		/// ## Parameters
		/// - `from`: The `account_id` of the issuing user.
		/// - `pool_id`: A unique identifier specifying the Pool to interact with.
		/// - `deposits`: A vector of [`Deposit`](composable_traits::pool::Deposit) structs specifying
		/// 	the balance of each asset to deposit
		/// 
		/// ## Requirements
		/// 1. pool id p_i ⇒ exists
		///	2. ∀ asset a_i in pool : ∃ asset a_i in deposit
		///	3. ∀ asset a_i in deposit : dep_i > 0
		///	4. ∀ asset a_i in deposit : user_balance_i >= dep_i
		///	5. ∀ asset a_i in deposit : dep_min ≤ dep_i ≤ dep_max
		///	6. ∀ asset a_i in deposit : (dep_i / Σ dep_j) = (pool_balance_i / Σ pool_balance_j)
		/// 		
		/// ## Emits 
		/// - [`Event::AllAssetWithdraw`](Event::AllAssetWithdraw)
		///
		/// ## State Changes
		/// - [`PoolAssetBalance`](PoolAssetBalance): Updates the mapping of the Pool's `pool_id` 
		/// 	and underlying asset id's to remove the withdrawn balances.
		/// - [`PoolAssetTotalBalance`](PoolAssetTotalBalance): Updates the mapping of the Pool's 
		/// 	`pool_id` and underlying asset id's to remove the withdrawn balances.
		///
		/// ## Errors
		/// - `PoolDoesNotExist`: `pool_id` doesn't correspond with an active Pool.
		/// - `IssuerDoesNotHaveBalanceTryingToDeposit`: The issuer specified a balance of the Pool's
		/// 	LP token larger than they possess.
		/// - `WithdrawIsOutsideOfPoolsWithdrawBounds`: The withdraw would decrease the Pool's value by a 
		/// 	factor outside of its `withdraw_bounds`.
		/// - `WithdrawDoesNotMatchUnderlyingValueDistribution`: The value distribution of the all-asset 
		/// 	withdraw must closely resemble the value distribution of the Pool.
		/// - `TransferFromFailed`: An issue arised transfering funds from the Pool into the issuer's 
		/// 	account. 
		/// - `FailedToBurnLpTokens`: An issue arised burning the LP token's from the issuer's account.
		/// - `ArithmeticError::Overflow`: When updating the runtime storage objects results in an underflow.
		/// 
		/// # Examples
		/// ```
		/// ```
		/// 
		/// # Weight: O()
		fn all_asset_withdraw(
			to: &Self::AccountId,
			pool_id: &Self::PoolId,
			lp_amount: Self::Balance,
		) -> Result<Vec<Deposit<T>>, DispatchError> {
			// Requirement 1) the desired pool index must exist
			ensure!(Self::pool_exists(pool_id), Error::<T>::PoolDoesNotExist);

			let lp_token_id = Self::lp_token_id(pool_id)?;
			// Requirement 2) the user who issued the extrinsic must have the total balance trying to deposit
			ensure!(
				Self::user_has_specified_balance_for_asset(to, lp_token_id, lp_amount),
				Error::<T>::IssuerDoesNotHaveBalanceTryingToDeposit
			);

			// Requirement 3) liquidity requested lies within the Pool's withdraw transaction bounds
			ensure!(
				Self::withdraw_is_within_pools_withdraw_bounds(pool_id, lp_amount)?,
				Error::<T>::WithdrawIsOutsideOfPoolsWithdrawBounds
			);

			// Obtain the pro-rata share of the Pool's reserves the lp_amount corresponds to 
			let lps_share: Vec<Withdraw<T>> = Self::lps_share_of_pool(pool_id, lp_amount)?;

			let reserve_total: BalanceOf<T> = Self::reserves_of(pool_id)?.iter()
				.fold(T::Balance::zero(), |total, reserve| total + reserve.amount);
			let share_total: BalanceOf<T> = lps_share.iter()
				.fold(T::Balance::zero(), |total, reserve| total + reserve.amount);

			// TODO: (Nevin)
			//  - once all-asset withdraw math is 100% - this validity check can be removed.

			// Requirement 4) withdraw amount matches the pools current reserve ratio
			for asset_share in &lps_share {
				ensure!(
					Self::deposit_matches_underlying_value_distribution(pool_id, asset_share, share_total, reserve_total)?,
					Error::<T>::WithdrawDoesNotMatchUnderlyingValueDistribution
				);
			}

			let assets_withdrawn = Self::do_all_asset_withdraw(to, pool_id, lp_amount, lps_share)?;
			
			Self::deposit_event(Event::AllAssetWithdraw { 
				account:          to.clone(), 
				pool_id:          *pool_id, 
				withdrawn:        assets_withdrawn.clone(), 
				lp_tokens_burned: lp_amount,
			});

			Ok(assets_withdrawn)
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                 Helper Functions - Core Functionality                               
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		// Helper function for the spot price trait function. Calculates the spot price formula
		//     spot price = (balance of asset / weight of asset) / (balance of numeraire / weight of numeraire).
		// 
		// # Errors
		//  - When there is an overflow with any of the divisions in the formula above
		fn do_spot_price(
			pool_id: &T::PoolId,
			asset: &AssetIdOf<T>,
			numeraire: &AssetIdOf<T>
		) -> Result<FixedBalance, DispatchError> {
			let asset_balance: BalanceOf<T> = PoolAssetBalance::<T>::get(pool_id, asset);
			let asset_balance: u128 = 
				<T::Convert as Convert<BalanceOf<T>, u128>>::convert(asset_balance);
			let asset_balance: FixedBalance = FixedBalance::saturating_from_num(asset_balance);

			let asset_weight: WeightOf<T> = PoolAssetWeight::<T>::get(pool_id, asset);
			let asset_weight: FixedBalance = 
				FixedBalance::from_num(asset_weight.deconstruct().into())
					.checked_div(FixedBalance::from_num(WeightOf::<T>::one().deconstruct().into()))
					.ok_or(ArithmeticError::Overflow)?;

			let numeraire_balance: BalanceOf<T> = PoolAssetBalance::<T>::get(pool_id, numeraire);
			let numeraire_balance: u128 = 
				<T::Convert as Convert<BalanceOf<T>, u128>>::convert(numeraire_balance);
			let numeraire_balance: FixedBalance = FixedBalance::saturating_from_num(numeraire_balance);

			let numeraire_weight: WeightOf<T> = 
				PoolAssetWeight::<T>::get(pool_id, numeraire);
			let numeraire_weight: FixedBalance = FixedBalance::from_num(numeraire_weight.deconstruct().into())
				.checked_div(FixedBalance::from_num(WeightOf::<T>::one().deconstruct().into()))
				.ok_or(ArithmeticError::Overflow)?;

			let numerator: FixedBalance = asset_balance.checked_div(asset_weight)
				.ok_or(ArithmeticError::Overflow)?;
				
			let denominator: FixedBalance = numeraire_balance.checked_div(numeraire_weight)
				.ok_or(ArithmeticError::Overflow)?;

			let result: FixedBalance = numerator.checked_div(denominator)
				.ok_or(ArithmeticError::Overflow)?;
			// let result: u128 = result.saturating_to_num();
			// let result: BalanceOf<T> =
			//     <T::Convert as Convert<u128, BalanceOf<T>>>::convert(result);

			Ok(result)
		}
		
		/// Helper function for the the create trait function. Obtains a new LP Token Id for the pool, creates a 
		///     vault for each asset desired in the pool, and saves all important info into runtime storage
		///
		/// # Errors
		/// - 'ErrorCreatingLpTokenForPool': there was an issue creating an lp token for the pool.
		/// - 'DepositingIntoPoolFailed': transfering creation deposit into the Pool failed.
		/// - Other: an issue arised when creating one of the underlying vaults.
		fn do_create(
			from: AccountIdOf<T>,
			config: PoolConfig<AccountIdOf<T>, AssetIdOf<T>, WeightOf<T>>,
			creation_fee: Deposit<T>,
		) -> Result<(T::PoolId, PoolInfoOf<T>), DispatchError>  {
			PoolCount::<T>::try_mutate(|id| {
				let id = {
					*id += One::one();
					*id
				};

				let pool_account = Self::account_id(&id);

				// Requirement 1) Obtain a new asset id for this pools lp token 
				let lp_token =
					{ T::CurrencyFactory::create().map_err(|_| Error::<T>::ErrorCreatingLpTokenForPool)? };

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
					//		'-> might not me needed as they won't have any reserves and will be automatically dusted

					let vault_id: <T::Vault as Vault>::VaultId = T::Vault::create(
						Duration::Existential,
						VaultConfig::<AccountIdOf<T>, AssetIdOf<T>> {
							asset_id: *asset_id,
							reserved: Perquintill::from_percent(100),
							manager: pool_account.clone(),
							strategies: [].iter().cloned().collect(),
						},
					)?;
					
					PoolAssetVault::<T>::insert(id, asset_id, vault_id.clone());
				}

				// Requirement 3) Transfer native tokens from users account to pools account
				T::Currency::transfer(T::NativeAssetId::get(), &from, &pool_account, creation_fee.amount, true)
					.map_err(|_| Error::<T>::DepositingIntoPoolFailed)?;
				
				// Requirement 4) Save the pools assets in global storage
				PoolAssets::<T>::insert(&id, config.assets);

				// Requirement 5) Save each assets weight in global storage
				for share in &config.weights {
					PoolAssetWeight::<T>::insert(&id, share.asset_id, share.weight);
				}

				// Requirement 6) Keep track of the pool's configuration
				let pool_info = PoolInfoOf::<T> {
					owner: config.owner,
					lp_token,
					fee: config.fee,
					asset_bounds: config.asset_bounds,
					weight_bounds: config.weight_bounds,
					deposit_bounds: config.deposit_bounds,
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
		//  - When there is an issue creating an lp token for the pool.
		//  - When there is an issue creating an underlying vault.
		//  - When any `deposit` < `CreationFee` + `ExistentialDeposit`.
		//  - When the issuer has insufficient funds to lock each deposit.
		fn do_all_asset_deposit(
			from: &AccountIdOf<T>,
			pool_id: &T::PoolId,
			deposits: Vec<Deposit<T>>,
		) -> Result<BalanceOf<T>, DispatchError> {
			let pool_info = Pools::<T>::get(&pool_id);
			let to = &Self::account_id(pool_id);

			// Requirement 1) Calculate the number of LP tokens that need to be minted
			let lp_tokens_to_mint = Self::calculate_lp_tokens_to_mint(pool_id, &deposits)?;

			// Requirement 2) Deposit each asset into the pool's underlying vaults
			for deposit in &deposits {
				// TODO (Nevin):
			    //  - transfer all assets into the pool's account before beginning
			    //     '-> should be its own function to abstract away details

				let vault_id = &PoolAssetVault::<T>::get(pool_id, deposit.asset_id);

				T::Currency::transfer(deposit.asset_id, from, to, deposit.amount, true)
					.map_err(|_| Error::<T>::DepositingIntoPoolFailed)?;

				// TODO (Nevin):
				//  - check for errors in vault depositing, and if so revert all deposits so far
				//     '-> surround T::Vault::deposit call in match statement checking for Ok(lp_token_amount)
				//             and DispatchError

				let _vault_lp_token_amount = T::Vault::deposit(
					vault_id,
					to,
					deposit.amount,
				).map_err(|_| Error::<T>::DepositingIntoVaultFailed)?;
				
				// Requirement 3) Update the pool's reserve runtime storage objects
				Self::increase_pool_asset_balance_storage(pool_id, &deposit.asset_id, &deposit.amount)?;
				Self::increase_pool_asset_total_balance_storage(pool_id, &deposit.asset_id, &deposit.amount)?;
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

		fn do_all_asset_withdraw(
			to: &AccountIdOf<T>,
			pool_id: &T::PoolId,
			lp_amount: BalanceOf<T>,
			lps_share: Vec<Withdraw<T>>
		) -> Result<Vec<Withdraw<T>>, DispatchError> {
			let pool_account = &Self::account_id(pool_id);

			// Requirement 1) Calculate and withdraw the lp tokens share of the each asset in the Pool
			for asset_share in &lps_share {
				let vault_id = &PoolAssetVault::<T>::get(pool_id, asset_share.asset_id);
			
				// Withdraw the vaults assets into the pools account
				let vault_balance_withdrawn = T::Vault::withdraw(
					vault_id,
					pool_account,
					asset_share.amount
				)?;

				// Withdraw the assets now in the pools account into the issuers account
				T::Currency::transfer(asset_share.asset_id, pool_account, to, vault_balance_withdrawn, true)
					.map_err(|_| Error::<T>::TransferFromFailed)?;
			}

			let lp_token: AssetIdOf<T> = Self::lp_token_id(pool_id)?;

			// Requirement 2) burn the lp tokens that were deposited during this withdraw
			T::Currency::burn_from(lp_token, to, lp_amount)
				.map_err(|_| Error::<T>::FailedToBurnLpTokens)?;

			// TODO: (Nevin)
			//  - move into seperate functions
			for asset_share in &lps_share {
				PoolAssetTotalBalance::<T>::mutate(
					pool_id,
					asset_share.asset_id,
					|balance| -> DispatchResult {
						*balance = balance.checked_sub(&asset_share.amount)
							.ok_or(ArithmeticError::Overflow)?;
						Ok(())
					},
				)?;
	
				PoolAssetBalance::<T>::mutate(
					pool_id,
					asset_share.asset_id,
					|balance| -> DispatchResult {
						*balance = balance.checked_sub(&asset_share.amount)
							.ok_or(ArithmeticError::Overflow)?;
						Ok(())
					},
				)?;
			}

			Ok(lps_share)
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                 Helper Functions - Pallet Queries                               
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		pub fn account_id(pool_id: &T::PoolId) -> T::AccountId {
			T::PalletId::get().into_sub_account(pool_id)
		}
		
		pub fn pool_info(pool_id: &T::PoolId) -> Result<PoolInfoOf<T>, DispatchError> {
			Ok(Pools::<T>::try_get(pool_id).map_err(|_err| Error::<T>::PoolDoesNotExist)?)
		}

		pub fn lp_token_id(pool_id: &T::PoolId) -> Result<T::AssetId, DispatchError> {
			Ok(Self::pool_info(pool_id)?.lp_token)
		}

		pub fn lp_circulating_supply(pool_id: &T::PoolId) -> Result<T::Balance, DispatchError> {
			let lp_token_id = Self::lp_token_id(pool_id)?;

			Ok(T::Currency::total_issuance(lp_token_id))
		}

		pub fn reserves_of(pool_id: &T::PoolId) -> Result<Vec<Reserve<T::AssetId, T::Balance>>, DispatchError> {
			let assets = PoolAssets::<T>::try_get(pool_id).map_err(|_err| Error::<T>::PoolDoesNotExist)?;

			let mut reserves = Vec::<Reserve<T::AssetId, T::Balance>>::new();

			for asset in assets {
				reserves.push(Reserve {
					asset_id: asset,
					amount:   Self::balance_of(pool_id, &asset)?,
				});
			}

			Ok(reserves)
		}

		pub fn balance_of(pool_id: &T::PoolId, asset_id: &T::AssetId) -> Result<T::Balance, DispatchError> {
			let vault_id = &PoolAssetVault::<T>::get(pool_id, asset_id);
			let vault_lp_token_id = T::Vault::lp_asset_id(vault_id)?;
		
			let pool_account = &Self::account_id(pool_id);

			Ok(T::Currency::balance(vault_lp_token_id, pool_account))
		}

		pub fn weight_of(pool_id: &T::PoolId, asset_id: &T::AssetId) -> Result<WeightOf<T>, DispatchError> {
			Ok(Self::pool_asset_weight(pool_id, asset_id))
		}
	}

	// ----------------------------------------------------------------------------------------------------
    //                                  Helper functions - Validity Checks                                 
	// ----------------------------------------------------------------------------------------------------

	// These functions are caled by the Constant Mean Market trait functions to validate the 
	//     input parameters and state space
	impl<T: Config> Pallet<T> {

		// Checks that the provided PoolId corresponds to an active Pool
		fn pool_exists(pool_id: &T::PoolId) -> bool {
			Pools::<T>::contains_key(pool_id)
		}

		// Checks if the input vector has duplicate enteries and returns true if it doesn't, false otherwise
		//  '-> Conditions:
		//        i.  Σ(i, j) a_i != a_j
		fn no_duplicate_assets_provided(assets: &[AssetIdOf<T>]) -> bool {
			let unique_assets = BTreeSet::<AssetIdOf<T>>::from_iter(assets.iter().copied());

			// Condition i
			unique_assets.len() == assets.len()
		}

		// Checks that the 'bounds' input parameter is valid in the sense that if there is a provided
		//     lower and upper boudn then the lwoer bound must be less than or equal to the upper bound 
		fn bounds_are_valid<U>(bounds: &Bound<U>) -> bool 
		where U: Copy + PartialOrd {
			let (lower_bounds, upper_bound) = (bounds.minimum, bounds.maximum);

			match (lower_bounds, upper_bound) {
				(Some(minimum), Some(maximum)) => minimum <= maximum,
				_ => true
			}
		}

		// Checks the pool size is strictly in between (inclusive) the required asset bounds
		//  '-> Conditions:
		//        i. min_size ≤ pool_size ≤ max_size
		fn pool_size_is_within_asset_bounds(
			pool_size: u8,
			bounds: &Bound<u8>
		) -> bool {
			let (lower_bound, upper_bound) = (bounds.minimum, bounds.maximum);

			// Condition i
			match (lower_bound, upper_bound) {
				(None, None) => true,
				(Some(minimum), None) => minimum <= pool_size,
				(None, Some(maximum)) => pool_size <= maximum,
				(Some(minimum), Some(maximum)) => minimum <= pool_size && pool_size <= maximum,
			}
		}

		// Checks that there is a one-to-one correspondence of weights to assets
		//  '-> Conditions:
		//        i.  ∀ assets a_i ⇒ ∃ weight w_i
		pub fn each_asset_has_exactly_one_corresponding_weight(
			assets: &[AssetIdOf<T>], 
			weights: &WeightsVec<AssetIdOf<T>, WeightOf<T>>
		) -> bool {
			if weights.len() != assets.len() {
				return false;
			}

			// Get unique asset_ids from 'assets'
			let assets = BTreeSet::<AssetIdOf<T>>::from_iter(
				assets.iter()
				.copied()
			);

			// Get unique asset_ids from 'weights'
			let weights = BTreeSet::<AssetIdOf<T>>::from_iter(
				weights.iter()
				.map(|weight| weight.asset_id)
			);

			// Condition i
			assets.is_subset(&weights) && assets.is_superset(&weights)
		}

		// Checks the provided weights are all strictly nonnegative (greater than or equal to one)
		//  '-> Conditions:
		//        i.  w_i ≥ 0
		pub fn weights_are_nonnegative(
			weights: &WeightsVec<AssetIdOf<T>, WeightOf<T>>
		) -> bool {
			let zero = WeightOf::<T>::zero();

			weights.iter()
				.all(|weight| zero < weight.weight)
		}

		// Checks the provided weights are weights_are_normalized (they sum to one plus or minus a 
		//  |  margin of error)
		//  '-> Conditions:
		//        i. Σ w_i ≈ 1
		pub fn weights_are_normalized(
			weights: &WeightsVec<AssetIdOf<T>, WeightOf<T>>
		) -> bool {
			let epsilon: u128 = T::Epsilon::get().deconstruct().into();
			let one: u128 = WeightOf::<T>::one().deconstruct().into();
			
			let sum: u128 = weights.iter()
				.map(|weight| weight.weight.deconstruct().into())
				.sum();

			// Condition i
			(one - epsilon) <= sum && sum <= (one + epsilon)
		}

		// Checks the provided weights are all strictly in between (inclusive) the required weight bounds
		//  '-> Conditions:
		//        i. min_weight ≤ w_i ≤ max_weight
		pub fn weights_are_in_weight_bounds(
			weights: &WeightsVec<AssetIdOf<T>, WeightOf<T>>, 
			weight_bounds: &Bound<WeightOf<T>>
		) -> bool {
			let (lower_bound, upper_bound) = (weight_bounds.minimum, weight_bounds.maximum);

			weights.iter()
				.all(|weight| match (lower_bound, upper_bound) {
					(None, None) => true,
					(Some(minimum), None) => minimum <= weight.weight,
					(None, Some(maximum)) => weight.weight <= maximum,
					(Some(minimum), Some(maximum)) => minimum <= weight.weight && weight.weight <= maximum,
				})
		}

		// Checks that, for an all-asset deposit, there is actually one deposit for each asset
		pub fn there_is_one_deposit_for_each_underlying_asset(
			pool_id: &T::PoolId, 
			deposits: &[Deposit<T>],
		) -> Result<bool, DispatchError> {
			let underlying_assets: Assets<AssetIdOf<T>> = PoolAssets::<T>::get(&pool_id)
				.ok_or(Error::<T>::PoolDoesNotExist)?;

			let underlying_assets: BTreeSet::<AssetIdOf<T>> = 
				BTreeSet::<AssetIdOf<T>>::from_iter(underlying_assets.iter().copied());

			let deposit_assets: BTreeSet::<AssetIdOf<T>> = 
				BTreeSet::<AssetIdOf<T>>::from_iter(deposits.iter().map(|deposit| deposit.asset_id));

			Ok(underlying_assets.is_subset(&deposit_assets) && underlying_assets.is_superset(&deposit_assets))
		}

		// Checks that the specified user has at least *amount* of *asset* for each asset in the deposit
		//  '-> Conditions:
		//        i. ∀ assets a_i : amount ≤ user_balance
		pub fn user_has_specified_balance_for_deposits(
			user: &AccountIdOf<T>, 
			deposits: &[Deposit<T>]
		) -> bool {
			deposits.iter()
				.all(|deposit| Self::user_has_specified_balance_for_asset(user, deposit.asset_id, deposit.amount))
		}

		// Checks that the specified user has at least *amount* of *asset*
		//  '-> Conditions:
		//        i. amount ≤ user_balance
		pub fn user_has_specified_balance_for_asset(
			user: &AccountIdOf<T>,
			asset: AssetIdOf<T>,
			amount: BalanceOf<T>
		) -> bool {
			T::Currency::can_withdraw(asset, user, amount) == WithdrawConsequence::Success
		}

		// Redirects to the correct validity check method depending on if the pool is empty or not as
		//     initial deposits have different requirements for deposit bounds.
		pub fn deposit_is_within_pools_deposit_bounds(
			pool_id: &T::PoolId, 
			deposit: &Deposit<T>
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
			_deposit: &Deposit<T>
		) -> Result<bool, DispatchError> {
			// TODO (Nevin):
			//  - allow pool configurations to have initial deposit limits or have a default initial deposit limit

			Ok(true)
		}

		// Checks that the specified deposit doesn't increase the Pool's reserves by a factor limited
		//     by the Pool's deposit bounds
		pub fn deposit_is_within_nonempty_pools_deposit_bounds(
			pool_id: &T::PoolId, 
			deposit: &Deposit<T>
		) -> Result<bool, DispatchError> {
			let asset: AssetIdOf<T> = deposit.asset_id;

			// TODO: (Nevin)
			//  - update version 1 below to match correct implementaiton and benchmark the two functions

			// Version 1: ~219 seconds for 10_000 runs of 
			// depositing_into_a_non_empty_pool_with_duplicate_deposits_correctly_mints_lp_tokens
			{
			// let deposit: u128 = 
			// 	<T::Convert as Convert<BalanceOf<T>, u128>>::convert(deposit.amount);

			// let reserve: BalanceOf<T> = Self::balance_of(pool_id, &asset)?;
			// let reserve: u128 = 
			// 	<T::Convert as Convert<BalanceOf<T>, u128>>::convert(reserve);

			// let reserve_increase: u128 = multiply_by_rational(1u128,
			// 	deposit,
			// 	reserve
			// ).map_err(|_| ArithmeticError::Overflow)?;

			// let deposit_bounds = Self::pool_info(pool_id)?.deposit_bounds;
			// let lower_bound: u128 = match deposit_bounds.minimum {
			// 	None => 0_128,
			// 	Some(percent) => multiply_by_rational(1u128,
			// 		percent.deconstruct().into(),
			// 		WeightOf::<T>::one().deconstruct().into()
			// 	).map_err(|_| ArithmeticError::Overflow)?,
			// };

			// let upper_bound: u128 = match deposit_bounds.maximum {
			// 	None => u128::MAX,
			// 	Some(percent) => multiply_by_rational(1u128,
			// 		percent.deconstruct().into(),
			// 		WeightOf::<T>::one().deconstruct().into()
			// 	).map_err(|_| ArithmeticError::Overflow)?
			// };

			// Ok(lower_bound <= reserve_increase && reserve_increase <= upper_bound)
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

			let reserve_increase: FixedBalance = deposit
				.checked_div(reserve)
				.ok_or(ArithmeticError::Overflow)?;

			let deposit_bounds = Self::pool_info(pool_id)?.deposit_bounds;
			let lower_bound: FixedBalance = match deposit_bounds.minimum {
				None => FixedBalance::from_num(0_u8),
				Some(percent) => FixedBalance::from_num(percent.deconstruct().into())
					.checked_div(FixedBalance::from_num(WeightOf::<T>::one().deconstruct().into()))
					.ok_or(ArithmeticError::Overflow)?
			};

			let upper_bound: FixedBalance = match deposit_bounds.maximum {
				None => FixedBalance::MAX,
				Some(percent) => FixedBalance::from_num(percent.deconstruct().into())
					.checked_div(FixedBalance::from_num(WeightOf::<T>::one().deconstruct().into()))
					.ok_or(ArithmeticError::Overflow)?
			};

			Ok(lower_bound <= reserve_increase && reserve_increase <= upper_bound)
		}

		// Checks that the value distribution of the deposited asset (deposit.amount / deposit_total) matches
		//     that of the underlying pool's reserves.
		pub fn deposit_matches_underlying_value_distribution(
			pool_id: &T::PoolId, 
			deposit: &Deposit<T>,
			deposit_total: BalanceOf<T>,
			reserve_total: BalanceOf<T>
		) -> Result<bool, DispatchError> {
			let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;
			
			if lp_circulating_supply == (BalanceOf::<T>::zero()) { 
				return Ok(true);
			}

			// Version 1: ~220 seconds for 10_000 runs of 
			// depositing_into_a_non_empty_pool_with_duplicate_deposits_correctly_mints_lp_tokens
			
			let asset: AssetIdOf<T> = deposit.asset_id;
			
			let deposit = <T::Convert as Convert<BalanceOf<T>, u128>>::convert(deposit.amount);
			let deposit_total = <T::Convert as Convert<BalanceOf<T>, u128>>::convert(deposit_total);
			let deposit_value_distribution = multiply_by_rational(1, deposit, deposit_total)
				.map_err(|_| ArithmeticError::Overflow)?;

			let reserve: BalanceOf<T> = Self::balance_of(pool_id, &asset)?;
			let reserve: u128 = <T::Convert as Convert<BalanceOf<T>, u128>>::convert(reserve);
			let reserve_total = <T::Convert as Convert<BalanceOf<T>, u128>>::convert(reserve_total);
			let reserve_value_distribution = multiply_by_rational(1, reserve, reserve_total)
				.map_err(|_| ArithmeticError::Overflow)?;

			let epsilon: u128 = T::Epsilon::get().deconstruct().into();
			let one: u128 = WeightOf::<T>::one().deconstruct().into();

			let margin_of_error: u128 = multiply_by_rational(reserve_value_distribution, epsilon, one)
				.map_err(|_| ArithmeticError::Overflow)?;

			let lower_bound = reserve_value_distribution - margin_of_error;
			let upper_bound = reserve_value_distribution + margin_of_error;

			Ok(lower_bound <= deposit_value_distribution && deposit_value_distribution <= upper_bound)

			// Version 2: ~217 seconds for 10_000 runs of
			//depositing_into_a_non_empty_pool_with_duplicate_deposits_correctly_mints_lp_tokens
			// let asset: AssetIdOf<T> = deposit.asset_id;
			
			// let deposit: u128 =
			// 	<T::Convert as Convert<BalanceOf<T>, u128>>::convert(deposit.amount);
			// let deposit: FixedBalance = FixedBalance::saturating_from_num(deposit);

			// let deposit_total: u128 =
			// 	<T::Convert as Convert<BalanceOf<T>, u128>>::convert(deposit_total);
			// let deposit_total: FixedBalance = FixedBalance::saturating_from_num(deposit_total);

			// let deposit_value_distribution: FixedBalance = deposit.checked_div(deposit_total)
			// 	.ok_or(ArithmeticError::Overflow)?;

			// let reserve: BalanceOf<T> = Self::balance_of(pool_id, &asset)?;
			// let reserve: u128 =
			// 	<T::Convert as Convert<BalanceOf<T>, u128>>::convert(reserve);
			// let reserve: FixedBalance = FixedBalance::saturating_from_num(reserve);

			// let reserve_total: u128 =
			// 	<T::Convert as Convert<BalanceOf<T>, u128>>::convert(reserve_total);
			// let reserve_total: FixedBalance = FixedBalance::saturating_from_num(reserve_total);

			// let reserve_value_distribution: FixedBalance = reserve.checked_div(reserve_total)
			// 	.ok_or(ArithmeticError::Overflow)?;

			// let margin_of_error: Perquintill = T::Epsilon::get();
			// let margin_of_error: FixedBalance = FixedBalance::from_num(
			// 	margin_of_error.deconstruct() as f64 / Perquintill::one().deconstruct() as f64
			// );

			// let lower_bound = reserve_value_distribution - margin_of_error;
			// let upper_bound = reserve_value_distribution + margin_of_error;

			// Ok(lower_bound <= deposit_value_distribution && deposit_value_distribution <= upper_bound)
		}

		// Checks that the specified asset is one of the Pool's underlying assets
		fn asset_is_tracked_by_pool(
			pool_id: &T::PoolId,
			asset_id: &AssetIdOf<T>
		) -> bool {
			PoolAssetBalance::<T>::contains_key(pool_id, asset_id)
		}

		// Checks that the withdraw won't decrease the Pool's reserves by a factor that is limited
		//     by the Pool's withdraw bounds
		pub fn withdraw_is_within_pools_withdraw_bounds(
			pool_id: &T::PoolId, 
			lp_amount: BalanceOf<T>
		) -> Result<bool, DispatchError> {
			let lp_amount: u128 = 
				<T::Convert as Convert<BalanceOf<T>, u128>>::convert(lp_amount);
			
			let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;
			let lp_circulating_supply: u128 = 
				<T::Convert as Convert<BalanceOf<T>, u128>>::convert(lp_circulating_supply);
			
			let lp_share: WeightOf<T> = WeightOf::<T>::from_rational(
				lp_amount, 
				lp_circulating_supply
			);

			let withdraw_bounds: Bound<WeightOf<T>> = Self::pool_info(pool_id)?.withdraw_bounds;
			let (lower_bound, upper_bound) = (withdraw_bounds.minimum, withdraw_bounds.maximum);
			match (lower_bound, upper_bound) {
				(None, None) => Ok(true),
				(Some(minimum), None) => Ok(minimum <= lp_share),
				(None, Some(maximum)) => Ok(lp_share <= maximum),
				(Some(minimum), Some(maximum)) => Ok(minimum <= lp_share && lp_share <= maximum),
			}

			// TODO: (Nevin)
			//  - delete
			
			// let lower_bound: WeightOf<T> = withdraw_bounds.minimum;
			// let upper_bound: WeightOf<T> = withdraw_bounds.maximum;

			// Ok(lower_bound <= lp_share && lp_share <= upper_bound)
		}
	}

	// ----------------------------------------------------------------------------------------------------
    //                              Helper functions - Low-Level Functionality                             
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		// Adds the asset amount to the PoolAssetTotalBalance runtime storage object for the sepcified asset
		fn increase_pool_asset_total_balance_storage(
			pool_id: &T::PoolId,
			asset_id: &AssetIdOf<T>,
			amount: &BalanceOf<T>
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
		fn increase_pool_asset_balance_storage(
			pool_id: &T::PoolId,
			asset_id: &AssetIdOf<T>,
			amount: &BalanceOf<T>
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
				<T::Convert as Convert<u128, BalanceOf<T>>>::convert(number_of_assets);
				
			let result = creation_fee
				.checked_add(&existential_deposit)
				.ok_or(ArithmeticError::Overflow)?
				.checked_mul(&number_of_assets)
				.ok_or(ArithmeticError::Overflow)?;
					
			Ok(result)
		}

		// Calculate the number of lp tokens to mint as a result of this deposit
		//  |-> when depositig funds to an empty pool, initialize lp token ratio to
		//  |       the weighted gemoetric mean of the amount deposited.
		//  '-> when depositing funds to a non-empty pool, the amount of lp tokens minted
		//          should be calculated relative to the increase in the invariant value
		fn calculate_lp_tokens_to_mint(
			pool_id: &T::PoolId,
			deposits: &[Deposit<T>]
		) -> Result<BalanceOf<T>, DispatchError> {
			let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;

			if lp_circulating_supply == BalanceOf::<T>::zero() {
				// TODO (Jesper):
				//  - accounting for MIN LIQ too like uniswap (to avoid sybil, and other issues)
				Self::weighted_geometric_mean(pool_id, deposits)
			} else {
				Self::increase_in_weighted_geometric_mean(pool_id, deposits)
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
			deposits: &[Deposit<T>]
		) -> Result<BalanceOf<T>, DispatchError> {
			let mut result: FixedBalance = FixedBalance::from_num(1u8);

			for deposit in deposits {
				let balance: u128 = 
					<T::Convert as Convert<BalanceOf<T>, u128>>::convert(deposit.amount);
				let balance: FixedBalance = FixedBalance::saturating_from_num(balance);

				let weight: WeightOf<T> = PoolAssetWeight::<T>::get(pool_id, deposit.asset_id);
				
				// TODO: (Kevin)
				//  - check if this math works
				
				// let weight: FixedBalance = FixedBalance::from_num(weight.deconstruct().into())
				// 	.checked_div(FixedBalance::from_num(WeightOf::<T>::one().deconstruct().into()))
				// 	.ok_or(ArithmeticError::Overflow)?;

				let weight: FixedBalance = FixedBalance::from_num(
					weight.deconstruct().into() as f64 / WeightOf::<T>::one().deconstruct().into() as f64
				);

				result = result.checked_mul(
					pow(balance, weight).map_err(|_| ArithmeticError::Overflow)?
				).ok_or(ArithmeticError::Overflow)?;

			}

			let result: u128 = FixedBalance::saturating_to_num::<u128>(result);
			let result: BalanceOf<T> = 
				<T::Convert as Convert<u128, BalanceOf<T>>>::convert(result);

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
			deposits: &[Deposit<T>]
		) -> Result<T::Balance, DispatchError> {
			let mut deposit_ratio = FixedBalance::from_num(0_u8);
			
			for deposit in deposits {
				let deposit_amount: u128 = 
					<T::Convert as Convert<BalanceOf<T>, u128>>::convert(deposit.amount);
				let deposit_amount: FixedBalance = FixedBalance::saturating_from_num(deposit_amount);

				let reserve: BalanceOf<T> = Self::balance_of(pool_id, &deposit.asset_id)?;
				let reserve: u128 = 
					<T::Convert as Convert<BalanceOf<T>, u128>>::convert(reserve);
				let reserve: FixedBalance = FixedBalance::saturating_from_num(reserve);

				let weight: WeightOf<T>  = Self::weight_of(pool_id, &deposit.asset_id)?;
				let weight: FixedBalance = FixedBalance::from_num(
					weight.deconstruct().into() as f64 / WeightOf::<T>::one().deconstruct().into() as f64
				);

				let asset_ratio: FixedBalance = weight.checked_mul(deposit_amount)
					.ok_or(ArithmeticError::Overflow)?
					.checked_div(reserve)
					.ok_or(ArithmeticError::Overflow)?;
				
				deposit_ratio = deposit_ratio.checked_add(asset_ratio)
					.ok_or(ArithmeticError::Overflow)?;
			}

			let lp_circulating_supply: BalanceOf<T> = Self::lp_circulating_supply(pool_id)?;
			let lp_circulating_supply: u128 = 
				<T::Convert as Convert<BalanceOf<T>, u128>>::convert(lp_circulating_supply);
			let lp_circulating_supply: FixedBalance = FixedBalance::saturating_from_num(lp_circulating_supply);

			// Converts the floor of the FixedBalance into a u128
			let lp_tokens_to_mint: FixedBalance = lp_circulating_supply.checked_mul(deposit_ratio)
				.ok_or(ArithmeticError::Overflow)?
				.ceil();
			let lp_tokens_to_mint: u128 = FixedBalance::saturating_to_num::<u128>(lp_tokens_to_mint);
			let lp_tokens_to_mint: BalanceOf<T> = 
				<T::Convert as Convert<u128, T::Balance>>::convert(lp_tokens_to_mint);

			Ok(lp_tokens_to_mint)
		}

		fn lps_share_of_pool(
			pool_id: &T::PoolId,
			lp_amount: T::Balance
		) -> Result<Vec<Reserve<T::AssetId, T::Balance>>, DispatchError> {
			let assets = PoolAssets::<T>::try_get(pool_id).map_err(|_err| Error::<T>::PoolDoesNotExist)?;
			let lp_circulating_supply = Self::lp_circulating_supply(pool_id)?;

			// Used to keep track of the amount of each asset withdrawn from the pool's underlying vaults
			let mut lps_share = Vec::<Reserve<T::AssetId, T::Balance>>::new();

			for asset_id in &assets {
				let reserve = Self::balance_of(pool_id, asset_id)?;

				// Calculate the percentage of the pool's assets that correspond to the deposited lp tokens
				let lp_share_of_asset: T::Balance = Self::lps_share_of_asset(
					reserve,
					lp_amount,
					lp_circulating_supply
				).map_err(|_| ArithmeticError::Overflow)?;
				
				lps_share.push(
					Reserve {
						asset_id: *asset_id,
						amount: lp_share_of_asset,
					}
				);
			}

			Ok(lps_share)
		}

		// Calculates the exact balance of assets that the provided lp tokens (shares) correspond to
		//  '-> LP Share = pool_balance_of_token * (lp_amount/lp_circulating_supply)
		//
		// # Errors
		//  - When calculating the LP Share amount results in an overflow error.
		fn lps_share_of_asset(
			reserve: T::Balance,
			lp_amount: T::Balance,
			lp_circulating_supply: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			// multiply by rational method
			
			// Convert all three arguments to u128
			let pool_balance_of_token: u128 = 
				<T::Convert as Convert<T::Balance, u128>>::convert(reserve);

			let lp_amount: u128 = 
				<T::Convert as Convert<T::Balance, u128>>::convert(lp_amount);

			let lp_circulating_supply: u128 = 
				<T::Convert as Convert<T::Balance, u128>>::convert(lp_circulating_supply);

			// Calculate the LP Share amount
			let lp_share_of_asset = multiply_by_rational(
				pool_balance_of_token,
				lp_amount,
				lp_circulating_supply
			).map_err(|_| ArithmeticError::Overflow)?;

			// Convert back to Balance type
			Ok(<T::Convert as Convert<u128, T::Balance>>::convert(lp_share_of_asset))
		

			// FixedBalance method
			
			// // Convert all three arguments to u128
			// let reserve: u128 = 
			// 	<T::Convert as Convert<T::Balance, u128>>::convert(reserve);
			// let reserve: FixedBalance = FixedBalance::saturating_from_num(reserve);

			// let lp_amount: u128 = 
			// 	<T::Convert as Convert<T::Balance, u128>>::convert(lp_amount);
			// let lp_amount: FixedBalance = FixedBalance::saturating_from_num(lp_amount);

			// let lp_circulating_supply: u128 = 
			// 	<T::Convert as Convert<T::Balance, u128>>::convert(lp_circulating_supply);
			// let lp_circulating_supply: FixedBalance = FixedBalance::saturating_from_num(lp_circulating_supply);

			// // Calculate the LP Share amount
			// let lp_share_of_asset: FixedBalance = lp_amount.checked_div(lp_circulating_supply)
			// 	.ok_or(ArithmeticError::Overflow)?
			// 	.checked_mul(reserve)
			// 	.ok_or(ArithmeticError::Overflow)?;

			// let lp_share_of_asset: u128 = FixedBalance::saturating_to_num::<u128>(lp_share_of_asset);
			// let lp_share_of_asset: BalanceOf<T> = 
			// 	<T::Convert as Convert<u128, T::Balance>>::convert(lp_share_of_asset);

			// // Convert back to Balance type
			// Ok(lp_share_of_asset)
			
		} 
	}

}