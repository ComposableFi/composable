//! # Virtual Automated Market Maker
//!
//! Common traits for vamm implementation
use codec::{Codec, FullCodec};
use frame_support::{pallet_prelude::*, sp_std::fmt::Debug};
use sp_runtime::traits::{
	AtLeast32BitUnsigned, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Zero,
};

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

/// Represents the direction a of a position.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum SwapDirection {
	Add,
	Remove,
}

/// Data relating to the state of a virtual market.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Copy, PartialEq, Debug)]
pub struct VammState<Balance, Timestamp> {
	/// The total amount of base asset present in the virtual market.
	pub base_asset_reserves: Balance,

	/// The total amount of quote asset present in the virtual market.
	pub quote_asset_reserves: Balance,

	/// The magnitude of the quote asset reserve.
	pub peg_multiplier: Balance,

	/// Whether this market is deprecated or not.
	///
	/// This variable function as a signal to allow pallets who uses the Vamm to
	/// set a market as "operating as normal" or "not to be used anymore".  If
	/// the value is `None` it means the market is operating as normal, but if
	/// the value is `Some(timestamp)` it means the market is deprecated and the
	/// deprecation will take (or took) effect at the time `timestamp`.
	pub deprecated: Option<Timestamp>,
}
