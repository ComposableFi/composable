//! # Virtual Automated Market Maker
//!
//! Common traits for vamm implementation
use codec::Codec;
use frame_support::{pallet_prelude::*, sp_std::fmt::Debug};
use sp_runtime::traits::{
	AtLeast32BitUnsigned, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Zero,
};

pub trait Vamm {
	type Balance: Default
		+ AtLeast32BitUnsigned
		+ CheckedAdd
		+ CheckedDiv
		+ CheckedMul
		+ CheckedSub
		+ Codec
		+ Copy
		+ MaxEncodedLen
		+ Ord
		+ Parameter
		+ Zero;

	type VammId: Default + Clone + Codec + Debug + Parameter + PartialEq;

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
