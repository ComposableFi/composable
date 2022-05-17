//! # Clearing House
//!
//! Common traits for clearing house implementations
use frame_support::pallet_prelude::DispatchError;

/// Exposes functionality for trading of perpetual contracts
///
/// Provides functionality for:
/// * creating and stopping perpetual futures markets
/// * leveraged trading of perpetual contracts
pub trait ClearingHouse {
	/// The trader's account identifier type
	type AccountId;
	/// The asset identifier type
	type AssetId;
	/// The balance type for an account
	type Balance;
	/// The direction type for a position. Usually to disambiguate long and short positions
	type Direction;
	/// The identifier type for each market
	type MarketId;
	/// Specification for market creation
	type MarketConfig;

	/// Add margin to a user's account
	///
	/// Assumes margin account is unique to each wallet address, i.e., there's only one margin
	/// account per user.
	///
	/// ## Parameters
	/// - `account_id`: the trader's margin account Id
	/// - `asset_id`: the type of asset to deposit as collateral
	/// - `amount`: the amount of collateral
	fn add_margin(
		account_id: &Self::AccountId,
		asset_id: Self::AssetId,
		amount: Self::Balance,
	) -> Result<(), DispatchError>;

	/// Create a new perpetuals market
	///
	/// ## Parameters
	/// - `config`: specification for market creation
	///
	/// ## Returns
	/// The new market's id, if successful
	fn create_market(config: &Self::MarketConfig) -> Result<Self::MarketId, DispatchError>;

	/// Open a position in a market
	///
	/// This may result in the following outcomes:
	/// - Creation of a whole new position in the market, if one didn't already exist
	/// - An increase in the size of an existing position, if the trade's direction matches the
	///   existing position's one
	/// - A decrease in the size of an existing position, if the trade's direction is counter to the
	///   existing position's one and its magnitude is smaller than the existing postion's size
	/// - Closing of the existing position, if the trade's direction is counter to the existion
	///   position's one and its magnitude is approximately the existing position's size
	/// - Reversing of the existing position, if the trade's direction is counter to the existion
	///   position's one and its magnitude is greater than the existing postion's size
	///
	/// ## Parameters
	/// - `account_id`: the trader's margin account Id
	/// - `market_id`: the perpetuals market Id to open a position in
	/// - `direction`: whether to long or short the base asset
	/// - `quote_asset_amount`: the amount of exposure to the base asset in quote asset value
	/// - `base_asset_amount_limit`: the minimum absolute amount of base asset to add to the
	///   position. Prevents slippage
	///
	/// ## Returns
	/// The absolute amount of base asset exchanged
	fn open_position(
		account_id: &Self::AccountId,
		market_id: &Self::MarketId,
		direction: Self::Direction,
		quote_asset_amount: Self::Balance,
		base_asset_amount_limit: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Update the funding rate for a market
	///
	/// This should be called periodically for each market so that subsequent calculations of
	/// unrealized funding for each position are up-to-date
	///
	/// # Parameters
	/// - `market_id`: the perpetuals market Id
	fn update_funding(market_id: &Self::MarketId) -> Result<(), DispatchError>;

	/// Liquidates a user's account if below margin requirements.
	///
	/// Note that both unrealized PnL and funding payments contribute to an account being brought
	/// below the maintenance margin ratio. Liquidation realizes a user's PnL and funding payments.
	///
	/// Liquidation can be either full or partial. In the former case, positions are closed
	/// entirely, while in the latter, they are partially closed until the account is brought back
	/// above the initial margin requirement.
	///
	/// Positions in markets with the highest margin requirements (i.e., the lowest max leverage for
	/// opening a position) are liquidated first.
	///
	/// # Parameters
	/// - `liquidator_id`: the liquidator's account Id
	/// - `user_id`: the user's account Id
	fn liquidate(
		liquidator_id: &Self::AccountId,
		user_id: &Self::AccountId,
	) -> Result<(), DispatchError>;
}

/// Exposes functionality for querying funding-related quantities of synthetic instruments
///
/// Provides functions for:
/// * querying the current funding rate for a market
/// * computing a position's unrealized funding payments
/// * updating the cumulative funding rate of a market
pub trait Instruments {
	/// Data relating to a derivatives market
	type Market;
	/// Data relating to a trader's position in a market
	type Position;
	/// Signed decimal number implementation
	type Decimal;

	/// Computes the funding rate for a derivatives market
	///
	/// The funding rate is a function of the open interest and the index to mark price divergence.
	///
	/// ## Parameters
	/// * `market`: the derivatives market data
	///
	/// ## Returns
	/// The current funding rate as a signed decimal number
	fn funding_rate(market: &Self::Market) -> Result<Self::Decimal, DispatchError>;

	/// Computes a position's unrealized funding payments
	///
	/// The unrealized funding may be positive or negative. In the former case, the position's owner
	/// has a 'debt' to its counterparty (e.g., the derivative writer, the protocol, or automated
	/// market maker). The reverse is true in the latter case.
	///
	/// Note that this is similar to unrealized PnL, in that market conditions may change and a
	/// previously negative unrealized funding can turn positive.
	///
	/// ## Parameters
	/// * `market`: the derivatives market data
	/// * `position`: the position in said market
	///
	/// ## Returns
	/// The position's unrealized funding payments as a signed decimal number
	fn unrealized_funding(
		market: &Self::Market,
		position: &Self::Position,
	) -> Result<Self::Decimal, DispatchError>;
}
