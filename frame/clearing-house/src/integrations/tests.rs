#![allow(clippy::disallowed_methods)]
use crate::{Direction::Long, Market, MarketConfig as MarketConfigGeneric};
use composable_support::validation::Validated;
use composable_traits::time::ONE_HOUR;
use frame_support::{assert_ok, pallet_prelude::Hooks};
use pallet_vamm::VammStateOf;
use proptest::prelude::*;
use sp_runtime::{traits::Zero, FixedPointNumber, Percent};

use super::mock::{
	AccountId, AssetId, Balance, BlockNumber, Decimal, ExtBuilder, MarketId, Moment, Oracle,
	Origin, Runtime, StalePrice, System, TestPallet, Timestamp, UnsignedDecimal, Vamm, VammId,
	ALICE, BOB, DOT, PICA, USDC,
};

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

// -------------------------------------------------------------------------------------------------
//                                  Helper Functions and Traits
// -------------------------------------------------------------------------------------------------

fn advance_blocks_by(n: BlockNumber) {
	for _ in 0..n {
		let curr_block = System::block_number();
		if curr_block > 0 {
			Timestamp::on_finalize(curr_block);
			Oracle::on_finalize(curr_block);
			System::on_finalize(curr_block);
		}
		let next_block = curr_block + 1;
		System::set_block_number(next_block);
		// Time is set in milliseconds, so at each block we increment the timestamp by 1000ms = 1s
		let _ = Timestamp::set(Origin::none(), next_block * 1000);
		System::on_initialize(next_block);
		Timestamp::on_initialize(next_block);
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

	// Advance block so that Oracle block finalization hook is called
	advance_blocks_by(1);
}

fn get_collateral(account_id: &AccountId) -> Balance {
	TestPallet::get_collateral(account_id).unwrap()
}

fn get_outstanding_profits(account_id: &AccountId, market_id: &MarketId) -> Balance {
	TestPallet::outstanding_profits(account_id, market_id).unwrap_or_else(Zero::zero)
}

fn get_market(market_id: &MarketId) -> Market<Runtime> {
	TestPallet::get_market(market_id).unwrap()
}

fn get_vamm(vamm_id: &VammId) -> VammStateOf<Runtime> {
	Vamm::get_vamm(vamm_id).unwrap()
}

impl Default for MarketConfig {
	fn default() -> Self {
		Self {
			asset: DOT,
			vamm_config: VammConfig {
				base_asset_reserves: UNIT * 100,
				quote_asset_reserves: UNIT * 100_000,
				peg_multiplier: 1,
				twap_period: ONE_HOUR,
			},
			margin_ratio_initial: Decimal::from_float(0.1),
			margin_ratio_maintenance: Decimal::from_float(0.02),
			margin_ratio_partial: Decimal::from_float(0.04),
			minimum_trade_size: 0.into(),
			funding_frequency: ONE_HOUR,
			funding_period: ONE_HOUR * 24,
			taker_fee: 0,
			twap_period: ONE_HOUR,
		}
	}
}

// -------------------------------------------------------------------------------------------------
//                                      Types & Constants
// -------------------------------------------------------------------------------------------------

pub type MarketConfig = MarketConfigGeneric<AssetId, Balance, Decimal, VammConfig>;
pub type VammConfig = composable_traits::vamm::VammConfig<Balance, Moment>;

pub const UNIT: Balance = UnsignedDecimal::DIV;

// -------------------------------------------------------------------------------------------------
//                                        Create Market
// -------------------------------------------------------------------------------------------------

proptest! {
	#[test]
	fn should_succeed_in_creating_first_market(
		asset_id in prop_oneof![Just(DOT), Just(PICA)]
	) {
		ExtBuilder {
			native_balances: vec![(ALICE, UNIT), (BOB, UNIT)],
			balances: vec![(ALICE, PICA, UNIT), (BOB, PICA, UNIT)],
			..Default::default()
		}
		.build()
		.execute_with(|| {
			set_oracle_for(asset_id, 1_000); // 10 in cents
			let config = MarketConfig { asset: asset_id, ..Default::default() };
			assert_ok!(TestPallet::create_market(Origin::signed(ALICE), config));

			let market_id = MarketId::zero();
			let market = TestPallet::get_market(&market_id).unwrap();
			assert_eq!(market.asset_id, asset_id);
			assert_eq!(market.last_oracle_price, Decimal::from(10));
			assert_eq!(market.last_oracle_twap, Decimal::from(10));
		})
	}
}

// -------------------------------------------------------------------------------------------------
//                                         Open Position
// -------------------------------------------------------------------------------------------------

#[test]
fn should_succeed_in_opening_first_position() {
	ExtBuilder {
		native_balances: vec![(ALICE, UNIT), (BOB, UNIT)],
		balances: vec![(ALICE, PICA, UNIT), (BOB, PICA, UNIT), (BOB, USDC, UNIT * 100)],
		..Default::default()
	}
	.build()
	.execute_with(|| {
		set_oracle_for(DOT, 1_000);
		let config = MarketConfig {
			asset: DOT,
			vamm_config: VammConfig {
				base_asset_reserves: UNIT * 100,
				quote_asset_reserves: UNIT * 10_000,
				peg_multiplier: 10,
				twap_period: ONE_HOUR,
			},
			margin_ratio_initial: Decimal::from_float(0.1),
			margin_ratio_maintenance: Decimal::from_float(0.02),
			margin_ratio_partial: Decimal::from_float(0.04),
			minimum_trade_size: 0.into(),
			funding_frequency: ONE_HOUR,
			funding_period: ONE_HOUR * 24,
			taker_fee: 0,
			twap_period: ONE_HOUR,
		};
		assert_ok!(TestPallet::create_market(Origin::signed(ALICE), config));

		assert_ok!(TestPallet::deposit_collateral(Origin::signed(BOB), USDC, UNIT * 100));

		let market = get_market(&MarketId::zero());
		let vamm_state = get_vamm(&market.vamm_id);

		assert_ok!(TestPallet::open_position(
			Origin::signed(BOB),
			Zero::zero(),
			Long,
			UNIT * 100,
			0
		));

		assert_ne!(get_market(&MarketId::zero()), market);
		assert_ne!(get_vamm(&market.vamm_id), vamm_state);
	})
}

#[test]
fn should_succeed_with_two_traders_in_a_market() {
	ExtBuilder {
		native_balances: vec![(ALICE, UNIT), (BOB, UNIT)],
		balances: vec![
			(ALICE, PICA, UNIT),
			(BOB, PICA, UNIT),
			(ALICE, USDC, UNIT * 100),
			(BOB, USDC, UNIT * 100),
		],
		..Default::default()
	}
	.build()
	.execute_with(|| {
		let asset_id = PICA;
		set_oracle_for(asset_id, 1_000);

		let config = MarketConfig {
			asset: asset_id,
			vamm_config: VammConfig {
				base_asset_reserves: UNIT * 100,
				quote_asset_reserves: UNIT * 100_000,
				peg_multiplier: 1,
				twap_period: ONE_HOUR,
			},
			..Default::default()
		};
		assert_ok!(TestPallet::create_market(Origin::signed(ALICE), config));
		let market_id = Zero::zero();
		let market = get_market(&market_id);
		let vamm_state_before = get_vamm(&market.vamm_id);

		assert_ok!(TestPallet::deposit_collateral(Origin::signed(ALICE), USDC, UNIT * 100));
		assert_ok!(TestPallet::deposit_collateral(Origin::signed(BOB), USDC, UNIT * 100));

		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Long,
			UNIT * 100,
			0
		));
		assert_ok!(TestPallet::open_position(Origin::signed(BOB), market_id, Long, UNIT * 100, 0));

		assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
		assert_ok!(TestPallet::close_position(Origin::signed(BOB), market_id));

		// Alice closes her position in profit, Bob closes his position in loss
		// However, since Alice closes her position first, there are no realized losses in the
		// market yet, so her profits are outstanding
		let alice_col = get_collateral(&ALICE);
		let alice_outstanding_profits = get_outstanding_profits(&ALICE, &market_id);
		let bob_col = get_collateral(&BOB);
		assert!(alice_col + alice_outstanding_profits > bob_col);
		assert_eq!(alice_col + alice_outstanding_profits + bob_col, UNIT * 200);

		assert_ok!(TestPallet::withdraw_collateral(
			Origin::signed(ALICE),
			alice_col + alice_outstanding_profits
		));

		// vAMM is back to its initial state due to path independence
		let vamm_state_after = get_vamm(&market.vamm_id);
		assert_eq!(vamm_state_before.base_asset_reserves, vamm_state_after.base_asset_reserves);
		assert_eq!(vamm_state_before.quote_asset_reserves, vamm_state_after.quote_asset_reserves);
	})
}
