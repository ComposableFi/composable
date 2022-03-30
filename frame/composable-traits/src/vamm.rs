//! # Virtual Automated Market Maker
//!
//! Common traits for vamm implementation
use frame_support::pallet_prelude::DispatchError;

pub trait Vamm {
	/// The identifier type for each market
	type VammId;

	/// Create a new virtual market.
	///
	/// ## Parameters:
	/// - `base_asset_reserves`: The amount of base asset
	/// - `quote_asset_reserves`: The amount of quote asset
	/// - `peg_multiplier`: The constant multiplier responsible to balance quote and base asset
	///
	/// ## Returns
	/// The new virtual market id, if successful.
	fn create(
		base_asset_reserves: u128,
		quote_asset_reserves: u128,
		peg_multiplier: u128,
	) -> Result<Self::VammId, DispatchError>;
}
