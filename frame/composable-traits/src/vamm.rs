//! # Virtual Automated Market Maker
//!
//! Common traits and data structures for vamm implementation.
use codec::{FullCodec, MaxEncodedLen};
use frame_support::pallet_prelude::DispatchError;
use num_integer::Integer;
use scale_info::TypeInfo;
use sp_arithmetic::traits::Unsigned;
use sp_runtime::FixedPointNumber;

/// Exposes functionality for creation and management of virtual automated market makers.
///
/// Provides functionality for:
/// - creating and closing vamms
/// - updating vamm's parameters
pub trait Vamm {
	/// The balance type for an account.
	type Balance;

	/// The __signed__ balance-like type.
	type Integer: Integer;

	/// Signed fixed point number implementation
	type Decimal: FixedPointNumber;

	/// Configuration for creating and initializing a new vAMM instance. May be
	/// used in extrinsic signatures
	type VammConfig;

	/// Configuration for swap assets in a vamm.
	type SwapConfig;

	/// Configuration for simulation of asset swap in a vamm.
	type SwapSimulationConfig;

	/// The identifier type for each virtual automated market maker.
	type VammId: Unsigned;

	/// Create a new virtual automated market maker.
	///
	/// ## Returns
	/// The identifier of the newly created vamm.
	fn create(config: &Self::VammConfig) -> Result<Self::VammId, DispatchError>;

	/// Performs swap of assets.
	fn swap(config: &Self::SwapConfig) -> Result<Self::Integer, DispatchError>;

	/// Performs swap simulation.
	fn swap_simulation(config: &Self::SwapSimulationConfig)
		-> Result<Self::Integer, DispatchError>;

	/// Get the quote asset mark price for the specified vamm.
	fn get_price(
		vamm_id: Self::VammId,
		asset_type: AssetType,
	) -> Result<Self::Balance, DispatchError>;

	/// Compute the time-weighted average price of a virtual AMM.
	#[allow(unused_variables)]
	fn get_twap(vamm_id: &Self::VammId) -> Result<Self::Decimal, DispatchError>;
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

/// Specify a common encapsulation layer for the swap functions.
pub struct SwapConfig<VammId, Balance> {
	pub vamm_id: VammId,
	pub asset: AssetType,
	pub input_amount: Balance,
	pub direction: Direction,
	pub output_amount_limit: Balance,
}

/// Specify a common encapsulation layer for the swap simulation functions.
pub struct SwapSimulationConfig<VammId, Balance> {
	pub vamm_id: VammId,
	pub asset: AssetType,
	pub input_amount: Balance,
	pub direction: Direction,
}

/// Distinguish between asset types present in the vamm.
pub enum AssetType {
	Base,
	Quote,
}

/// The two possible directions to go when opening/closing a position in the vamm.
pub enum Direction {
	Add,
	Remove,
}
