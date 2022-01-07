//! Naive time

/// `std::time::Duration` is not used because it is to precise with 128 bits and microseconds.
pub type DurationSeconds = u64;

/// Unix now seconds 
pub type Timestamp = u64;

pub const ONE_HOUR: DurationSeconds = 60 * 60;

/// current notion of year will take away 1/365 from lenders and give away to borrowers (as does no
/// accounts to length of year)
pub const SECONDS_PER_YEAR_NAIVE: DurationSeconds = 365 * 24 * ONE_HOUR;
