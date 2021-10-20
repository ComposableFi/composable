//! shared types across lending/liquidation/auctions pallets
use codec::{Codec, Decode, Encode, FullCodec};
use frame_support::{
	pallet_prelude::MaybeSerializeDeserialize,
	traits::fungibles::{Inspect, Mutate, Transfer},
	Parameter,
};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Zero},
	FixedPointOperand,
};

use crate::math::LiftedFixedBalance;

/// `std::time::Duration` is not used because it is to precise with 128 bits and microseconds.
pub type DurationSeconds = u64;

/// seconds
pub type Timestamp = u64;

pub const ONE_HOUR: DurationSeconds = 60 * 60;

/// allows for price to favor some group within some period of time
#[derive(Debug, Decode, Encode, Default)]
pub struct PriceStructure<GroupId, Balance> {
	pub initial_price: Balance,
	pub preference: Option<(GroupId, DurationSeconds)>,
}

impl<GroupId, Balance> PriceStructure<GroupId, Balance> {
	pub fn new(initial_price: Balance) -> Self {
		Self { initial_price, preference: None }
	}
}

pub trait DeFiComposableConfig: frame_system::Config {
	// what.
	type AssetId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Default;

	type Balance: Default
		+ Parameter
		+ Codec
		+ Copy
		+ Ord
		+ CheckedAdd
		+ CheckedSub
		+ CheckedMul
		+ CheckedSub
		+ AtLeast32BitUnsigned
		+ From<u64> // at least 64 bit
		+ Zero
		+ FixedPointOperand
		+ Into<LiftedFixedBalance> // integer part not more than bits in this
		+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit

	/// bank. vault owned - can transfer, cannot mint
	type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
		+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
		// used to check balances before any storage updates allowing acting without rollback
		+ Inspect<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;
}
