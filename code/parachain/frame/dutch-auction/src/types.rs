use crate::prelude::*;

use composable_traits::{
	defi::{Sell, Take},
	time::Timestamp,
};

#[derive(Encode, Decode, MaxEncodedLen, Default, TypeInfo, Clone, Debug, PartialEq, Eq)]
pub struct SellOrder<AssetId, Balance, AccountId, Context, Configuration> {
	pub from_to: AccountId,
	pub order: Sell<AssetId, Balance>,
	/// is take from input parameters, example continuity of order lifetime or price decay function
	pub configuration: Configuration,
	/// context captured when sell started, example current timestamp or ED captured
	pub context: Context,
	/// amount of `quote` received up to now
	pub total_amount_received: Balance,
}

/// existential deposit context with date of creation
#[derive(Encode, Decode, MaxEncodedLen, Default, TypeInfo, Clone, Debug, PartialEq, Eq)]
pub struct EDContext<Balance> {
	pub added_at: Timestamp,
	pub deposit: Balance,
}

#[derive(Encode, Decode, MaxEncodedLen, Default, TypeInfo, PartialEq, Eq)]
pub struct TakeOrder<Balance, AccountId> {
	pub from_to: AccountId,
	pub take: Take<Balance>,
}
