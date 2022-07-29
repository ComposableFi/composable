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

pub const BASE_ASSET_RESERVES: Balance = as_decimal_inner(2); // 2  units
pub const QUOTE_ASSET_RESERVES: Balance = as_decimal_inner(50); // 50 units
pub const PEG_MULTIPLIER: Balance = 1;
pub const TWAP_PERIOD: Timestamp = 3600;

//----------------------------------------------------------------------------------------------------
//                                          Test Swap Config
//----------------------------------------------------------------------------------------------------

/// The amount of base or quote asset that will be swaped.
pub const INPUT_AMOUNT: Balance = as_decimal_inner(1); // 1 unit

//----------------------------------------------------------------------------------------------------
//                                                Swap
//----------------------------------------------------------------------------------------------------

// Adding base, receiving quote in return.
pub const QUOTE_RETURNED_AFTER_ADDING_BASE: Balance = 16666666666666666667; // 16.6 units
pub const BASE_ASSET_RESERVES_AFTER_ADDING_BASE: Balance = BASE_ASSET_RESERVES + INPUT_AMOUNT;
pub const QUOTE_ASSET_RESERVES_AFTER_ADDING_BASE: Balance =
	QUOTE_ASSET_RESERVES - QUOTE_RETURNED_AFTER_ADDING_BASE;

// Removing base, requiring quote in return.
pub const QUOTE_REQUIRED_FOR_REMOVING_BASE: Balance = 50000000000000000000; // 50 units
pub const BASE_ASSET_RESERVES_AFTER_REMOVING_BASE: Balance = BASE_ASSET_RESERVES - INPUT_AMOUNT;
pub const QUOTE_ASSET_RESERVES_AFTER_REMOVING_BASE: Balance =
	QUOTE_ASSET_RESERVES + QUOTE_REQUIRED_FOR_REMOVING_BASE;

// Adding quote, receiving base in return.
pub const BASE_RETURNED_AFTER_ADDING_QUOTE: Balance = 39215686274509804; // 0.039 units
pub const BASE_ASSET_RESERVES_AFTER_ADDING_QUOTE: Balance =
	BASE_ASSET_RESERVES - BASE_RETURNED_AFTER_ADDING_QUOTE;
pub const QUOTE_ASSET_RESERVES_AFTER_ADDING_QUOTE: Balance = QUOTE_ASSET_RESERVES + INPUT_AMOUNT;

// Removing quote, requiring base in return.
pub const BASE_REQUIRED_FOR_REMOVING_QUOTE: Balance = 40816326530612244; // 0.040 units
pub const BASE_ASSET_RESERVES_AFTER_REMOVING_QUOTE: Balance =
	BASE_ASSET_RESERVES + BASE_REQUIRED_FOR_REMOVING_QUOTE;
pub const QUOTE_ASSET_RESERVES_AFTER_REMOVING_QUOTE: Balance = QUOTE_ASSET_RESERVES - INPUT_AMOUNT;
