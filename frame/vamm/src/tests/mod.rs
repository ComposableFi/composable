// Allow use of .unwrap() in tests and unused Results from function calls
#![allow(clippy::disallowed_methods, unused_must_use, dead_code)]

use crate::{
	mock::{Balance, MockRuntime},
	pallet::{self, VammState},
	tests::helpers::as_decimal,
};
use composable_traits::vamm::{AssetType, Direction, SwapConfig, VammConfig};
use frame_benchmarking::Zero;
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
//                                          Types & Constants
// ----------------------------------------------------------------------------------------------------

pub type Decimal = <MockRuntime as pallet::Config>::Decimal;
pub type Timestamp = <MockRuntime as pallet::Config>::Moment;
pub type VammId = <MockRuntime as pallet::Config>::VammId;

pub const ZERO_RESERVE: Balance = Balance::MIN;
pub const MINIMUM_RESERVE: Balance = ZERO_RESERVE + 1;
pub const MAXIMUM_RESERVE: Balance = Balance::MAX;
pub const RUN_CASES: u32 = 1000;

#[derive(Clone, Copy)]
pub struct TestSwapConfig<VammId, Balance> {
	vamm_id: VammId,
	asset: AssetType,
	input_amount: Balance,
	direction: Direction,
	output_amount_limit: Balance,
}

impl Default for TestSwapConfig<VammId, Balance> {
	fn default() -> TestSwapConfig<VammId, Balance> {
		TestSwapConfig {
			vamm_id: Zero::zero(),
			asset: AssetType::Base,
			input_amount: as_decimal(1).into_inner(),
			direction: Direction::Add,
			output_amount_limit: Zero::zero(),
		}
	}
}

impl From<TestSwapConfig<VammId, Balance>> for SwapConfig<VammId, Balance> {
	fn from(v: TestSwapConfig<VammId, Balance>) -> Self {
		Self {
			vamm_id: v.vamm_id,
			asset: v.asset,
			input_amount: v.input_amount,
			direction: v.direction,
			output_amount_limit: Some(v.output_amount_limit),
		}
	}
}

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
