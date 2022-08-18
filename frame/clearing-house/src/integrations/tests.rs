#![allow(clippy::disallowed_methods)]
use crate::{
	integrations::mock::{
		AssetId, Balance, BlockNumber, Decimal, ExtBuilder, Moment, Oracle, Origin, StalePrice,
		System, TestPallet, Timestamp, ALICE, BOB, DOT, USDC,
	},
	MarketConfig as MarketConfigGeneric,
};
use composable_support::validation::Validated;
use composable_traits::time::ONE_HOUR;
use frame_support::{assert_ok, pallet_prelude::Hooks};
use sp_runtime::Percent;

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

fn advance_blocks_by(n: BlockNumber) {
	for _ in 0..n {
		let curr_block = System::block_number();
		if curr_block > 0 {
			Timestamp::on_finalize(curr_block);
			System::on_finalize(curr_block);
			Oracle::on_finalize(curr_block);
		}
		let next_block = curr_block + 1;
		System::set_block_number(next_block);
		// Time is set in milliseconds, so at each block we increment the timestamp by 1000ms = 1s
		let _ = Timestamp::set(Origin::none(), next_block * 1000);
		Timestamp::on_initialize(next_block);
		System::on_initialize(next_block);
		Oracle::on_initialize(next_block);
	}
}

fn set_oracle_for(asset_id: AssetId, price: Balance) {
	// Must be strictly greater than StalePrice to pass validation below
	let block_interval = StalePrice::get() + 1;
	assert_ok!(Oracle::add_asset_and_info(
		Origin::signed(ALICE),
		asset_id,
		Validated::new(Percent::from_percent(80)).unwrap(), // threshold
		Validated::new(1).unwrap(),                         // min_answers
		Validated::new(3).unwrap(),                         // max_answers
		Validated::new(block_interval).unwrap(),            // block_interval
		5,                                                  // reward
		5                                                   // slash
	));

	// Must be strictly greater than block_interval for price to be considered 'requested'
	advance_blocks_by(block_interval + 1);

	assert_ok!(Oracle::set_signer(Origin::signed(ALICE), BOB));
	assert_ok!(Oracle::set_signer(Origin::signed(BOB), ALICE));

	assert_ok!(Oracle::add_stake(Origin::signed(ALICE), 50));
	assert_ok!(Oracle::add_stake(Origin::signed(BOB), 50));

	assert_ok!(Oracle::submit_price(Origin::signed(BOB), price, asset_id));

	// Advance block so that Oracle finalizes?
	advance_blocks_by(1);
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
			vamm_config: VammConfig {
				base_asset_reserves: 1_000_000_000_000_000_000,
				quote_asset_reserves: 100_000_000_000_000_000_000,
				peg_multiplier: 1,
				twap_period: ONE_HOUR,
			},
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
	ExtBuilder {
		native_balances: vec![(ALICE, 1_000_000_000_000_000), (BOB, 1_000_000_000_000_000)],
		..Default::default()
	}
	.build()
	.execute_with(|| {
		set_oracle_for(DOT, 1_000); // 10 in cents
		let config = MarketConfig::default();
		assert_ok!(TestPallet::create_market(Origin::signed(ALICE), config));
	})
}
