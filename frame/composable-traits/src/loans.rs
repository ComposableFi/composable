//! shared types across lending/liquidation/auctions pallets

/// `std::time::Duration` is not used because it is to precise with 128 bits and microseconds.
pub type DurationSeconds = u64;

/// seconds
pub type Timestamp = u64;

pub const ONE_HOUR: DurationSeconds = 60 * 60;
