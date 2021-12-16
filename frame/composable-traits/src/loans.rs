//! shared types across lending/liquidation/auctions pallets
use codec::{Decode, Encode,};

use scale_info::TypeInfo;

/// `std::time::Duration` is not used because it is to precise with 128 bits and microseconds.
pub type DurationSeconds = u64;

/// seconds
pub type Timestamp = u64;

pub const ONE_HOUR: DurationSeconds = 60 * 60;

/// allows for price to favor some group within some period of time
#[derive(Debug, Decode, Encode, Default, TypeInfo)]
pub struct PriceStructure<GroupId, Balance> {
	pub initial_price: Balance,
	pub preference: Option<(GroupId, DurationSeconds)>,
}

impl<GroupId, Balance> PriceStructure<GroupId, Balance> {
	pub fn new(initial_price: Balance) -> Self {
		Self { initial_price, preference: None }
	}
}



	// /// bank. vault owned - can transfer, cannot mint
	// type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
	// 	+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
	// 	// used to check balances before any storage updates allowing acting without rollback
	// 	+ Inspect<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;