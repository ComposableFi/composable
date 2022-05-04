//! Naive time things

use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::Permill;

/// `std::time::Duration` is not used because it is to precise with 128 bits and microseconds.
pub type DurationSeconds = u64;

/// Unix now seconds on chain
pub type Timestamp = u64;

pub const ONE_HOUR: DurationSeconds = 60 * 60;

/// current notion of year will take away 1/365 from lenders and give away to borrowers (as does no
/// accounts to length of year)
pub const SECONDS_PER_YEAR_NAIVE: DurationSeconds = 365 * 24 * ONE_HOUR;

#[derive(Decode, Encode, MaxEncodedLen, Clone, Debug, PartialEq, TypeInfo)]
pub enum TimeReleaseFunction {
	LinearDecrease(LinearDecrease),
	StairstepExponentialDecrease(StairstepExponentialDecrease),
}

impl Default for TimeReleaseFunction {
	fn default() -> Self {
		Self::LinearDecrease(Default::default())
	}
}

#[derive(Default, Decode, Encode, MaxEncodedLen, Clone, Debug, PartialEq, TypeInfo)]
pub struct LinearDecrease {
	/// Seconds after start when the amount reaches zero
	pub total: DurationSeconds,
}

#[derive(Default, Decode, Encode, MaxEncodedLen, Clone, Debug, PartialEq, TypeInfo)]
pub struct StairstepExponentialDecrease {
	// Length of time between drops
	pub step: DurationSeconds,
	// Per-step multiplicative factor, usually more than 50%, mostly closer to 100%, but not 100%.
	// Drop per unit of `step`.
	pub cut: Permill,
}
