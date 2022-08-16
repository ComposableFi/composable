use crate::{
	integrations::mock::{
		AssetId, Balance, Decimal, ExtBuilder, Moment, Origin, System, TestPallet, Timestamp,
		ALICE, DOT, USDC,
	},
	MarketConfig as MarketConfigGeneric,
};
use composable_traits::time::ONE_HOUR;
use frame_support::{assert_ok, pallet_prelude::Hooks};

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			native_balances: vec![],
			balances: vec![],
			collateral_type: Some(USDC),
			max_price_divergence: Decimal::from_inner(i128::MAX),
		}
	}
}

#[test]
fn externalities_builder_works() {
	ExtBuilder::default().build().execute_with(|| {});
}

// ----------------------------------------------------------------------------------------------------
//                                        Helper Functions
// ----------------------------------------------------------------------------------------------------

fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 0 {
			Timestamp::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		// Time is set in milliseconds, so at each block we increment the timestamp by 1000ms = 1s
		let _ = Timestamp::set(Origin::none(), (System::block_number() - 1) * 1000);
		System::on_initialize(System::block_number());
		Timestamp::on_initialize(System::block_number());
	}
}

// ----------------------------------------------------------------------------------------------------
//                                            Types
// ----------------------------------------------------------------------------------------------------

pub type MarketConfig = MarketConfigGeneric<AssetId, Balance, Decimal, VammConfig>;
pub type VammConfig = composable_traits::vamm::VammConfig<Balance, Moment>;

impl Default for MarketConfig {
	fn default() -> Self {
		Self {
			asset: DOT,
			vamm_config: Default::default(),
			// 10x max leverage to open a position
			margin_ratio_initial: Decimal::from_float(0.1),
			// fully liquidate when above 50x leverage
			margin_ratio_maintenance: Decimal::from_float(0.02),
			// partially liquidate when above 25x leverage
			margin_ratio_partial: Decimal::from_float(0.04),
			minimum_trade_size: 0.into(),
			funding_frequency: ONE_HOUR,
			funding_period: ONE_HOUR * 24,
			taker_fee: 0,
			twap_period: ONE_HOUR,
		}
	}
}

// ----------------------------------------------------------------------------------------------------
//                                         Create Market
// ----------------------------------------------------------------------------------------------------

#[test]
fn should_succeed_in_creating_first_market() {
	ExtBuilder::default().build().execute_with(|| {
		run_to_block(1);

		let config = MarketConfig::default();
		assert_ok!(TestPallet::create_market(Origin::signed(ALICE), config));
	})
}
