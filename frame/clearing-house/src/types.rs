use crate::Config;
use composable_maths::labs::numbers::{FixedPointMath, TryIntoBalance, TryIntoDecimal};
use composable_traits::{
	oracle::Oracle,
	time::DurationSeconds,
	vamm::{Direction as VammDirection, Vamm},
};
use frame_support::{
	pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo},
	traits::UnixTime,
};
use num_traits::Zero;
use sp_runtime::{traits::One, ArithmeticError, DispatchError, FixedPointNumber};
use Direction::{Long, Short};

pub const BASIS_POINT_DENOMINATOR: u32 = 10_000;

/// Indicates the direction of a position
#[derive(Encode, Decode, TypeInfo, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
	/// For a long position, the position is long the asset.
	Long,
	/// For a short position, the position is short the asset.
	Short,
}

impl Direction {
	/// Gives the opposite direction of the current one.
	pub fn opposite(&self) -> Self {
		match self {
			Self::Long => Self::Short,
			Self::Short => Self::Long,
		}
	}
}

impl From<Direction> for VammDirection {
	fn from(direction: Direction) -> Self {
		match direction {
			Long => Self::Add,
			Short => Self::Remove,
		}
	}
}

/// Stores the user's position in a particular market
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Position<T: Config> {
	/// The Id of the virtual market.
	pub market_id: T::MarketId,
	/// Virtual base asset amount. Positive implies long position and negative, short.
	pub base_asset_amount: T::Decimal,
	/// Virtual quote asset notional amount (margin * leverage * direction) used to open the
	/// position.
	pub quote_asset_notional_amount: T::Decimal,
	/// Last cumulative funding rate used to update this position. The market's latest
	/// cumulative funding rate minus this gives the funding rate this position must pay. This
	/// rate multiplied by this position's size (base asset amount * amm price) gives the total
	/// funding owed, which is deducted from the trader account's margin. This debt is
	/// accounted for in margin ratio calculations, which may lead to liquidation.
	pub last_cum_funding: T::Decimal,
}

impl<T: Config> Position<T> {
	/// Returns the direction of the position, if any.
	pub fn direction(&self) -> Option<Direction> {
		if self.base_asset_amount.is_zero() {
			None
		} else if self.base_asset_amount.is_positive() {
			Some(Long)
		} else {
			Some(Short)
		}
	}
}

/// Data relating to a perpetual contracts market
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Market<T: Config> {
	// ---------------------------------------------------------------------------------------------
	//                                         Static
	// ---------------------------------------------------------------------------------------------
	/// The Id of the vAMM used for price discovery in the virtual market.
	pub vamm_id: T::VammId,
	/// The Id of the underlying asset (base-quote pair). A price feed from one or more oracles
	/// must be available for this symbol.
	pub asset_id: T::MayBeAssetId,
	/// Minimum margin ratio for opening a new position.
	pub margin_ratio_initial: T::Decimal,
	/// Margin ratio below which full liquidations can occur.
	pub margin_ratio_maintenance: T::Decimal,
	/// Margin ratio below which partial liquidations can occur.
	pub margin_ratio_partial: T::Decimal,
	/// Minimum amount of quote asset to exchange when opening a position. Also serves to round
	/// a trade if it results in closing an existing position.
	pub minimum_trade_size: T::Decimal,
	/// The time span between each funding rate update.
	pub funding_frequency: DurationSeconds,
	/// Period of time over which funding (the difference between mark and index prices) gets
	/// paid.
	///
	/// Setting the funding period too long may cause the perpetual to start trading at a
	/// very dislocated price to the index because there's less of an incentive for basis
	/// arbitrageurs to push the prices back in line since they would have to carry the basis
	/// risk for a longer period of time.
	///
	/// Setting the funding period too short may cause nobody to trade the perpetual because
	/// there’s too punitive of a price to pay in the case the funding rate flips sign.
	pub funding_period: DurationSeconds,
	/// Taker fee, in basis points, applied to all market orders.
	pub taker_fee: T::Balance,
	/// The reference time span used for weighting the EMA updates for the Oracle and Vamm TWAPs.
	/// ```text
	///                                                       twap_period
	///                                        |---------------------------------------|
	///                                         from_start          since_last
	///                                        |-----------|---------------------------|
	/// -------------------------------------------------------------------------------|
	///                                        ^           ^                           ^
	///                                      now -         |                          now
	///                                   twap_period      |
	///                                               last_twap_ts
	/// ```
	/// In the example above, the current price is given a weight of `since_last` and the last
	/// TWAP, `from_start`. The new TWAP is then the weighted average of the two.
	pub twap_period: DurationSeconds,
	// ---------------------------------------------------------------------------------------------
	//                                         Dynamic
	// ---------------------------------------------------------------------------------------------
	/// The current total realized losses which haven't been claimed by traders in profit.
	pub available_profits: T::Balance,
	/// Total position, in base asset, of all traders that are long. Must be positive. Used to
	/// compute parameter adjustment costs and funding payments from/to the Clearing House.
	pub base_asset_amount_long: T::Decimal,
	/// Total position, in base asset, of all traders that are short. Must be negative. Used to
	/// compute parameter adjustment costs and funding payments from/to the Clearing House.
	pub base_asset_amount_short: T::Decimal,
	/// The latest cumulative funding rate for long positions in this market. Must be updated
	/// periodically.
	pub cum_funding_rate_long: T::Decimal,
	/// The latest cumulative funding rate for short positions in this market. Must be updated
	/// periodically.
	pub cum_funding_rate_short: T::Decimal,
	/// The timestamp for the latest funding rate update.
	pub funding_rate_ts: DurationSeconds,
	/// Last oracle price used to update the index TWAP. This has likely gone through
	/// preprocessing, i.e., is not the actual oracle price reported at the time.
	pub last_oracle_price: T::Decimal,
	/// The last calculated oracle TWAP.
	pub last_oracle_twap: T::Decimal,
	/// The timestamp for [`last_oracle_twap`](Market::last_oracle_twap) and
	/// [`last_oracle_ts`](Market::last_oracle_ts).
	pub last_oracle_ts: DurationSeconds,
}

impl<T: Config> Market<T> {
	/// Construct new market from `MarketConfig`.
	pub fn new(
		config: MarketConfig<T::MayBeAssetId, T::Balance, T::Decimal, T::VammConfig>,
	) -> Result<Self, DispatchError> {
		// TODO(0xangelo): should we consider querying the oracle's TWAP here so that the initial
		// price is not one that's too volatile?
		let oracle_price = Self::get_oracle_price(config.asset)?;
		Ok(Self {
			vamm_id: T::Vamm::create(&config.vamm_config)?,
			asset_id: config.asset,
			margin_ratio_initial: config.margin_ratio_initial,
			margin_ratio_maintenance: config.margin_ratio_maintenance,
			margin_ratio_partial: config.margin_ratio_partial,
			minimum_trade_size: config.minimum_trade_size,
			funding_frequency: config.funding_frequency,
			funding_period: config.funding_period,
			taker_fee: config.taker_fee,
			twap_period: config.twap_period,
			available_profits: Zero::zero(),
			base_asset_amount_long: Zero::zero(),
			base_asset_amount_short: Zero::zero(),
			cum_funding_rate_long: Zero::zero(),
			cum_funding_rate_short: Zero::zero(),
			funding_rate_ts: T::UnixTime::now().as_secs(),
			last_oracle_price: oracle_price,
			last_oracle_twap: oracle_price,
			last_oracle_ts: T::UnixTime::now().as_secs(),
		})
	}

	/// Returns the current funding rate for positions with the given direction.
	pub fn cum_funding_rate(&self, direction: Direction) -> T::Decimal {
		match direction {
			Long => self.cum_funding_rate_long,
			Short => self.cum_funding_rate_short,
		}
	}

	/// Returns the total base asset amount of positions with the given direction.
	pub fn base_asset_amount(&self, direction: Direction) -> T::Decimal {
		match direction {
			Long => self.base_asset_amount_long,
			Short => self.base_asset_amount_short,
		}
	}

	/// Adds to the total base asset amount of positions with the given direction.
	pub fn add_base_asset_amount(
		&mut self,
		amount: &T::Decimal,
		direction: Direction,
	) -> Result<(), ArithmeticError> {
		match direction {
			Long => self.base_asset_amount_long.try_add_mut(amount)?,
			Short => self.base_asset_amount_short.try_add_mut(amount)?,
		};
		Ok(())
	}

	/// Subtracts from the total base asset amount of positions with the given direction.
	pub fn sub_base_asset_amount(
		&mut self,
		amount: &T::Decimal,
		direction: Direction,
	) -> Result<(), ArithmeticError> {
		match direction {
			Long => self.base_asset_amount_long.try_sub_mut(amount)?,
			Short => self.base_asset_amount_short.try_sub_mut(amount)?,
		};
		Ok(())
	}

	/// Returns the current oracle status, including the index price and its validity.
	///
	/// Reasons for invalidity:
	/// - The index price is nonpositive
	/// - The index price is too volatile, i.e., too far from its last twap, i.e., `index / max(1,
	///   twap) > 5` OR `twap / max(1, index) > 5`
	pub fn get_oracle_status(&self) -> Result<OracleStatus<T>, DispatchError> {
		let price = Self::get_oracle_price(self.asset_id)?;

		let is_positive = price.is_positive();
		let is_too_volatile = price.try_div(&self.last_oracle_twap.max(One::one()))? >
			T::Decimal::saturating_from_integer(5) ||
			self.last_oracle_twap.try_div(&price.max(One::one()))? >
				T::Decimal::saturating_from_integer(5);

		Ok(OracleStatus::<T> { price, is_valid: !is_too_volatile && is_positive })
	}

	/// Returns the current oracle price as a decimal.
	pub fn get_oracle_price(asset_id: T::MayBeAssetId) -> Result<T::Decimal, DispatchError> {
		// Oracle returns prices in USDT cents
		let price_cents =
			T::Oracle::get_price(asset_id, T::Decimal::one().try_into_balance()?)?.price;
		T::Decimal::checked_from_rational(price_cents, 100)
			.ok_or_else(|| ArithmeticError::Overflow.into())
	}
}

/// Contains the index price and its validity.
pub struct OracleStatus<T: Config> {
	/// Whether the index price is valid.
	pub is_valid: bool,
	/// The index price.
	pub price: T::Decimal,
}

/// Specifications for market creation
#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug, TypeInfo)]
pub struct MarketConfig<AssetId, Balance, Decimal, VammConfig> {
	/// Asset id of the underlying for the derivatives market.
	pub asset: AssetId,
	/// Configuration for creating and initializing the vAMM for price discovery.
	pub vamm_config: VammConfig,
	/// Minimum margin ratio for opening a new position.
	pub margin_ratio_initial: Decimal,
	/// Margin ratio below which full liquidations can occur.
	pub margin_ratio_maintenance: Decimal,
	/// Margin ratio below which partial liquidations can occur.
	pub margin_ratio_partial: Decimal,
	/// Minimum amount of quote asset to exchange when opening a position. Also serves to round
	/// a trade if it results in closing an existing position.
	pub minimum_trade_size: Decimal,
	/// Time span between each funding rate update.
	pub funding_frequency: DurationSeconds,
	/// Period of time over which funding (the difference between mark and index prices) gets
	/// paid.
	pub funding_period: DurationSeconds,
	/// Taker fee, in basis points, applied to all market orders.
	pub taker_fee: Balance,
	/// The reference time span used for weighting the EMA updates for the Oracle and Vamm TWAPs.
	pub twap_period: DurationSeconds,
}

// -------------------------------------------------------------------------------------------------
//                                            Trading
// -------------------------------------------------------------------------------------------------

pub struct TraderPositionState<T: Config> {
	pub collateral: T::Balance,
	pub market: Market<T>,
	pub position: Position<T>,
	pub outstanding_profits: T::Balance,
}

pub struct TradeResponse<T: Config> {
	pub collateral: T::Balance,
	pub market: Market<T>,
	pub position: Option<Position<T>>,
	pub outstanding_profits: T::Balance,
	pub base_swapped: T::Balance,
	pub is_risk_increasing: bool,
}

// -------------------------------------------------------------------------------------------------
//                                          Liquidations
// -------------------------------------------------------------------------------------------------

pub struct PositionInfo<T: Config> {
	pub direction: Direction,
	pub margin_requirement_maintenance: T::Decimal,
	pub margin_requirement_partial: T::Decimal,
	pub base_asset_value: T::Decimal,
	pub unrealized_pnl: T::Decimal,
	pub unrealized_funding: T::Decimal,
}

pub struct AccountSummary<T: Config> {
	pub collateral: T::Balance,
	pub margin: T::Decimal,
	pub margin_requirement_maintenance: T::Decimal,
	pub margin_requirement_partial: T::Decimal,
	pub base_asset_value: T::Decimal,
	pub positions_summary: Vec<(Market<T>, Position<T>, PositionInfo<T>)>,
}

impl<T: Config> AccountSummary<T> {
	/// Creates a new account summary with no positions accounted for.
	pub fn new(collateral: T::Balance) -> Result<Self, DispatchError> {
		Ok(Self {
			collateral,
			margin: collateral.try_into_decimal()?,
			margin_requirement_maintenance: Zero::zero(),
			margin_requirement_partial: Zero::zero(),
			base_asset_value: Zero::zero(),
			positions_summary: Default::default(),
		})
	}

	/// Updates the account summary with the given position's info.
	pub fn update(
		&mut self,
		market: Market<T>,
		position: Position<T>,
		info: PositionInfo<T>,
	) -> Result<(), DispatchError> {
		self.margin = self
			.margin
			.try_add(&info.unrealized_funding.try_add(&info.unrealized_pnl)?)?
			.max(Zero::zero());
		self.margin_requirement_maintenance
			.try_add_mut(&info.margin_requirement_maintenance)?;
		self.margin_requirement_partial.try_add_mut(&info.margin_requirement_partial)?;
		self.base_asset_value.try_add_mut(&info.base_asset_value)?;
		self.positions_summary.push((market, position, info));
		Ok(())
	}
}
