use composable_traits::clearing_house::ClearingHouse;
use frame_support::{
	assert_noop, assert_ok,
	traits::fungibles::{Inspect, Unbalanced},
};

use crate::{
	mock::{
		accounts::{ALICE, BOB},
		assets::USDC,
		runtime::{
			Assets as AssetsPallet, Balance, ExtBuilder, Origin, Runtime, System as SystemPallet,
			TestPallet, Vamm as VammPallet,
		},
	},
	tests::{
		any_balance, as_balance, get_collateral, get_outstanding_profits, run_for_seconds,
		set_fee_pool_depth, set_oracle_twap, traders_in_one_market_context, with_market_context,
		with_trading_context, MarketConfig,
	},
	Direction::{Long, Short},
	Error, Event,
};
use proptest::prelude::*;

// -------------------------------------------------------------------------------------------------
//                                          Unit Tests
// -------------------------------------------------------------------------------------------------

#[test]
fn should_fail_if_withdrawal_amount_is_zero() {
	// Dummy market
	let config = MarketConfig::default();

	with_trading_context(config, as_balance(100), |_| {
		assert_noop!(
			TestPallet::withdraw_collateral(Origin::signed(ALICE), 0),
			Error::<Runtime>::ZeroWithdrawalAmount
		);
	});
}

#[test]
fn cannot_withdraw_outstanding_profits() {
	let config = MarketConfig::default();
	let collateral = as_balance(100);

	with_trading_context(config, collateral, |market_id| {
		VammPallet::set_price(Some(10.into()));

		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Long,
			collateral,
			collateral / 10,
		));

		VammPallet::set_price(Some(20.into()));
		assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));
		// No one realized losses, so profits are outstanding.
		assert_eq!(get_outstanding_profits(ALICE), collateral);

		assert_noop!(
			TestPallet::withdraw_collateral(Origin::signed(ALICE), collateral * 2),
			Error::<Runtime>::InsufficientCollateral
		);
	});
}

#[test]
fn can_withdraw_realized_profits() {
	let config = MarketConfig::default();
	let collateral = as_balance(100);
	let margins = vec![(ALICE, collateral), (BOB, collateral / 2)];

	traders_in_one_market_context(config, margins, |market_id| {
		// Make sure no funding is incurred
		set_oracle_twap(&market_id, 10.into());
		VammPallet::set_twap(Some(10.into()));

		VammPallet::set_price(Some(10.into()));

		let base = as_balance(10);
		assert_ok!(TestPallet::open_position(
			Origin::signed(ALICE),
			market_id,
			Long,
			collateral,
			base,
		));

		let base = as_balance(10);
		assert_ok!(TestPallet::open_position(
			Origin::signed(BOB),
			market_id,
			Short,
			collateral / 2,
			base,
		));

		// Price moves so that Alice is up 100% and Bob is down 100%
		VammPallet::set_price(Some(20.into()));
		assert_ok!(TestPallet::close_position(Origin::signed(BOB), market_id));
		assert_ok!(TestPallet::close_position(Origin::signed(ALICE), market_id));

		// Since Bob's size was lower than Alice's, even his whole collateral cannot cover Alice's
		// gains
		assert_eq!(get_collateral(BOB), 0);
		assert_eq!(get_collateral(ALICE), collateral + collateral / 2);
		assert_eq!(get_outstanding_profits(ALICE), collateral / 2);

		// Alice withdraws her realized profits
		assert_ok!(TestPallet::withdraw_collateral(
			Origin::signed(ALICE),
			collateral + collateral / 2
		));
		assert_eq!(AssetsPallet::balance(USDC, &ALICE), collateral + collateral / 2);

		SystemPallet::assert_last_event(
			Event::CollateralWithdrawn { user: ALICE, amount: collateral + collateral / 2 }.into(),
		);
	});
}

// TODO(0xangelo): the Insurance Fund should cover losses incurred by traders with realized bad debt
// when a trader in profit withdraws.

#[test]
fn can_withdraw_unrealized_funding_payments_by_settling_them() {
	let config = MarketConfig::default();
	let collateral = as_balance(500);

	with_trading_context(config.clone(), collateral, |market_id| {
		// Alice opens a position
		VammPallet::set_price(Some(100.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_id,
			Long,
			collateral,
			collateral / 100, // 5
		));

		// Time passes and index price moves in favor of Alice's position
		run_for_seconds(config.funding_frequency);
		// Time passes and funding rates are updated
		VammPallet::set_twap(Some(100.into()));
		set_oracle_twap(&market_id, (1006, 10).into() /* 100.6 */);
		// HACK: set Fee Pool depth so as not to worry about capped funding rates
		set_fee_pool_depth(&market_id, as_balance(1_000_000));
		// HACK: add balance to collateral vault so that it can be withdrawn
		AssetsPallet::set_balance(
			USDC,
			&TestPallet::get_collateral_account(),
			as_balance(1_000_000),
		);
		// - unrealized funding = (100.6 - 100) * 5 = 3
		assert_ok!(<TestPallet as ClearingHouse>::update_funding(&market_id));

		// Alice leaves her position open, but withdraws her funding payments
		// withdraw_collateral automatically settles funding payments
		assert_ok!(TestPallet::withdraw_collateral(Origin::signed(ALICE), as_balance(3)));
		assert_eq!(AssetsPallet::balance(USDC, &ALICE), as_balance(3));
	});
}

#[test]
fn cannot_withdraw_funds_reserved_for_market_fee_pool() {
	let config = MarketConfig { taker_fee: 100 /* 1% */, ..Default::default() };
	let collateral = as_balance(102);

	with_trading_context(config, collateral, |market_id| {
		// Alice opens a position and pays 1 as a fee
		VammPallet::set_price(Some(10.into()));
		assert_ok!(<TestPallet as ClearingHouse>::open_position(
			&ALICE,
			&market_id,
			Long,
			as_balance(100),
			as_balance(10),
		));

		// Alice closes her position and pays 1 as a fee again
		assert_ok!(<TestPallet as ClearingHouse>::close_position(&ALICE, &market_id));

		// Alice should have 100 as collateral and the Fee Pool should have the remaining 2
		assert_noop!(
			<TestPallet as ClearingHouse>::withdraw_collateral(&ALICE, collateral),
			Error::<Runtime>::InsufficientCollateral
		);

		assert_ok!(<TestPallet as ClearingHouse>::withdraw_collateral(&ALICE, as_balance(100)));
		assert_eq!(
			AssetsPallet::balance(USDC, &TestPallet::get_fee_pool_account(market_id)),
			as_balance(2)
		);
	});
}

// -------------------------------------------------------------------------------------------------
//                                         Prop Compose
// -------------------------------------------------------------------------------------------------

prop_compose! {
	fn balance_a_less_than_b()(b in any_balance())(a in 0..b, b in Just(b)) -> (Balance, Balance) {
	   (a, b)
	}
}

// -------------------------------------------------------------------------------------------------
//                                        Property Tests
// -------------------------------------------------------------------------------------------------

proptest! {
	#[test]
	fn should_fail_if_withdrawal_amount_is_less_than_collateral_deposited(
		(a, b) in balance_a_less_than_b()
	) {
		// Dummy market
		let config = MarketConfig::default();

		with_market_context(ExtBuilder::default(), config, |_| {
			TestPallet::deposit_collateral(Origin::signed(ALICE), USDC, a);
			assert_noop!(
				TestPallet::withdraw_collateral(Origin::signed(ALICE), b),
				Error::<Runtime>::InsufficientCollateral
			);
		});
	}

	#[test]
	fn cannot_withdraw_another_users_collateral(balance in any_balance()) {
		// Dummy market
		let config = MarketConfig::default();

		with_market_context(ExtBuilder::default(), config, |_| {
			TestPallet::deposit_collateral(Origin::signed(ALICE), USDC, balance);
			assert_noop!(
				TestPallet::withdraw_collateral(Origin::signed(BOB), balance),
				Error::<Runtime>::InsufficientCollateral
			);
		});
	}

	#[test]
	fn should_fail_if_withdrawal_causes_account_to_go_below_initial_margin_ratio(
		balance in 1..as_balance(50)
	) {
		let config = MarketConfig {
			margin_ratio_initial: (1, 2).into(),      // 2x max leverage
			margin_ratio_maintenance: (1, 10).into(), // 10% MMR
			margin_ratio_partial: (2, 10).into(),     // 20% PMR
			..Default::default()
		};

		let collateral = as_balance(50);
		with_trading_context(config, collateral, |market_id| {
			VammPallet::set_price(Some(100.into()));

			// Alice opens a position
			// margin_required = 50
			assert_ok!(
				<TestPallet as ClearingHouse>::open_position(
					&ALICE,
					&market_id,
					Short,
					as_balance(100),
					as_balance(1),
				),
				as_balance(1)
			);

			// Alice can't withdraw because her account is already at max leverage
			assert_noop!(
				TestPallet::withdraw_collateral(Origin::signed(ALICE), balance),
				Error::<Runtime>::InsufficientCollateral
			);
		});
	}

	#[test]
	fn should_not_panic_when_computing_withdrawal_amounts(
		collateral_balance in any_balance(),
		insurance_balance in any_balance(),
		amount in any_balance(),
	) {
		ExtBuilder::default().build().execute_with(|| {
			let collateral_account = TestPallet::get_collateral_account();
			let insurance_account = TestPallet::get_insurance_account();

			AssetsPallet::set_balance(USDC, &collateral_account, collateral_balance);
			AssetsPallet::set_balance(USDC, &insurance_account, insurance_balance);

			TestPallet::get_withdrawal_amounts(USDC, &collateral_account, &insurance_account, amount);
		});
	}
}
