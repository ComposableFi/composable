//! Naive time things

use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::Permill;

/// `std::time::Duration` is not used because it is to precise with 128 bits and microseconds.
pub type DurationSeconds = u64;

/// Unix now seconds on chain
pub type Timestamp = u64;

pub const ONE_MINUTE: DurationSeconds = 60;

pub const ONE_HOUR: DurationSeconds = 60 * ONE_MINUTE;
pub const ONE_DAY: DurationSeconds = 24 * ONE_HOUR;
pub const ONE_WEEK: DurationSeconds = 7 * ONE_DAY;
pub const ONE_MONTH: DurationSeconds = 4 * ONE_WEEK;

/// current notion of year will take away 1/365 from lenders and give away to borrowers (as does no
/// accounts to length of year)
pub const SECONDS_PER_YEAR_NAIVE: DurationSeconds = 365 * 24 * ONE_HOUR;
pub const MS_PER_YEAR_NAIVE: DurationSeconds = SECONDS_PER_YEAR_NAIVE * 1000;

#[derive(Decode, Encode, MaxEncodedLen, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub enum TimeReleaseFunction {
	LinearDecrease(LinearDecrease),
	StairstepExponentialDecrease(StairstepExponentialDecrease),
}

impl Default for TimeReleaseFunction {
	fn default() -> Self {
		Self::LinearDecrease(Default::default())
	}
}

#[derive(Default, Decode, Encode, MaxEncodedLen, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub struct LinearDecrease {
	/// Seconds after start when the amount reaches zero
	pub total: DurationSeconds,
}

#[derive(Default, Decode, Encode, MaxEncodedLen, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub struct StairstepExponentialDecrease {
	// Length of time between drops
	pub step: DurationSeconds,
	// Per-step multiplicative factor, usually more than 50%, mostly closer to 100%, but not 100%.
	// Drop per unit of `step`.
	pub cut: Permill,
}
