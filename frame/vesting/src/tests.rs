//! Unit tests for the vesting module.

#![cfg(test)]

use super::*;
use composable_traits::vesting::VestingSchedule;
use frame_support::{assert_noop, assert_ok, error::BadOrigin};
use mock::{Event, *};
use orml_tokens::BalanceLock;

#[test]
fn vesting_from_chain_spec_works() {
	ExtBuilder::build().execute_with(|| {
		// From the vesting below, only 20 out of 50 are locked at block 0.
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 30));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 31).is_err());

		assert_eq!(
			Vesting::vesting_schedules(&CHARLIE, MockCurrencyId::BTC),
			vec![
				/*
					+------+------+-----+
					|block |vested|total|
					|      |      |     |
					+------+------+-----+
					|5     |5     |5    |
					+------+------+-----+
				*/
				VestingSchedule { start: 2u64, period: 3u64, period_count: 1u32, per_period: 5u64 },
				/*
				  +------+------+-----+
				  |block |vested|total|
				  |      |      |     |
				  +------+------+-----+
				  |8     |5     |5    |
				  +------+------+-----+
				  |11    |5     |10   |
				  +------+------+-----+
				  |14    |5     |15   |
				  +------+------+-----+
				*/
				VestingSchedule {
					start: 2u64 + 3u64,
					period: 3u64,
					period_count: 3u32,
					per_period: 5u64,
				}
			]
		);

		System::set_block_number(13);

		assert_ok!(Vesting::claim(Origin::signed(CHARLIE), MockCurrencyId::BTC));
		// At block 13, we only have 5 out of the 50 that are locked.
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 45));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 46).is_err());

		System::set_block_number(14);

		assert_ok!(Vesting::claim(Origin::signed(CHARLIE), MockCurrencyId::BTC));
		// Everything is unlocked from blcok 14 onwards.
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 50));
	});
}

#[test]
fn vested_transfer_works() {
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(1);

		let schedule =
			VestingSchedule { start: 0u64, period: 10u64, period_count: 1u32, per_period: 100u64 };
		assert_ok!(Vesting::vested_transfer(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			schedule.clone(),
			false
		));
		assert_eq!(Vesting::vesting_schedules(&BOB, MockCurrencyId::BTC), vec![schedule.clone()]);
		System::assert_last_event(Event::Vesting(crate::Event::VestingScheduleAdded {
			from: ALICE,
			to: BOB,
			asset: MockCurrencyId::BTC,
			schedule,
		}));
	});
}

#[test]
fn add_new_vesting_schedule_merges_with_current_locked_balance_and_until() {
	ExtBuilder::build().execute_with(|| {
		let schedule =
			VestingSchedule { start: 0u64, period: 10u64, period_count: 2u32, per_period: 10u64 };
		assert_ok!(Vesting::vested_transfer(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			schedule,
			false
		));

		System::set_block_number(12);

		let another_schedule =
			VestingSchedule { start: 10u64, period: 13u64, period_count: 1u32, per_period: 7u64 };
		assert_ok!(Vesting::vested_transfer(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			another_schedule,
			false
		));

		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 17u64 })
		);
	});
}

#[test]
fn cannot_use_fund_if_not_claimed() {
	ExtBuilder::build().execute_with(|| {
		let schedule =
			VestingSchedule { start: 10u64, period: 10u64, period_count: 1u32, per_period: 50u64 };
		assert_ok!(Vesting::vested_transfer(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			schedule,
			false
		));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &BOB, 49).is_err());
	});
}

#[test]
fn vested_transfer_fails_if_zero_period_or_count() {
	ExtBuilder::build().execute_with(|| {
		let schedule =
			VestingSchedule { start: 1u64, period: 0u64, period_count: 1u32, per_period: 100u64 };
		assert_noop!(
			Vesting::vested_transfer(
				Origin::signed(ALICE),
				BOB,
				MockCurrencyId::BTC,
				schedule,
				false
			),
			Error::<Runtime>::ZeroVestingPeriod
		);

		let schedule =
			VestingSchedule { start: 1u64, period: 1u64, period_count: 0u32, per_period: 100u64 };
		assert_noop!(
			Vesting::vested_transfer(
				Origin::signed(ALICE),
				BOB,
				MockCurrencyId::BTC,
				schedule,
				false
			),
			Error::<Runtime>::ZeroVestingPeriodCount
		);
	});
}

#[test]
fn vested_transfer_fails_if_transfer_err() {
	ExtBuilder::build().execute_with(|| {
		let schedule =
			VestingSchedule { start: 1u64, period: 1u64, period_count: 1u32, per_period: 100u64 };
		assert_noop!(
			Vesting::vested_transfer(
				Origin::signed(BOB),
				ALICE,
				MockCurrencyId::BTC,
				schedule,
				false
			),
			orml_tokens::Error::<Runtime>::BalanceTooLow,
		);
	});
}

#[test]
fn vested_transfer_fails_if_overflow() {
	ExtBuilder::build().execute_with(|| {
		let schedule =
			VestingSchedule { start: 1u64, period: 1u64, period_count: 2u32, per_period: u64::MAX };
		assert_noop!(
			Vesting::vested_transfer(
				Origin::signed(ALICE),
				BOB,
				MockCurrencyId::BTC,
				schedule,
				false
			),
			ArithmeticError::Overflow,
		);

		let another_schedule =
			VestingSchedule { start: u64::MAX, period: 1u64, period_count: 2u32, per_period: 1u64 };
		assert_noop!(
			Vesting::vested_transfer(
				Origin::signed(ALICE),
				BOB,
				MockCurrencyId::BTC,
				another_schedule,
				false
			),
			ArithmeticError::Overflow,
		);
	});
}

#[test]
fn vested_transfer_fails_if_bad_origin() {
	ExtBuilder::build().execute_with(|| {
		let schedule =
			VestingSchedule { start: 0u64, period: 10u64, period_count: 1u32, per_period: 100u64 };
		assert_noop!(
			Vesting::vested_transfer(
				Origin::signed(CHARLIE),
				BOB,
				MockCurrencyId::BTC,
				schedule,
				false
			),
			BadOrigin
		);
	});
}

#[test]
fn claim_works() {
	ExtBuilder::build().execute_with(|| {
		let schedule =
			VestingSchedule { start: 0u64, period: 10u64, period_count: 2u32, per_period: 10u64 };
		assert_ok!(Vesting::vested_transfer(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			schedule,
			false
		));

		System::set_block_number(11);
		// remain locked if not claimed
		assert!(Tokens::transfer(Origin::signed(BOB), ALICE, MockCurrencyId::BTC, 10).is_err());
		// unlocked after claiming
		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC));
		assert!(VestingSchedules::<Runtime>::contains_key(BOB, MockCurrencyId::BTC));
		assert_ok!(Tokens::transfer(Origin::signed(BOB), ALICE, MockCurrencyId::BTC, 10));
		// more are still locked
		assert!(Tokens::transfer(Origin::signed(BOB), ALICE, MockCurrencyId::BTC, 1).is_err());

		System::set_block_number(21);
		// claim more
		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC));
		assert!(!VestingSchedules::<Runtime>::contains_key(BOB, MockCurrencyId::BTC));
		assert_ok!(Tokens::transfer(Origin::signed(BOB), ALICE, MockCurrencyId::BTC, 10));
		// all used up
		assert_eq!(Tokens::free_balance(MockCurrencyId::BTC, &BOB), 0);

		// no locks anymore
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
	});
}

#[test]
fn claim_for_works() {
	ExtBuilder::build().execute_with(|| {
		let schedule =
			VestingSchedule { start: 0u64, period: 10u64, period_count: 2u32, per_period: 10u64 };
		assert_ok!(Vesting::vested_transfer(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			schedule,
			false
		));

		assert_ok!(Vesting::claim_for(Origin::signed(ALICE), BOB, MockCurrencyId::BTC));

		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20u64 })
		);
		assert!(VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));

		System::set_block_number(21);

		assert_ok!(Vesting::claim_for(Origin::signed(ALICE), BOB, MockCurrencyId::BTC));

		// no locks anymore
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
		assert!(!VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));
	});
}

#[test]
fn update_vesting_schedules_works() {
	ExtBuilder::build().execute_with(|| {
		let schedule =
			VestingSchedule { start: 0u64, period: 10u64, period_count: 2u32, per_period: 10u64 };
		assert_ok!(Vesting::vested_transfer(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			schedule,
			false
		));

		let updated_schedule =
			VestingSchedule { start: 0u64, period: 20u64, period_count: 2u32, per_period: 10u64 };
		assert_ok!(Vesting::update_vesting_schedules(
			Origin::root(),
			BOB,
			MockCurrencyId::BTC,
			vec![updated_schedule],
			false
		));

		System::set_block_number(11);
		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC));
		assert!(Tokens::transfer(Origin::signed(BOB), ALICE, MockCurrencyId::BTC, 1).is_err());

		System::set_block_number(21);
		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC));
		assert_ok!(Tokens::transfer(Origin::signed(BOB), ALICE, MockCurrencyId::BTC, 10));

		// empty vesting schedules cleanup the storage and unlock the fund
		assert!(VestingSchedules::<Runtime>::contains_key(BOB, MockCurrencyId::BTC));
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 10u64 })
		);
		assert_ok!(Vesting::update_vesting_schedules(
			Origin::root(),
			BOB,
			MockCurrencyId::BTC,
			vec![],
			false
		));
		assert!(!VestingSchedules::<Runtime>::contains_key(BOB, MockCurrencyId::BTC));
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
	});
}

#[test]
fn update_vesting_schedules_fails_if_unexpected_existing_locks() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(Tokens::transfer(Origin::signed(ALICE), BOB, MockCurrencyId::BTC, 1));
		assert_ok!(Tokens::set_lock(*b"prelocks", MockCurrencyId::BTC, &BOB, 0u64));
	});
}

#[test]
fn vested_transfer_check_for_min() {
	ExtBuilder::build().execute_with(|| {
		let schedule =
			VestingSchedule { start: 1u64, period: 1u64, period_count: 1u32, per_period: 3u64 };
		assert_noop!(
			Vesting::vested_transfer(
				Origin::signed(BOB),
				ALICE,
				MockCurrencyId::BTC,
				schedule,
				false
			),
			Error::<Runtime>::AmountLow
		);
	});
}

#[test]
fn multiple_vesting_schedule_claim_works() {
	ExtBuilder::build().execute_with(|| {
		let schedule =
			VestingSchedule { start: 0u64, period: 10u64, period_count: 2u32, per_period: 10u64 };
		assert_ok!(Vesting::vested_transfer(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			schedule.clone(),
			false
		));

		let schedule2 =
			VestingSchedule { start: 0u64, period: 10u64, period_count: 3u32, per_period: 10u64 };
		assert_ok!(Vesting::vested_transfer(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			schedule2.clone(),
			false
		));

		assert_eq!(
			Vesting::vesting_schedules(&BOB, MockCurrencyId::BTC),
			vec![schedule, schedule2.clone()]
		);

		System::set_block_number(21);

		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC));

		assert_eq!(Vesting::vesting_schedules(&BOB, MockCurrencyId::BTC), vec![schedule2]);

		System::set_block_number(31);

		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC));

		assert!(!VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));

		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
	});
}

#[test]
fn exceeding_maximum_schedules_should_fail() {
	ExtBuilder::build().execute_with(|| {
		let schedule =
			VestingSchedule { start: 0u64, period: 10u64, period_count: 2u32, per_period: 10u64 };
		assert_ok!(Vesting::vested_transfer(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			schedule.clone(),
			false
		));
		assert_ok!(Vesting::vested_transfer(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			schedule.clone(),
			false
		));
		assert_noop!(
			Vesting::vested_transfer(
				Origin::signed(ALICE),
				BOB,
				MockCurrencyId::BTC,
				schedule.clone(),
				false
			),
			Error::<Runtime>::MaxVestingSchedulesExceeded
		);

		let schedules = vec![schedule.clone(), schedule.clone(), schedule];

		assert_noop!(
			Vesting::update_vesting_schedules(
				Origin::root(),
				BOB,
				MockCurrencyId::BTC,
				schedules,
				false
			),
			Error::<Runtime>::MaxVestingSchedulesExceeded
		);
	});
}
