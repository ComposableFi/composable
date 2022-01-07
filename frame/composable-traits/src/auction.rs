use crate::loans::DurationSeconds;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::Permill;

#[derive(Decode, Encode, Clone, TypeInfo, Debug, PartialEq)]
pub enum AuctionStepFunction {
	/// default - direct pass through to dex without steps, just to satisfy defaults and reasonably
	/// for testing
	LinearDecrease(LinearDecrease),
	StairstepExponentialDecrease(StairstepExponentialDecrease),
}

impl Default for AuctionStepFunction {
	fn default() -> Self {
		Self::LinearDecrease(Default::default())
	}
}

#[derive(Default, Decode, Encode, Clone, TypeInfo, Debug, PartialEq)]
pub struct LinearDecrease {
	/// Seconds after auction start when the price reaches zero
	pub total: DurationSeconds,
}

#[derive(Default, Decode, Encode, Clone, TypeInfo, Debug, PartialEq)]
pub struct StairstepExponentialDecrease {
	// Length of time between price drops
	pub step: DurationSeconds,
	// Per-step multiplicative factor, usually more than 50%, mostly closer to 100%, but not 100%.
	// Drop per unit of `step`.
	pub cut: Permill,
}
