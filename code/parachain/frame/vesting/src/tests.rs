//! Unit tests for the vesting module.

#![cfg(test)]

use super::*;
use composable_traits::vesting::{
	VestingSchedule, VestingScheduleInfo,
	VestingWindow::{BlockNumberBased, MomentBased},
};
use frame_support::{
	assert_noop, assert_ok,
	error::BadOrigin,
	traits::{fungibles::Mutate, TryCollect},
};
use mock::{RuntimeEvent, *};
use orml_tokens::BalanceLock;

#[test]
fn vesting_from_chain_spec_works() {
	ExtBuilder::build().execute_with(|| {
		// From the vesting below, only 20 out of 50 are locked at block 0.
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 30));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 31).is_err());
		let schedules: BoundedBTreeMap<_, _, MaxVestingSchedule> = [
			(
				1_u128,
				/*
					+------+------+-----+
					|block |vested|total|
					|      |      |     |
					+------+------+-----+
					|5     |5     |5    |
					+------+------+-----+
				*/
				VestingSchedule {
					vesting_schedule_id: 1_u128,
					window: BlockNumberBased { start: 2_u64, period: 3_u64 },
					period_count: 1_u32,
					per_period: 5_u64,
					already_claimed: 0_u64,
				},
			),
			(
				2_u128,
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
					vesting_schedule_id: 2_u128,
					window: BlockNumberBased { start: 2_u64 + 3_u64, period: 3_u64 },
					period_count: 3_u32,
					per_period: 5_u64,
					already_claimed: 0_u64,
				},
			),
			(
				3_u128,
				/*
				  +---------+-----------+-----------+
				  |block    |timestamp  |vested | total |
				  |         |      	    |      	|       |
				  +---------+-----------+-------+-------|
				  |8     	|48000     	|0      |0		|
				  +---------+-----------+-------+-------|
				  |14    	|84000      |0      |0		|
				  +---------+-----------+-------+-------|
				  |18    	|108000     |5      |5
				  +---------+-----------+-------+-------|
				  |25    	|150000     |5      |10		|
				  +---------+-----------+-------+-------|
				  |34    	|204000     |5      |15		|
				  +---------+-----------+-------+-------|
				*/
				VestingSchedule {
					vesting_schedule_id: 3_u128,
					window: MomentBased { start: 40000_u64, period: 50000_u64 },
					period_count: 3_u32,
					per_period: 5_u64,
					already_claimed: 0_u64,
				},
			),
		]
		.into_iter()
		.try_collect()
		.unwrap();

		assert_eq!(Vesting::vesting_schedules(&CHARLIE, MockCurrencyId::BTC), schedules);
		System::set_block_number(1);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(CHARLIE),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 30));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 31).is_err());

		System::set_block_number(11);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(CHARLIE),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 45));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 46).is_err());

		System::set_block_number(14);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(CHARLIE),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));
		// Block number based schedules are unlocked from block 14 onwards.
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 50));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 51).is_err());

		System::set_block_number(25);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(CHARLIE),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 60));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 61).is_err());

		System::set_block_number(34);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		// everything unlocked
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(CHARLIE),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 65));
	});
}

#[test]
fn vested_transfer_self_vest_ko() {
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(1);

		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(
				RuntimeOrigin::root(),
				ALICE,
				ALICE,
				MockCurrencyId::BTC,
				schedule_input,
			),
			Error::<Runtime>::TryingToSelfVest
		);
	});
}

#[test]
fn vested_transfer_works() {
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(1);

		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_input.clone(),
		));
		let schedule = VestingSchedule::from_input(4_u128, schedule_input.clone());
		let schedules: BoundedBTreeMap<_, _, MaxVestingSchedule> =
			[(4_u128, schedule.clone())].into_iter().try_collect().unwrap();
		assert_eq!(Vesting::vesting_schedules(&BOB, MockCurrencyId::BTC), schedules);
		System::assert_last_event(RuntimeEvent::Vesting(crate::Event::VestingScheduleAdded {
			from: ALICE,
			to: BOB,
			asset: MockCurrencyId::BTC,
			schedule,
			vesting_schedule_id: 4_u128,
			schedule_amount: 100,
		}));
	});
}

#[test]
fn vested_transfer_trait_emits_vesting_schedule_added_event() {
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(Tokens::mint_into(MockCurrencyId::ETH, &ALICE, 100));

		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
		};

		let schedule = VestingSchedule::from_input(4_u128, schedule_input.clone());

		assert_ok!(<Vesting as VestedTransfer>::vested_transfer(
			MockCurrencyId::ETH,
			&ALICE,
			&BOB,
			schedule_input,
		));

		System::assert_last_event(RuntimeEvent::Vesting(crate::Event::VestingScheduleAdded {
			from: ALICE,
			to: BOB,
			asset: MockCurrencyId::ETH,
			schedule,
			vesting_schedule_id: 4_u128,
			schedule_amount: 100,
		}));
	});
}

#[test]
fn vested_transfer_for_moment_based_schedule_works() {
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(1);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);

		let schedule_input = VestingScheduleInfo {
			window: MomentBased { start: 0_u64, period: 10_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
		};
		let schedule = VestingSchedule::from_input(4_u128, schedule_input.clone());
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_input.clone(),
		));
		let schedules: BoundedBTreeMap<_, _, MaxVestingSchedule> =
			[(4_u128, schedule.clone())].into_iter().try_collect().unwrap();

		assert_eq!(Vesting::vesting_schedules(&BOB, MockCurrencyId::BTC), schedules);
		System::assert_last_event(RuntimeEvent::Vesting(crate::Event::VestingScheduleAdded {
			from: ALICE,
			to: BOB,
			asset: MockCurrencyId::BTC,
			schedule,
			vesting_schedule_id: 4_u128,
			schedule_amount: 100,
		}));
	});
}

#[test]
fn add_new_vesting_schedule_merges_with_current_locked_balance_and_until() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_input,
		));

		System::set_block_number(12);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);

		let another_schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 10_u64, period: 13_u64 },
			period_count: 1_u32,
			per_period: 7_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			another_schedule_input,
		));

		let moment_based_schedule_input = VestingScheduleInfo {
			window: MomentBased { start: 72000_u64, period: 5000_u64 },
			period_count: 2_u32,
			per_period: 7_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			moment_based_schedule_input,
		));

		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 31_u64 })
		);
	});
}

#[test]
fn cannot_use_fund_if_not_claimed() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 10_u64, period: 10_u64 },
			period_count: 1_u32,
			per_period: 50_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_input,
		));
		let moment_based_schedule_input = VestingScheduleInfo {
			window: MomentBased { start: 1000_u64, period: 5000_u64 },
			period_count: 1_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			moment_based_schedule_input,
		));
		System::set_block_number(21);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &BOB, 59).is_err());
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(BOB),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &BOB, 59));
	});
}

#[test]
fn vested_transfer_fails_if_zero_period_count() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 1_u64, period: 1_u64 },
			period_count: 0_u32,
			per_period: 100_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(
				RuntimeOrigin::root(),
				ALICE,
				BOB,
				MockCurrencyId::BTC,
				schedule_input,
			),
			Error::<Runtime>::ZeroVestingPeriodCount
		);
	});
}

#[test]
fn vested_transfer_fails_if_zero_period() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 1_u64, period: 0_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(
				RuntimeOrigin::root(),
				ALICE,
				BOB,
				MockCurrencyId::BTC,
				schedule_input.clone(),
			),
			Error::<Runtime>::ZeroVestingPeriod
		);

		assert_noop!(
			Vesting::vested_transfer(
				RuntimeOrigin::root(),
				ALICE,
				BOB,
				MockCurrencyId::BTC,
				schedule_input,
			),
			Error::<Runtime>::ZeroVestingPeriod
		);
	});
}

#[test]
fn vested_transfer_fails_if_transfer_err() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 1_u64, period: 1_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(
				RuntimeOrigin::root(),
				BOB,
				ALICE,
				MockCurrencyId::BTC,
				schedule_input,
			),
			orml_tokens::Error::<Runtime>::BalanceTooLow,
		);
	});
}

#[test]
fn vested_transfer_fails_if_overflow() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 1_u64, period: 1_u64 },
			period_count: 2_u32,
			per_period: u64::MAX,
		};
		assert_noop!(
			Vesting::vested_transfer(
				RuntimeOrigin::root(),
				ALICE,
				BOB,
				MockCurrencyId::BTC,
				schedule_input,
			),
			ArithmeticError::Overflow,
		);

		let another_schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: u64::MAX, period: 1_u64 },
			period_count: 2_u32,
			per_period: 1_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(
				RuntimeOrigin::root(),
				ALICE,
				BOB,
				MockCurrencyId::BTC,
				another_schedule_input,
			),
			ArithmeticError::Overflow,
		);

		let moment_based_schedule_input = VestingScheduleInfo {
			window: MomentBased { start: u64::MAX, period: 1_u64 },
			period_count: 2_u32,
			per_period: 1_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(
				RuntimeOrigin::root(),
				ALICE,
				BOB,
				MockCurrencyId::BTC,
				moment_based_schedule_input,
			),
			ArithmeticError::Overflow,
		);
	});
}

#[test]
fn vested_transfer_fails_if_bad_origin() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(
				RuntimeOrigin::signed(CHARLIE),
				CHARLIE,
				BOB,
				MockCurrencyId::BTC,
				schedule_input,
			),
			BadOrigin
		);
	});
}

#[test]
fn claim_works() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_input,
		));

		System::set_block_number(11);
		// remain locked if not claimed
		assert!(Tokens::transfer(RuntimeOrigin::signed(BOB), ALICE, MockCurrencyId::BTC, 10).is_err());
		// unlocked after claiming
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(BOB),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(4_u128)
		));
		let claimed_amount_per_schedule: BoundedBTreeMap<_, _, MaxVestingSchedule> =
			[(4_u128, 10)].into_iter().try_collect().unwrap();
		System::assert_last_event(RuntimeEvent::Vesting(crate::Event::Claimed {
			who: BOB,
			asset: MockCurrencyId::BTC,
			locked_amount: 10,
			vesting_schedule_ids: VestingScheduleIdSet::One(4_u128),
			claimed_amount_per_schedule,
		}));

		assert!(VestingSchedules::<Runtime>::contains_key(BOB, MockCurrencyId::BTC));
		assert_ok!(Tokens::transfer(RuntimeOrigin::signed(BOB), ALICE, MockCurrencyId::BTC, 10));
		// more are still locked
		assert!(Tokens::transfer(RuntimeOrigin::signed(BOB), ALICE, MockCurrencyId::BTC, 1).is_err());

		System::set_block_number(21);
		// claim more
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(BOB),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));

		let claimed_amount_per_schedule: BoundedBTreeMap<_, _, MaxVestingSchedule> =
			[(4_u128, 10)].into_iter().try_collect().unwrap();

		System::assert_last_event(RuntimeEvent::Vesting(crate::Event::Claimed {
			who: BOB,
			asset: MockCurrencyId::BTC,
			locked_amount: 0,
			vesting_schedule_ids: VestingScheduleIdSet::All,
			claimed_amount_per_schedule,
		}));

		assert!(!VestingSchedules::<Runtime>::contains_key(BOB, MockCurrencyId::BTC));
		assert_ok!(Tokens::transfer(RuntimeOrigin::signed(BOB), ALICE, MockCurrencyId::BTC, 10));
		// all used up
		assert_eq!(Tokens::free_balance(MockCurrencyId::BTC, &BOB), 0);

		// no locks anymore
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
	});
}

#[test]
fn claim_nonexistent_schedules() {
	ExtBuilder::build().execute_with(|| {
		// Claim schedule 10, which does not exist
		assert_noop!(
			Vesting::claim(
				RuntimeOrigin::signed(BOB),
				MockCurrencyId::BTC,
				VestingScheduleIdSet::One(10_u128)
			),
			Error::<Runtime>::VestingScheduleNotFound
		);

		// Claim all schedules
		assert_noop!(
			Vesting::claim(RuntimeOrigin::signed(BOB), MockCurrencyId::BTC, VestingScheduleIdSet::All,),
			Error::<Runtime>::VestingScheduleNotFound
		);

		// Add schedule 4
		let schedule_4_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_4_input,
		));

		// Locked balance should be 2*10 = 20
		//                          ----
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20_u64 })
		);

		System::set_block_number(11);

		// Claim schedules 4 and 10
		let claim_schedules =
			BoundedVec::<u128, MaxVestingSchedule>::try_from(vec![4_u128, 10_u128]).unwrap();

		assert_noop!(
			Vesting::claim(
				RuntimeOrigin::signed(BOB),
				MockCurrencyId::BTC,
				VestingScheduleIdSet::Many(claim_schedules),
			),
			Error::<Runtime>::VestingScheduleNotFound
		);

		// Locked balance should still be 2*10 = 20
		//                          ----
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20_u64 })
		);

		// Claim schedule 4
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(BOB),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(4_u128),
		));

		// Locked balance should be 20 - 10 = 10
		//                             ----
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 10_u64 })
		);
	});
}

#[test]
fn claim_with_id_works() {
	ExtBuilder::build().execute_with(|| {
		// Add schedule 4
		let schedule_4_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_4_input,
		));

		// Add schedule 5
		let schedule_5_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 15_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_5_input,
		));

		// Add schedule 6
		let schedule_6_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 3_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_6_input,
		));

		// Locked balance should be 2*10 + 2*15 + 2*3 = 56
		//                          -----------------
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 56_u64 })
		);

		// Claim for schedule 4
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(4_u128)
		));

		// Nothing should be claimed, so locked balance should still be 2*10 + 2*15 + 2*3 = 56
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 56_u64 })
		);

		assert!(VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));

		// Set block 11, which is halfway through all schedules
		System::set_block_number(11);

		// Claim for schedule 5
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(5_u128)
		));

		// Half of schedule 5 should be claimed
		// Locked balance should be (2*10 + 2*15 + 2*3) - 15 = 41
		//                                              ----
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 41_u64 })
		);

		// Claim for schedule 6
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(6_u128)
		));

		// Half of schedule 3 should be claimed
		// Locked balance should be (2*10 + 2*15 + 2*3) - 15 - 3 = 38
		//                                                   ---
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 38_u64 })
		);

		// Set block 21, in which all schedules have vested
		System::set_block_number(21);

		// Claim for schedule 4
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(4_u128)
		));

		// All of schedule 4 should be claimed
		// Locked balance should be (2*10 + 2*15 + 2*3) - 15 - 3 - 2*10 = 18
		//                                                       ------
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 18_u64 })
		);

		let schedules: BoundedBTreeMap<u128, _, _> =
			VestingSchedules::<Runtime>::get(&BOB, MockCurrencyId::BTC);

		// Schedule 4 should be removed
		assert!(!schedules.contains_key(&4_u128));

		// Schedules 5 and 6 should NOT be removed
		assert!(schedules.contains_key(&5_u128));
		assert!(schedules.contains_key(&6_u128));

		// Claim for schedule 6
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(6_u128)
		));

		// All of schedule 6 should be claimed
		// Locked balance should be (2*10 + 2*15 + 2*3) - 15 - 3 - 2*10 - 3 = 15
		//                                                              ---
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 15_u64 })
		);

		// Claim remaining for schedule 5
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(5_u128)
		));

		// All of schedule 5 should be claimed
		// Nothing left, so locked balance should be None
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC).get(0), None);
	});
}

#[test]
fn claim_with_multiple_ids_works() {
	ExtBuilder::build().execute_with(|| {
		// Add schedule 4
		let schedule_4_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_4_input,
		));

		// Add schedule 5
		let schedule_5_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 15_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_5_input,
		));

		// Add schedule 6
		let schedule_6_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 3_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_6_input,
		));

		// Locked balance should be 2*10 + 2*15 + 2*3 = 56
		//                          -----------------
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 56_u64 })
		);

		// Claim for schedule 4 and 5
		let claim_schedules =
			BoundedVec::<u128, MaxVestingSchedule>::try_from(vec![4_u128, 5_u128]).unwrap();
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::Many(claim_schedules.clone())
		));

		// Nothing should be claimed, so locked balance should still be 2*10 + 2*15 + 2*3 = 56
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 56_u64 })
		);

		assert!(VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));

		// Set block 11, which is halfway through all schedules
		System::set_block_number(11);

		// Claim for schedule 4 and 5
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::Many(claim_schedules)
		));

		// Half of schedule 4 and 5 should be claimed
		// Locked balance should be (2*10 + 2*15 + 2*3) - 10 - 15 = 31
		//                                              ---------
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 31_u64 })
		);

		// Set block 21, in which all schedules have vested
		System::set_block_number(21);

		// Claim for schedule 4 and 6
		let claim_schedules =
			BoundedVec::<u128, MaxVestingSchedule>::try_from(vec![4_u128, 6_u128]).unwrap();
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::Many(claim_schedules)
		));

		// All of schedule 4 and 6 should be claimed
		// Locked balance should be (2*10 + 2*15 + 2*3) - 15 - 2*10 - 2*3 = 15
		//                                                     ----------
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 15_u64 })
		);
	});
}

#[test]
fn claim_for_with_id_works() {
	ExtBuilder::build().execute_with(|| {
		// Add schedule 4
		let schedule_4_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_4_input,
		));

		// Add schedule 5
		let schedule_5_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 15_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_5_input,
		));

		// Locked balance should be 2*10 + 2*15 = 50
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 50_u64 })
		);

		// Claim for schedule 4
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(4_u128)
		));

		// Nothing should be claimed, so locked balance should still be 2*10 + 2*15 = 50
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 50_u64 })
		);
		assert!(VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));

		System::set_block_number(21);

		// Claim for schedule 5
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(5_u128)
		));

		// All of schedule 5 should be claimed
		// Locked balance should be (2*10 + 2*15) - 2*15 = 20
		//                                        ------
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20_u64 })
		);

		// Claim for schedule 4
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(4_u128)
		));

		// There should not be any locks left
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
		assert!(!VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));
	});
}

#[test]
fn claim_for_works() {
	ExtBuilder::build().execute_with(|| {
		// Add schedule 4
		let schedule_4_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_4_input,
		));

		// Locked balance should be 2*10 = 20
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20_u64 })
		);

		// Claim all schedules
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));

		// Nothing should be claimed, so locked balance should still be 2*10 = 20
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20_u64 })
		);
		assert!(VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));

		System::set_block_number(21);

		// Claim for all schedules
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));

		// There should not be any locks left
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
		assert!(!VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));
	});
}

#[test]
fn claim_for_works_moment_based() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: MomentBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_input,
		));

		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(4_u128)
		));
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20_u64 })
		);
		assert!(VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));

		Timestamp::set_timestamp(21);
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20_u64 })
		);
		assert_ok!(Vesting::claim_for(
			RuntimeOrigin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(4_u128)
		));
		// no locks anymore
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
		assert!(!VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));
	});
}

#[test]
fn update_vesting_schedules_works() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_input,
		));

		let moment_based_schedule_input = VestingScheduleInfo {
			window: MomentBased { start: 0_u64, period: 60000_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			moment_based_schedule_input,
		));

		let updated_schedule = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 20_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		let updated_moment_based_schedule = VestingScheduleInfo {
			window: MomentBased { start: 0_u64, period: 120000_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::update_vesting_schedules(
			RuntimeOrigin::root(),
			BOB,
			MockCurrencyId::BTC,
			vec![updated_schedule, updated_moment_based_schedule],
		));

		System::set_block_number(11);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(BOB),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));
		assert!(Tokens::transfer(RuntimeOrigin::signed(BOB), ALICE, MockCurrencyId::BTC, 1).is_err());

		System::set_block_number(21);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(BOB),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));
		assert_ok!(Tokens::transfer(RuntimeOrigin::signed(BOB), ALICE, MockCurrencyId::BTC, 20));

		// empty vesting schedules cleanup the storage and unlock the fund
		assert!(VestingSchedules::<Runtime>::contains_key(BOB, MockCurrencyId::BTC));
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20_u64 })
		);
		assert_ok!(Vesting::update_vesting_schedules(
			RuntimeOrigin::root(),
			BOB,
			MockCurrencyId::BTC,
			vec![],
		));
		assert!(!VestingSchedules::<Runtime>::contains_key(BOB, MockCurrencyId::BTC));
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
	});
}

#[test]
fn update_vesting_schedules_does_not_break_locking_amount() {
	ExtBuilder::build().execute_with(|| {
		// Mint missing tokens for vesting schedules
		assert_ok!(Tokens::mint_into(MockCurrencyId::BTC, &ALICE, 1_300));

		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC).get(0), None);

		// Locks 100 * 10 = 1_000
		let schedule_input_1 = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 1_u64 },
			period_count: 100_u32,
			per_period: 10_u64,
		};

		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_input_1,
		));

		// Locks 50 * 8 = 400
		let schedule_input_2 = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 1_u64 },
			period_count: 50_u32,
			per_period: 8_u64,
		};

		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_input_2,
		));

		// Should have locked 1_000 + 400 = 1_400
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC).get(0).unwrap().amount, 1_400);

		System::set_block_number(25);

		// Claim all. Should have locked 1_400 - (25 * 10) - (25 * 8) = 950
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(BOB),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));

		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC).get(0).unwrap().amount, 950);

		// Locks 60 * 5 = 300
		let schedule_input_3 = VestingScheduleInfo {
			window: BlockNumberBased { start: 30_u64, period: 1_u64 },
			period_count: 60_u32,
			per_period: 5_u64,
		};

		// Locks 200 * 2 = 400
		let schedule_input_4 = VestingScheduleInfo {
			window: BlockNumberBased { start: 40_u64, period: 1_u64 },
			period_count: 200_u32,
			per_period: 2_u64,
		};

		// Unlocks all and locks 300 + 400 = 700
		assert_ok!(Vesting::update_vesting_schedules(
			RuntimeOrigin::root(),
			BOB,
			MockCurrencyId::BTC,
			vec![schedule_input_3.clone(), schedule_input_4.clone()]
		));

		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC).get(0).unwrap().amount, 700);

		System::set_block_number(50);

		// Claim all. Should have locked 700 - (20 * 5) - (10 * 2) = 580
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(BOB),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::All
		));

		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC).get(0).unwrap().amount, 580);
	});
}

#[test]
fn update_vesting_schedules_fails_if_unexpected_existing_locks() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(Tokens::transfer(RuntimeOrigin::signed(ALICE), BOB, MockCurrencyId::BTC, 1));
		assert_ok!(Tokens::set_lock(*b"prelocks", MockCurrencyId::BTC, &BOB, 0_u64));
	});
}

#[test]
fn vested_transfer_check_for_min() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 1_u64, period: 1_u64 },
			period_count: 1_u32,
			per_period: 3_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(
				RuntimeOrigin::root(),
				BOB,
				ALICE,
				MockCurrencyId::BTC,
				schedule_input,
			),
			Error::<Runtime>::AmountLow
		);
	});
}

#[test]
fn multiple_vesting_schedule_claim_works() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		let schedule = VestingSchedule::from_input(4_u128, schedule_input.clone());
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_input,
		));

		let schedule2_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 3_u32,
			per_period: 10_u64,
		};
		let schedule2 = VestingSchedule::from_input(5_u128, schedule2_input.clone());

		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule2_input,
		));

		let all_schedules =
			BTreeMap::from([(4_u128, schedule.clone()), (5_u128, schedule2.clone())]);
		let bounded_all_schedules: BoundedBTreeMap<_, _, MaxVestingSchedule> =
			BoundedBTreeMap::try_from(all_schedules).unwrap();

		assert_eq!(Vesting::vesting_schedules(&BOB, MockCurrencyId::BTC), bounded_all_schedules);

		System::set_block_number(21);
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(BOB),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(4_u128)
		));
		let schedules_after_claim = BTreeMap::from([(5_u128, schedule2.clone())]);
		let bounded_schedules_after_claim: BoundedBTreeMap<_, _, MaxVestingSchedule> =
			BoundedBTreeMap::try_from(schedules_after_claim).unwrap();
		assert_eq!(
			Vesting::vesting_schedules(&BOB, MockCurrencyId::BTC),
			bounded_schedules_after_claim
		);

		System::set_block_number(31);
		assert_ok!(Vesting::claim(
			RuntimeOrigin::signed(BOB),
			MockCurrencyId::BTC,
			VestingScheduleIdSet::One(5_u128)
		));
		assert!(!VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));

		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
	});
}

#[test]
fn exceeding_maximum_schedules_should_fail() {
	ExtBuilder::build().execute_with(|| {
		let schedule_input = VestingScheduleInfo {
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		let moment_schedule_input = VestingScheduleInfo {
			window: MomentBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_input.clone(),
		));
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_input.clone(),
		));
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			moment_schedule_input,
		));
		assert_noop!(
			Vesting::vested_transfer(
				RuntimeOrigin::root(),
				ALICE,
				BOB,
				MockCurrencyId::BTC,
				schedule_input.clone(),
			),
			Error::<Runtime>::MaxVestingSchedulesExceeded
		);

		let schedule_inputs = vec![
			schedule_input.clone(),
			schedule_input.clone(),
			schedule_input.clone(),
			schedule_input,
		];

		assert_noop!(
			Vesting::update_vesting_schedules(
				RuntimeOrigin::root(),
				BOB,
				MockCurrencyId::BTC,
				schedule_inputs,
			),
			Error::<Runtime>::MaxVestingSchedulesExceeded
		);
	});
}
