use crate::{
	mock::MockRuntime,
	pallet::{self},
	tests::constants::{
		DEFAULT_BASE_ASSET_RESERVES, DEFAULT_INPUT_AMOUNT, DEFAULT_PEG_MULTIPLIER,
		DEFAULT_QUOTE_ASSET_RESERVES, DEFAULT_TWAP_PERIOD,
	},
};
use composable_traits::vamm::{AssetType, Direction, SwapConfig, VammConfig};
use frame_benchmarking::Zero;

pub type Balance = <MockRuntime as pallet::Config>::Balance;
pub type Decimal = <MockRuntime as pallet::Config>::Decimal;
pub type Timestamp = <MockRuntime as pallet::Config>::Moment;
pub type VammId = <MockRuntime as pallet::Config>::VammId;

#[derive(Clone, Copy, Debug)]
pub struct TestSwapConfig<VammId, Balance> {
	pub vamm_id: VammId,
	pub asset: AssetType,
	pub input_amount: Balance,
	pub direction: Direction,
	pub output_amount_limit: Option<Balance>,
}

impl Default for TestSwapConfig<VammId, Balance> {
	fn default() -> TestSwapConfig<VammId, Balance> {
		TestSwapConfig {
			vamm_id: Zero::zero(),
			asset: AssetType::Base,
			input_amount: DEFAULT_INPUT_AMOUNT,
			direction: Direction::Add,
			output_amount_limit: None,
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
			output_amount_limit: v.output_amount_limit,
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct TestVammConfig<Balance, Moment> {
	pub base_asset_reserves: Balance,
	pub quote_asset_reserves: Balance,
	pub peg_multiplier: Balance,
	pub twap_period: Moment,
}

impl Default for TestVammConfig<Balance, Timestamp> {
	fn default() -> TestVammConfig<Balance, Timestamp> {
		TestVammConfig {
			base_asset_reserves: DEFAULT_BASE_ASSET_RESERVES,
			quote_asset_reserves: DEFAULT_QUOTE_ASSET_RESERVES,
			peg_multiplier: DEFAULT_PEG_MULTIPLIER,
			twap_period: DEFAULT_TWAP_PERIOD,
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
