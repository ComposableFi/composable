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
//! - **Trader**: Primary user of the public extrinsics of the pallet
//! - **Derivative**: A financial instrument which derives its value from another asset, a.k.a. the
//!   _underlying_.
//! - **Perpetual contract**: A derivative product that allows a trader to have exposure to the
//!   underlying's price without owning it. See
//!   [The Cartoon Guide to Perps](https://www.paradigm.xyz/2021/03/the-cartoon-guide-to-perps)
//!   for intuitions.
//! - **Market**: Perpetual contracts market, where users trade virtual tokens mirroring the
//!   base-quote asset pair of spot markets. A.k.a. a virtual market.
//! - **VAMM**: Virtual automated market maker allowing price discovery in virtual markets based on
//!   the supply of virtual base/quote assets.
//! - **Position**: Amount of a particular virtual asset owned by a trader. Implies debt (positive
//!   or negative) to the Clearing House.
//! - **Collateral**: 'Real' asset(s) backing the trader's position(s), ensuring he/she can pay back
//!   the Clearing House.
//! - **`PnL`**: Profit and Loss, i.e., the difference between the current/exit and entry prices of
//!   a position
//! - **Margin**: A user's equity in a group of positions, i.e., it's collateral + total unrealized
//!   `PnL` + total unrealized funding payments
//! - **Margin ratio**: The ratio of the user's margin to his total position value. May be measured
//!   using either index (oracle) or mark (VAMM) prices
//! - **IMR**: Acronym for 'Initial Margin Ratio'. The mininum allowable margin ratio resulting from
//!   opening new positions. Inversely proportional to the maximum leverage of an account
//! - **MMR**: Acronym for 'Maintenance Margin Ratio'. The margin ratio below which a full
//!   liquidation of a user's account can be triggered by a liquidator (permissionless)
//! - **PMR**: Acronym for 'Partial Margin Ratio'. The margin ratio below which a partial
//!   liquidation of a user's account can be triggered by a liquidator (permissionless)
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
//! - [`deposit_collateral`](Call::deposit_collateral)
//! - [`create_market`](Call::create_market)
//! - [`open_position`](Call::open_position)
//! - [`update_funding`](Call::update_funding)
//! - [`liquidate`](Call::liquidate)
//!
//! ### Implemented Functions
//!
//! - [`deposit_collateral`](pallet/struct.Pallet.html#method.deposit_collateral-1)
//! - [`create_market`](pallet/struct.Pallet.html#method.create_market-1)
//! - [`open_position`](pallet/struct.Pallet.html#method.open_position-1)
//! - [`update_funding`](pallet/struct.Pallet.html#method.update_funding-1)
//! - [`liquidate`](pallet/struct.Pallet.html#method.liquidate-1)
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
		clippy::dbg_macro,
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

mod types;

mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	// ---------------------------------------------------------------------------------------------
	//                                Imports and Dependencies
	// ---------------------------------------------------------------------------------------------

	pub use crate::types::{
		Direction::{self as Direction, Long, Short},
		Market, MarketConfig, Position,
	};
	use crate::{
		math::{FixedPointMath, FromBalance, IntoBalance, IntoDecimal, IntoSigned, UnsignedMath},
		types::{AccountSummary, PositionInfo, BASIS_POINT_DENOMINATOR},
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
		transactional, Blake2_128Concat, PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use num_traits::Signed;
	use sp_runtime::{
		traits::{AccountIdConversion, CheckedAdd, CheckedDiv, CheckedMul, One, Saturating, Zero},
		ArithmeticError, FixedPointNumber, FixedPointOperand,
	};
	use sp_std::{cmp::Ordering, fmt::Debug, ops::Neg};

	// ---------------------------------------------------------------------------------------------
	//                             Declaration Of The Pallet Type
	// ---------------------------------------------------------------------------------------------

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// ---------------------------------------------------------------------------------------------
	//                                      Config Trait
	// ---------------------------------------------------------------------------------------------

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

		/// Integer type underlying fixed point decimal implementation. Must be convertible to/from
		/// the balance type
		type Integer: CheckedDiv
			+ CheckedMul
			+ Debug
			+ FixedPointOperand
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
			SwapConfig = SwapConfig<Self::VammId, Self::Balance>,
			SwapSimulationConfig = SwapSimulationConfig<Self::VammId, Self::Balance>,
			VammConfig = Self::VammConfig,
			VammId = Self::VammId,
		>;

		/// Configuration for creating and initializing a new vAMM instance. To be used as an
		/// extrinsic input
		type VammConfig: Clone + Debug + FullCodec + MaxEncodedLen + PartialEq + TypeInfo;

		/// Virtual automated market maker identifier; usually an integer
		type VammId: Clone + Copy + FullCodec + MaxEncodedLen + TypeInfo + Zero;

		/// Weight information for this pallet's extrinsics
		type WeightInfo: WeightInfo;
	}

	// ---------------------------------------------------------------------------------------------
	//                                      Pallet Types
	// ---------------------------------------------------------------------------------------------

	type AssetIdOf<T> = <T as DeFiComposableConfig>::MayBeAssetId;
	type BalanceOf<T> = <T as DeFiComposableConfig>::Balance;
	type DecimalOf<T> = <T as Config>::Decimal;
	type VammConfigOf<T> = <T as Config>::VammConfig;
	type VammIdOf<T> = <T as Config>::VammId;
	type SwapConfigOf<T> = SwapConfig<VammIdOf<T>, BalanceOf<T>>;
	type SwapSimulationConfigOf<T> = SwapSimulationConfig<VammIdOf<T>, BalanceOf<T>>;
	type MarketConfigOf<T> =
		MarketConfig<AssetIdOf<T>, BalanceOf<T>, DecimalOf<T>, VammConfigOf<T>>;

	// ---------------------------------------------------------------------------------------------
	//                                     Runtime Storage
	// ---------------------------------------------------------------------------------------------

	/// Supported collateral asset id.
	#[pallet::storage]
	pub type CollateralType<T: Config> = StorageValue<_, AssetIdOf<T>, OptionQuery>;

	/// Ratio of user's margin to be seized as fees upon a full liquidation event.
	#[pallet::storage]
	#[pallet::getter(fn full_liquidation_penalty)]
	#[allow(clippy::disallowed_types)]
	pub type FullLiquidationPenalty<T: Config> = StorageValue<_, T::Decimal, ValueQuery>;

	/// Ratio of full liquidation fees for compensating the liquidator.
	#[pallet::storage]
	#[pallet::getter(fn full_liquidation_penalty_liquidator_share)]
	#[allow(clippy::disallowed_types)]
	pub type FullLiquidationPenaltyLiquidatorShare<T: Config> =
		StorageValue<_, T::Decimal, ValueQuery>;

	/// Ratio of user's margin to be seized as fees upon a partial liquidation event.
	#[pallet::storage]
	#[pallet::getter(fn partial_liquidation_penalty)]
	#[allow(clippy::disallowed_types)]
	pub type PartialLiquidationPenalty<T: Config> = StorageValue<_, T::Decimal, ValueQuery>;

	/// Ratio of position's base asset to close in a partial liquidation.
	#[pallet::storage]
	#[pallet::getter(fn partial_liquidation_close_ratio)]
	#[allow(clippy::disallowed_types)]
	pub type PartialLiquidationCloseRatio<T: Config> = StorageValue<_, T::Decimal, ValueQuery>;

	/// Ratio of partial liquidation fees for compensating the liquidator.
	#[pallet::storage]
	#[pallet::getter(fn partial_liquidation_penalty_liquidator_share)]
	#[allow(clippy::disallowed_types)]
	pub type PartialLiquidationPenaltyLiquidatorShare<T: Config> =
		StorageValue<_, T::Decimal, ValueQuery>;

	/// Maps [AccountId](frame_system::Config::AccountId) to its collateral
	/// [Balance](DeFiComposableConfig::Balance), if set.
	#[pallet::storage]
	#[pallet::getter(fn get_collateral)]
	pub type Collateral<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance>;

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

	/// Maps [MarketId](Config::MarketId) to the corresponding virtual [Market] specs.
	#[pallet::storage]
	#[pallet::getter(fn get_market)]
	pub type Markets<T: Config> = StorageMap<_, Blake2_128Concat, T::MarketId, Market<T>>;

	// ---------------------------------------------------------------------------------------------
	//                                  Genesis Configuration
	// ---------------------------------------------------------------------------------------------

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		/// Genesis accepted collateral asset type.
		pub collateral_type: Option<AssetIdOf<T>>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { collateral_type: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			CollateralType::<T>::set(self.collateral_type);
		}
	}

	// ---------------------------------------------------------------------------------------------
	//                                     Runtime Events
	// ---------------------------------------------------------------------------------------------

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Margin successfully added to account.
		MarginAdded {
			/// Account id that received the deposit.
			account: T::AccountId,
			/// Asset type deposited.
			asset: AssetIdOf<T>,
			/// Amount of asset deposited.
			amount: T::Balance,
		},
		/// New virtual market successfully created.
		MarketCreated {
			/// Id for the newly created market.
			market: T::MarketId,
			/// Id of the underlying asset.
			asset: AssetIdOf<T>,
		},
		/// New trade successfully executed.
		TradeExecuted {
			/// Id of the market.
			market: T::MarketId,
			/// Direction of the trade (long/short).
			direction: Direction,
			/// Notional amount of quote asset exchanged.
			quote: T::Balance,
			/// Amount of base asset exchanged.
			base: T::Balance,
		},
		/// Market funding rate successfully updated.
		FundingUpdated {
			/// Id of the market.
			market: T::MarketId,
			/// Timestamp of the funding rate update.
			time: DurationSeconds,
		},
		/// Account fully liquidated.
		FullLiquidation {
			/// Id of the liquidated user.
			user: T::AccountId,
		},
		/// Account partially liquidated.
		PartialLiquidation {
			/// Id of the liquidated user.
			user: T::AccountId,
		},
	}

	// ---------------------------------------------------------------------------------------------
	//                                     Runtime Errors
	// ---------------------------------------------------------------------------------------------

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Attempted to create a new market but the funding period is not a multiple of the
		/// funding frequency.
		FundingPeriodNotMultipleOfFrequency,
		/// Raised when opening a risk-increasing position that takes the account below the IMR.
		InsufficientCollateral,
		/// Attempted to create a new market but the ordering initial > partial > maintenance is
		/// broken.
		InvalidMarginRatioOrdering,
		/// Attempted to create a new market but either the initial margin ratio is outside (0, 1]
		/// or the maintenance margin ratio is outside (0, 1).
		InvalidMarginRatioRequirement,
		/// Raised when querying a market with an invalid or nonexistent market Id.
		MarketIdNotFound,
		/// Raised when creating a new position but exceeding the maximum number of positions for
		/// an account.
		MaxPositionsExceeded,
		/// Attempted to create a new market but the minimum trade size is negative.
		NegativeMinimumTradeSize,
		/// An operation required the asset id of a valid collateral type but none were registered.
		NoCollateralTypeSet,
		/// Attempted to create a new market but the underlying asset is not supported by the
		/// oracle.
		NoPriceFeedForAsset,
		/// Raised when trying to fetch a position from the positions vector with an invalid index.
		PositionNotFound,
		/// Attempted to liquidate a user's account but it has sufficient collateral to back its
		/// positions.
		SufficientCollateral,
		/// Raised when creating a new position with quote asset amount less than the market's
		/// minimum trade size.
		TradeSizeTooSmall,
		/// User attempted to deposit an unsupported asset type as collateral in its margin
		/// account.
		UnsupportedCollateralType,
		/// Raised when trying to update the funding rate for a market before its funding frequency
		/// has passed since its last update.
		UpdatingFundingTooEarly,
		/// Raised when trying to liquidate a user with no open positions.
		UserHasNoPositions,
		/// Attempted to create a new market but the funding period or frequency is 0 seconds long.
		ZeroLengthFundingPeriodOrFrequency,
	}

	// ---------------------------------------------------------------------------------------------
	//                                       Extrinsics
	// ---------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Deposit collateral to a trader's account.
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
		/// The collateral type must be supported, i.e., the one contained in [`CollateralType`].
		///
		/// ## Emits
		/// * [`MarginAdded`](Event::<T>::MarginAdded)
		///
		/// ## State Changes
		/// Updates the [`Collateral`] storage map. If an account does not exist in
		/// [`Collateral`], it is created and initialized with 0 margin.
		///
		/// ## Errors
		/// * [`UnsupportedCollateralType`](Error::<T>::UnsupportedCollateralType)
		///
		/// # Weight/Runtime
		/// `O(1)`
		#[pallet::weight(<T as Config>::WeightInfo::add_margin())]
		pub fn deposit_collateral(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			amount: T::Balance,
		) -> DispatchResult {
			let account_id = ensure_signed(origin)?;
			<Self as ClearingHouse>::deposit_collateral(&account_id, asset_id, amount)?;
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
		/// * Initial and Maintenance margin ratios must be in the (0, 1] and (0, 1) intervals
		///   respectively
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
		/// - [`InvalidMarginRatioOrdering`](Error::<T>::InvalidMarginRatioOrdering)
		///
		/// # Weight/Runtime
		/// `O(1)`
		#[pallet::weight(<T as Config>::WeightInfo::create_market())]
		pub fn create_market(origin: OriginFor<T>, config: MarketConfigOf<T>) -> DispatchResult {
			ensure_signed(origin)?;
			let _ = <Self as ClearingHouse>::create_market(&config)?;
			Ok(())
		}

		/// Opens a position in a market.
		///
		/// # Overview
		///
		/// This may result in the following outcomes:
		/// - Creation of a whole new position in the market, if one didn't already exist
		/// - An increase in the size of an existing position, if the trade's direction matches the
		///   existing position's one
		/// - A decrease in the size of an existing position, if the trade's direction is counter to
		///   the existing position's one and its magnitude is smaller than the existing postion's
		///   size
		/// - Closing of the existing position, if the trade's direction is counter to the existion
		///   position's one and its magnitude is approximately the existing position's size
		/// - Reversing of the existing position, if the trade's direction is counter to the
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
		/// - The market must exist and have been initialized prior to calling this extrinsic
		/// - There's a maximum number of positions ([`Config::MaxPositions`]) that can be open for
		///   each account id at any given time. If opening a position in a new market exceeds this
		///   number, the transactions fails.
		/// - Each market has a [minimum trade size](Market::minimum_trade_size) required, so trades
		///   with quote asset amount less than this threshold will be rejected
		/// - Trades which increase the total risk of an account (and thus its margin requirement),
		///   will be rejected if they result in the account falling below its aggregate IMR
		///
		/// ## Emits
		///
		/// - [`TradeExecuted`](Event::<T>::TradeExecuted)
		///
		/// ## State Changes
		///
		/// The following storage items may be modified:
		/// - [`Collateral`]: if trade decreases, closes, or reverses a position, its PnL is
		///   realized
		/// - [`Positions`]: a new entry may be added or an existing one updated/removed
		///
		/// ## Errors
		///
		/// - [`TradeSizeTooSmall`](Error::<T>::TradeSizeTooSmall)
		/// - [`MarketIdNotFound`](Error::<T>::MarketIdNotFound)
		/// - [`MaxPositionsExceeded`](Error::<T>::MaxPositionsExceeded)
		/// - [`InsufficientCollateral`](Error::<T>::InsufficientCollateral)
		/// - [`ArithmeticError`]
		///
		/// # Weight/Runtime
		///
		/// The total runtime is O(`n`), where `n` is the number of open positions after executing
		/// the trade.
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

		/// Update the funding rate for a market.
		///
		/// # Overview
		///
		/// This should be called periodically for each market so that subsequent calculations of
		/// unrealized funding for each position are up-to-date.
		///
		/// If there's Long-Short imbalance in the market, funding payments may be transferred
		/// between the market Fee Pool and the Clearing House. This is done symbolically by
		/// updating the Fee Pool and cumulative funding rate attributes of the market. The latter
		/// influences the unrealized funding of all traders. Settlement of the unrealized funding
		/// is done by traders via other extrinsics.
		///
		/// ![](https://www.plantuml.com/plantuml/svg/FOqx2iCm40Nxd28vWFtwL8P0xh6MrfPWzM4_vFenAL8DmnIpcPDwDBazQayIcKFbNjodFO6pUebzJQFXDTeSHhlmkoBz1KeViAN2YaEfCP8mQUtdKaOO8rSwbPeXPYRdvOYUhxfEeVxxRjppnIy0)
		///
		/// ## Parameters
		/// - `market_id`: the perpetuals market Id
		///
		/// ## Assumptions or Requirements
		///
		/// Each market has a [`funding_frequency`](Market::<T>::funding_frequency) which defines
		/// the minimum time between calls to this extrinsic. If one attempts to call this before
		/// `funding_frequency` has elapsed since the last funding update, the transaction will
		/// fail.
		///
		/// ## Emits
		///
		/// - [`FundingUpdated`](Event::<T>::FundingUpdated)
		///
		/// ## State Changes
		///
		/// [`Markets`] is updated, specifically the [`Market`] attributes:
		/// - [`cum_funding_rate`](Market::<T>::cum_funding_rate)
		/// - [`funding_rate_ts`](Market::<T>::funding_rate_ts)
		/// - [`fee_pool`](Market::<T>::fee_pool), if there's Long-Short imbalance
		///
		/// ## Errors
		///
		/// - [`MarketIdNotFound`](Error::<T>::MarketIdNotFound)
		/// - [`UpdatingFundingTooEarly`](Error::<T>::UpdatingFundingTooEarly)
		///
		/// ## Weight/Runtime
		///
		/// `O(1)`
		#[pallet::weight(<T as Config>::WeightInfo::update_funding())]
		pub fn update_funding(origin: OriginFor<T>, market_id: T::MarketId) -> DispatchResult {
			ensure_signed(origin)?;
			<Self as ClearingHouse>::update_funding(&market_id)?;
			Ok(())
		}

		/// Liquidates a user's account if below margin requirements.
		///
		/// # Overview
		///
		/// Liquidation can be either full or partial. In the former case, positions are closed
		/// entirely, while in the latter, they are partially closed until the account is brought
		/// back above the initial margin requirement.
		///
		/// Note that both unrealized PnL and funding payments contribute to an account being
		/// brought below the maintenance margin ratio. Liquidation realizes a user's PnL and
		/// funding payments.
		///
		/// Positions in markets with the highest margin requirements (i.e., the lowest max leverage
		/// for opening a position) are liquidated first.
		///
		/// The caller of the function, the 'liquidator', may be credited with a liquidation fee in
		/// their account, which can be withdrawn via another extrinsic.
		///
		/// ## Parameters
		///
		/// - `user_id`: the account Id of the user to be liquidated
		///
		/// ## Assumptions or Requirements
		///
		/// ### For full liquidation
		///
		/// The user's margin ration must be stricly less than the combined margin ratios of all the
		/// markets in which it has open positions in. In other words, the user's margin (collateral
		/// + total unrealized pnl + total unrealized funding) must be strictly less than the sum of
		/// margin requirements (MMR * base asset value) for each market it has an open position in.
		///
		/// ## Emits
		///
		/// - [`FullLiquidation`](Event::<T>::FullLiquidation)
		/// - [`PartialLiquidation`](Event::<T>::PartialLiquidation)
		///
		/// ## State Changes
		///
		/// - Updates the base asset amount of the [`markets`](Markets) of closed positions
		/// - Removes closed [`positions`](Positions)
		/// - Updates the user's account [`collateral`](Collateral)
		/// - Updates the liquidator's account [`collateral`](Collateral) if fees are due
		/// - Transfers collateral from collateral account to Insurance Fund account if fees apply
		///
		/// ## Errors
		///
		/// - [`UserHasNoPositions`](Error::<T>::UserHasNoPositions)
		/// - [`SufficientCollateral`](Error::<T>::SufficientCollateral)
		/// - [`NoCollateralTypeSet`](Error::<T>::NoCollateralTypeSet)
		/// - [`ArithmeticError`]
		///
		/// ## Weight/Runtime
		///
		/// `O(n * log(n))` worst case, where `n` is the number of positions of the target user.
		/// This is due to the ordering of positions by margin requirement.
		#[pallet::weight(<T as Config>::WeightInfo::liquidate())]
		pub fn liquidate(origin: OriginFor<T>, user_id: T::AccountId) -> DispatchResult {
			let liquidator_id = ensure_signed(origin)?;
			<Self as ClearingHouse>::liquidate(&liquidator_id, &user_id)?;
			Ok(())
		}
	}

	// ---------------------------------------------------------------------------------------------
	//                                  Trait Implementations
	// ---------------------------------------------------------------------------------------------

	impl<T: Config> ClearingHouse for Pallet<T> {
		type AccountId = T::AccountId;
		type AssetId = AssetIdOf<T>;
		type Balance = T::Balance;
		type Direction = Direction;
		type MarketId = T::MarketId;
		type MarketConfig = MarketConfigOf<T>;

		fn deposit_collateral(
			account_id: &Self::AccountId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<(), DispatchError> {
			ensure!(
				CollateralType::<T>::get().ok_or(Error::<T>::NoCollateralTypeSet)? == asset_id,
				Error::<T>::UnsupportedCollateralType
			);

			// Assuming stablecoin collateral and all markets quoted in dollars
			let pallet_acc = Self::get_collateral_account();
			T::Assets::transfer(asset_id, account_id, &pallet_acc, amount, true)?;

			let old_collateral = Self::get_collateral(&account_id).unwrap_or_else(T::Balance::zero);
			let new_collateral = old_collateral.try_add(&amount)?;
			Collateral::<T>::insert(&account_id, new_collateral);

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
					config.margin_ratio_initial <= T::Decimal::one() &&
					config.margin_ratio_maintenance > T::Decimal::zero() &&
					config.margin_ratio_maintenance < T::Decimal::one() &&
					config.margin_ratio_partial > T::Decimal::zero() &&
					config.margin_ratio_partial < T::Decimal::one(),
				Error::<T>::InvalidMarginRatioRequirement
			);
			ensure!(
				config.margin_ratio_initial > config.margin_ratio_partial &&
					config.margin_ratio_partial > config.margin_ratio_maintenance,
				Error::<T>::InvalidMarginRatioOrdering
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
					margin_ratio_partial: config.margin_ratio_partial,
					minimum_trade_size: config.minimum_trade_size,
					base_asset_amount_long: Zero::zero(),
					base_asset_amount_short: Zero::zero(),
					fee_pool: Zero::zero(),
					funding_frequency: config.funding_frequency,
					funding_period: config.funding_period,
					cum_funding_rate_long: Zero::zero(),
					cum_funding_rate_short: Zero::zero(),
					funding_rate_ts: T::UnixTime::now().as_secs(),
					taker_fee: config.taker_fee,
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
			let mut market = Self::get_market(&market_id).ok_or(Error::<T>::MarketIdNotFound)?;
			let mut quote_abs_amount_decimal = T::Decimal::from_balance(quote_asset_amount)?;
			ensure!(
				quote_abs_amount_decimal >= market.minimum_trade_size,
				Error::<T>::TradeSizeTooSmall
			);

			let mut positions = Self::get_positions(&account_id);
			let (position, position_index) =
				Self::get_or_create_position(&mut positions, market_id, &market, direction)?;
			let position_direction = position.direction().unwrap_or(direction);

			let mut collateral = Self::get_collateral(account_id).unwrap_or_else(T::Balance::zero);
			// Settle funding for position before any modifications
			Self::settle_funding(position, &market, &mut collateral)?;

			// Whether or not the trade increases the risk exposure of the account
			let mut is_risk_increasing = false;
			let base_swapped: T::Balance;
			if direction == position_direction {
				base_swapped = Self::increase_position(
					position,
					&mut market,
					direction,
					&quote_abs_amount_decimal,
					base_asset_amount_limit,
				)?;

				is_risk_increasing = true;
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
				(base_swapped, entry_value, exit_value) =
					match quote_abs_amount_decimal.cmp(&abs_base_asset_value) {
						Ordering::Less => Self::decrease_position(
							position,
							&mut market,
							direction,
							&quote_abs_amount_decimal,
							base_asset_amount_limit,
						)?,
						Ordering::Equal => Self::close_position(
							&mut positions,
							position_index,
							position_direction,
							&mut market,
							quote_abs_amount_decimal.into_balance()?,
						)?,
						Ordering::Greater => {
							is_risk_increasing = quote_abs_amount_decimal
								.try_sub(&abs_base_asset_value)? >
								abs_base_asset_value;

							Self::reverse_position(
								position,
								&mut market,
								direction,
								&quote_abs_amount_decimal,
								base_asset_amount_limit,
								&abs_base_asset_value,
							)?
						},
					};

				let pnl = exit_value.try_sub(&entry_value)?;
				// Realize PnL
				// TODO(0xangelo): properly handle bad debt incurred by large negative PnL
				collateral = Self::update_margin_with_pnl(&collateral, &pnl)?;
			}

			// Charge fees
			let fee = Self::fee_for_trade(&market, &quote_abs_amount_decimal)?;
			collateral.try_sub_mut(&fee)?;
			market.fee_pool.try_add_mut(&fee)?;

			// Check account risk
			if is_risk_increasing {
				ensure!(
					Self::meets_initial_margin_ratio(&positions, collateral)?,
					Error::<T>::InsufficientCollateral
				);
			}

			// Update storage
			Collateral::<T>::insert(account_id, collateral);
			Positions::<T>::insert(account_id, positions);
			Markets::<T>::insert(market_id, market);

			Self::deposit_event(Event::TradeExecuted {
				market: market_id.clone(),
				direction,
				quote: quote_asset_amount,
				base: base_swapped,
			});
			Ok(base_swapped)
		}

		#[transactional]
		fn update_funding(market_id: &Self::MarketId) -> Result<(), DispatchError> {
			let mut market = Self::try_get_market(market_id)?;
			let now = T::UnixTime::now().as_secs();
			ensure!(
				Self::is_funding_update_time(&market, now)?,
				Error::<T>::UpdatingFundingTooEarly
			);

			// Pay funding
			// net position sign | funding rate sign | transfer
			// --------------------------------------------------------------
			//                -1 |                -1 | Collateral -> Fee Pool
			//                -1 |                 1 | Fee Pool -> Collateral
			//                 1 |                -1 | Fee Pool -> Collateral
			//                 1 |                 1 | Collateral -> Fee Pool
			//                 - |                 0 | n/a
			//                 0 |                 - | n/a
			let net_base_asset_amount =
				market.base_asset_amount_long.try_add(&market.base_asset_amount_short)?;
			let funding_rate = <Self as Instruments>::funding_rate(&market)?;
			let mut funding_rate_long = funding_rate;
			let mut funding_rate_short = funding_rate;

			if !(funding_rate.is_zero() | net_base_asset_amount.is_zero()) {
				let uncapped_funding = funding_rate.try_mul(&net_base_asset_amount)?;

				if uncapped_funding.is_positive() {
					// Fee Pool receives funding
					market.fee_pool.try_add_mut(&uncapped_funding.into_balance()?)?;
				} else {
					// Fee Pool pays funding
					// TODO(0xangelo): set limits for
					// - total Fee Pool usage (reserve some funds for other operations)
					// - Fee Pool usage for funding payments per call to `update_funding`
					let usable_fees: T::Decimal = -market.fee_pool.into_decimal()?;
					let mut capped_funding = sp_std::cmp::max(uncapped_funding, usable_fees);

					// Since we're dealing with negatives, we check if the uncapped funding is
					// *smaller* (greater in absolute terms) than the capped one
					if capped_funding > uncapped_funding {
						// Total funding paid to one side is the sum of the funding paid by the
						// opposite side plus the complement from the Fee Pool
						let excess;
						if net_base_asset_amount.is_positive() {
							let counterparty_funding = usable_fees
								.try_sub(&funding_rate.try_mul(&market.base_asset_amount_short)?)?;
							(funding_rate_long, excess) =
								counterparty_funding.try_div_rem(&market.base_asset_amount_long)?;
						} else {
							let counterparty_funding = funding_rate
								.try_mul(&market.base_asset_amount_long)?
								.try_sub(&usable_fees)?;
							(funding_rate_short, excess) = counterparty_funding
								.try_div_rem(&market.base_asset_amount_short)?;
						}
						capped_funding.try_sub_mut(&excess)?;
					}

					market.fee_pool.try_sub_mut(&capped_funding.into_balance()?)?;
				};
			}

			// Update market state
			market.cum_funding_rate_long.try_add_mut(&funding_rate_long)?;
			market.cum_funding_rate_short.try_add_mut(&funding_rate_short)?;
			market.funding_rate_ts = now;
			Markets::<T>::insert(market_id, market);

			Self::deposit_event(Event::FundingUpdated { market: market_id.clone(), time: now });
			Ok(())
		}

		#[transactional]
		fn liquidate(
			liquidator_id: &Self::AccountId,
			user_id: &Self::AccountId,
		) -> Result<(), DispatchError> {
			let positions = Self::get_positions(user_id);
			ensure!(positions.len() > 0, Error::<T>::UserHasNoPositions);

			let summary = Self::summarize_account_state(user_id, positions)?;

			let liquidator_fee: T::Balance;
			let insurance_fee: T::Balance;
			let event: Event<T>;
			if summary.margin < summary.margin_requirement_maintenance {
				(liquidator_fee, insurance_fee) = Self::fully_liquidate_account(user_id, summary)?;
				event = Event::<T>::FullLiquidation { user: user_id.clone() };
			} else if summary.margin < summary.margin_requirement_partial {
				(liquidator_fee, insurance_fee) =
					Self::partially_liquidate_account(user_id, summary)?;
				event = Event::<T>::PartialLiquidation { user: user_id.clone() };
			} else {
				return Err(Error::<T>::SufficientCollateral.into())
			}

			if !liquidator_fee.is_zero() {
				let col = Self::get_collateral(liquidator_id).unwrap_or_else(Zero::zero);
				Collateral::<T>::insert(liquidator_id, col.try_add(&liquidator_fee)?);
			}
			if !insurance_fee.is_zero() {
				let asset_id = CollateralType::<T>::get().ok_or(Error::<T>::NoCollateralTypeSet)?;
				T::Assets::transfer(
					asset_id,
					&Self::get_collateral_account(),
					&Self::get_insurance_account(),
					insurance_fee,
					false,
				)?;
			}

			Self::deposit_event(event);
			Ok(())
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

			let vamm_twap: Self::Decimal = T::Vamm::get_twap(&market.vamm_id, AssetType::Base)
				.and_then(|p| p.into_signed().map_err(|e| e.into()))?;

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
			if let Some(direction) = position.direction() {
				let cum_funding_delta =
					market.cum_funding_rate(direction).try_sub(&position.last_cum_funding)?;
				let payment = cum_funding_delta.try_mul(&position.base_asset_amount)?;
				Ok(payment.neg())
			} else {
				Ok(Zero::zero())
			}
		}
	}

	// ---------------------------------------------------------------------------------------------
	//                                    Helper Functions
	// ---------------------------------------------------------------------------------------------

	// Helper functions - core functionality
	impl<T: Config> Pallet<T> {
		fn summarize_account_state(
			account_id: &T::AccountId,
			positions: BoundedVec<Position<T>, T::MaxPositions>,
		) -> Result<AccountSummary<T>, DispatchError> {
			let collateral = Self::get_collateral(account_id).unwrap_or_else(Zero::zero);

			let mut summary = AccountSummary::<T>::new(collateral)?;
			for position in positions {
				let market = Self::try_get_market(&position.market_id)?;
				if let Some(direction) = position.direction() {
					// should always succeed
					let (base_asset_value, unrealized_pnl) =
						Self::abs_position_notional_and_pnl(&market, &position, direction)?;

					let info = PositionInfo::<T> {
						direction,
						margin_requirement_maintenance: base_asset_value
							.try_mul(&market.margin_ratio_maintenance)?,
						margin_requirement_partial: base_asset_value
							.try_mul(&market.margin_ratio_partial)?,
						base_asset_value,
						unrealized_pnl,
						unrealized_funding: <Self as Instruments>::unrealized_funding(
							&market, &position,
						)?,
					};

					summary.update(market, position, info)?;
				}
			}

			Ok(summary)
		}

		/// Fully liquidates the user's positions until its account is brought above the MMR.
		///
		/// This function does **not** check if the account is below the MMR beforehand.
		///
		/// ## Storage modifications
		///
		/// - Updates the [`markets`](Markets) of closed positions (according to changes in
		///   [`Self::close_position_in_market`])
		/// - Removes closed [`positions`](Positions)
		/// - Updates the user's account [`collateral`](Collateral)
		///
		/// ## Returns
		///
		/// The fees for the liquidator and insurance fund
		fn fully_liquidate_account(
			user_id: &T::AccountId,
			summary: AccountSummary<T>,
		) -> Result<(T::Balance, T::Balance), DispatchError> {
			let AccountSummary::<T> {
				mut collateral,
				mut margin,
				margin_requirement_maintenance: mut margin_requirement,
				base_asset_value,
				mut positions_summary,
				..
			} = summary;

			let mut positions = BoundedVec::<Position<T>, T::MaxPositions>::default();
			let maximum_fee = Self::full_liquidation_penalty().try_mul(&margin)?;
			let mut fees = T::Balance::zero();
			// Sort positions from greatest to lowest margin requirement
			positions_summary.sort_by_key(|(_, _, info)| info.margin_requirement_maintenance.neg());
			for (mut market, position, info) in positions_summary {
				if margin < margin_requirement {
					Self::close_position_in_market(
						&position,
						info.direction,
						&mut market,
						info.base_asset_value.into_balance()?,
					)?;
					Markets::<T>::insert(&position.market_id, market);

					let base_asset_value_share =
						info.base_asset_value.try_div(&base_asset_value)?;
					let fee_decimal = base_asset_value_share.try_mul(&maximum_fee)?;
					margin.try_sub_mut(&fee_decimal)?;
					margin_requirement.try_sub_mut(&info.margin_requirement_maintenance)?;
					fees.try_add_mut(&fee_decimal.into_balance()?)?;
					collateral = Self::updated_balance(
						&collateral,
						&info
							.unrealized_pnl
							.try_add(&info.unrealized_funding)?
							.try_sub(&fee_decimal)?,
					)?;
				} else {
					// AccountSummary::positions_summary isn't constrained to be shorter than the
					// maximum number of positions, so we keep the error checking here.
					positions.try_push(position).map_err(|_| Error::<T>::MaxPositionsExceeded)?;
				}
			}

			// Charge fees
			let liquidator_fee =
				Self::full_liquidation_penalty_liquidator_share().saturating_mul_int(fees);
			let insurance_fee = fees.try_sub(&liquidator_fee)?;

			Positions::<T>::insert(user_id, positions);
			Collateral::<T>::insert(user_id, collateral);

			Ok((liquidator_fee, insurance_fee))
		}

		/// Partially liquidates the user's positions until its account is brought above the PMR.
		///
		/// This function does **not** check if the account is below the PMR beforehand.
		///
		/// ## Storage modifications
		///
		/// - Updates the [`markets`](Markets) of decreased positions (according to changes made by
		///   [`Self::decrease_position`])
		/// - Updates reduced [`positions`](Positions) (according to changes made by
		///   [`Self::decrease_position`])
		/// - Updates the user's account [`collateral`](Collateral)
		///
		/// ## Returns
		///
		/// The fees for the liquidator and insurance fund
		fn partially_liquidate_account(
			user_id: &T::AccountId,
			summary: AccountSummary<T>,
		) -> Result<(T::Balance, T::Balance), DispatchError> {
			let AccountSummary::<T> {
				mut collateral,
				mut margin,
				margin_requirement_partial: mut margin_requirement,
				base_asset_value,
				mut positions_summary,
				..
			} = summary;

			let mut positions = BoundedVec::<Position<T>, T::MaxPositions>::default();
			let mut fees = T::Balance::zero();
			let maximum_fee = Self::partial_liquidation_penalty().try_mul(&margin)?;
			let close_ratio = Self::partial_liquidation_close_ratio();
			let maximum_close_value = close_ratio.try_mul(&base_asset_value)?;
			// Sort positions from greatest to lowest margin requirement
			positions_summary.sort_by_key(|(_, _, info)| info.margin_requirement_partial.neg());
			for (mut market, mut position, info) in positions_summary {
				if margin < margin_requirement {
					Self::settle_funding(&mut position, &market, &mut collateral)?;

					let base_value_to_close = close_ratio.try_mul(&info.base_asset_value)?;
					let (_, entry_value, exit_value) = Self::decrease_position(
						&mut position,
						&mut market,
						info.direction.opposite(),
						&base_value_to_close,
						Zero::zero(), /* No slippage control is necessary since it was already
						               * taken into account when computing `base_asset_value` */
					)?;
					Markets::<T>::insert(&position.market_id, market);

					let closed_share = base_value_to_close.try_div(&maximum_close_value)?;
					let fee_decimal = closed_share.try_mul(&maximum_fee)?;
					let requirement_freed =
						closed_share.try_mul(&info.margin_requirement_partial)?;
					let realized_pnl = exit_value.try_sub(&entry_value)?;

					fees.try_add_mut(&fee_decimal.into_balance()?)?;
					collateral =
						Self::updated_balance(&collateral, &realized_pnl.try_sub(&fee_decimal)?)?;
					margin.try_sub_mut(&fee_decimal)?;
					margin_requirement.try_sub_mut(&requirement_freed)?;
				}

				// No positions are fully closed, so we push all.
				// AccountSummary::positions_summary isn't constrained to be shorter than the
				// maximum number of positions, so we keep the error checking here.
				positions.try_push(position).map_err(|_| Error::<T>::MaxPositionsExceeded)?;
			}

			// Charge fees
			let liquidator_fee =
				Self::partial_liquidation_penalty_liquidator_share().saturating_mul_int(fees);
			let insurance_fee = fees.try_sub(&liquidator_fee)?;

			Positions::<T>::insert(user_id, positions);
			Collateral::<T>::insert(user_id, collateral);

			Ok((liquidator_fee, insurance_fee))
		}

		fn settle_funding(
			position: &mut Position<T>,
			market: &Market<T>,
			margin: &mut T::Balance,
		) -> Result<(), DispatchError> {
			if let Some(direction) = position.direction() {
				let payment = <Self as Instruments>::unrealized_funding(market, position)?;
				// TODO(0xangelo): can we have bad debt from unrealized funding if user wasn't
				// liquidated in time?
				*margin = Self::updated_balance(margin, &payment)?;
				position.last_cum_funding = market.cum_funding_rate(direction);
			}
			Ok(())
		}

		fn fee_for_trade(
			market: &Market<T>,
			quote_abs_amount: &T::Decimal,
		) -> Result<T::Balance, ArithmeticError> {
			quote_abs_amount
				.into_balance()?
				.try_mul(&market.taker_fee)?
				.try_div(&BASIS_POINT_DENOMINATOR.into())
		}

		fn increase_position(
			position: &mut Position<T>,
			market: &mut Market<T>,
			direction: Direction,
			quote_abs_amount_decimal: &T::Decimal,
			base_asset_amount_limit: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let base_swapped = Self::swap_quote(
				market,
				direction,
				quote_abs_amount_decimal,
				base_asset_amount_limit,
			)?;
			let base_delta_decimal = Self::decimal_from_swapped(base_swapped, direction)?;

			position.base_asset_amount.try_add_mut(&base_delta_decimal)?;
			position.quote_asset_notional_amount.try_add_mut(&match direction {
				Long => *quote_abs_amount_decimal,
				Short => quote_abs_amount_decimal.neg(),
			})?;

			market.add_base_asset_amount(&base_delta_decimal, direction)?;

			Ok(base_swapped)
		}

		fn decrease_position(
			position: &mut Position<T>,
			market: &mut Market<T>,
			direction: Direction,
			quote_abs_amount_decimal: &T::Decimal,
			base_asset_amount_limit: T::Balance,
		) -> Result<(T::Balance, T::Decimal, T::Decimal), DispatchError> {
			let base_swapped = Self::swap_quote(
				market,
				direction,
				quote_abs_amount_decimal,
				base_asset_amount_limit,
			)?;
			let base_delta_decimal = Self::decimal_from_swapped(base_swapped, direction)?;

			// Compute proportion of quote asset notional amount closed
			let entry_value = position.quote_asset_notional_amount.try_mul(
				&base_delta_decimal
					.saturating_abs()
					.try_div(&position.base_asset_amount.saturating_abs())?,
			)?;
			// Trade direction is opposite of position direction, so we compute the exit value
			// accordingly
			let exit_value = match direction {
				Long => quote_abs_amount_decimal.neg(),
				Short => *quote_abs_amount_decimal,
			};

			position.base_asset_amount.try_add_mut(&base_delta_decimal)?;
			position.quote_asset_notional_amount.try_sub_mut(&entry_value)?;

			market.add_base_asset_amount(&base_delta_decimal, direction.opposite())?;

			Ok((base_swapped, entry_value, exit_value))
		}

		fn close_position(
			positions: &mut BoundedVec<Position<T>, T::MaxPositions>,
			position_index: usize,
			position_direction: Direction,
			market: &mut Market<T>,
			quote_asset_amount_limit: T::Balance,
		) -> Result<(T::Balance, T::Decimal, T::Decimal), DispatchError> {
			// This should always succeed if called by <Self as ClearingHouse>::open_position
			let position = positions.get(position_index).ok_or(Error::<T>::PositionNotFound)?;
			let close_result = Self::close_position_in_market(
				position,
				position_direction,
				market,
				quote_asset_amount_limit,
			)?;
			positions.swap_remove(position_index);
			Ok(close_result)
		}

		fn close_position_in_market(
			position: &Position<T>,
			position_direction: Direction,
			market: &mut Market<T>,
			quote_asset_amount_limit: T::Balance,
		) -> Result<(T::Balance, T::Decimal, T::Decimal), DispatchError> {
			let base_swapped = position.base_asset_amount.into_balance()?;
			let quote_swapped = Self::swap_base(
				market,
				position_direction,
				base_swapped,
				quote_asset_amount_limit,
			)?;

			let entry_value = position.quote_asset_notional_amount;
			let quote_amount_decimal: T::Decimal = quote_swapped.into_decimal()?;
			let exit_value = match position_direction {
				Long => quote_amount_decimal,
				Short => quote_amount_decimal.neg(),
			};

			market.sub_base_asset_amount(&position.base_asset_amount, position_direction)?;
			Ok((base_swapped, entry_value, exit_value))
		}

		fn reverse_position(
			position: &mut Position<T>,
			market: &mut Market<T>,
			direction: Direction,
			quote_abs_amount_decimal: &T::Decimal,
			base_asset_amount_limit: T::Balance,
			abs_base_asset_value: &T::Decimal,
		) -> Result<(T::Balance, T::Decimal, T::Decimal), DispatchError> {
			let base_swapped = Self::swap_quote(
				market,
				direction,
				quote_abs_amount_decimal,
				base_asset_amount_limit,
			)?;

			// Since reversing is equivalent to closing a position and then opening a
			// new one in the opposite direction, all of the current position's PnL is
			// realized
			let entry_value = position.quote_asset_notional_amount;
			// Trade direction is opposite of position direction, so we compute the exit value
			// accordingly
			let exit_value = match direction {
				Long => abs_base_asset_value.neg(),
				Short => *abs_base_asset_value,
			};

			// Account for the implicit position closing
			market.sub_base_asset_amount(&position.base_asset_amount, direction.opposite())?;

			position
				.base_asset_amount
				.try_add_mut(&Self::decimal_from_swapped(base_swapped, direction)?)?;
			position.quote_asset_notional_amount = exit_value.try_add(&match direction {
				Long => *quote_abs_amount_decimal,
				Short => quote_abs_amount_decimal.neg(),
			})?;
			// Update to account for direction change
			position.last_cum_funding = market.cum_funding_rate(direction);

			market.add_base_asset_amount(&position.base_asset_amount, direction)?;

			Ok((base_swapped, entry_value, exit_value))
		}
	}

	// Helper functions - validity checks
	impl<T: Config> Pallet<T> {
		fn is_funding_update_time(
			market: &Market<T>,
			now: DurationSeconds,
		) -> Result<bool, DispatchError> {
			let funding_frequency = market.funding_frequency;
			let mut next_update_wait = funding_frequency;

			if funding_frequency > 0 {
				// Usual update times are at multiples of funding frequency
				// Safe since funding frequency is positive
				let last_update_delay = market.funding_rate_ts.rem_euclid(funding_frequency);

				if last_update_delay > 0 {
					let max_delay_for_not_skipping = funding_frequency.try_div(&3)?;

					next_update_wait = if last_update_delay > max_delay_for_not_skipping {
						// Skip update at the next multiple of funding frequency
						funding_frequency.try_mul(&2)?.try_sub(&last_update_delay)?
					} else {
						// Allow update at the next multiple of funding frequency
						funding_frequency.try_sub(&last_update_delay)?
					};
				}
			}

			// Check that enough time has passed since last update
			Ok(now.try_sub(&market.funding_rate_ts)? >= next_update_wait)
		}

		fn meets_initial_margin_ratio(
			positions: &BoundedVec<Position<T>, T::MaxPositions>,
			margin: T::Balance,
		) -> Result<bool, DispatchError> {
			let mut min_equity = T::Decimal::zero();
			let mut equity: T::Decimal = margin.into_decimal()?;
			for position in positions.iter() {
				if let Some(direction) = position.direction() {
					// Should always succeed
					let market = Self::try_get_market(&position.market_id)?;
					let value = Self::base_asset_value(&market, position, direction)?;
					let abs_value = value.saturating_abs();

					min_equity.try_add_mut(&abs_value.try_mul(&market.margin_ratio_initial)?)?;

					// Add PnL
					equity.try_add_mut(&value.try_sub(&position.quote_asset_notional_amount)?)?;
					// Add unrealized funding
					equity.try_add_mut(&<Self as Instruments>::unrealized_funding(
						&market, position,
					)?)?;
				}
			}

			Ok(equity >= min_equity)
		}
	}

	// Helper functions - low-level functionality
	impl<T: Config> Pallet<T> {
		fn try_get_market(market_id: &T::MarketId) -> Result<Market<T>, DispatchError> {
			Markets::<T>::get(market_id).ok_or_else(|| Error::<T>::MarketIdNotFound.into())
		}

		fn to_vamm_direction(direction: Direction) -> VammDirection {
			match direction {
				Long => VammDirection::Add,
				Short => VammDirection::Remove,
			}
		}

		fn swap_base(
			market: &Market<T>,
			direction: Direction,
			base_amount: T::Balance,
			quote_limit: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			Ok(T::Vamm::swap(&SwapConfigOf::<T> {
				vamm_id: market.vamm_id,
				asset: AssetType::Base,
				input_amount: base_amount,
				direction: Self::to_vamm_direction(direction),
				output_amount_limit: quote_limit,
			})?
			.output)
		}

		fn swap_quote(
			market: &Market<T>,
			direction: Direction,
			quote_abs_decimal: &T::Decimal,
			base_limit: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			Ok(T::Vamm::swap(&SwapConfigOf::<T> {
				vamm_id: market.vamm_id,
				asset: AssetType::Quote,
				input_amount: quote_abs_decimal.into_balance()?,
				direction: Self::to_vamm_direction(direction),
				output_amount_limit: base_limit,
			})?
			.output)
		}

		pub fn get_collateral_account() -> T::AccountId {
			T::PalletId::get().into_sub_account("Collateral")
		}

		pub fn get_insurance_account() -> T::AccountId {
			T::PalletId::get().into_sub_account("Insurance")
		}

		fn decimal_from_swapped(
			swapped: T::Balance,
			direction: Direction,
		) -> Result<T::Decimal, DispatchError> {
			let abs: T::Decimal = swapped.into_decimal()?;
			Ok(match direction {
				Long => abs,
				Short => abs.neg(),
			})
		}

		fn get_or_create_position<'a>(
			positions: &'a mut BoundedVec<Position<T>, T::MaxPositions>,
			market_id: &T::MarketId,
			market: &Market<T>,
			direction: Direction,
		) -> Result<(&'a mut Position<T>, usize), DispatchError> {
			Ok(match positions.iter().position(|p| p.market_id == *market_id) {
				Some(index) =>
					(positions.get_mut(index).expect("Item successfully found above"), index),
				None => {
					positions
						.try_push(Position::<T> {
							market_id: market_id.clone(),
							base_asset_amount: Zero::zero(),
							quote_asset_notional_amount: Zero::zero(),
							last_cum_funding: market.cum_funding_rate(direction),
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

		fn abs_position_notional_and_pnl(
			market: &Market<T>,
			position: &Position<T>,
			position_direction: Direction,
		) -> Result<(T::Decimal, T::Decimal), DispatchError> {
			let position_notional = Self::base_asset_value(market, position, position_direction)?;
			let pnl = position_notional.try_sub(&position.quote_asset_notional_amount)?;
			Ok((position_notional.saturating_abs(), pnl))
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
				direction: Self::to_vamm_direction(position_direction),
			})?;

			Self::decimal_from_swapped(sim_swapped, position_direction)
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

		fn updated_balance(
			balance: &T::Balance,
			delta: &T::Decimal,
		) -> Result<T::Balance, DispatchError> {
			let abs_delta = delta.into_balance()?;

			Ok(match delta.is_positive() {
				true => balance.try_add(&abs_delta)?,
				false => balance.saturating_sub(abs_delta),
			})
		}

		fn update_margin_with_pnl(
			margin: &T::Balance,
			pnl: &T::Decimal,
		) -> Result<T::Balance, DispatchError> {
			Self::updated_balance(margin, pnl)
		}
	}
}
