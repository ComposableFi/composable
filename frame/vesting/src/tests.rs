//! Unit tests for the vesting module.

#![cfg(test)]

use super::*;
use composable_traits::vesting::{
	VestingSchedule,
	VestingWindow::{BlockNumberBased, MomentBased},
};
use frame_support::{assert_noop, assert_ok, error::BadOrigin, traits::fungibles::Mutate};
use mock::{Event, *};
use orml_tokens::BalanceLock;

#[test]
fn vesting_from_chain_spec_works() {
	ExtBuilder::build().execute_with(|| {
		// From the vesting below, only 20 out of 50 are locked at block 0.
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 30));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 31).is_err());
		let schedules = vec![
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
		];

		assert_eq!(Vesting::vesting_schedules(&CHARLIE, MockCurrencyId::BTC), schedules);
		System::set_block_number(1);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert_ok!(Vesting::claim(Origin::signed(CHARLIE), MockCurrencyId::BTC, None));
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 30));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 31).is_err());

		System::set_block_number(11);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert_ok!(Vesting::claim(Origin::signed(CHARLIE), MockCurrencyId::BTC, None));
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 45));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 46).is_err());

		System::set_block_number(14);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert_ok!(Vesting::claim(Origin::signed(CHARLIE), MockCurrencyId::BTC, None));
		// Block number based schedules are unlocked from block 14 onwards.
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 50));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 51).is_err());

		System::set_block_number(25);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert_ok!(Vesting::claim(Origin::signed(CHARLIE), MockCurrencyId::BTC, None));
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 60));
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 61).is_err());

		System::set_block_number(34);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		// everything unlocked
		assert_ok!(Vesting::claim(Origin::signed(CHARLIE), MockCurrencyId::BTC, None));
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &CHARLIE, 65));
	});
}

#[test]
fn vested_transfer_self_vest_ko() {
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(1);

		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
			already_claimed: 0_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(
				Origin::root(),
				ALICE,
				ALICE,
				MockCurrencyId::BTC,
				schedule.clone(),
			),
			Error::<Runtime>::TryingToSelfVest
		);
	});
}

#[test]
fn vested_transfer_works() {
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(1);

		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule.clone(),
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
fn vested_transfer_trait_emits_vesting_schedule_added_event() {
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(Tokens::mint_into(MockCurrencyId::ETH, &ALICE, 100));

		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
			already_claimed: 0_u64,
		};

		assert_ok!(<Vesting as VestedTransfer>::vested_transfer(
			MockCurrencyId::ETH,
			&ALICE,
			&BOB,
			schedule.clone(),
		));

		System::assert_last_event(Event::Vesting(crate::Event::VestingScheduleAdded {
			from: ALICE,
			to: BOB,
			asset: MockCurrencyId::ETH,
			schedule,
		}));
	});
}

#[test]
fn vested_transfer_for_moment_based_schedule_works() {
	ExtBuilder::build().execute_with(|| {
		System::set_block_number(1);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);

		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: MomentBased { start: 0_u64, period: 10_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule.clone(),
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
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule,
		));

		System::set_block_number(12);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);

		let another_schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 10_u64, period: 13_u64 },
			period_count: 1_u32,
			per_period: 7_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			another_schedule,
		));

		let moment_based_schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: MomentBased { start: 72000_u64, period: 5000_u64 },
			period_count: 2_u32,
			per_period: 7_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			moment_based_schedule,
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
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 10_u64, period: 10_u64 },
			period_count: 1_u32,
			per_period: 50_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule,
		));
		let moment_based_schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: MomentBased { start: 1000_u64, period: 5000_u64 },
			period_count: 1_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			moment_based_schedule,
		));
		System::set_block_number(21);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &BOB, 59).is_err());
		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC, Some(1_u128)));
		assert_ok!(Tokens::ensure_can_withdraw(MockCurrencyId::BTC, &BOB, 59));
	});
}

#[test]
fn vested_transfer_fails_if_zero_period_count() {
	ExtBuilder::build().execute_with(|| {
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 1_u64, period: 1_u64 },
			period_count: 0_u32,
			per_period: 100_u64,
			already_claimed: 0_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(Origin::root(), ALICE, BOB, MockCurrencyId::BTC, schedule,),
			Error::<Runtime>::ZeroVestingPeriodCount
		);
	});
}

#[test]
fn vested_transfer_fails_if_zero_period() {
	ExtBuilder::build().execute_with(|| {
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 1_u64, period: 0_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
			already_claimed: 0_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(Origin::root(), ALICE, BOB, MockCurrencyId::BTC, schedule,),
			Error::<Runtime>::ZeroVestingPeriod
		);

		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: MomentBased { start: 1_u64, period: 0_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
			already_claimed: 0_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(Origin::root(), ALICE, BOB, MockCurrencyId::BTC, schedule,),
			Error::<Runtime>::ZeroVestingPeriod
		);
	});
}

#[test]
fn vested_transfer_fails_if_transfer_err() {
	ExtBuilder::build().execute_with(|| {
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 1_u64, period: 1_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
			already_claimed: 0_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(Origin::root(), BOB, ALICE, MockCurrencyId::BTC, schedule,),
			orml_tokens::Error::<Runtime>::BalanceTooLow,
		);
	});
}

#[test]
fn vested_transfer_fails_if_overflow() {
	ExtBuilder::build().execute_with(|| {
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 1_u64, period: 1_u64 },
			period_count: 2_u32,
			per_period: u64::MAX,
			already_claimed: 0_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(Origin::root(), ALICE, BOB, MockCurrencyId::BTC, schedule,),
			ArithmeticError::Overflow,
		);

		let another_schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: u64::MAX, period: 1_u64 },
			period_count: 2_u32,
			per_period: 1_u64,
			already_claimed: 0_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(
				Origin::root(),
				ALICE,
				BOB,
				MockCurrencyId::BTC,
				another_schedule,
			),
			ArithmeticError::Overflow,
		);

		let moment_based_schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: MomentBased { start: u64::MAX, period: 1_u64 },
			period_count: 2_u32,
			per_period: 1_u64,
			already_claimed: 0_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(
				Origin::root(),
				ALICE,
				BOB,
				MockCurrencyId::BTC,
				moment_based_schedule,
			),
			ArithmeticError::Overflow,
		);
	});
}

#[test]
fn vested_transfer_fails_if_bad_origin() {
	ExtBuilder::build().execute_with(|| {
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 1_u32,
			per_period: 100_u64,
			already_claimed: 0_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(
				Origin::signed(CHARLIE),
				CHARLIE,
				BOB,
				MockCurrencyId::BTC,
				schedule,
			),
			BadOrigin
		);
	});
}

#[test]
fn claim_works() {
	ExtBuilder::build().execute_with(|| {
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule,
		));

		System::set_block_number(11);
		// remain locked if not claimed
		assert!(Tokens::transfer(Origin::signed(BOB), ALICE, MockCurrencyId::BTC, 10).is_err());
		// unlocked after claiming
		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC, Some(1_u128)));
		// TODO: test with Some(1_u128)
		assert!(VestingSchedules::<Runtime>::contains_key(BOB, MockCurrencyId::BTC));
		assert_ok!(Tokens::transfer(Origin::signed(BOB), ALICE, MockCurrencyId::BTC, 10));
		// more are still locked
		assert!(Tokens::transfer(Origin::signed(BOB), ALICE, MockCurrencyId::BTC, 1).is_err());

		System::set_block_number(21);
		// claim more
		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC, None));
		println!("{:?}", VestingSchedules::<Runtime>::get(BOB, MockCurrencyId::BTC));
		assert!(!VestingSchedules::<Runtime>::contains_key(BOB, MockCurrencyId::BTC));
		assert_ok!(Tokens::transfer(Origin::signed(BOB), ALICE, MockCurrencyId::BTC, 10));
		// all used up
		assert_eq!(Tokens::free_balance(MockCurrencyId::BTC, &BOB), 0);

		// no locks anymore
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
	});
}

#[test]
fn claim_with_id_works() {
	ExtBuilder::build().execute_with(|| {
		// Add schedule 1
		let schedule_1 = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_1,
		));

		// Add schedule 2
		let schedule_2 = VestingSchedule {
			vesting_schedule_id: 2_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 15_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_2,
		));

		// Add schedule 3
		let schedule_3 = VestingSchedule {
			vesting_schedule_id: 3_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 3_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_3,
		));

		// Locked balance should be 2*10 + 2*15 + 2*3 = 56
		//                          -----------------
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 56_u64 })
		);

		// Claim for schedule 1
		assert_ok!(Vesting::claim_for(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			Some(1_u128)
		));

		// Nothing should be claimed, so locked balance should still be 2*10 + 2*15 + 2*3 = 56
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 56_u64 })
		);

		assert!(VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));

		// Set block 11, which is halfway through all schedules
		System::set_block_number(11);

		println!("locks: {:?}", Tokens::locks(&BOB, MockCurrencyId::BTC).get(0));

		// Claim for schedule 2
		assert_ok!(Vesting::claim_for(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			Some(2_u128)
		));

		// Half of schedule 2 should be claimed
		// Locked balance should be (2*10 + 2*15 + 2*3) - 15 = 41
		//                                              ----
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 41_u64 })
		);

		// Claim for schedule 3
		assert_ok!(Vesting::claim_for(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			Some(3_u128)
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

		// Claim for schedule 1
		assert_ok!(Vesting::claim_for(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			Some(1_u128)
		));

		// All of schedule 1 should be claimed
		// Locked balance should be (2*10 + 2*15 + 2*3) - 15 - 3 - 2*10 = 18
		//                                                       ------
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 18_u64 })
		);

		// Claim for schedule 3
		println!("schedule 3 start");
		assert_ok!(Vesting::claim_for(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			Some(3_u128)
		));
		println!("schedule 3 end");

		// All of schedule 3 should be claimed
		// Locked balance should be (2*10 + 2*15 + 2*3) - 15 - 3 - 2*10 - 3 = 15
		//                                                              ---
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 15_u64 })
		);

		// Claim remaining for schedule 2
		assert_ok!(Vesting::claim_for(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			Some(2_u128)
		));

		// All of schedule 2 should be claimed
		// Nothing left, so locked balance should be None
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC).get(0), None);
	});
}

#[test]
fn claim_for_with_id_works() {
	ExtBuilder::build().execute_with(|| {
		// Add schedule 1
		let schedule_1 = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_1,
		));

		// Add schedule 2
		let schedule_2 = VestingSchedule {
			vesting_schedule_id: 2_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 15_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_2,
		));

		// Locked balance should be 2*10 + 2*15 = 50
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 50_u64 })
		);

		// Claim for schedule 1
		assert_ok!(Vesting::claim_for(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			Some(1_u128)
		));

		// Nothing should be claimed, so locked balance should still be 2*10 + 2*15 = 50
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 50_u64 })
		);
		assert!(VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));

		System::set_block_number(21);

		// Claim for schedule 2
		assert_ok!(Vesting::claim_for(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			Some(2_u128)
		));

		// All of schedule 2 should be claimed
		// Locked balance should be (2*10 + 2*15) - 2*15 = 20
		//                                        ------
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20_u64 })
		);

		// Claim for schedule 1
		assert_ok!(Vesting::claim_for(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			Some(1_u128)
		));

		// There should not be any locks left
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
		assert!(!VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));
	});
}

#[test]
fn claim_for_works() {
	ExtBuilder::build().execute_with(|| {
		// Add schedule 1
		let schedule_1 = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule_1,
		));

		// Locked balance should be 2*10 = 20
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20_u64 })
		);

		// Claim for schedule 1
		assert_ok!(Vesting::claim_for(Origin::signed(ALICE), BOB, MockCurrencyId::BTC, None));

		// Nothing should be claimed, so locked balance should still be 2*10 = 20
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20_u64 })
		);
		assert!(VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));

		System::set_block_number(21);

		// Claim for schedule 1
		assert_ok!(Vesting::claim_for(Origin::signed(ALICE), BOB, MockCurrencyId::BTC, None));

		// There should not be any locks left
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
		assert!(!VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));
	});
}

#[test]
fn claim_for_works_moment_based() {
	ExtBuilder::build().execute_with(|| {
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: MomentBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule,
		));

		assert_ok!(Vesting::claim_for(
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			Some(1_u128)
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
			Origin::signed(ALICE),
			BOB,
			MockCurrencyId::BTC,
			Some(1_u128)
		));
		// no locks anymore
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
		assert!(!VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));
	});
}

#[test]
fn update_vesting_schedules_works() {
	ExtBuilder::build().execute_with(|| {
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule,
		));

		let moment_based_schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: MomentBased { start: 0_u64, period: 60000_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			moment_based_schedule,
		));

		let updated_schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 20_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		let updated_moment_based_schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: MomentBased { start: 0_u64, period: 120000_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::update_vesting_schedules(
			Origin::root(),
			BOB,
			MockCurrencyId::BTC,
			vec![updated_schedule, updated_moment_based_schedule],
		));

		System::set_block_number(11);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC, None));
		assert!(Tokens::transfer(Origin::signed(BOB), ALICE, MockCurrencyId::BTC, 1).is_err());

		System::set_block_number(21);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC, None));
		assert_ok!(Tokens::transfer(Origin::signed(BOB), ALICE, MockCurrencyId::BTC, 20));

		// empty vesting schedules cleanup the storage and unlock the fund
		assert!(VestingSchedules::<Runtime>::contains_key(BOB, MockCurrencyId::BTC));
		assert_eq!(
			Tokens::locks(&BOB, MockCurrencyId::BTC).get(0),
			Some(&BalanceLock { id: VESTING_LOCK_ID, amount: 20_u64 })
		);
		assert_ok!(Vesting::update_vesting_schedules(
			Origin::root(),
			BOB,
			MockCurrencyId::BTC,
			vec![],
		));
		assert!(!VestingSchedules::<Runtime>::contains_key(BOB, MockCurrencyId::BTC));
		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
	});
}

#[test]
fn update_vesting_schedules_fails_if_unexpected_existing_locks() {
	ExtBuilder::build().execute_with(|| {
		assert_ok!(Tokens::transfer(Origin::signed(ALICE), BOB, MockCurrencyId::BTC, 1));
		assert_ok!(Tokens::set_lock(*b"prelocks", MockCurrencyId::BTC, &BOB, 0_u64));
	});
}

#[test]
fn vested_transfer_check_for_min() {
	ExtBuilder::build().execute_with(|| {
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 1_u64, period: 1_u64 },
			period_count: 1_u32,
			per_period: 3_u64,
			already_claimed: 0_u64,
		};
		assert_noop!(
			Vesting::vested_transfer(Origin::root(), BOB, ALICE, MockCurrencyId::BTC, schedule,),
			Error::<Runtime>::AmountLow
		);
	});
}

#[test]
fn multiple_vesting_schedule_claim_works() {
	ExtBuilder::build().execute_with(|| {
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule.clone(),
		));

		let schedule2 = VestingSchedule {
			vesting_schedule_id: 2_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 3_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule2.clone(),
		));

		assert_eq!(
			Vesting::vesting_schedules(&BOB, MockCurrencyId::BTC),
			vec![schedule, schedule2.clone()]
		);

		System::set_block_number(21);
		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC, Some(1_u128)));
		assert_eq!(Vesting::vesting_schedules(&BOB, MockCurrencyId::BTC), vec![schedule2]);

		System::set_block_number(31);
		assert_ok!(Vesting::claim(Origin::signed(BOB), MockCurrencyId::BTC, Some(2_u128)));
		assert!(!VestingSchedules::<Runtime>::contains_key(&BOB, MockCurrencyId::BTC));

		assert_eq!(Tokens::locks(&BOB, MockCurrencyId::BTC), vec![]);
	});
}

#[test]
fn exceeding_maximum_schedules_should_fail() {
	ExtBuilder::build().execute_with(|| {
		let schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: BlockNumberBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		let moment_schedule = VestingSchedule {
			vesting_schedule_id: 1_u128,
			window: MomentBased { start: 0_u64, period: 10_u64 },
			period_count: 2_u32,
			per_period: 10_u64,
			already_claimed: 0_u64,
		};
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule.clone(),
		));
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			schedule.clone(),
		));
		assert_ok!(Vesting::vested_transfer(
			Origin::root(),
			ALICE,
			BOB,
			MockCurrencyId::BTC,
			moment_schedule,
		));
		assert_noop!(
			Vesting::vested_transfer(
				Origin::root(),
				ALICE,
				BOB,
				MockCurrencyId::BTC,
				schedule.clone(),
			),
			Error::<Runtime>::MaxVestingSchedulesExceeded
		);

		let schedules = vec![schedule.clone(), schedule.clone(), schedule.clone(), schedule];

		assert_noop!(
			Vesting::update_vesting_schedules(Origin::root(), BOB, MockCurrencyId::BTC, schedules,),
			Error::<Runtime>::MaxVestingSchedulesExceeded
		);
	});
}
