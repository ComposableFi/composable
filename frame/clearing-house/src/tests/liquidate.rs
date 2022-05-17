use composable_traits::clearing_house::ClearingHouse;
use frame_support::{assert_noop, assert_ok};

use super::{traders_in_one_market_context, valid_market_config};
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
