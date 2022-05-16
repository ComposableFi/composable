use crate::{math::FixedPointMath, Config};
use composable_traits::time::DurationSeconds;
use frame_support::pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo};
use num_traits::Zero;
use sp_runtime::{ArithmeticError, FixedPointNumber};
use Direction::{Long, Short};

pub const BASIS_POINT_DENOMINATOR: u32 = 10_000;

/// Indicates the direction of a position
#[derive(Encode, Decode, TypeInfo, Debug, Clone, Copy, PartialEq)]
pub enum Direction {
	Long,
	Short,
}

impl Direction {
	pub fn opposite(&self) -> Self {
		match self {
			Self::Long => Self::Short,
			Self::Short => Self::Long,
		}
	}
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

impl<T: Config> Position<T> {
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
	/// The Id of the vAMM used for price discovery in the virtual market.
	pub vamm_id: T::VammId,
	/// The Id of the underlying asset (base-quote pair). A price feed from one or more oracles
	/// must be available for this symbol.
	pub asset_id: T::MayBeAssetId,
	/// Minimum margin ratio for opening a new position.
	pub margin_ratio_initial: T::Decimal,
	/// Margin ratio below which liquidations can occur.
	pub margin_ratio_maintenance: T::Decimal,
	/// Minimum amount of quote asset to exchange when opening a position. Also serves to round
	/// a trade if it results in closing an existing position.
	pub minimum_trade_size: T::Decimal,
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
	/// Amount, in quote asset, of fees collected from trades.
	pub fee_pool: T::Balance,
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
	/// Taker fee, in basis points, applied to all market orders.
	pub taker_fee: T::Balance,
}

impl<T: Config> Market<T> {
	pub fn cum_funding_rate(&self, direction: Direction) -> T::Decimal {
		match direction {
			Long => self.cum_funding_rate_long,
			Short => self.cum_funding_rate_short,
		}
	}

	pub fn base_asset_amount(&self, direction: Direction) -> T::Decimal {
		match direction {
			Long => self.base_asset_amount_long,
			Short => self.base_asset_amount_short,
		}
	}

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
}

// For testing purposes
#[cfg(feature = "std")]
impl<T: Config> Default for Market<T> {
	fn default() -> Self {
		Self {
			vamm_id: Zero::zero(),
			asset_id: Default::default(),
			margin_ratio_initial: Default::default(),
			margin_ratio_maintenance: Default::default(),
			minimum_trade_size: Default::default(),
			base_asset_amount_long: Default::default(),
			base_asset_amount_short: Default::default(),
			cum_funding_rate_long: Default::default(),
			cum_funding_rate_short: Default::default(),
			fee_pool: Default::default(),
			funding_rate_ts: Default::default(),
			funding_frequency: Default::default(),
			funding_period: Default::default(),
			taker_fee: Default::default(),
		}
	}
}

/// Specifications for market creation
#[derive(Encode, Decode, PartialEq, Clone, Debug, TypeInfo)]
pub struct MarketConfig<AssetId, Balance, Decimal, VammConfig> {
	/// Asset id of the underlying for the derivatives market.
	pub asset: AssetId,
	/// Configuration for creating and initializing the vAMM for price discovery.
	pub vamm_config: VammConfig,
	/// Minimum margin ratio for opening a new position.
	pub margin_ratio_initial: Decimal,
	/// Margin ratio below which liquidations can occur.
	pub margin_ratio_maintenance: Decimal,
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
}
