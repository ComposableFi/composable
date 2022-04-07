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
	fn create(params: VammParams<Self::Balance>) -> Result<Self::VammId, DispatchError>;
}

/// Specify a common encapsulation layer for the [`create`](Vamm::create) function.
pub struct VammParams<Balance> {
	/// The total amount of base assets to be set in vamm's creation.
	pub base_asset_reserves: Balance,
	/// The total amount of quote assets to be set in vamm's creation.
	pub quote_asset_reserves: Balance,
	/// The magnitude of the quote asset reserve.
	pub peg_multiplier: Balance,
}
