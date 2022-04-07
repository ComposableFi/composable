//! # Virtual Automated Market Maker
//!
//! Common traits for vamm implementation
use frame_support::pallet_prelude::DispatchError;

pub trait Vamm {
	type Balance;

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
		base_asset_reserves: Self::Balance,
		quote_asset_reserves: Self::Balance,
		peg_multiplier: Self::Balance,
	) -> Result<Self::VammId, DispatchError>;
}
