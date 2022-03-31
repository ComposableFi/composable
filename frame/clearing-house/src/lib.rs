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
//! * **vAMM**: Virtual automated market maker allowing price discovery in virtual markets based on
//!   the supply of virtual base/quote assets.
//! * **Position**: Amount of a particular virtual asset owned by a trader. Implies debt (positive
//!   or negative) to the Clearing House.
//! * **Collateral**: 'Real' asset(s) backing the trader's position(s), ensuring he/she can pay back
//!   the Clearing House.
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
//!
//! ### Implemented Functions
//!
//! ## Usage
//!
//! ### Example
//!
//! ## Related Modules
//!
//! - `pallet-virtual-amm`
//! - `pallet-oracle`
//!
//! <!-- Original author: @0xangelo -->
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies
	// ----------------------------------------------------------------------------------------------------

	use std::fmt::Debug;

	use crate::weights::WeightInfo;
	use codec::FullCodec;
	use composable_traits::{
		clearing_house::{ClearingHouse, Instruments},
		defi::DeFiComposableConfig,
		oracle::Oracle,
		time::DurationSeconds,
		vamm::VirtualAMM,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{tokens::fungibles::Transfer, GenesisBuild},
		Blake2_128Concat, PalletId, Twox64Concat,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_runtime::{
		traits::{AccountIdConversion, CheckedAdd, CheckedMul, CheckedSub, One, Zero},
		ArithmeticError, FixedPointNumber,
	};

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
		/// Event type emitted by this pallet. Depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Weight information for this pallet's extrinsics
		type WeightInfo: WeightInfo;
		/// The market ID type for this pallet.
		type MarketId: CheckedAdd
			+ One
			+ Default
			+ FullCodec
			+ MaxEncodedLen
			+ TypeInfo
			+ Clone
			+ PartialEq
			+ Debug;
		/// Signed decimal fixed point number.
		type Decimal: FullCodec + MaxEncodedLen + TypeInfo + FixedPointNumber;
		/// Implementation for querying the current Unix timestamp
		// TODO(0xangelo): uncomment this and set mocks
		// type UnixTime: UnixTime;
		/// Virtual Automated Market Maker pallet implementation
		type VirtualAMM: VirtualAMM<Decimal = Self::Decimal>;
		/// Price feed (in USDT) Oracle pallet implementation
		type Oracle: Oracle<AssetId = Self::MayBeAssetId, Balance = Self::Balance>;
		/// Pallet implementation of asset transfers.
		type Assets: Transfer<
			Self::AccountId,
			Balance = Self::Balance,
			AssetId = Self::MayBeAssetId,
		>;
		/// The id used as the `AccountId` of the clearing house. This should be unique across all
		/// pallets to avoid name collisions with other pallets and clearing houses.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Pallet Types
	// ----------------------------------------------------------------------------------------------------

	/// Stores the user's position in a particular market
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct Position<MarketId, Decimal> {
		/// The Id of the virtual market
		market_id: MarketId,
		/// Virtual base asset amount. Positive implies long position and negative, short.
		base_asset_amount: Decimal,
		/// Virtual quote asset notional amount (margin * leverage * direction) used to open the
		/// position
		quote_asset_notional_amount: Decimal,
		/// Last cumulative funding rate used to update this position. The market's latest
		/// cumulative funding rate minus this gives the funding rate this position must pay. This
		/// rate multiplied by this position's size (base asset amount * amm price) gives the total
		/// funding owed, which is deducted from the trader account's margin. This debt is
		/// accounted for in margin ratio calculations, which may lead to liquidation.
		last_cum_funding: Decimal,
	}

	/// Data relating to a perpetual contracts market
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct Market<AssetId, Decimal, VammId> {
		/// The Id of the vAMM used for price discovery in the virtual market
		vamm_id: VammId,
		/// The Id of the underlying asset (base-quote pair). A price feed from one or more oracles
		/// must be available for this symbol
		asset_id: AssetId,
		/// Minimum margin ratio for opening a new position
		margin_ratio_initial: Decimal,
		/// Margin ratio below which liquidations can occur
		margin_ratio_maintenance: Decimal,
		/// The latest cumulative funding rate of this market. Must be updated periodically.
		cum_funding_rate: Decimal,
		/// The timestamp for the latest funding rate update.
		funding_rate_ts: DurationSeconds,
		/// The time span between each funding rate update.
		funding_frequency: DurationSeconds,
		/// Period of time over what funding (the difference between mark and index prices) gets
		/// paid.
		///
		/// Setting the funding period too long may cause the perpetual to start trading at a
		/// very dislocated price to the index because there’s less of an incentive for basis
		/// arbitrageurs to push the prices back in line since they would have to carry the basis
		/// risk for a longer period of time.
		///
		/// Setting the funding period too short may cause nobody to trade the perpetual because
		/// there’s too punitive of a price to pay in the case the funding rate flips sign.
		funding_period: DurationSeconds,
	}

	type AssetIdOf<T> = <T as DeFiComposableConfig>::MayBeAssetId;
	type MarketIdOf<T> = <T as Config>::MarketId;
	type DecimalOf<T> = <T as Config>::Decimal;
	type VammParamsOf<T> = <<T as Config>::VirtualAMM as VirtualAMM>::VammParams;
	type VammIdOf<T> = <<T as Config>::VirtualAMM as VirtualAMM>::VammId;
	type PositionOf<T> = Position<MarketIdOf<T>, DecimalOf<T>>;
	type MarketOf<T> = Market<AssetIdOf<T>, DecimalOf<T>, VammIdOf<T>>;

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

	/// Maps [AccountId](frame_system::Config::AccountId) and [MarketId](Config::MarketId) to its
	/// respective [Position](Position), if it exists.
	#[pallet::storage]
	#[pallet::getter(fn get_position)]
	pub type Positions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Twox64Concat,
		T::MarketId,
		PositionOf<T>,
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
	pub type Markets<T: Config> = StorageMap<_, Blake2_128Concat, T::MarketId, MarketOf<T>>;

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
		/// - `asset`: The identifier of the asset type being deposited
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
			asset: AssetIdOf<T>,
			amount: T::Balance,
		) -> DispatchResult {
			let acc = ensure_signed(origin)?;
			<Self as ClearingHouse>::add_margin(&acc, asset, amount)?;
			Ok(())
		}

		/// # Overview
		/// Creates a new perpetuals market with the desired parameters.
		///
		/// ## Parameters
		/// - `asset`: Asset id of the underlying for the derivatives market
		/// - `vamm_params`: Parameters for creating and initializing the vAMM for price discovery
		/// - `margin_ratio_initial`: Minimum margin ratio for opening a new position
		/// - `margin_ratio_maintenance`: Margin ratio below which liquidations can occur
		///
		/// ## Assumptions or Requirements
		/// * The underlying must have a stable price feed via another pallet
		///
		/// ## Emits
		/// * [`MarketCreated`](Event::<T>::MarketCreated)
		///
		/// ## State Changes
		/// Adds an entry to the [`Markets`] storage map.
		///
		/// ## Errors
		/// - [`NoPriceFeedForAsset`](Error::<T>::NoPriceFeedForAsset)
		///
		/// # Weight/Runtime
		/// `O(1)`
		#[pallet::weight(<T as Config>::WeightInfo::create_market())]
		pub fn create_market(
			origin: OriginFor<T>,
			asset: AssetIdOf<T>,
			vamm_params: VammParamsOf<T>,
			margin_ratio_initial: T::Decimal,
			margin_ratio_maintenance: T::Decimal,
			funding_frequency: DurationSeconds,
			funding_period: DurationSeconds,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let market = <Self as ClearingHouse>::create_market(
				asset,
				vamm_params,
				margin_ratio_initial,
				margin_ratio_maintenance,
				funding_frequency,
				funding_period,
			)?;
			Self::deposit_event(Event::MarketCreated { market, asset });
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
		type Decimal = T::Decimal;
		type DurationSeconds = DurationSeconds;
		type MarketId = T::MarketId;
		type VammParams = VammParamsOf<T>;

		fn add_margin(
			account: &Self::AccountId,
			asset: Self::AssetId,
			amount: Self::Balance,
		) -> Result<(), DispatchError> {
			ensure!(
				CollateralTypes::<T>::contains_key(asset),
				Error::<T>::UnsupportedCollateralType
			);

			// Assuming stablecoin collateral and all markets quoted in dollars
			T::Assets::transfer(asset, account, &T::PalletId::get().into_account(), amount, true)?;

			let old_margin = Self::get_margin(&account).unwrap_or_else(T::Balance::zero);
			let new_margin = old_margin.checked_add(&amount).ok_or(ArithmeticError::Overflow)?;
			AccountsMargin::<T>::insert(&account, new_margin);

			Self::deposit_event(Event::MarginAdded { account: account.clone(), asset, amount });
			Ok(())
		}

		fn create_market(
			asset: Self::AssetId,
			vamm_params: Self::VammParams,
			margin_ratio_initial: Self::Decimal,
			margin_ratio_maintenance: Self::Decimal,
			funding_frequency: Self::DurationSeconds,
			funding_period: Self::DurationSeconds,
		) -> Result<Self::MarketId, DispatchError> {
			MarketCount::<T>::try_mutate(|id| {
				ensure!(T::Oracle::is_supported(asset)?, Error::<T>::NoPriceFeedForAsset);
				// TODO(0xangelo): ensure funding_period is multiple of funding_frequency

				let market_id = id.clone();
				let market = Market {
					asset_id: asset,
					vamm_id: T::VirtualAMM::create(vamm_params)?,
					margin_ratio_initial,
					margin_ratio_maintenance,
					funding_frequency,
					funding_period,
					cum_funding_rate: Default::default(),
					// TODO(0xangelo): set ts to UnixTime::now()
					funding_rate_ts: Default::default(),
				};
				Markets::<T>::insert(&market_id, market);

				// Change the market count at the end
				*id = id.checked_add(&One::one()).ok_or(ArithmeticError::Overflow)?;
				Ok(market_id)
			})
		}
	}

	impl<T: Config> Instruments for Pallet<T> {
		type Market = MarketOf<T>;
		type Decimal = T::Decimal;

		fn funding_rate(market: &Self::Market) -> Result<Self::Decimal, DispatchError> {
			// Oracle returns prices in USDT cents
			let unnormalized_oracle_twap = T::Oracle::get_twap(market.asset_id, vec![])?;
			let oracle_twap = Self::Decimal::checked_from_rational(unnormalized_oracle_twap, 10u32)
				.ok_or(ArithmeticError::Overflow)?;

			let vamm_twap = T::VirtualAMM::get_twap(&market.vamm_id)?;

			let price_spread =
				vamm_twap.checked_sub(&oracle_twap).ok_or(ArithmeticError::Underflow)?;
			let period_adjustment = Self::Decimal::checked_from_rational(
				market.funding_frequency,
				market.funding_period,
			)
			.ok_or(ArithmeticError::Underflow)?;
			let rate =
				price_spread.checked_mul(&period_adjustment).ok_or(ArithmeticError::Underflow)?;
			Ok(rate)
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Helper Functions
	// ----------------------------------------------------------------------------------------------------

	// Helper functions - core functionality
	impl<T: Config> Pallet<T> {}

	// Helper functions - validity checks
	impl<T: Config> Pallet<T> {}

	// Helper functions - low-level functionality
	impl<T: Config> Pallet<T> {}
}
