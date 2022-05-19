//! # Virtual Automated Market Maker
//!
//! Common traits and data structures for vamm implementation.
use frame_support::pallet_prelude::*;
use sp_arithmetic::traits::Unsigned;
use sp_core::U256;
use sp_runtime::FixedPointNumber;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Exposes functionality for creation and management of virtual automated market makers.
///
/// Provides functionality for:
/// - creating and closing vamms
/// - updating vamm's parameters
pub trait Vamm {
	/// The balance type for an account.
	type Balance: Unsigned;

	/// Signed fixed point number implementation
	type Decimal: FixedPointNumber<Inner = Self::Balance>;

	/// Configuration for creating and initializing a new vAMM instance. May be
	/// used in extrinsic signatures
	type VammConfig;

	/// Configuration for swap assets in a vamm.
	type SwapConfig;

	/// Configuration for simulation of asset swap in a vamm.
	type SwapSimulationConfig;

	/// Configuration for moving prices in a vamm.
	type MovePriceConfig;

	/// The identifier type for each virtual automated market maker.
	type VammId: Unsigned;

	/// Create a new virtual automated market maker.
	///
	/// ## Returns
	/// The identifier of the newly created vamm.
	fn create(config: &Self::VammConfig) -> Result<Self::VammId, DispatchError>;

	/// Performs swap of assets.
	fn swap(config: &Self::SwapConfig) -> Result<SwapOutput<Self::Balance>, DispatchError>;

	/// Performs swap simulation.
	fn swap_simulation(config: &Self::SwapSimulationConfig)
		-> Result<Self::Balance, DispatchError>;

	/// Sets the amount of base and quote asset reserves, modifying the
	/// invariant of the desired vamm.
	fn move_price(config: &Self::MovePriceConfig) -> Result<U256, DispatchError>;

	/// Get the quote asset mark price for the specified vamm.
	fn get_price(
		vamm_id: Self::VammId,
		asset_type: AssetType,
	) -> Result<Self::Decimal, DispatchError>;

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

/// Specify a common encapsulation layer for the swap function.
#[derive(Clone, Debug)]
pub struct SwapConfig<VammId, Balance> {
	pub vamm_id: VammId,
	pub asset: AssetType,
	pub input_amount: Balance,
	pub direction: Direction,
	pub output_amount_limit: Balance,
}

/// Specify a common encapsulation layer for the swap simulation function.
#[derive(Clone, Debug)]
pub struct SwapSimulationConfig<VammId, Balance> {
	pub vamm_id: VammId,
	pub asset: AssetType,
	pub input_amount: Balance,
	pub direction: Direction,
}

/// Distinguish between asset types present in the vamm.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AssetType {
	Base,
	Quote,
}

/// The two possible directions to go when opening/closing a position in the vamm.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Direction {
	Add,
	Remove,
}

/// Specify a common encapsulation layer for the move price function.
#[derive(Copy, Clone, Debug)]
pub struct MovePriceConfig<VammId, Balance> {
	pub vamm_id: VammId,
	pub base_asset_reserves: Balance,
	pub quote_asset_reserves: Balance,
}

/// Specify the return type for [`Vamm::swap`].
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SwapOutput<Balance> {
	pub output: Balance,
	pub negative: bool,
}
