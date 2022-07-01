use composable_traits::clearing_house::ClearingHouse;
use frame_support::{assert_err, assert_noop, assert_ok, traits::tokens::fungibles::Inspect};
use sp_runtime::FixedI128;

use super::{
	as_balance, comp::approx_eq_lower, multi_market_and_trader_context, run_for_seconds,
	run_to_time, set_fee_pool_depth, traders_in_one_market_context, MarketConfig,
};
use crate::{
	mock::{
		accounts::{ALICE, BOB},
		assets::USDC,
		runtime::{
			Assets as AssetsPallet, Balance, Origin, Runtime, System as SystemPallet, TestPallet,
			Vamm as VammPallet,
		},
	},
	tests::set_oracle_twap,
	Direction, Error, Event, FullLiquidationPenalty, FullLiquidationPenaltyLiquidatorShare,
	PartialLiquidationCloseRatio, PartialLiquidationPenalty,
	PartialLiquidationPenaltyLiquidatorShare,
};

// -------------------------------------------------------------------------------------------------
//                                            Helpers
// -------------------------------------------------------------------------------------------------

fn set_full_liquidation_penalty(decimal: FixedI128) {
	FullLiquidationPenalty::<Runtime>::set(decimal);
}

fn set_liquidator_share_full(decimal: FixedI128) {
	FullLiquidationPenaltyLiquidatorShare::<Runtime>::set(decimal);
}

fn set_partial_liquidation_penalty(decimal: FixedI128) {
	PartialLiquidationPenalty::<Runtime>::set(decimal);
}

fn set_partial_liquidation_close(decimal: FixedI128) {
	PartialLiquidationCloseRatio::<Runtime>::set(decimal);
}

fn set_liquidator_share_partial(decimal: FixedI128) {
	PartialLiquidationPenaltyLiquidatorShare::<Runtime>::set(decimal);
}

fn get_insurance_acc_balance() -> Balance {
	AssetsPallet::balance(USDC, &TestPallet::get_insurance_account())
}

// -------------------------------------------------------------------------------------------------
//                                           Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn cant_liquidate_user_with_no_open_positions() {
	let config = MarketConfig::default();
	let margins = vec![(ALICE, 0), (BOB, 0)];

	// Set everything as usual, except that ALICE doesn't open any positions
	traders_in_one_market_context(config, margins, |_| {
		VammPallet::set_price(Some(1.into()));

		// Bob can't liquidate Alice
		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::UserHasNoPositions
		);
	});
}

#[test]
fn cant_liquidate_if_above_partial_margin_ratio_by_pnl() {
	let config = MarketConfig {
		margin_ratio_initial: (1, 2).into(),      // 2x max leverage
		margin_ratio_maintenance: (1, 10).into(), // 10% MMR
		margin_ratio_partial: (2, 10).into(),     // 20% PMR
		..Default::default()
	};

	let margins = vec![(ALICE, as_balance(52)), (BOB, 0)];
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

		// Price moves so that Alice's account is at exactly 20% margin ratio
		// 100 -> 60
		VammPallet::set_price(Some(60.into()));
		// At price 60:
		// - margin required = 12
		// - PnL = -40
		// - margin = 12

		// Still not liquidatable; just enough collateral
		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);
	});
}

#[test]
fn can_partially_liquidate_if_below_partial_margin_ratio_by_funding() {
	let config = MarketConfig {
		funding_frequency: 60,
		funding_period: 60,
		margin_ratio_initial: (1, 2).into(),       // 2x max leverage
		margin_ratio_maintenance: (5, 100).into(), // 5% MMR
		margin_ratio_partial: (7, 100).into(),     // 7% PMR
		..Default::default()
	};

	let margins = vec![(ALICE, as_balance(50)), (BOB, 0)];
	traders_in_one_market_context(config, margins, |market_id| {
		set_partial_liquidation_close((25, 100).into());
		set_partial_liquidation_penalty((25, 1000).into());
		set_liquidator_share_partial((50, 100).into());

		set_oracle_twap(&market_id, 1.into());
		VammPallet::set_twap(Some(1.into()));

		// Alice opens a position
		VammPallet::set_price(Some(1.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_id,
			Direction::Short,
			as_balance(100),
			as_balance(100),
		));

		run_for_seconds(60);
		// Time passes and funding rates are updated
		VammPallet::set_twap(Some(1.into()));
		// Index price moves against Alice's position
		set_oracle_twap(&market_id, (144, 100).into());
		assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));
		// Alice should now owe 0.44 * 100 = 44 in funding, bringing her account's
		// margin ratio to slightly below the PMR
		// - margin requirement = 7
		// - margin = 50 - 44 = 6

		assert_ok!(TestPallet::liquidate(Origin::signed(BOB), ALICE));
	});
}

#[test]
fn cant_liquidate_if_above_partial_margin_ratio_by_funding() {
	let config = MarketConfig {
		funding_frequency: 60,
		funding_period: 60,
		margin_ratio_initial: (1, 2).into(),       // 2x max leverage
		margin_ratio_maintenance: (5, 100).into(), // 5% MMR
		margin_ratio_partial: (7, 100).into(),     // 7% PMR
		..Default::default()
	};

	let margins = vec![(ALICE, as_balance(50)), (BOB, 0)];
	traders_in_one_market_context(config, margins, |market_id| {
		set_oracle_twap(&market_id, 1.into());
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
		set_oracle_twap(&market_id, (143, 100).into());
		assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));
		// Alice should now owe 0.43 * 100 = 43 in funding, bringing her account's
		// margin ratio to exactly the PMR
		// - margin requirement = 7
		// - margin = 50 - 43 = 7

		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);
	});
}

#[test]
fn cant_liquidate_if_above_partial_margin_ratio_by_funding_2() {
	let config = MarketConfig {
		funding_frequency: 60,
		funding_period: 60,
		margin_ratio_initial: (10, 100).into(),     // 10x leverage
		margin_ratio_maintenance: (4, 100).into(),  // 25x leverage
		margin_ratio_partial: (625, 10_000).into(), // 16x leverage
		..Default::default()
	};

	let margins = vec![(ALICE, as_balance(50)), (BOB, 0)];
	traders_in_one_market_context(config.clone(), margins, |market_id| {
		set_partial_liquidation_close((25, 100).into());
		set_partial_liquidation_penalty((25, 1000).into());
		set_liquidator_share_partial((50, 100).into());

		set_oracle_twap(&market_id, 100.into());
		VammPallet::set_twap(Some(100.into()));

		// Alice opens a position
		VammPallet::set_price(Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_id,
			Direction::Long,
			as_balance(500),
			as_balance(5)
		));

		// Price moves so that Alice's account is below the PMR
		VammPallet::set_price(Some(95.into()));
		// 100 -> 95
		// - margin requirement (partial) = 29.688
		// - margin requirement (full) = 19
		// - PnL = 475 - 500 = -25
		// - margin = 50 - 25 = 25

		// Time passes and funding rates are updated
		// Index price moves in favor of Alice's position
		// Alice now has +5 in unrealized funding
		// funding = -(95 - 96) * 5 = 5
		// margin = 50 - 25 + 5 = 30
		run_for_seconds(config.funding_frequency);
		VammPallet::set_twap(Some(95.into()));
		set_oracle_twap(&market_id, 96.into());
		// HACK: pretend the market's Fee Pool has enough funds
		set_fee_pool_depth(&market_id, as_balance(1_000_000));
		assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

		// Bob cannot liquidate Alice's account because she is above the PMR
		assert_noop!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);
	});
}

#[test]
fn can_partially_liquidate_if_below_partial_margin_ratio_by_pnl() {
	let config = MarketConfig {
		margin_ratio_initial: (10, 100).into(),     // 10x leverage
		margin_ratio_maintenance: (4, 100).into(),  // 25x leverage
		margin_ratio_partial: (625, 10_000).into(), // 16x leverage
		..Default::default()
	};

	let margins = vec![(ALICE, as_balance(50)), (BOB, 0)];
	traders_in_one_market_context(config, margins, |market_id| {
		set_partial_liquidation_close((25, 100).into());
		set_partial_liquidation_penalty((25, 1000).into());
		set_liquidator_share_partial((50, 100).into());

		// Alice opens a position
		VammPallet::set_price(Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_id,
			Direction::Long,
			as_balance(500),
			as_balance(5)
		));

		// Price moves so that Alice's account is below the PMR
		VammPallet::set_price(Some(95.into()));
		// 100 -> 95
		// - margin requirement (partial) = 29.688
		// - margin requirement (full) = 19
		// - PnL = 475 - 500 = -25
		// - margin = 50 - 25 = 25

		// Bob liquidates Alice's account
		assert_ok!(TestPallet::liquidate(Origin::signed(BOB), ALICE));
		// Reference:
		// ```python
		// def liquidate(account):
		//     collateral, entry_value, base_amount = account
		//     print("base_value:", base_value := base_amount * PRICE)
		//     print("curr_pnl:", curr_pnl := base_value - entry_value)
		//     print("margin:", margin := collateral + curr_pnl)
		//
		//     print("realized_pnl:", realized_pnl := curr_pnl * CLOSE_RATIO)
		//     print("fees:", fees := margin * FEE_RATIO)
		//     print("liquidator_share:", fees / 2)
		//
		//     new_collateral = collateral + realized_pnl - fees
		//     new_base_amount = base_amount - base_amount * CLOSE_RATIO
		//     new_entry_value = entry_value - entry_value * CLOSE_RATIO
		//
		//     new_base_value = PRICE * new_base_amountnew_pnl = new_base_value - new_entry_value
		//     print("new_margin:", new_margin := new_collateral + new_pnl)
		//     print("MRP:", new_base_value * 0.0625)
		//     print("MRF:", new_base_value * 0.04)
		//     return new_collateral, new_entry_value, new_base_amount
		// ```
		// 25% of Alice's position is closed
		// base_value: 475
		// curr_pnl: -25
		// margin: 25
		// realized_pnl: -6.25
		// fees: 0.625
		// liquidator_share: 0.3125
		// new_margin: 24.375
		// MRP: 22.265625
		// MRF: 14.25
		// new_collateral: 43.125
		// Thus, Alice's account is back above the PMR
		assert_eq!(TestPallet::get_collateral(&ALICE).unwrap(), as_balance((43125, 1000)));
		let fees = (3125, 10000);
		assert_eq!(TestPallet::get_collateral(&BOB).unwrap(), as_balance(fees));
		assert_eq!(get_insurance_acc_balance(), as_balance(fees));
		SystemPallet::assert_last_event(Event::PartialLiquidation { user: ALICE }.into());

		assert_err!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);
	});
}

#[test]
fn partial_liquidation_realizes_funding_payments() {
	let config = MarketConfig {
		margin_ratio_initial: (10, 100).into(),     // 10x leverage
		margin_ratio_maintenance: (4, 100).into(),  // 25x leverage
		margin_ratio_partial: (625, 10_000).into(), // 16x leverage
		funding_frequency: 60,
		funding_period: 60,
		..Default::default()
	};

	let margins = vec![(ALICE, as_balance(50)), (BOB, 0)];
	traders_in_one_market_context(config.clone(), margins, |market_id| {
		// Set last funding update at multiple of funding frequency
		run_to_time(config.funding_frequency);
		assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

		set_partial_liquidation_close((25, 100).into());
		set_partial_liquidation_penalty((25, 1000).into());
		set_liquidator_share_partial((50, 100).into());

		// Alice opens a position
		VammPallet::set_price(Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_id,
			Direction::Long,
			as_balance(500),
			as_balance(5)
		));

		// Time passes and index price moves in favor of Alice's position
		run_for_seconds(config.funding_frequency);
		// Time passes and funding rates are updated
		VammPallet::set_twap(Some(100.into()));
		set_oracle_twap(&market_id, (1006, 10).into() /* 100.6 */);
		// HACK: set Fee Pool depth so as not to worry about capped funding rates
		set_fee_pool_depth(&market_id, as_balance(1_000_000));
		assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

		// Price moves so that Alice's account is below the PMR
		VammPallet::set_price(Some(95.into()));
		// 100 -> 95
		// - margin requirement (partial) = 29.688
		// - margin requirement (full) = 19
		// - unrealized PnL = 475 - 500 = -25
		// - unrealized funding = (100.6 - 100) * 5 = 3
		// - margin = 50 - 25 + 3 = 28

		// Bob liquidates Alice's account
		assert_ok!(TestPallet::liquidate(Origin::signed(BOB), ALICE));
		// 25% of Alice's position is closed
		// base_value: 475
		// curr_pnl: -25
		// margin: 28
		// realized_pnl: -6.25
		// fees: 0.7
		// liquidator_share: 0.35
		// new_margin: 27.3
		// MRP: 22.265625
		// MRF: 14.25
		// new_collateral: 46.05
		// Thus, Alice's account is back above the PMR
		assert_eq!(TestPallet::get_collateral(&ALICE).unwrap(), as_balance((4605, 100)));
		let fee = (35, 100);
		assert_eq!(TestPallet::get_collateral(&BOB).unwrap(), as_balance(fee));
		assert_eq!(get_insurance_acc_balance(), as_balance(fee));
		SystemPallet::assert_last_event(Event::PartialLiquidation { user: ALICE }.into());

		assert_err!(
			TestPallet::liquidate(Origin::signed(BOB), ALICE),
			Error::<Runtime>::SufficientCollateral
		);
	});
}

#[test]
fn can_fully_liquidate_if_below_maintenance_margin_ratio_by_pnl() {
	let config = MarketConfig {
		margin_ratio_initial: (1, 2).into(),       // 2x max leverage
		margin_ratio_maintenance: (6, 100).into(), // 6% MMR
		margin_ratio_partial: (10, 100).into(),    // 10% PMR
		..Default::default()
	};

	let margins = vec![(ALICE, as_balance(100)), (BOB, 0)];
	traders_in_one_market_context(config, margins, |market_id| {
		// 100% of liquidated amount goes to fees
		set_full_liquidation_penalty(1.into());
		// 50% of liquidation fee to liquidator
		set_liquidator_share_full((1, 2).into());

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
		let insurance_fund = get_insurance_acc_balance();
		assert_eq!(bob_collateral + insurance_fund, as_balance(4));

		SystemPallet::assert_last_event(Event::FullLiquidation { user: ALICE }.into());
	});
}

#[test]
fn negative_accounts_imply_no_liquidation_fees() {
	let config = MarketConfig {
		margin_ratio_initial: (1, 2).into(),       // 2x max leverage
		margin_ratio_maintenance: (6, 100).into(), // 6% MMR
		margin_ratio_partial: (20, 100).into(),    // 20% PMR
		..Default::default()
	};

	let margins = vec![(ALICE, as_balance(100)), (BOB, 0)];
	traders_in_one_market_context(config, margins, |market_id| {
		// 100% of liquidated amount goes to fees
		set_full_liquidation_penalty(1.into());
		// 50% of liquidation fee to liquidator
		set_liquidator_share_full((1, 2).into());

		// Alice opens a position
		VammPallet::set_price(Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_id,
			Direction::Long,
			as_balance(200),
			as_balance(2),
		));

		// Price moves so that Alice's account is negative
		VammPallet::set_price(Some(48.into()));
		// 100 -> 48
		// At price 48:
		// - Base asset value = 96
		// - margin requirement = 5.76
		// - PnL = -104
		// - margin = 100 - 104 = -4

		// Bob liquidates Alice's account
		assert_ok!(TestPallet::liquidate(Origin::signed(BOB), ALICE));
		// Alice's entire collateral is seized to pay her PnL
		assert_eq!(TestPallet::get_collateral(&ALICE).unwrap(), 0);
		// Bob doesn't get any fees since there's no collateral left
		let bob_collateral = TestPallet::get_collateral(&BOB).unwrap();
		assert_eq!(bob_collateral, 0);
		// Insurance Fund balance gets nothing for the same reason above
		let insurance_fund = get_insurance_acc_balance();
		assert_eq!(insurance_fund, 0);

		SystemPallet::assert_last_event(Event::FullLiquidation { user: ALICE }.into());
	});
}

#[test]
fn position_in_market_with_greatest_margin_requirement_gets_liquidated_first() {
	let config0 = MarketConfig {
		margin_ratio_initial: (1, 2).into(),
		margin_ratio_maintenance: (20, 100).into(),
		margin_ratio_partial: (40, 100).into(),
		..Default::default()
	};
	let config1 = MarketConfig { margin_ratio_maintenance: (36, 100).into(), ..config0.clone() };
	let configs = vec![config0, config1];

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
		set_full_liquidation_penalty(0.into());
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
	let config0 = MarketConfig {
		margin_ratio_initial: (1, 2).into(),
		margin_ratio_maintenance: (15, 100).into(),
		margin_ratio_partial: (25, 100).into(),
		..Default::default()
	};
	let config1 = MarketConfig { margin_ratio_maintenance: (20, 100).into(), ..config0.clone() };
	let configs = vec![config0, config1];

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
		set_full_liquidation_penalty(1.into());
		// 100% of liquidation fees go to liquidator
		set_liquidator_share_full(1.into());
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
	let config0 = MarketConfig {
		margin_ratio_initial: (1, 2).into(),
		margin_ratio_maintenance: (20, 100).into(),
		margin_ratio_partial: (40, 100).into(),
		..Default::default()
	};
	let config1 = MarketConfig { margin_ratio_maintenance: (30, 100).into(), ..config0.clone() };
	let configs = vec![config0, config1];

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
		set_full_liquidation_penalty(1.into());
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
	let config0 = MarketConfig {
		margin_ratio_initial: (50, 100).into(),
		margin_ratio_maintenance: (10, 100).into(),
		margin_ratio_partial: (20, 100).into(),
		..Default::default()
	};
	let configs = vec![config0; 2];

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
		// - margin requirement (partial) = 13
		// - Pnl = -35
		VammPallet::set_price_of(&market_0.vamm_id, Some(65.into()));
		// Market 1 at price 60:
		// - margin requirement (partial) = 12
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
