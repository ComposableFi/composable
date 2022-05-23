use composable_traits::clearing_house::ClearingHouse;
use frame_support::{assert_noop, assert_ok, traits::tokens::fungibles::Inspect};

use super::{
	as_balance, comp::approx_eq_lower, multi_market_and_trader_context,
	traders_in_one_market_context, valid_market_config,
};
use crate::{
	mock::{
		accounts::{ALICE, BOB},
		assets::USDC,
		runtime::{
			Assets as AssetsPallet, Oracle as OraclePallet, Origin, Runtime, TestPallet,
			Vamm as VammPallet,
		},
	},
	tests::run_for_seconds,
	Direction, Error, FullLiquidationPenalty, FullLiquidationPenaltyLiquidatorShare,
};

// -------------------------------------------------------------------------------------------------
//                                           Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn cant_fully_liquidate_if_above_maintenance_margin_ratio_by_pnl() {
	let mut config = valid_market_config();
	config.margin_ratio_initial = (1, 2).into(); // 2x max leverage
	config.margin_ratio_maintenance = (1, 10).into(); // 10% MMR
	config.taker_fee = 0;

	let margins = vec![(ALICE, as_balance(55)), (BOB, 0)];
	traders_in_one_market_context(config, margins, |market_id| {
		VammPallet::set_price(Some(100.into()));

		// Alice opens a position
		assert_ok!(
			<TestPallet as ClearingHouse>::open_position(
				&ALICE,
				&market_id,
				Direction::Long,
				as_balance(100),
				as_balance(1),
			),
			as_balance(1)
		);

		// Bob can't liquidate Alice
		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);

		// Price moves so that Alice's account is at exactly 10% margin ratio
		VammPallet::set_price(Some(50.into())); // 100 -> 50
										// At price 50:
										// - margin required = 5
										// - PnL = -50
										// - margin = 5

		// Still not liquidatable; just enough collateral
		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);
	});
}

#[test]
fn cant_fully_liquidate_if_above_maintenance_margin_ratio_by_funding() {
	let mut config = valid_market_config();
	config.funding_frequency = 60;
	config.funding_period = 60;
	config.margin_ratio_initial = (1, 2).into(); // 2x max leverage
	config.margin_ratio_maintenance = (7, 100).into(); // 7% MMR
	config.taker_fee = 0;

	let margins = vec![(ALICE, as_balance(50)), (BOB, 0)];
	traders_in_one_market_context(config, margins, |market_id| {
		VammPallet::set_price(Some(1.into()));

		// Alice opens a position
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_id,
			Direction::Short,
			as_balance(100),
			as_balance(100),
		));

		// Bob can't liquidate Alice
		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);

		run_for_seconds(60);
		// Time passes and funding rates are updated
		VammPallet::set_twap(Some(1.into()));
		// Index price moves against Alice's position
		OraclePallet::set_twap(Some(143)); // 100 -> 143 cents
		assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));
		// Alice should now owe 0.43 * 100 = 43 in funding, bringing her account's
		// margin ratio to exactly the MMR
		// - margin requirement = 7
		// - margin = 50 - 43 = 7

		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);
	});
}

#[test]
fn can_liquidate_if_below_maintenance_margin_ratio_by_pnl() {
	let mut config = valid_market_config();
	config.margin_ratio_initial = (1, 2).into(); // 2x max leverage
	config.margin_ratio_maintenance = (6, 100).into(); // 6% MMR
	config.taker_fee = 0;

	let margins = vec![(ALICE, as_balance(100)), (BOB, 0)];
	traders_in_one_market_context(config, margins, |market_id| {
		// 100% of liquidated amount goes to fees
		FullLiquidationPenalty::<Runtime>::set(1.into());
		// 50% of liquidation fee to liquidator
		FullLiquidationPenaltyLiquidatorShare::<Runtime>::set((1, 2).into());

		// Alice opens a position
		VammPallet::set_price(Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_id,
			Direction::Long,
			as_balance(200),
			as_balance(2),
		));

		// Price moves so that Alice's account is below the IMR
		VammPallet::set_price(Some(52.into()));
		// 100 -> 52
		// At price 52:
		// - margin requirement = 6.24
		// - PnL = -96
		// - margin = 100 - 96 = 4

		// Bob liquidates Alice's account
		assert_ok!(TestPallet::liquidate(Origin::signed(BOB), ALICE));
		// Alice's entire collateral is seized as fees in a full liquidation
		assert_eq!(TestPallet::get_collateral(&ALICE).unwrap(), 0);
		// Bob gets half of Alice's collateral as a fee
		let bob_collateral = TestPallet::get_collateral(&BOB).unwrap();
		assert_eq!(bob_collateral, as_balance(2));

		// Insurance Fund balance gets the rest of the liquidation fee
		// bob_collateral + insurance_fund = margin
		let insurance_fund = AssetsPallet::balance(USDC, &TestPallet::get_insurance_account());
		assert_eq!(bob_collateral + insurance_fund, as_balance(4));
	});
}

#[test]
fn position_in_market_with_greatest_margin_requirement_gets_liquidated_first() {
	let mut config = valid_market_config();
	config.margin_ratio_initial = (1, 2).into();
	config.margin_ratio_maintenance = (20, 100).into();
	config.taker_fee = 0;
	let mut configs = vec![config.clone()];
	config.margin_ratio_maintenance = (36, 100).into();
	configs.push(config);

	let margins = vec![(ALICE, as_balance(100)), (BOB, 0)];
	multi_market_and_trader_context(configs, margins, |market_ids| {
		let (market0_id, market1_id) = (market_ids[0], market_ids[1]);

		// Alice opens position in market 0
		let market_0 = TestPallet::get_market(&market0_id).unwrap();
		VammPallet::set_price_of(&market_0.vamm_id, Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market0_id,
			Direction::Long,
			as_balance(100),
			as_balance(1)
		));

		// Alice opens position in market 1
		let market_1 = TestPallet::get_market(&market1_id).unwrap();
		VammPallet::set_price_of(&market_1.vamm_id, Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market1_id,
			Direction::Long,
			as_balance(100),
			as_balance(1)
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
		// - 0: 25.2
		// - 1: 12
		// Thus, closing either position would bring the account above the MMR (assuming no fees to
		// neither the Insurance Fund nor the liquidator)

		// Bob liquidates Alice's account
		// 0% of liquidated amount goes to fees
		FullLiquidationPenalty::<Runtime>::set(0.into());
		assert_ok!(TestPallet::liquidate(Origin::signed(BOB), ALICE));

		// We expect Alice's position in market 1 to be fully liquidated, since it has the highest
		// margin requirement, but the one in market 0 to be left open.
		let positions = TestPallet::get_positions(&ALICE);
		assert_eq!(positions.len(), 1);
		assert_eq!(positions[0].market_id, market0_id);
	});
}

#[test]
fn fees_are_proportional_to_base_asset_value_liquidated() {
	let mut config = valid_market_config();
	config.margin_ratio_initial = (1, 2).into();
	config.margin_ratio_maintenance = (15, 100).into();
	config.taker_fee = 0;
	let mut configs = vec![config.clone()];
	config.margin_ratio_maintenance = (20, 100).into();
	configs.push(config);

	let margins = vec![(ALICE, as_balance(450)), (BOB, 0)];
	multi_market_and_trader_context(configs, margins, |market_ids| {
		let (market0_id, market1_id) = (market_ids[0], market_ids[1]);

		// Alice opens position in market 0
		let market_0 = TestPallet::get_market(&market0_id).unwrap();
		VammPallet::set_price_of(&market_0.vamm_id, Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market0_id,
			Direction::Long,
			as_balance(300),
			as_balance(3)
		));

		// Alice opens position in market 1
		let market_1 = TestPallet::get_market(&market1_id).unwrap();
		VammPallet::set_price_of(&market_1.vamm_id, Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market1_id,
			Direction::Long,
			as_balance(600),
			as_balance(6)
		));

		// Market 0 at price 60:
		// - Base asset value = 180
		// - Margin requirement = 27
		// - PnL = -120
		VammPallet::set_price_of(&market_0.vamm_id, Some(60.into()));
		// Market 1 at price 60:
		// - Base asset value = 360
		// - Margin requirement = 72
		// - PnL = -240
		VammPallet::set_price_of(&market_1.vamm_id, Some(60.into()));
		// Total:
		// - margin = 450 - 360 = 90
		// - margin required = 99

		// 100% of liquidated collateral goes to fees
		FullLiquidationPenalty::<Runtime>::set(1.into());
		// 100% of liquidation fees go to liquidator
		FullLiquidationPenaltyLiquidatorShare::<Runtime>::set(1.into());
		// Liquidation fee for closing
		// - 0: (180 / 540) * margin = 30
		// - 1: (360 / 540) * margin = 60
		// Margin after closing
		// - 0: 90 - 30 = 60
		// - 1: 90 - 60 = 30
		// Margin required after closing
		// - 0: 72
		// - 1: 27
		assert_ok!(TestPallet::liquidate(Origin::signed(BOB), ALICE));
		let positions = TestPallet::get_positions(&ALICE);
		// Position 1 should get liquidated due to its higher margin requirement, leaving position 0
		// with enough margin to keep it alive, even after fees
		assert_eq!(positions.len(), 1);
		assert_eq!(positions[0].market_id, market0_id);
		// Bob should receive an amount of fees proportional to the amount of base asset value
		// closed
		assert!(approx_eq_lower(TestPallet::get_collateral(&BOB).unwrap(), as_balance(60)));
	});
}

#[test]
fn fees_decrease_margin_for_remaining_positions() {
	let mut config = valid_market_config();
	config.margin_ratio_initial = (1, 2).into();
	config.margin_ratio_maintenance = (20, 100).into();
	config.taker_fee = 0;
	let mut configs = vec![config.clone()];
	config.margin_ratio_maintenance = (30, 100).into();
	configs.push(config);

	let margins = vec![(ALICE, as_balance(100)), (BOB, 0)];
	multi_market_and_trader_context(configs, margins, |market_ids| {
		let (market0_id, market1_id) = (market_ids[0], market_ids[1]);

		// Alice opens position in market 0
		let market_0 = TestPallet::get_market(&market0_id).unwrap();
		VammPallet::set_price_of(&market_0.vamm_id, Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market0_id,
			Direction::Long,
			as_balance(50),
			as_balance(1) / 2
		));

		// Alice opens position in market 1
		let market_1 = TestPallet::get_market(&market1_id).unwrap();
		VammPallet::set_price_of(&market_1.vamm_id, Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market1_id,
			Direction::Long,
			as_balance(150),
			as_balance(15) / 10
		));

		// Market 0 at price 60:
		// - Base asset value = 30
		// - Margin requirement = 6
		// - PnL = -20
		VammPallet::set_price_of(&market_0.vamm_id, Some(60.into()));
		// Market 1 at price 60:
		// - Base asset value = 90
		// - Margin requirement = 27
		// - PnL = -60
		VammPallet::set_price_of(&market_1.vamm_id, Some(60.into()));
		// Total:
		// - margin = 100 - 80 = 20
		// - margin required = 33

		// 100% of liquidated collateral goes to fees
		FullLiquidationPenalty::<Runtime>::set(1.into());
		// Liquidation fee for closing
		// - 0: (30 / 120) * margin = 5
		// - 1: (90 / 120) * margin = 15
		// Margin after closing
		// - 0: 20 - 5 = 15
		// - 1: 20 - 15 = 5
		// Margin required after closing
		// - 0: 27
		// - 1: 6
		// Thus, because of the fees, both positions will be liquidated, whereas if there were no
		// fees, only the position in market 1 would need to be liquidated
		assert_ok!(TestPallet::liquidate(Origin::signed(BOB), ALICE));
		let positions = TestPallet::get_positions(&ALICE);
		assert_eq!(positions.len(), 0);
	});
}

#[test]
fn above_water_position_can_protect_underwater_position() {
	let mut config = valid_market_config();
	config.margin_ratio_initial = (1, 2).into();
	config.margin_ratio_maintenance = (20, 100).into();
	config.taker_fee = 0;
	let mut configs = vec![config.clone()];
	config.margin_ratio_maintenance = (20, 100).into();
	configs.push(config);

	let margins = vec![(ALICE, as_balance(100)), (BOB, 0)];
	multi_market_and_trader_context(configs, margins, |market_ids| {
		let (market0_id, market1_id) = (market_ids[0], market_ids[1]);

		// Alice opens position in market 0
		let market_0 = TestPallet::get_market(&market0_id).unwrap();
		VammPallet::set_price_of(&market_0.vamm_id, Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market0_id,
			Direction::Long,
			as_balance(100),
			as_balance(1),
		));

		// Alice opens position in market 1
		let market_1 = TestPallet::get_market(&market1_id).unwrap();
		VammPallet::set_price_of(&market_1.vamm_id, Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market1_id,
			Direction::Long,
			as_balance(100),
			as_balance(1),
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
