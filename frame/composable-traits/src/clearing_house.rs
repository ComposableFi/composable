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
	/// The identifier type for each market
	type MarketId;
	/// Specification for market creation
	type MarketConfig;

	/// Add margin to a user's account
	///
	/// Assumes margin account is unique to each wallet address, i.e., there's only one margin
	/// account per user.
	fn add_margin(
		acc: &Self::AccountId,
		asset: Self::AssetId,
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
