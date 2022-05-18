use composable_traits::clearing_house::ClearingHouse;
use frame_support::{assert_noop, assert_ok};

use super::{multi_market_and_trader_context, traders_in_one_market_context, valid_market_config};
use crate::{
	mock::{
		accounts::{ALICE, BOB},
		runtime::{Oracle as OraclePallet, Origin, Runtime, TestPallet, Vamm as VammPallet},
	},
	Direction, Error,
};

// -------------------------------------------------------------------------------------------------
//                                           Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
#[ignore = "unimplemented"]
fn cant_fully_liquidate_if_above_maintenance_margin_ratio_by_pnl() {
	let mut config = valid_market_config();
	config.margin_ratio_initial = 1.into(); // 1x max leverage
	config.margin_ratio_maintenance = (65, 1_000).into(); // 6.5% MMR
	config.taker_fee = 0;

	let margins = vec![(ALICE, 100), (BOB, 0)];
	traders_in_one_market_context(config, margins, |market_id| {
		VammPallet::set_price(Some(100.into()));

		// Alice opens a position
		assert_ok!(
			<TestPallet as ClearingHouse>::open_position(
				&ALICE,
				&market_id,
				Direction::Long,
				100,
				1,
			),
			1
		);

		// Bob can't liquidate Alice
		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);

		// Price moves so that Alice's account is at exactly 6.5% margin ratio
		VammPallet::set_price(Some((65, 10).into())); // 100 -> 6.5
											  // Still not liquidatable; just enough collateral
		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);
	});
}

#[test]
#[ignore = "unimplemented"]
fn cant_fully_liquidate_if_above_maintenance_margin_ratio_by_funding() {
	let mut config = valid_market_config();
	config.funding_frequency = 60;
	config.funding_period = 60;
	config.margin_ratio_initial = 1.into(); // 1x max leverage
	config.margin_ratio_maintenance = (7, 100).into(); // 7% MMR
	config.taker_fee = 0;

	let margins = vec![(ALICE, 100), (BOB, 0)];
	traders_in_one_market_context(config, margins, |market_id| {
		VammPallet::set_price(Some(1.into()));

		// Alice opens a position
		assert_ok!(
			<TestPallet as ClearingHouse>::open_position(
				&ALICE,
				&market_id,
				Direction::Short,
				100,
				100,
			),
			100
		);

		// Bob can't liquidate Alice
		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);

		// Time passes and funding rates are updated
		VammPallet::set_twap(Some(1.into()));
		// Index price moves against Alice's position
		OraclePallet::set_twap(Some(193)); // 100 -> 193 cents
		assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

		// Alice should now owe 93% of her position's value in funding, bringing her account's
		// margin ratio to exactly the MMR
		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);
	});
}

#[test]
#[ignore = "unimplemented"]
fn can_liquidate_if_below_maintenance_margin_ratio_by_pnl() {
	let mut config = valid_market_config();
	config.margin_ratio_initial = 1.into(); // 1x max leverage
	config.margin_ratio_maintenance = (65, 1_000).into(); // 6.5% MMR
	config.taker_fee = 0;
	// TODO(0xangelo): set a liquidation fee

	let margins = vec![(ALICE, 100), (BOB, 0)];
	traders_in_one_market_context(config, margins, |market_id| {
		VammPallet::set_price(Some(100.into()));

		// Alice opens a position
		assert_ok!(
			<TestPallet as ClearingHouse>::open_position(
				&ALICE,
				&market_id,
				Direction::Long,
				100,
				1,
			),
			1
		);

		// Price moves so that Alice's account is at 6% margin ratio
		VammPallet::set_price(Some(6.into())); // 100 -> 6
		assert_ok!(TestPallet::liquidate(Origin::signed(BOB), ALICE));

		// Bob get a liquidation fee
		assert!(TestPallet::get_margin(&BOB).unwrap() > 0);

		// TODO(0xangelo): check Insurance Fund balance
	});
}

#[test]
#[ignore = "unimplemented"]
fn position_in_market_with_greatest_margin_requirement_gets_liquidated_first() {
	let mut config = valid_market_config();
	config.margin_ratio_initial = (1, 2).into();
	config.margin_ratio_maintenance = (20, 100).into();
	config.taker_fee = 0;
	let mut configs = vec![config.clone()];
	config.margin_ratio_maintenance = (36, 100).into();
	configs.push(config);

	let margins = vec![(ALICE, 100), (BOB, 0)];
	multi_market_and_trader_context(configs, margins, |market_ids| {
		// Alice opens position in market 0
		let market_0 = TestPallet::get_market(&market_ids[0]).unwrap();
		VammPallet::set_price_of(&market_0.vamm_id, Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_ids[0],
			Direction::Long,
			100,
			1
		));

		// Alice opens position in market 1
		let market_1 = TestPallet::get_market(&market_ids[1]).unwrap();
		VammPallet::set_price_of(&market_1.vamm_id, Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_ids[0],
			Direction::Long,
			100,
			1
		));

		// Market 0:
		// - Margin requirement at price 60 = 12
		// - PnL = -40
		VammPallet::set_price_of(&market_0.vamm_id, Some(60.into()));
		// Market 1:
		// - Margin requirement at price 70 = 25.2
		// - PnL = -30
		VammPallet::set_price_of(&market_1.vamm_id, Some(70.into()));
		// Total:
		// - margin = 100 - 70 = 30
		// - margin required = 37.2
		// Margin required after closing the position in market
		// - 0: 12
		// - 1: 25.2
		// Thus, closing either position would bring the account above the MMR (assuming no fees to
		// neither the Insurance Fund nor the liquidator)

		// Bob liquidates Alice's account
		assert_ok!(TestPallet::liquidate(Origin::signed(BOB), ALICE));

		// We expect Alice's position in market 1 to be fully liquidated, since it has the highest
		// margin requirement, but the one in market 0 to be left open.
		let positions = TestPallet::get_positions(&ALICE);
		assert_eq!(positions.len(), 1);
		assert_eq!(positions[0].market_id, market_ids[0]);
	});
}

#[test]
#[ignore = "unimplemented"]
fn above_water_position_can_protect_underwater_position() {
	let mut config = valid_market_config();
	config.margin_ratio_initial = (1, 2).into();
	config.margin_ratio_maintenance = (20, 100).into();
	config.taker_fee = 0;
	let mut configs = vec![config.clone()];
	config.margin_ratio_maintenance = (20, 100).into();
	configs.push(config);

	let margins = vec![(ALICE, 100), (BOB, 0)];
	multi_market_and_trader_context(configs, margins, |market_ids| {
		// Alice opens position in market 0
		let market_0 = TestPallet::get_market(&market_ids[0]).unwrap();
		VammPallet::set_price_of(&market_0.vamm_id, Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_ids[0],
			Direction::Long,
			100,
			1,
		));

		// Alice opens position in market 1
		let market_1 = TestPallet::get_market(&market_ids[1]).unwrap();
		VammPallet::set_price_of(&market_1.vamm_id, Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_ids[1],
			Direction::Long,
			100,
			1,
		));

		// In this example, both markets are equal in MMR. If we had just opened a 100 USDC long on
		// one market with 50 collateral, the liquidation threshold would be at price 62.5. However,
		// since we have two positions, one's margin surplus can cover for the other's deficit
		// Market 0 at price 65:
		// - margin requirement = 13
		// - Pnl = -35
		VammPallet::set_price_of(&market_0.vamm_id, Some(65.into()));
		// Market 1 at price 60:
		// - margin requirement = 12
		// - Pnl = -40
		VammPallet::set_price_of(&market_1.vamm_id, Some(60.into()));
		// Total:
		// - margin = 100 - 75 = 25
		// - margin required = 25

		// Bob tries to liquidate Alice's account but fails
		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);
	});
}
