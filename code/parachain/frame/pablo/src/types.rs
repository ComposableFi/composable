use codec::{Decode, Encode};
use composable_traits::defi::Rate;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

#[derive(Encode, Decode, TypeInfo, Clone, Default, PartialEq, Eq, RuntimeDebug)]
pub struct TimeWeightedAveragePrice<Timestamp, Balance> {
	pub timestamp: Timestamp,
	pub base_price_cumulative: Balance,
	pub quote_price_cumulative: Balance,
	pub base_twap: Rate,
	pub quote_twap: Rate,
}
#[derive(Encode, Decode, TypeInfo, Clone, Default, PartialEq, Eq, RuntimeDebug)]
pub struct PriceCumulative<Timestamp, Balance> {
	pub timestamp: Timestamp,
	pub base_price_cumulative: Balance,
	pub quote_price_cumulative: Balance,
}
