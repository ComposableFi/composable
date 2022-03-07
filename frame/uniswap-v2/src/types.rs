use codec::{Decode, Encode, MaxEncodedLen};
use composable_traits::defi::Rate;
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, RuntimeDebug)]
pub struct PriceCumulatives<Timestamp, Balance> {
	pub timestamp: Timestamp,
	pub price_cumulative_base: Balance,
	pub price_cumulative_quote: Balance,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Default, PartialEq, RuntimeDebug)]
pub struct TWAP {
	pub average_price_base: Rate,
	pub average_price_quote: Rate,
}
