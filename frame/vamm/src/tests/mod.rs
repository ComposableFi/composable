// Allow use of .unwrap() in tests and unused Results from function calls
#![allow(clippy::disallowed_methods, unused_must_use, dead_code)]

use crate::{
	mock::{Balance, MockRuntime},
	pallet::{self, VammState},
	tests::helpers::as_decimal,
};
use composable_traits::vamm::VammConfig;
use sp_runtime::FixedPointNumber;

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

#[derive(Clone, Copy)]
pub struct TestVammConfig<Balance, Moment> {
	base_asset_reserves: Balance,
	quote_asset_reserves: Balance,
	peg_multiplier: Balance,
	twap_period: Moment,
}

impl Default for TestVammConfig<Balance, Timestamp> {
	fn default() -> TestVammConfig<Balance, Timestamp> {
		TestVammConfig {
			base_asset_reserves: as_decimal(2).into_inner(),
			quote_asset_reserves: as_decimal(50).into_inner(),
			peg_multiplier: 1,
			twap_period: 3600,
		}
	}
}

impl From<TestVammConfig<Balance, Timestamp>> for VammConfig<Balance, Timestamp> {
	fn from(v: TestVammConfig<Balance, Timestamp>) -> Self {
		Self {
			base_asset_reserves: v.base_asset_reserves,
			quote_asset_reserves: v.quote_asset_reserves,
			peg_multiplier: v.peg_multiplier,
			twap_period: v.twap_period,
		}
	}
}
