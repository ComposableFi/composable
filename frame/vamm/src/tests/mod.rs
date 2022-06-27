// Allow use of .unwrap() in tests and unused Results from function calls
#![allow(clippy::disallowed_methods, unused_must_use, dead_code)]

use crate::{
	mock::{Balance, MockRuntime},
	pallet::{self, VammState},
};
use composable_traits::vamm::{AssetType, Direction};

pub mod compute_invariant;
pub mod create_vamm;
pub mod get_price;
pub mod get_twap;
pub mod helpers;
pub mod helpers_propcompose;
pub mod move_price;
pub mod swap_asset;
pub mod update_twap;

// ----------------------------------------------------------------------------------------------------
//                                             Setup
// ----------------------------------------------------------------------------------------------------

pub type Decimal = <MockRuntime as pallet::Config>::Decimal;
pub type Timestamp = <MockRuntime as pallet::Config>::Moment;
pub type VammId = <MockRuntime as pallet::Config>::VammId;

#[derive(Default)]
pub struct TestVammState<Balance, Timestamp> {
	base_asset_reserves: Option<Balance>,
	quote_asset_reserves: Option<Balance>,
	peg_multiplier: Option<Balance>,
	closed: Option<Option<Timestamp>>,
}

#[derive(Default)]
pub struct TestSwapConfig<VammId, Balance> {
	vamm_id: Option<VammId>,
	asset: Option<AssetType>,
	input_amount: Option<Balance>,
	direction: Option<Direction>,
	output_amount_limit: Option<Balance>,
}

pub const ZERO_RESERVE: Balance = Balance::MIN;
pub const MINIMUM_RESERVE: Balance = ZERO_RESERVE + 1;
pub const MAXIMUM_RESERVE: Balance = Balance::MAX;
pub const RUN_CASES: u32 = 1000;
