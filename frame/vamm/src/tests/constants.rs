use crate::tests::{
	helpers::as_decimal_inner,
	types::{Balance, Timestamp},
};

// ----------------------------------------------------------------------------------------------------
//                                              General
// ----------------------------------------------------------------------------------------------------

pub const ZERO_RESERVE: Balance = Balance::MIN;
pub const MINIMUM_RESERVE: Balance = ZERO_RESERVE + 1;
pub const MAXIMUM_RESERVE: Balance = Balance::MAX;
pub const RUN_CASES: u32 = 1000;

// ----------------------------------------------------------------------------------------------------
//                                          Test Vamm Config
// ----------------------------------------------------------------------------------------------------

pub const DEFAULT_BASE_ASSET_RESERVES: Balance = as_decimal_inner(2); // 2  units
pub const DEFAULT_QUOTE_ASSET_RESERVES: Balance = as_decimal_inner(50); // 50 units
pub const DEFAULT_PEG_MULTIPLIER: Balance = 1;
pub const DEFAULT_TWAP_PERIOD: Timestamp = 3600;

//----------------------------------------------------------------------------------------------------
//                                          Test Swap Config
//----------------------------------------------------------------------------------------------------

/// The amount of base or quote asset that will be swaped.
pub const DEFAULT_INPUT_AMOUNT: Balance = as_decimal_inner(1); // 1 unit

//----------------------------------------------------------------------------------------------------
//                                          Swap Simulation
//----------------------------------------------------------------------------------------------------

/// These constants take into account the default values for TestVammConfig and
/// TestSwapConfig, all values are dependent of the values set for those default
/// structs and also of `DEFAUL_INPUT_AMOUNT`.
///
/// The value we expect to receive in return of a swap adding base with an
/// amount equal to `DEFAULT_INPUT_AMOUNT`.
pub const DEFAULT_QUOTE_RETURNED_AFTER_ADDING_BASE: Balance = 39215686274509804; // 0.0039 units
/// The value we expect to give in return of a swap removing base with an amount
/// equal to `DEFAULT_INPUT_AMOUNT`.
pub const DEFAULT_QUOTE_REQUIRED_FOR_REMOVING_BASE: Balance = 50000000000000000000; // 50 units
/// The value we expect to receive in return of a swap adding quote with an
/// amount equal to `DEFAULT_INPUT_AMOUNT`.
pub const DEFAULT_BASE_RETURNED_AFTER_ADDING_QUOTE: Balance = 39215686274509804; // 0.0039 units
/// The value we expect to give in return of a swap removing quote with an
/// amount equal to `DEFAULT_INPUT_AMOUNT`.
pub const DEFAULT_BASE_REQUIRED_FOR_REMOVING_QUOTE: Balance = 40816326530612244; // 0.0040 units
/// The value expected to return as output when adding one unit to the existing
/// vamm.
pub const DEFAULT_OUTPUT_ADDING_BASE: Balance = 16666666666666666667;
/// The value expected to return as output when removing one unit to the existing
/// vamm.
pub const DEFAULT_OUTPUT_REMOVING_BASE: Balance = 50000000000000000000;
