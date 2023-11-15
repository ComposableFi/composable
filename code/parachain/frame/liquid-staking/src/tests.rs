use frame_support::{
	assert_noop, assert_ok,
	dispatch::DispatchResult,
	error::BadOrigin,
	storage::with_transaction,
	traits::{fungibles::Inspect, Hooks},
};
use sp_runtime::{
	traits::{BlakeTwo256, One, Saturating, Zero},
	ArithmeticError::Underflow,
	MultiAddress::Id,
	TransactionOutcome,
};
use sp_trie::StorageProof;
use xcm_simulator::TestExt;

use crate::{mock::*, types::*, *};

#[test]
fn stake_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(LiquidStaking::stake(RuntimeOrigin::signed(ALICE), ksm(10f64)));

		// Check storage is correct
		assert_eq!(ExchangeRate::<Test>::get(), Rate::one());
		assert_eq!(
			MatchingPool::<Test>::get(),
			MatchingLedger {
				total_stake_amount: ReservableAmount { total: ksm(9.95f64), reserved: 0 },
				total_unstake_amount: Default::default(),
			}
		);

		// Check balance is correct
		assert_eq!(<Test as Config>::Assets::balance(KSM, &ALICE), ksm(90f64));
		assert_eq!(<Test as Config>::Assets::balance(SKSM, &ALICE), ksm(109.95f64));

		assert_eq!(
			<Test as Config>::Assets::balance(KSM, &LiquidStaking::account_id()),
			ksm(10f64)
		);

		assert_ok!(with_transaction(|| -> TransactionOutcome<DispatchResult> {
			LiquidStaking::do_advance_era(1).unwrap();
			LiquidStaking::do_matching().unwrap();
			LiquidStaking::notification_received(
				pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
				0,
				Response::ExecutionResult(None),
			)
			.unwrap();
			TransactionOutcome::Commit(Ok(()))
		}));

		assert_eq!(
			<Test as Config>::Assets::balance(KSM, &LiquidStaking::account_id()),
			ksm(0.05f64)
		);

		assert_eq!(
			MatchingPool::<Test>::get(),
			MatchingLedger {
				total_stake_amount: Default::default(),
				total_unstake_amount: Default::default(),
			}
		);

		let derivative_index = 0u16;
		assert_eq!(
			StakingLedgers::<Test>::get(&0).unwrap(),
			StakingLedger {
				stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
				total: ksm(9.95f64),
				active: ksm(9.95f64),
				unlocking: vec![],
				claimed_rewards: vec![]
			}
		);

		assert_ok!(LiquidStaking::stake(RuntimeOrigin::signed(ALICE), ksm(10f64)));

		assert_ok!(with_transaction(|| -> TransactionOutcome<DispatchResult> {
			LiquidStaking::do_advance_era(1).unwrap();
			LiquidStaking::do_matching().unwrap();
			LiquidStaking::notification_received(
				pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
				1,
				Response::ExecutionResult(None),
			)
			.unwrap();
			TransactionOutcome::Commit(Ok(()))
		}));

		assert_eq!(
			<Test as Config>::Assets::balance(KSM, &LiquidStaking::account_id()),
			ksm(0.1f64)
		);

		assert_eq!(
			StakingLedgers::<Test>::get(&0).unwrap(),
			StakingLedger {
				stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
				total: ksm(19.9f64),
				active: ksm(19.9f64),
				unlocking: vec![],
				claimed_rewards: vec![]
			}
		);
	});
}

#[test]
fn unstake_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(LiquidStaking::stake(RuntimeOrigin::signed(ALICE), ksm(10f64)));
		assert_ok!(LiquidStaking::unstake(
			RuntimeOrigin::signed(ALICE),
			ksm(6f64),
			Default::default()
		));

		// Check storage is correct
		assert_eq!(ExchangeRate::<Test>::get(), Rate::one());
		assert_eq!(
			MatchingPool::<Test>::get(),
			MatchingLedger {
				total_stake_amount: ReservableAmount { total: ksm(9.95f64), reserved: 0 },
				total_unstake_amount: ReservableAmount { total: ksm(6f64), reserved: 0 }
			}
		);

		assert_eq!(
			Unlockings::<Test>::get(ALICE).unwrap(),
			vec![UnlockChunk { value: ksm(6f64), era: 4 }]
		);

		assert_ok!(with_transaction(|| -> TransactionOutcome<DispatchResult> {
			LiquidStaking::do_advance_era(1).unwrap();
			LiquidStaking::do_matching().unwrap();
			LiquidStaking::notification_received(
				pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
				0,
				Response::ExecutionResult(None),
			)
			.unwrap();
			TransactionOutcome::Commit(Ok(()))
		}));

		assert_eq!(
			MatchingPool::<Test>::get(),
			MatchingLedger {
				total_stake_amount: Default::default(),
				total_unstake_amount: Default::default(),
			}
		);

		let derivative_index = 0u16;
		assert_eq!(
			StakingLedgers::<Test>::get(&0).unwrap(),
			StakingLedger {
				stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
				total: ksm(3.95f64),
				active: ksm(3.95f64),
				unlocking: vec![],
				claimed_rewards: vec![]
			}
		);
		// Just make it 1 to calculate.
		ExchangeRate::<Test>::set(Rate::one());
		assert_ok!(LiquidStaking::unstake(
			RuntimeOrigin::signed(ALICE),
			ksm(3.95f64),
			Default::default()
		));

		assert_eq!(
			Unlockings::<Test>::get(ALICE).unwrap(),
			vec![
				UnlockChunk { value: ksm(6f64), era: 4 },
				UnlockChunk { value: ksm(3.95f64), era: 5 }
			]
		);

		assert_ok!(with_transaction(|| -> TransactionOutcome<DispatchResult> {
			LiquidStaking::do_advance_era(1).unwrap();
			LiquidStaking::do_matching().unwrap();
			LiquidStaking::notification_received(
				pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
				1,
				Response::ExecutionResult(None),
			)
			.unwrap();
			TransactionOutcome::Commit(Ok(()))
		}));

		assert_eq!(
			StakingLedgers::<Test>::get(&0).unwrap(),
			StakingLedger {
				stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
				total: ksm(3.95),
				active: 0,
				unlocking: vec![UnlockChunk { value: ksm(3.95), era: 5 }],
				claimed_rewards: vec![]
			}
		);
	});
}

enum StakeOp {
	Stake(Balance),
	Unstake(Balance),
}

impl StakeOp {
	fn execute(self) {
		match self {
			Self::Stake(amount) =>
				LiquidStaking::stake(RuntimeOrigin::signed(ALICE), amount).unwrap(),
			Self::Unstake(amount) =>
				LiquidStaking::unstake(RuntimeOrigin::signed(ALICE), amount, Default::default())
					.unwrap(),
		};
	}
}

#[test]
fn test_matching_should_work() {
	use StakeOp::*;
	TestNet::reset();
	ParaA::execute_with(|| {
		let test_case: Vec<(Vec<StakeOp>, Balance, Balance, (Balance, Balance, Balance))> = vec![
			(vec![Stake(ksm(5000f64)), Unstake(ksm(1000f64))], 0, 0, (ksm(3975f64), 0, 0)),
			// Calculate right here.
			(
				vec![Unstake(ksm(10f64)), Unstake(ksm(5f64)), Stake(ksm(10f64))],
				ksm(3975f64),
				0,
				(0, 0, ksm(5.05f64)),
			),
		];
		for (i, (stake_ops, _bonding_amount, unbonding_amount, matching_result)) in
			test_case.into_iter().enumerate()
		{
			stake_ops.into_iter().for_each(StakeOp::execute);
			assert_eq!(
				LiquidStaking::matching_pool().matching(unbonding_amount),
				Ok(matching_result)
			);
			assert_ok!(with_transaction(|| -> TransactionOutcome<DispatchResult> {
				LiquidStaking::do_advance_era(1).unwrap();
				LiquidStaking::do_matching().unwrap();
				LiquidStaking::notification_received(
					pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
					i.try_into().unwrap(),
					Response::ExecutionResult(None),
				)
				.unwrap();
				TransactionOutcome::Commit(Ok(()))
			}));
		}
	});
}

#[test]
fn test_transact_bond_work() {
	TestNet::reset();
	let derivative_index = 0u16;
	ParaA::execute_with(|| {
		assert_ok!(LiquidStaking::stake(RuntimeOrigin::signed(ALICE), ksm(2000f64),));
		assert_ok!(LiquidStaking::bond(
			RuntimeOrigin::signed(ALICE),
			derivative_index,
			ksm(3f64),
			RewardDestination::Staked
		));

		ParaSystem::assert_has_event(mock::RuntimeEvent::LiquidStaking(crate::Event::Bonding(
			derivative_index,
			LiquidStaking::derivative_sovereign_account_id(derivative_index),
			ksm(3f64),
			RewardDestination::Staked,
		)));
	});

	Relay::execute_with(|| {
		RelaySystem::assert_has_event(RelayEvent::Staking(RelayStakingEvent::Bonded {
			stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
			amount: ksm(3f64),
		}));
		let ledger =
			RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(derivative_index))
				.unwrap();
		assert_eq!(ledger.total, ksm(3f64));
	});
}

#[test]
fn test_transact_bond_extra_work() {
	TestNet::reset();
	let derivative_index = 0u16;
	ParaA::execute_with(|| {
		assert_ok!(LiquidStaking::stake(RuntimeOrigin::signed(ALICE), ksm(4000f64),));
		let bond_amount = ksm(2f64);
		assert_ok!(LiquidStaking::bond(
			RuntimeOrigin::signed(ALICE),
			derivative_index,
			bond_amount,
			RewardDestination::Staked
		));
		assert_ok!(LiquidStaking::notification_received(
			pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
			0,
			Response::ExecutionResult(None),
		));

		assert_ok!(LiquidStaking::bond_extra(
			RuntimeOrigin::signed(ALICE),
			derivative_index,
			ksm(3f64)
		));
	});

	Relay::execute_with(|| {
		let ledger =
			RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(derivative_index))
				.unwrap();
		assert_eq!(ledger.total, ksm(5f64));
	});
}

#[test]
fn test_transact_unbond_work() {
	TestNet::reset();
	let derivative_index = 0u16;
	ParaA::execute_with(|| {
		assert_ok!(LiquidStaking::stake(RuntimeOrigin::signed(ALICE), ksm(6000f64),));
		assert_ok!(LiquidStaking::unstake(
			RuntimeOrigin::signed(ALICE),
			ksm(1000f64),
			Default::default()
		));
		let bond_amount = ksm(5f64);

		assert_ok!(LiquidStaking::bond(
			RuntimeOrigin::signed(ALICE),
			derivative_index,
			bond_amount,
			RewardDestination::Staked
		));

		assert_ok!(LiquidStaking::notification_received(
			pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
			0,
			Response::ExecutionResult(None),
		));
	});

	Relay::execute_with(|| {
		RelaySystem::assert_has_event(RelayEvent::Staking(RelayStakingEvent::Bonded {
			stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
			amount: ksm(5f64),
		}));
	});

	ParaA::execute_with(|| {
		assert_ok!(LiquidStaking::unbond(
			RuntimeOrigin::signed(ALICE),
			derivative_index,
			ksm(2f64)
		));
	});

	Relay::execute_with(|| {
		RelaySystem::assert_has_event(RelayEvent::Staking(RelayStakingEvent::Unbonded {
			stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
			amount: ksm(2f64),
		}));
		let ledger =
			RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(derivative_index))
				.unwrap();
		assert_eq!(ledger.total, ksm(5f64));
		assert_eq!(ledger.active, ksm(3f64));
	});
}

#[test]
fn test_transact_withdraw_unbonded_work() {
	TestNet::reset();
	let derivative_index = 0u16;
	ParaA::execute_with(|| {
		assert_ok!(LiquidStaking::stake(RuntimeOrigin::signed(ALICE), ksm(6000f64),));
		assert_ok!(LiquidStaking::unstake(
			RuntimeOrigin::signed(ALICE),
			ksm(2000f64),
			Default::default()
		));
		let bond_amount = ksm(5f64);
		assert_ok!(LiquidStaking::bond(
			RuntimeOrigin::signed(ALICE),
			derivative_index,
			bond_amount,
			RewardDestination::Staked
		));
		assert_ok!(LiquidStaking::notification_received(
			pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
			0,
			Response::ExecutionResult(None),
		));
	});

	Relay::execute_with(|| {
		RelaySystem::assert_has_event(RelayEvent::Staking(RelayStakingEvent::Bonded {
			stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
			amount: ksm(5f64),
		}));
	});

	ParaA::execute_with(|| {
		let unbond_amount = ksm(2f64);
		assert_ok!(LiquidStaking::unbond(
			RuntimeOrigin::signed(ALICE),
			derivative_index,
			unbond_amount
		));
		assert_ok!(LiquidStaking::notification_received(
			pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
			1,
			Response::ExecutionResult(None),
		));
	});

	Relay::execute_with(|| {
		let ledger =
			RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(derivative_index))
				.unwrap();
		assert_eq!(ledger.total, ksm(5f64));
		assert_eq!(ledger.active, ksm(3f64));
		assert_eq!(ledger.unlocking.len(), 1);

		RelaySystem::assert_has_event(RelayEvent::Staking(RelayStakingEvent::Unbonded {
			stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
			amount: ksm(2f64),
		}));

		pallet_staking::CurrentEra::<KusamaRuntime>::put(
			<KusamaRuntime as pallet_staking::Config>::BondingDuration::get(),
		);
	});

	ParaA::execute_with(|| {
		assert_ok!(LiquidStaking::force_set_current_era(
			RuntimeOrigin::root(),
			<KusamaRuntime as pallet_staking::Config>::BondingDuration::get(),
		));

		assert_ok!(LiquidStaking::withdraw_unbonded(RuntimeOrigin::root(), derivative_index, 0));
	});

	Relay::execute_with(|| {
		let ledger =
			RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(derivative_index))
				.unwrap();
		assert_eq!(ledger.total, ksm(3f64));
		assert_eq!(ledger.active, ksm(3f64));
		assert_eq!(ledger.unlocking.len(), 0);
	});
}