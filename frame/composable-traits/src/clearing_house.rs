//! # Clearing House
//!
//! Common traits for clearing house implementations
use frame_support::pallet_prelude::DispatchError;
use sp_runtime::FixedPointNumber;

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
	/// Signed fixed point number implementation
	type Decimal: FixedPointNumber;
	/// The identifier type for each market
	type MarketId;
	/// Parameters for creating and initializing a new vAMM instance.
	type VammParams;

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
	///
	/// ## Returns
	/// The new market id, if successful
	fn create_market(
		asset: Self::AssetId,
		vamm_params: Self::VammParams,
		margin_ratio_initial: Self::Decimal,
		margin_ratio_maintenance: Self::Decimal,
	) -> Result<Self::MarketId, DispatchError>;
}
