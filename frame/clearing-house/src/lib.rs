//! # Clearing House Pallet
//!
//! ## Overview
//!
//! The Clearing House pallet provides functionality for creating and managing perpetual futures
//! markets. To use it in your runtime, you must provide compatible implementations of virtual AMMs
//! and price oracles.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ### Terminology
//!
//! * **Trader**: Primary user of the public extrinsics of the pallet
//! * **Derivative**: A financial instrument which derives its value from another asset, a.k.a. the
//!   _underlying_.
//! * **Perpetual contract**: A derivative product that allows a trader to have exposure to the underlying's price without owning it. See [The Cartoon Guide to Perps](https://www.paradigm.xyz/2021/03/the-cartoon-guide-to-perps) for intuitions.
//! * **Market**: Perpetual contracts market, where users trade virtual tokens mirroring the
//!   base-quote asset pair of spot markets. A.k.a. a virtual market.
//! * **VAMM**: Virtual automated market maker allowing price discovery in virtual markets based on
//!   the supply of virtual base/quote assets.
//! * **Position**: Amount of a particular virtual asset owned by a trader. Implies debt (positive
//!   or negative) to the Clearing House.
//! * **Collateral**: 'Real' asset(s) backing the trader's position(s), ensuring he/she can pay back
//!   the Clearing House.
//! * **IMR**: acronym for 'Initial Margin Ratio'
//!
//! ### Goals
//!
//! ### Implementations
//!
//! The Clearing House pallet provides implementations for the following traits:
//!
//! - [`ClearingHouse`](composable_traits::clearing_house::ClearingHouse): Exposes functionality for
//!   trading of perpetual contracts
//! - [`Instruments`](composable_traits::clearing_house::Instruments): Exposes functionality for
//!   querying funding-related quantities of synthetic instruments
//!
//! ## Interface
//!
//! ### Extrinsics
//!
//! - [`add_margin`](Call::add_margin)
//! - [`create_market`](Call::create_market)
//! - [`open_position`](Call::open_position)
//!
//! ### Implemented Functions
//!
//! - [`add_margin`](pallet/struct.Pallet.html#method.add_margin-1)
//! - [`create_market`](pallet/struct.Pallet.html#method.create_market-1)
//! - [`open_position`](pallet/struct.Pallet.html#method.open_position-1)
//! - [`funding_rate`](Pallet::funding_rate)
//! - [`unrealized_funding`](Pallet::unrealized_funding)
//!
//! ## Usage
//!
//! ### Example
//!
//! ## Related Modules
//!
//! - [`pallet-vamm`](../vamm/index.html)
//! - [`pallet-oracle`](../oracle/index.html)
//!
//! <!-- Original author: @0xangelo -->
#![cfg_attr(not(feature = "std"), no_std)]
// Allow some linters
#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic,
		clippy::doc_markdown
	)
)]
// Specify linters to Clearing House Pallet.
#![warn(clippy::unseparated_literal_suffix)]
#![deny(
	dead_code,
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

pub use pallet::*;
// Bring to scope so that 'Implemented Functions' hyperlinks work
#[allow(unused_imports)]
use composable_traits::clearing_house::Instruments;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod math;

mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies
	// ----------------------------------------------------------------------------------------------------

	use crate::{
		math::{FromBalance, IntoBalance, TryMath},
		weights::WeightInfo,
	};
	use codec::FullCodec;
	use composable_traits::{
		clearing_house::{ClearingHouse, Instruments},
		defi::DeFiComposableConfig,
		oracle::Oracle,
		time::DurationSeconds,
		vamm::{AssetType, Direction as VammDirection, SwapConfig, SwapSimulationConfig, Vamm},
	};
	use frame_support::{
		pallet_prelude::*,
		storage::bounded_vec::BoundedVec,
		traits::{tokens::fungibles::Transfer, GenesisBuild, UnixTime},
		transactional, Blake2_128Concat, PalletId, Twox64Concat,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use num_integer::Integer;
	use num_traits::Signed;
	use sp_runtime::{
		traits::{AccountIdConversion, CheckedAdd, CheckedDiv, CheckedMul, One, Saturating, Zero},
		ArithmeticError, FixedPointNumber, FixedPointOperand,
	};
	use sp_std::{cmp::Ordering, fmt::Debug, ops::Neg};

	// ----------------------------------------------------------------------------------------------------
	//                                       Declaration Of The Pallet Type
	// ----------------------------------------------------------------------------------------------------

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// ----------------------------------------------------------------------------------------------------
	//                                             Config Trait
	// ----------------------------------------------------------------------------------------------------

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: DeFiComposableConfig + frame_system::Config {
		/// Pallet implementation of asset transfers.
		type Assets: Transfer<
			Self::AccountId,
			AssetId = Self::MayBeAssetId,
			Balance = Self::Balance,
		>;

		/// Signed decimal fixed point number.
		type Decimal: FixedPointNumber<Inner = Self::Integer>
			+ FullCodec
			+ MaxEncodedLen
			+ Neg<Output = Self::Decimal>
			+ TypeInfo;

		/// Event type emitted by this pallet. Depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Integer type underlying fixed point decimal and Vamm swap implementations
		type Integer: CheckedDiv
			+ CheckedMul
			+ Debug
			+ FixedPointOperand
			+ Integer
			+ One
			+ Signed
			+ TryFrom<Self::Balance>
			+ TryInto<Self::Balance>;

		/// The market ID type for this pallet.
		type MarketId: CheckedAdd
			+ Clone
			+ Debug
			+ Default
			+ FullCodec
			+ MaxEncodedLen
			+ One
			+ PartialEq
			+ TypeInfo;

		/// The maximum number of open positions (one for each market) for a trader
		type MaxPositions: Get<u32>;

		/// Price feed (in USDT) Oracle pallet implementation
		type Oracle: Oracle<AssetId = Self::MayBeAssetId, Balance = Self::Balance>;

		/// The id used as the `AccountId` of the clearing house. This should be unique across all
		/// pallets to avoid name collisions with other pallets and clearing houses.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Implementation for querying the current Unix timestamp
		type UnixTime: UnixTime;

		/// Virtual Automated Market Maker pallet implementation
		type Vamm: Vamm<
			Balance = Self::Balance,
			Decimal = Self::Decimal,
			Integer = Self::Integer,
			SwapConfig = SwapConfig<Self::VammId, Self::Balance>,
			SwapSimulationConfig = SwapSimulationConfig<Self::VammId, Self::Balance>,
			VammConfig = Self::VammConfig,
			VammId = Self::VammId,
		>;

		/// Configuration for creating and initializing a new vAMM instance. To be used as an
		/// extrinsic input
		type VammConfig: Clone + Debug + FullCodec + MaxEncodedLen + PartialEq + TypeInfo;

		/// Virtual automated market maker identifier; usually an integer
		type VammId: Clone + Copy + FullCodec + MaxEncodedLen + TypeInfo;

		/// Weight information for this pallet's extrinsics
		type WeightInfo: WeightInfo;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Pallet Types
	// ----------------------------------------------------------------------------------------------------

	#[derive(Encode, Decode, TypeInfo, Debug, Clone, Copy, PartialEq)]
	pub enum Direction {
		Long,
		Short,
	}

	/// Stores the user's position in a particular market
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Position<T: Config> {
		/// The Id of the virtual market
		pub market_id: T::MarketId,
		/// Virtual base asset amount. Positive implies long position and negative, short.
		pub base_asset_amount: T::Decimal,
		/// Virtual quote asset notional amount (margin * leverage * direction) used to open the
		/// position
		pub quote_asset_notional_amount: T::Decimal,
		/// Last cumulative funding rate used to update this position. The market's latest
		/// cumulative funding rate minus this gives the funding rate this position must pay. This
		/// rate multiplied by this position's size (base asset amount * amm price) gives the total
		/// funding owed, which is deducted from the trader account's margin. This debt is
		/// accounted for in margin ratio calculations, which may lead to liquidation.
		pub last_cum_funding: T::Decimal,
	}

	/// Data relating to a perpetual contracts market
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Market<T: Config> {
		/// The Id of the vAMM used for price discovery in the virtual market
		pub vamm_id: T::VammId,
		/// The Id of the underlying asset (base-quote pair). A price feed from one or more oracles
		/// must be available for this symbol
		pub asset_id: AssetIdOf<T>,
		/// Minimum margin ratio for opening a new position
		pub margin_ratio_initial: T::Decimal,
		/// Margin ratio below which liquidations can occur
		pub margin_ratio_maintenance: T::Decimal,
		/// Minimum amount of quote asset to exchange when opening a position. Also serves to round
		/// a trade if it results in closing an existing position
		pub minimum_trade_size: T::Decimal,
		/// The latest cumulative funding rate of this market. Must be updated periodically.
		pub cum_funding_rate: T::Decimal,
		/// The timestamp for the latest funding rate update.
		pub funding_rate_ts: DurationSeconds,
		/// The time span between each funding rate update.
		pub funding_frequency: DurationSeconds,
		/// Period of time over which funding (the difference between mark and index prices) gets
		/// paid.
		///
		/// Setting the funding period too long may cause the perpetual to start trading at a
		/// very dislocated price to the index because there’s less of an incentive for basis
		/// arbitrageurs to push the prices back in line since they would have to carry the basis
		/// risk for a longer period of time.
		///
		/// Setting the funding period too short may cause nobody to trade the perpetual because
		/// there’s too punitive of a price to pay in the case the funding rate flips sign.
		pub funding_period: DurationSeconds,
	}

	/// Specifications for market creation
	#[derive(Encode, Decode, PartialEq, Clone, Debug, TypeInfo)]
	pub struct MarketConfig<AssetId, Decimal, VammConfig> {
		/// Asset id of the underlying for the derivatives market
		pub asset: AssetId,
		/// Configuration for creating and initializing the vAMM for price discovery
		pub vamm_config: VammConfig,
		/// Minimum margin ratio for opening a new position
		pub margin_ratio_initial: Decimal,
		/// Margin ratio below which liquidations can occur
		pub margin_ratio_maintenance: Decimal,
		/// Minimum amount of quote asset to exchange when opening a position. Also serves to round
		/// a trade if it results in closing an existing position
		pub minimum_trade_size: Decimal,
		/// Time span between each funding rate update
		pub funding_frequency: DurationSeconds,
		/// Period of time over which funding (the difference between mark and index prices) gets
		/// paid.
		pub funding_period: DurationSeconds,
	}

	type AssetIdOf<T> = <T as DeFiComposableConfig>::MayBeAssetId;
	type BalanceOf<T> = <T as DeFiComposableConfig>::Balance;
	type DecimalOf<T> = <T as Config>::Decimal;
	type VammConfigOf<T> = <T as Config>::VammConfig;
	type VammIdOf<T> = <T as Config>::VammId;
	type SwapConfigOf<T> = SwapConfig<VammIdOf<T>, BalanceOf<T>>;
	type SwapSimulationConfigOf<T> = SwapSimulationConfig<VammIdOf<T>, BalanceOf<T>>;
	type MarketConfigOf<T> = MarketConfig<AssetIdOf<T>, DecimalOf<T>, VammConfigOf<T>>;

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime Storage
	// ----------------------------------------------------------------------------------------------------

	/// Supported collateral asset ids
	#[pallet::storage]
	pub type CollateralTypes<T: Config> = StorageMap<_, Twox64Concat, AssetIdOf<T>, ()>;

	/// Maps [AccountId](frame_system::Config::AccountId) to its collateral
	/// [Balance](DeFiComposableConfig::Balance), if set.
	#[pallet::storage]
	#[pallet::getter(fn get_margin)]
	pub type AccountsMargin<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance>;

	/// Maps [AccountId](frame_system::Config::AccountId) to its respective [Positions](Position),
	/// as a vector.
	#[pallet::storage]
	#[pallet::getter(fn get_positions)]
	#[allow(clippy::disallowed_types)]
	pub type Positions<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<Position<T>, T::MaxPositions>,
		ValueQuery,
	>;

	/// The number of markets, also used to generate the next market identifier.
	///
	/// # Note
	///
	/// Frozen markets do not decrement the counter.
	#[pallet::storage]
	#[pallet::getter(fn market_count)]
	#[allow(clippy::disallowed_types)]
	pub type MarketCount<T: Config> = StorageValue<_, T::MarketId, ValueQuery>;

	/// Maps [MarketId](Config::MarketId) to the corresponding virtual [Market] specs
	#[pallet::storage]
	#[pallet::getter(fn get_market)]
	pub type Markets<T: Config> = StorageMap<_, Blake2_128Concat, T::MarketId, Market<T>>;

	// ----------------------------------------------------------------------------------------------------
	//                                         Genesis Configuration
	// ----------------------------------------------------------------------------------------------------

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		/// Genesis accepted collateral asset types
		pub collateral_types: Vec<AssetIdOf<T>>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { collateral_types: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			self.collateral_types.iter().for_each(|asset| {
				CollateralTypes::<T>::insert(asset, ());
			})
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                             Runtime Events
	// ----------------------------------------------------------------------------------------------------

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Margin successfully added to account
		MarginAdded {
			/// Account id that received the deposit
			account: T::AccountId,
			/// Asset type deposited
			asset: AssetIdOf<T>,
			/// Amount of asset deposited
			amount: T::Balance,
		},
		/// New virtual market successfully created
		MarketCreated {
			/// Id for the newly created market
			market: T::MarketId,
			/// Id of the underlying asset
			asset: AssetIdOf<T>,
		},
		/// New trade successfully executed
		TradeExecuted {
			/// Id of the market
			market: T::MarketId,
			/// Direction of the trade (long/short)
			direction: Direction,
			/// Notional amount of quote asset exchanged
			quote: T::Balance,
			/// Amount of base asset exchanged
			base: T::Balance,
		},
	}

	// ----------------------------------------------------------------------------------------------------
	//                                             Runtime Errors
	// ----------------------------------------------------------------------------------------------------

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// User attempted to deposit unsupported asset type as collateral in its margin account
		UnsupportedCollateralType,
		/// Attempted to create a new market but the underlying asset is not supported by the
		/// oracle
		NoPriceFeedForAsset,
		/// Attempted to create a new market but the funding period is not a multiple of the
		/// funding frequency
		FundingPeriodNotMultipleOfFrequency,
		/// Attempted to create a new market but the funding period or frequency is 0 seconds long
		ZeroLengthFundingPeriodOrFrequency,
		/// Attempted to create a new market but either initial or maintenance margin ratios are
		/// outside the interval (0, 1)
		InvalidMarginRatioRequirement,
		/// Attempted to create a new market but the initial margin ratio is less than or equal to
		/// the maintenance one
		InitialMarginRatioLessThanMaintenance,
		/// Attempted to create a new market but the minimum trade size is negative
		NegativeMinimumTradeSize,
		/// Raised when querying a market with an invalid or nonexistent market Id
		MarketIdNotFound,
		/// Raised when opening a risk-increasing position that takes the account below the IMR
		InsufficientCollateral,
		/// Raised when creating a new position but exceeding the maximum number of positions for
		/// an account
		MaxPositionsExceeded,
		/// Raised when creating a new position with quote asset amount less than the market's
		/// minimum trade size
		TradeSizeTooSmall,
	}

	// ----------------------------------------------------------------------------------------------------
	//                                             Extrinsics
	// ----------------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds margin to a trader's account.
		///
		/// # Overview
		/// A user has to have enough margin to open new positions
		/// and can be liquidated if its margin ratio falls bellow maintenance. Deposited collateral
		/// backs all the positions of an account across multiple markets (cross-margining).
		///
		/// ![](http://www.plantuml.com/plantuml/svg/FSrD2W8n343XlQVG0ynaxsf0y1wPDhQ592tvmUihBbmztkexFD0YXI-teOMpKXfVUyJoEu3XUsyZUfxfP6LgaCPUfi1ZofgE9zDpGFaFa9TE1Yz38IXCQ4FRrcSwGHtO3CK1Qzq4hGtT5wF--8EqVli1)
		///
		/// ## Parameters:
		/// - `asset_id`: The identifier of the asset type being deposited
		/// - `amount`: The balance of `asset` to be transferred from the caller to the Clearing
		///   House
		///
		/// ## Assumptions or Requirements
		/// The collateral type must be supported, i.e., contained in [`CollateralTypes`].
		///
		/// ## Emits
		/// * [`MarginAdded`](Event::<T>::MarginAdded)
		///
		/// ## State Changes
		/// Updates the [`AccountsMargin`] storage map. If an account does not exist in
		/// [`AccountsMargin`], it is created and initialized with 0 margin.
		///
		/// ## Errors
		/// * [`UnsupportedCollateralType`](Error::<T>::UnsupportedCollateralType)
		///
		/// # Weight/Runtime
		/// `O(1)`
		#[pallet::weight(<T as Config>::WeightInfo::add_margin())]
		pub fn add_margin(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			amount: T::Balance,
		) -> DispatchResult {
			let account_id = ensure_signed(origin)?;
			<Self as ClearingHouse>::add_margin(&account_id, asset_id, amount)?;
			Ok(())
		}

		/// Creates a new perpetuals market with the desired parameters.
		///
		/// # Overview
		///
		/// ![](http://www.plantuml.com/plantuml/svg/FOux3i8m40LxJW47IBQdYeJ4FJQRHsnXhwFzYEiJKL2DPgfPFDWYUxlSgahB3MdjMY8ElnCPV-QzHiar7IP30ngpZ4wFqO_Xl3OyAybV22u5HY_Z3f86jghxL4OwQAkydzr931oOEjiRCH-DzNUmGBUJNm00)
		///
		/// ## Parameters
		/// - `config`: specification for market creation
		///
		/// ## Assumptions or Requirements
		/// * The underlying must have a stable price feed via another pallet
		/// * The funding period must be a multiple of its frequency
		/// * Both funding period and frequency must be nonzero
		/// * Initial and Maintenance margin ratios must be in the (0, 1) open interval
		/// * Initial margin ratio must be greater than maintenance
		///
		/// ## Emits
		/// * [`MarketCreated`](Event::<T>::MarketCreated)
		///
		/// ## State Changes
		/// Adds an entry to the [`Markets`] storage map.
		///
		/// ## Errors
		/// - [`NoPriceFeedForAsset`](Error::<T>::NoPriceFeedForAsset)
		/// - [`FundingPeriodNotMultipleOfFrequency`](
		///   Error::<T>::FundingPeriodNotMultipleOfFrequency)
		/// - [`ZeroLengthFundingPeriodOrFrequency`](Error::<T>::ZeroLengthFundingPeriodOrFrequency)
		/// - [`InvalidMarginRatioRequirement`](Error::<T>::InvalidMarginRatioRequirement)
		/// - [`InitialMarginRatioLessThanMaintenance`](
		///   Error::<T>::InitialMarginRatioLessThanMaintenance)
		///
		/// # Weight/Runtime
		/// `O(1)`
		#[pallet::weight(<T as Config>::WeightInfo::create_market())]
		pub fn create_market(origin: OriginFor<T>, config: MarketConfigOf<T>) -> DispatchResult {
			ensure_signed(origin)?;
			let _ = <Self as ClearingHouse>::create_market(&config)?;
			Ok(())
		}

		/// Opens a position in a market
		///
		/// # Overview
		///
		/// This may result in the following outcomes:
		/// * Creation of a whole new position in the market, if one didn't already exist
		/// * An increase in the size of an existing position, if the trade's direction matches the
		///   existing position's one
		/// * A decrease in the size of an existing position, if the trade's direction is counter to
		///   the existing position's one and its magnitude is smaller than the existing postion's
		///   size
		/// * Closing of the existing position, if the trade's direction is counter to the existion
		///   position's one and its magnitude is approximately the existing postion's size
		/// * Reversing of the existing position, if the trade's direction is counter to the
		///   existion position's one and its magnitude is greater than the existing postion's size
		///
		/// ![](http://www.plantuml.com/plantuml/svg/FOuzgiD030RxTugN0zZgKna2kOUyLhm2hRJeXrm_9aMgZszWOBP8zAmXVpVM9dLGkVptp1bt0CVtUdBssYl8cscIvjfimCF6jC1TwCdGVWSeMYU7b-CWQ4BehEVIhOBWO3ml7c2JTBaCJZPTfw1-2pRIuzeF)
		///
		/// ## Parameters
		///
		/// - `market_id`: the perpetuals market Id to open a position in
		/// - `direction`: whether to long or short the base asset
		/// - `quote_asset_amount`: the amount of exposure to the base asset in quote asset value
		/// - `base_asset_amount_limit`: the minimum absolute amount of base asset to add to the
		///   position. Prevents slippage
		///
		/// ## Assumptions or Requirements
		///
		/// There's a maximum number of positions ([`Config::MaxPositions`]) than can be open for
		/// each account id at any given time. If opening a position in a new market exceeds this
		/// number, the transactions fails.
		///
		/// ## Emits
		///
		/// - [`TradeExecuted`](Event::<T>::TradeExecuted)
		///
		/// ## State Changes
		///
		/// The following storage items may be modified:
		/// - [`AccountsMargin`]: if trade decreases, closes, or reverses a position, it's PnL is
		///   realized
		/// - [`Positions`]: either a new entry is added or an existing one is updated
		///
		/// ## Erros
		///
		/// - [`MarketIdNotFound`](Error::<T>::MarketIdNotFound)
		/// - [`MaxPositionsExceeded`](Error::<T>::MaxPositionsExceeded)
		///
		/// # Weight/Runtime
		/// TODO(0xangelo)
		#[pallet::weight(<T as Config>::WeightInfo::open_position())]
		pub fn open_position(
			origin: OriginFor<T>,
			market_id: T::MarketId,
			direction: Direction,
			quote_asset_amount: T::Balance,
			base_asset_amount_limit: T::Balance,
		) -> DispatchResult {
			let account_id = ensure_signed(origin)?;
			let _ = <Self as ClearingHouse>::open_position(
				&account_id,
				&market_id,
				direction,
				quote_asset_amount,
				base_asset_amount_limit,
			)?;
			Ok(())
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Trait Implementations
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> ClearingHouse for Pallet<T> {
		type AccountId = T::AccountId;
		type AssetId = AssetIdOf<T>;
		type Balance = T::Balance;
		type Direction = Direction;
		type MarketId = T::MarketId;
		type MarketConfig = MarketConfigOf<T>;

		fn add_margin(
			account_id: &Self::AccountId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<(), DispatchError> {
			ensure!(
				CollateralTypes::<T>::contains_key(asset_id),
				Error::<T>::UnsupportedCollateralType
			);

			// Assuming stablecoin collateral and all markets quoted in dollars
			T::Assets::transfer(
				asset_id,
				account_id,
				&T::PalletId::get().into_account(),
				amount,
				true,
			)?;

			let old_margin = Self::get_margin(&account_id).unwrap_or_else(T::Balance::zero);
			let new_margin = old_margin.checked_add(&amount).ok_or(ArithmeticError::Overflow)?;
			AccountsMargin::<T>::insert(&account_id, new_margin);

			Self::deposit_event(Event::MarginAdded {
				account: account_id.clone(),
				asset: asset_id,
				amount,
			});
			Ok(())
		}

		fn create_market(config: &Self::MarketConfig) -> Result<Self::MarketId, DispatchError> {
			ensure!(T::Oracle::is_supported(config.asset)?, Error::<T>::NoPriceFeedForAsset);
			ensure!(
				config.funding_period > 0 && config.funding_frequency > 0,
				Error::<T>::ZeroLengthFundingPeriodOrFrequency
			);
			ensure!(
				config.funding_period.rem_euclid(config.funding_frequency) == 0,
				Error::<T>::FundingPeriodNotMultipleOfFrequency
			);
			ensure!(
				config.margin_ratio_initial > T::Decimal::zero() &&
					config.margin_ratio_initial < T::Decimal::one() &&
					config.margin_ratio_maintenance > T::Decimal::zero() &&
					config.margin_ratio_maintenance < T::Decimal::one(),
				Error::<T>::InvalidMarginRatioRequirement
			);
			ensure!(
				config.margin_ratio_initial > config.margin_ratio_maintenance,
				Error::<T>::InitialMarginRatioLessThanMaintenance
			);
			ensure!(
				config.minimum_trade_size >= T::Decimal::zero(),
				Error::<T>::NegativeMinimumTradeSize
			);

			MarketCount::<T>::try_mutate(|id| {
				let market_id = id.clone();
				let market = Market {
					asset_id: config.asset,
					vamm_id: T::Vamm::create(&config.vamm_config)?,
					margin_ratio_initial: config.margin_ratio_initial,
					margin_ratio_maintenance: config.margin_ratio_maintenance,
					minimum_trade_size: config.minimum_trade_size,
					funding_frequency: config.funding_frequency,
					funding_period: config.funding_period,
					cum_funding_rate: Default::default(),
					funding_rate_ts: T::UnixTime::now().as_secs(),
				};
				Markets::<T>::insert(&market_id, market);

				// Change the market count at the end
				*id = id.checked_add(&One::one()).ok_or(ArithmeticError::Overflow)?;

				Self::deposit_event(Event::MarketCreated {
					market: market_id.clone(),
					asset: config.asset,
				});
				Ok(market_id)
			})
		}

		#[transactional]
		fn open_position(
			account_id: &Self::AccountId,
			market_id: &Self::MarketId,
			direction: Self::Direction,
			quote_asset_amount: Self::Balance,
			base_asset_amount_limit: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let market = Self::get_market(&market_id).ok_or(Error::<T>::MarketIdNotFound)?;
			let mut positions = Self::get_positions(&account_id);
			let (position, position_index) =
				Self::get_or_create_position(&mut positions, market_id, &market)?;

			let mut quote_abs_amount_decimal = T::Decimal::from_balance(quote_asset_amount)?;
			ensure!(
				quote_abs_amount_decimal >= market.minimum_trade_size,
				Error::<T>::TradeSizeTooSmall
			);

			let position_direction = Self::position_direction(position).unwrap_or(direction);

			let base_swapped: T::Balance;
			if direction == position_direction {
				base_swapped = Self::increase_position(
					position,
					&market,
					direction,
					&quote_abs_amount_decimal,
					base_asset_amount_limit,
				)?;
			} else {
				let abs_base_asset_value =
					Self::base_asset_value(&market, position, position_direction)?.saturating_abs();

				// Round trade if it nearly closes the position
				Self::round_trade_if_necessary(
					&market,
					&mut quote_abs_amount_decimal,
					&abs_base_asset_value,
				)?;

				let entry_value: T::Decimal;
				let exit_value: T::Decimal;
				match quote_abs_amount_decimal.cmp(&abs_base_asset_value) {
					Ordering::Less => {
						(base_swapped, entry_value, exit_value) = Self::decrease_position(
							position,
							&market,
							direction,
							&quote_abs_amount_decimal,
							base_asset_amount_limit,
						)?;
					},
					Ordering::Equal => {
						// close position
						base_swapped = position.base_asset_amount.into_balance()?;
						let _ = T::Vamm::swap(&SwapConfigOf::<T> {
							vamm_id: market.vamm_id,
							asset: AssetType::Base,
							input_amount: base_swapped,
							direction: match position_direction {
								Direction::Long => VammDirection::Add,
								Direction::Short => VammDirection::Remove,
							},
							output_amount_limit: quote_abs_amount_decimal.into_balance()?,
						})?;

						entry_value = position.quote_asset_notional_amount;
						exit_value = match position_direction {
							Direction::Long => quote_abs_amount_decimal,
							Direction::Short => quote_abs_amount_decimal.neg(),
						};

						positions.swap_remove(position_index);
					},
					Ordering::Greater => {
						// reverse position
						let swapped = T::Vamm::swap(&SwapConfigOf::<T> {
							vamm_id: market.vamm_id,
							asset: AssetType::Quote,
							input_amount: quote_abs_amount_decimal.into_balance()?,
							direction: match direction {
								Direction::Long => VammDirection::Add,
								Direction::Short => VammDirection::Remove,
							},
							output_amount_limit: base_asset_amount_limit,
						})?;
						base_swapped = swapped.into_balance()?;

						// Since reversing is equivalent to closing a position and then opening a
						// new one in the opposite direction, all of the current position's PnL is
						// realized
						entry_value = position.quote_asset_notional_amount;
						exit_value = match position_direction {
							Direction::Long => abs_base_asset_value,
							Direction::Short => abs_base_asset_value.neg(),
						};

						position.base_asset_amount.try_add_mut(&T::Decimal::from_inner(swapped))?;
						position.quote_asset_notional_amount =
							exit_value.try_add(&match direction {
								Direction::Long => quote_abs_amount_decimal,
								Direction::Short => quote_abs_amount_decimal.neg(),
							})?;
					},
				};

				let pnl = exit_value.try_sub(&entry_value)?;
				// Realize PnL
				let margin = Self::get_margin(account_id).unwrap_or_else(T::Balance::zero);
				// TODO(0xangelo): properly handle bad debt incurred by large negative PnL
				AccountsMargin::<T>::insert(
					account_id,
					Self::update_margin_with_pnl(&margin, &pnl)?,
				);
			}

			Positions::<T>::insert(account_id, positions);

			Self::deposit_event(Event::TradeExecuted {
				market: market_id.clone(),
				direction,
				quote: quote_asset_amount,
				base: base_swapped,
			});

			Ok(base_swapped)
		}
	}

	impl<T: Config> Instruments for Pallet<T> {
		type Market = Market<T>;
		type Position = Position<T>;
		type Decimal = T::Decimal;

		fn funding_rate(market: &Self::Market) -> Result<Self::Decimal, DispatchError> {
			// Oracle returns prices in USDT cents
			let nonnormalized_oracle_twap = T::Oracle::get_twap(market.asset_id, vec![])?;
			let oracle_twap = Self::Decimal::checked_from_rational(nonnormalized_oracle_twap, 100)
				.ok_or(ArithmeticError::Overflow)?;

			let vamm_twap = T::Vamm::get_twap(&market.vamm_id)?;

			let price_spread = vamm_twap.try_sub(&oracle_twap)?;
			let period_adjustment = Self::Decimal::checked_from_rational(
				market.funding_frequency,
				market.funding_period,
			)
			.ok_or(ArithmeticError::Underflow)?;
			Ok(price_spread.try_mul(&period_adjustment)?)
		}

		fn unrealized_funding(
			market: &Self::Market,
			position: &Self::Position,
		) -> Result<Self::Decimal, DispatchError> {
			let cum_funding_delta = market.cum_funding_rate.try_sub(&position.last_cum_funding)?;
			Ok(cum_funding_delta.try_mul(&position.base_asset_amount)?)
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Helper Functions
	// ----------------------------------------------------------------------------------------------------

	// Helper functions - core functionality
	impl<T: Config> Pallet<T> {
		fn increase_position(
			position: &mut Position<T>,
			market: &Market<T>,
			direction: Direction,
			quote_abs_amount_decimal: &T::Decimal,
			base_asset_amount_limit: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let swapped = T::Vamm::swap(&SwapConfigOf::<T> {
				vamm_id: market.vamm_id,
				asset: AssetType::Quote,
				input_amount: quote_abs_amount_decimal.into_balance()?,
				direction: match direction {
					Direction::Long => VammDirection::Add,
					Direction::Short => VammDirection::Remove,
				},
				output_amount_limit: base_asset_amount_limit,
			})?;

			position.base_asset_amount.try_add_mut(&T::Decimal::from_inner(swapped))?;
			position.quote_asset_notional_amount.try_add_mut(&match direction {
				Direction::Long => *quote_abs_amount_decimal,
				Direction::Short => quote_abs_amount_decimal.neg(),
			})?;

			Ok(swapped.into_balance()?)
		}

		fn decrease_position(
			position: &mut Position<T>,
			market: &Market<T>,
			direction: Direction,
			quote_abs_amount_decimal: &T::Decimal,
			base_asset_amount_limit: T::Balance,
		) -> Result<(T::Balance, T::Decimal, T::Decimal), DispatchError> {
			let swapped = T::Vamm::swap(&SwapConfigOf::<T> {
				vamm_id: market.vamm_id,
				asset: AssetType::Quote,
				input_amount: quote_abs_amount_decimal.into_balance()?,
				direction: match direction {
					Direction::Long => VammDirection::Add,
					Direction::Short => VammDirection::Remove,
				},
				output_amount_limit: base_asset_amount_limit,
			})?;
			let base_delta_decimal = T::Decimal::from_inner(swapped);

			// Compute proportion of quote asset notional amount closed
			let entry_value = position.quote_asset_notional_amount.try_mul(
				&base_delta_decimal
					.saturating_abs()
					.try_div(&position.base_asset_amount.saturating_abs())?,
			)?;
			let exit_value = match direction {
				Direction::Long => quote_abs_amount_decimal.neg(),
				Direction::Short => *quote_abs_amount_decimal,
			};

			position.base_asset_amount.try_add_mut(&base_delta_decimal)?;
			position.quote_asset_notional_amount.try_sub_mut(&entry_value)?;

			Ok((swapped.into_balance()?, entry_value, exit_value))
		}
	}

	// Helper functions - validity checks
	impl<T: Config> Pallet<T> {}

	// Helper functions - low-level functionality
	impl<T: Config> Pallet<T> {
		fn get_or_create_position<'a>(
			positions: &'a mut BoundedVec<Position<T>, T::MaxPositions>,
			market_id: &T::MarketId,
			market: &Market<T>,
		) -> Result<(&'a mut Position<T>, usize), DispatchError> {
			Ok(match positions.iter().position(|p| p.market_id == *market_id) {
				Some(index) =>
					(positions.get_mut(index).expect("Item succesfully found above"), index),
				None => {
					positions
						.try_push(Position::<T> {
							market_id: market_id.clone(),
							base_asset_amount: Zero::zero(),
							quote_asset_notional_amount: Zero::zero(),
							last_cum_funding: market.cum_funding_rate,
						})
						.map_err(|_| Error::<T>::MaxPositionsExceeded)?;
					let index = positions.len() - 1;
					let position = positions
						.get_mut(index)
						.expect("Will always succeed if the above push does");
					(position, index)
				},
			})
		}

		fn position_direction(position: &Position<T>) -> Option<Direction> {
			if position.base_asset_amount.is_zero() {
				None
			} else if position.base_asset_amount.is_positive() {
				Some(Direction::Long)
			} else {
				Some(Direction::Short)
			}
		}

		fn base_asset_value(
			market: &Market<T>,
			position: &Position<T>,
			position_direction: Direction,
		) -> Result<T::Decimal, DispatchError> {
			let sim_swapped = T::Vamm::swap_simulation(&SwapSimulationConfigOf::<T> {
				vamm_id: market.vamm_id,
				asset: AssetType::Base,
				input_amount: position.base_asset_amount.into_balance()?,
				direction: match position_direction {
					Direction::Long => VammDirection::Add,
					Direction::Short => VammDirection::Remove,
				},
			})?;

			Ok(T::Decimal::from_inner(sim_swapped))
		}

		fn round_trade_if_necessary(
			market: &Market<T>,
			quote_abs_amount: &mut T::Decimal,
			base_abs_value: &T::Decimal,
		) -> Result<(), DispatchError> {
			let diff = base_abs_value.try_sub(quote_abs_amount)?;
			if diff.saturating_abs() < market.minimum_trade_size {
				// round trade to close off position
				*quote_abs_amount = *base_abs_value;
			}
			Ok(())
		}

		fn update_margin_with_pnl(
			margin: &T::Balance,
			pnl: &T::Decimal,
		) -> Result<T::Balance, DispatchError> {
			let abs_pnl = pnl.into_balance()?;

			Ok(match pnl.is_positive() {
				true => margin.checked_add(&abs_pnl).ok_or(ArithmeticError::Overflow)?,
				false => margin.saturating_sub(abs_pnl),
			})
		}
	}
}
