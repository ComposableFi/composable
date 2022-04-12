//! # Virtual Automated Market Maker
//!
//! Common traits and data structures for vamm implementation.
use frame_support::pallet_prelude::DispatchError;

/// Exposes functionality for creation and management of virtual automated market makers.
///
/// Provides functionality for:
/// - creating and closing vamms
/// - updating vamm's parameters
pub trait Vamm {
	/// The balance type for an account.
	type Balance;

	/// The identifier type for each virtual automated market maker.
	type VammId;

	/// Create a new virtual automated market maker.
	///
	/// ## Returns
	/// The identifier of the newly created vamm.
	fn create(config: VammConfig<Self::Balance>) -> Result<Self::VammId, DispatchError>;
	fn get_price(vamm_id: Self::VammId) -> Result<Self::Balance, DispatchError>;
}

/// Specify a common encapsulation layer for the [`create`](Vamm::create) function.
pub struct VammConfig<Balance> {
	/// The total amount of base assets to be set in vamm's creation.
	pub base_asset_reserves: Balance,
	/// The total amount of quote assets to be set in vamm's creation.
	pub quote_asset_reserves: Balance,
	/// The magnitude of the quote asset reserve.
	pub peg_multiplier: Balance,
}
