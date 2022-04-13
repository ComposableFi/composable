//! # Virtual Automated Market Maker
//!
//! Common traits and data structures for vamm implementation.
use codec::{Decode, Encode, FullCodec, MaxEncodedLen};
use frame_support::pallet_prelude::DispatchError;
use scale_info::TypeInfo;
use sp_arithmetic::traits::Unsigned;
use sp_runtime::FixedPointNumber;
use sp_std::fmt::Debug;

/// Exposes functionality for creation and management of virtual automated market makers.
///
/// Provides functionality for:
/// - creating and closing vamms
/// - updating vamm's parameters
pub trait Vamm {
	/// The balance type for an account.
	type Balance;
	/// Signed fixed point number implementation
	type Decimal: FixedPointNumber;

	/// Configuration for creating and initializing a new vAMM instance. May be used in extrinsic
	/// signatures
	type VammConfig: FullCodec + MaxEncodedLen + TypeInfo + Debug + Clone + PartialEq;

	/// The identifier type for each virtual automated market maker.
	type VammId: FullCodec + MaxEncodedLen + TypeInfo + Unsigned;

	/// Create a new virtual automated market maker.
	///
	/// ## Returns
	/// The identifier of the newly created vamm.
	fn create(config: &Self::VammConfig) -> Result<Self::VammId, DispatchError>;

	/// Get the quote asset mark price for the specified vamm.
	fn get_price(vamm_id: Self::VammId) -> Result<Self::Balance, DispatchError>;

	/// Compute the time-weighted average price of a virtual AMM
	#[allow(unused_variables)]
	fn get_twap(vamm_id: &Self::VammId) -> Result<Self::Decimal, DispatchError>;
}

/// Specify a common encapsulation layer for the [`create`](Vamm::create) function.
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug, Clone, PartialEq)]
pub struct VammConfig<Balance> {
	/// The total amount of base assets to be set in vamm's creation.
	pub base_asset_reserves: Balance,
	/// The total amount of quote assets to be set in vamm's creation.
	pub quote_asset_reserves: Balance,
	/// The magnitude of the quote asset reserve.
	pub peg_multiplier: Balance,
}
