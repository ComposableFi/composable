//! shared types across lending/liquidation/auctions pallets

use sp_runtime::FixedU128;

/// seconds
pub type DurationSeconds = u64;


/// Number like of higher bits, so that amount and balance calculations are done it it with higher
/// precision via fixed point.
/// While this is 128 bit, cannot support u128 because 18 bits are for of mantissa.
/// Can support u128 it lifter to use FixedU256
pub type LiftedFixedBalance = FixedU128;
