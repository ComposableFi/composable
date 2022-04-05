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
	/// - `asset`: Asset id of the underlying for the derivatives market
	/// - `vamm_params`: Parameters for creating and initializing the vAMM for price discovery
	/// - `margin_ratio_initial`: Minimum margin ratio for opening a new position
	/// - `margin_ratio_maintenance`: Margin ratio below which liquidations can occur
	/// - `funding_frequency`: Time span between each funding rate update
	/// - `funding_period`: Period of time over which funding (the difference between mark and index
	///   prices) gets paid.
	///
	/// ## Returns
	/// The new market id, if successful
	fn create_market(config: &Self::MarketConfig) -> Result<Self::MarketId, DispatchError>;
}

/// Exposes functionality for querying funding-related quantities of synthetic instruments
///
/// Provides functions for:
/// * querying the current funding rate for a market
/// * computing the funding payments owed by a position
/// * updating the cumulative funding rate of a market
pub trait Instruments {
	/// Data relating to a derivatives market
	type Market;
	/// Signed decimal number implementation
	type Decimal;

	/// Computes the funding rate for a derivatives market
	///
	/// The funding rate is a function of the open interest and the index to mark price divergence.
	///
	/// ## Parameters
	/// * `market`: the derivatives [Market](Self::Market) data
	///
	/// ## Returns
	/// The current funding rate as a signed decimal number
	fn funding_rate(market: &Self::Market) -> Result<Self::Decimal, DispatchError>;
}
