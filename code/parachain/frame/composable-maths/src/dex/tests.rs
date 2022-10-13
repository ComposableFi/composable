use crate::dex::*;
use proptest::prelude::*;
use rust_decimal::prelude;
use sp_runtime::Permill;

/// Tests related to constant product math functions
mod constant_product {
	use super::*;
	/// Tests related to the function `compute_first_deposit_lp`
	mod compute_first_deposit_lp {}

	mod compute_existing_deposit_lp {}
}
