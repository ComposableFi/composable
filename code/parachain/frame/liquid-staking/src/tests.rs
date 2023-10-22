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

use pallet_traits::ump::RewardDestination;
use primitives::{
    tokens::{KSM, SKSM},
    Balance, Rate, Ratio,
};

use crate::{
    mock::{Loans, *},
    types::*,
    *,
};

#[test]
fn stake_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(10f64)
        ));
        // Check storage is correct
        assert_eq!(ExchangeRate::<Test>::get(), Rate::one());
        assert_eq!(
            MatchingPool::<Test>::get(),
            MatchingLedger {
                total_stake_amount: ReservableAmount {
                    total: ksm(9.95f64),
                    reserved: 0
                },
                total_unstake_amount: Default::default(),
            }
        );

        // Check balance is correct
        assert_eq!(<Test as Config>::Assets::balance(KSM, &ALICE), ksm(90f64));
        assert_eq!(
            <Test as Config>::Assets::balance(SKSM, &ALICE),
            ksm(109.95f64)
        );

        assert_eq!(
            <Test as Config>::Assets::balance(KSM, &LiquidStaking::account_id()),
            ksm(10f64)
        );

        assert_ok!(with_transaction(
            || -> TransactionOutcome<DispatchResult> {
                LiquidStaking::do_advance_era(1).unwrap();
                LiquidStaking::do_matching().unwrap();
                LiquidStaking::notification_received(
                    pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
                    0,
                    Response::ExecutionResult(None),
                )
                .unwrap();
                TransactionOutcome::Commit(Ok(()))
            }
        ));

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

        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(10f64)
        ));

        assert_ok!(with_transaction(
            || -> TransactionOutcome<DispatchResult> {
                LiquidStaking::do_advance_era(1).unwrap();
                LiquidStaking::do_matching().unwrap();
                LiquidStaking::notification_received(
                    pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
                    1,
                    Response::ExecutionResult(None),
                )
                .unwrap();
                TransactionOutcome::Commit(Ok(()))
            }
        ));

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
    })
}

#[test]
fn unstake_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(10f64)
        ));
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
                total_stake_amount: ReservableAmount {
                    total: ksm(9.95f64),
                    reserved: 0
                },
                total_unstake_amount: ReservableAmount {
                    total: ksm(6f64),
                    reserved: 0
                }
            }
        );

        assert_eq!(
            Unlockings::<Test>::get(ALICE).unwrap(),
            vec![UnlockChunk {
                value: ksm(6f64),
                era: 4
            }]
        );

        assert_ok!(with_transaction(
            || -> TransactionOutcome<DispatchResult> {
                LiquidStaking::do_advance_era(1).unwrap();
                LiquidStaking::do_matching().unwrap();
                LiquidStaking::notification_received(
                    pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
                    0,
                    Response::ExecutionResult(None),
                )
                .unwrap();
                TransactionOutcome::Commit(Ok(()))
            }
        ));

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
                UnlockChunk {
                    value: ksm(6f64),
                    era: 4
                },
                UnlockChunk {
                    value: ksm(3.95f64),
                    era: 5
                }
            ]
        );

        assert_ok!(with_transaction(
            || -> TransactionOutcome<DispatchResult> {
                LiquidStaking::do_advance_era(1).unwrap();
                LiquidStaking::do_matching().unwrap();
                LiquidStaking::notification_received(
                    pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
                    1,
                    Response::ExecutionResult(None),
                )
                .unwrap();
                TransactionOutcome::Commit(Ok(()))
            }
        ));

        assert_eq!(
            StakingLedgers::<Test>::get(&0).unwrap(),
            StakingLedger {
                stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
                total: ksm(3.95),
                active: 0,
                unlocking: vec![UnlockChunk {
                    value: ksm(3.95),
                    era: 5
                }],
                claimed_rewards: vec![]
            }
        );
    })
}

enum StakeOp {
    Stake(Balance),
    Unstake(Balance),
}

impl StakeOp {
    fn execute(self) {
        match self {
            Self::Stake(amount) => {
                LiquidStaking::stake(RuntimeOrigin::signed(ALICE), amount).unwrap()
            }
            Self::Unstake(amount) => {
                LiquidStaking::unstake(RuntimeOrigin::signed(ALICE), amount, Default::default())
                    .unwrap()
            }
        };
    }
}

#[test]
fn test_matching_should_work() {
    use StakeOp::*;
    TestNet::reset();
    ParaA::execute_with(|| {
        let test_case: Vec<(Vec<StakeOp>, Balance, Balance, (Balance, Balance, Balance))> = vec![
            (
                vec![Stake(ksm(5000f64)), Unstake(ksm(1000f64))],
                0,
                0,
                (ksm(3975f64), 0, 0),
            ),
            // Calculate right here.
            (
                vec![Unstake(ksm(10f64)), Unstake(ksm(5f64)), Stake(ksm(10f64))],
                ksm(3975f64),
                0,
                (0, 0, ksm(5.05f64)),
            ),
            // (vec![], 0, (0, 0, 0)),
        ];
        for (i, (stake_ops, _bonding_amount, unbonding_amount, matching_result)) in
            test_case.into_iter().enumerate()
        {
            stake_ops.into_iter().for_each(StakeOp::execute);
            assert_eq!(
                LiquidStaking::matching_pool().matching(unbonding_amount),
                Ok(matching_result)
            );
            assert_ok!(with_transaction(
                || -> TransactionOutcome<DispatchResult> {
                    LiquidStaking::do_advance_era(1).unwrap();
                    LiquidStaking::do_matching().unwrap();
                    LiquidStaking::notification_received(
                        pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
                        i.try_into().unwrap(),
                        Response::ExecutionResult(None),
                    )
                    .unwrap();
                    TransactionOutcome::Commit(Ok(()))
                }
            ));
        }
    });
}

#[test]
fn test_transact_bond_work() {
    TestNet::reset();
    let derivative_index = 0u16;
    ParaA::execute_with(|| {
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(2000f64),
        ));
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
        let ledger = RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(
            derivative_index,
        ))
        .unwrap();
        assert_eq!(ledger.total, ksm(3f64));
    });
}

#[test]
fn test_transact_bond_extra_work() {
    TestNet::reset();
    let derivative_index = 0u16;
    ParaA::execute_with(|| {
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(4000f64),
        ));
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
        let ledger = RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(
            derivative_index,
        ))
        .unwrap();
        assert_eq!(ledger.total, ksm(5f64));
    });
}

#[test]
fn test_transact_unbond_work() {
    TestNet::reset();
    let derivative_index = 0u16;
    ParaA::execute_with(|| {
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(6000f64),
        ));
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
        assert_ok!(LiquidStaking::unbond(
            RuntimeOrigin::signed(ALICE),
            derivative_index,
            ksm(2f64)
        ));
    });

    Relay::execute_with(|| {
        RelaySystem::assert_has_event(RelayEvent::Staking(RelayStakingEvent::Bonded {
            stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
            amount: ksm(5f64),
        }));
        RelaySystem::assert_has_event(RelayEvent::Staking(RelayStakingEvent::Unbonded {
            stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
            amount: ksm(2f64),
        }));
        let ledger = RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(
            derivative_index,
        ))
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
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(6000f64),
        ));
        assert_ok!(LiquidStaking::unstake(
            RuntimeOrigin::signed(ALICE),
            ksm(2000f64),
            Default::default()
        ));
        let bond_amount = ksm(5f64);
        let unbond_amount = ksm(2f64);
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
        let ledger = RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(
            derivative_index,
        ))
        .unwrap();
        assert_eq!(ledger.total, ksm(5f64));
        assert_eq!(ledger.active, ksm(3f64));
        assert_eq!(ledger.unlocking.len(), 1);

        RelaySystem::assert_has_event(RelayEvent::Staking(RelayStakingEvent::Bonded {
            stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
            amount: ksm(5f64),
        }));
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

        assert_ok!(LiquidStaking::withdraw_unbonded(
            RuntimeOrigin::root(),
            derivative_index,
            0
        ));
    });

    Relay::execute_with(|| {
        let ledger = RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(
            derivative_index,
        ))
        .unwrap();
        assert_eq!(ledger.total, ksm(3f64));
        assert_eq!(ledger.active, ksm(3f64));
        assert_eq!(ledger.unlocking.len(), 0);
    });
}

#[test]
fn test_transact_rebond_work() {
    TestNet::reset();
    let derivative_index = 0u16;
    ParaA::execute_with(|| {
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(6000f64),
        ));
        assert_ok!(LiquidStaking::unstake(
            RuntimeOrigin::signed(ALICE),
            ksm(1000f64),
            Default::default()
        ));
        let bond_amount = ksm(10f64);
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
        assert_ok!(LiquidStaking::unbond(
            RuntimeOrigin::signed(ALICE),
            derivative_index,
            ksm(5f64)
        ));
        assert_ok!(LiquidStaking::rebond(
            RuntimeOrigin::signed(ALICE),
            derivative_index,
            ksm(3f64)
        ));
    });

    Relay::execute_with(|| {
        RelaySystem::assert_has_event(RelayEvent::Staking(RelayStakingEvent::Bonded {
            stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
            amount: ksm(10f64),
        }));
        RelaySystem::assert_has_event(RelayEvent::Staking(RelayStakingEvent::Unbonded {
            stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
            amount: ksm(5f64),
        }));
        RelaySystem::assert_has_event(RelayEvent::Staking(RelayStakingEvent::Bonded {
            stash: LiquidStaking::derivative_sovereign_account_id(derivative_index),
            amount: ksm(3f64),
        }));
        let ledger = RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(
            derivative_index,
        ))
        .unwrap();
        assert_eq!(ledger.total, ksm(10f64));
        assert_eq!(ledger.active, ksm(8f64));
    });
}

#[test]
fn test_transact_nominate_work() {
    TestNet::reset();
    let derivative_index = 0u16;
    ParaA::execute_with(|| {
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(4000f64),
        ));
        let bond_amount = ksm(10f64);
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

        assert_ok!(LiquidStaking::nominate(
            RuntimeOrigin::signed(ALICE),
            derivative_index,
            vec![ALICE, BOB],
        ));
    });

    Relay::execute_with(|| {
        let ledger = RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(
            derivative_index,
        ))
        .unwrap();
        assert_eq!(ledger.total, ksm(10f64));
        let nominators = RelayStaking::nominators(LiquidStaking::derivative_sovereign_account_id(
            derivative_index,
        ))
        .unwrap();
        assert_eq!(nominators.targets, vec![ALICE, BOB]);
    });
}

#[test]
fn test_transfer_bond() {
    TestNet::reset();
    let xcm_transfer_amount = ksm(10f64);
    let derivative_index = 0u16;
    ParaA::execute_with(|| {
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(2000f64),
        ));
        assert_ok!(LiquidStaking::bond(
            RuntimeOrigin::signed(ALICE),
            derivative_index,
            xcm_transfer_amount,
            RewardDestination::Staked
        ));
        // print_events::<Test>("ParaA");
    });
    Relay::execute_with(|| {
        // print_events::<kusama_runtime::Runtime>("Relay");
        let ledger = RelayStaking::ledger(LiquidStaking::derivative_sovereign_account_id(
            derivative_index,
        ))
        .unwrap();
        assert_eq!(ledger.total, xcm_transfer_amount);
        assert_eq!(
            RelayBalances::free_balance(LiquidStaking::derivative_sovereign_account_id(
                derivative_index
            )),
            xcm_transfer_amount
        );
        assert_eq!(
            RelayBalances::usable_balance(LiquidStaking::derivative_sovereign_account_id(
                derivative_index
            )),
            0
        );
    });
}

#[test]
fn update_staking_ledger_cap_should_not_work_if_with_invalid_param() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            LiquidStaking::update_staking_ledger_cap(RuntimeOrigin::root(), Zero::zero()),
            Error::<Test>::InvalidCap
        );
    })
}

#[test]
fn update_reserve_factor_should_not_work_if_with_invalid_param() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            LiquidStaking::update_reserve_factor(RuntimeOrigin::root(), Ratio::zero()),
            Error::<Test>::InvalidFactor
        );
        assert_noop!(
            LiquidStaking::update_reserve_factor(RuntimeOrigin::root(), Ratio::one()),
            Error::<Test>::InvalidFactor
        );
    })
}

#[test]
fn claim_for_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(10f64)
        ));
        assert_eq!(<Test as Config>::Assets::balance(KSM, &ALICE), ksm(90f64));

        assert_ok!(LiquidStaking::unstake(
            RuntimeOrigin::signed(ALICE),
            ksm(1f64),
            Default::default()
        ));
        assert_ok!(LiquidStaking::unstake(
            RuntimeOrigin::signed(ALICE),
            ksm(3.95f64),
            Default::default()
        ));
        assert_eq!(
            Unlockings::<Test>::get(ALICE).unwrap(),
            vec![UnlockChunk {
                value: ksm(4.95f64),
                era: 4
            },]
        );

        assert_noop!(
            LiquidStaking::claim_for(RuntimeOrigin::signed(BOB), Id(ALICE)),
            Error::<Test>::NothingToClaim
        );

        let derivative_index = 0u16;
        assert_ok!(with_transaction(
            || -> TransactionOutcome<DispatchResult> {
                assert_ok!(LiquidStaking::do_advance_era(4));
                assert_ok!(LiquidStaking::do_matching());
                TransactionOutcome::Commit(Ok(()))
            }
        ));
        assert_ok!(LiquidStaking::notification_received(
            pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
            0,
            Response::ExecutionResult(None),
        ));
        assert_ok!(LiquidStaking::withdraw_unbonded(
            RuntimeOrigin::root(),
            derivative_index,
            0
        ));
        assert_ok!(LiquidStaking::notification_received(
            pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
            1,
            Response::ExecutionResult(None),
        ));

        assert_ok!(LiquidStaking::claim_for(
            RuntimeOrigin::signed(BOB),
            Id(ALICE)
        ));
        assert_eq!(
            <Test as Config>::Assets::balance(KSM, &ALICE),
            ksm(90f64) + ksm(4.95f64)
        );

        assert!(Unlockings::<Test>::get(ALICE).is_none());
    })
}

#[test]
fn test_on_initialize_work() {
    new_test_ext().execute_with(|| {
        let derivative_index = 0u16;
        let xcm_fees = XcmFees::get();
        let reserve_factor = LiquidStaking::reserve_factor();

        // 1.1 stake
        let bond_amount = ksm(10f64);
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            bond_amount
        ));
        let total_stake_amount = bond_amount - xcm_fees - reserve_factor.mul_floor(bond_amount);

        // 1.2 on_initialize_bond
        let total_era_blocknumbers = <Test as Config>::EraLength::get();
        assert_eq!(total_era_blocknumbers, 10);
        RelayChainValidationDataProvider::set(total_era_blocknumbers);
        LiquidStaking::on_initialize(System::block_number());
        assert_eq!(EraStartBlock::<Test>::get(), total_era_blocknumbers);
        assert_eq!(CurrentEra::<Test>::get(), 1);
        assert_eq!(LiquidStaking::staking_ledger(derivative_index), None);
        assert_eq!(
            LiquidStaking::matching_pool(),
            MatchingLedger {
                total_stake_amount: ReservableAmount {
                    total: total_stake_amount,
                    reserved: total_stake_amount
                },
                total_unstake_amount: Default::default(),
            }
        );

        // 1.3 notification_received bond
        assert_ok!(LiquidStaking::notification_received(
            pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
            0,
            Response::ExecutionResult(None),
        ));

        let staking_ledger = <StakingLedger<AccountId, BalanceOf<Test>>>::new(
            LiquidStaking::derivative_sovereign_account_id(derivative_index),
            total_stake_amount,
        );
        assert_eq!(
            LiquidStaking::staking_ledger(derivative_index).unwrap(),
            staking_ledger
        );

        assert_eq!(LiquidStaking::matching_pool(), MatchingLedger::default());
    })
}

#[test]
fn test_set_staking_ledger_work() {
    new_test_ext().execute_with(|| {
        let derivative_index = 0u16;
        let bond_amount = 100;
        let bond_extra_amount = 50;
        let mut staking_ledger = <StakingLedger<AccountId, BalanceOf<Test>>>::new(
            LiquidStaking::derivative_sovereign_account_id(derivative_index),
            bond_amount,
        );
        assert_noop!(
            LiquidStaking::set_staking_ledger(
                RuntimeOrigin::signed(ALICE),
                derivative_index,
                staking_ledger.clone(),
                get_mock_proof_bytes()
            ),
            Error::<Test>::NotBonded
        );
        StakingLedgers::<Test>::insert(derivative_index, staking_ledger.clone());
        assert_eq!(
            LiquidStaking::staking_ledger(derivative_index).unwrap(),
            staking_ledger.clone()
        );
        staking_ledger.bond_extra(bond_extra_amount);
        assert_noop!(
            LiquidStaking::set_staking_ledger(
                RuntimeOrigin::signed(ALICE),
                derivative_index,
                staking_ledger.clone(),
                get_mock_proof_bytes()
            ),
            Error::<Test>::InvalidProof
        );
        LiquidStaking::on_finalize(1);
        assert_ok!(LiquidStaking::set_staking_ledger(
            RuntimeOrigin::signed(ALICE),
            derivative_index,
            get_mock_staking_ledger(derivative_index),
            get_mock_proof_bytes()
        ));

        assert_noop!(
            LiquidStaking::set_staking_ledger(
                RuntimeOrigin::signed(ALICE),
                derivative_index,
                staking_ledger.clone(),
                get_mock_proof_bytes()
            ),
            Error::<Test>::StakingLedgerLocked
        );

        LiquidStaking::on_finalize(1);
        assert_eq!(
            LiquidStaking::staking_ledger(derivative_index)
                .unwrap()
                .total,
            MOCK_LEDGER_AMOUNT
        );
    })
}

#[test]
fn test_force_set_era_start_block_work() {
    new_test_ext().execute_with(|| {
        assert_eq!(EraStartBlock::<Test>::get(), 0);
        assert_ok!(LiquidStaking::force_set_era_start_block(
            RuntimeOrigin::root(),
            11
        ));
        assert_eq!(EraStartBlock::<Test>::get(), 11);
    })
}

#[test]
fn test_force_set_current_era_work() {
    new_test_ext().execute_with(|| {
        assert_eq!(CurrentEra::<Test>::get(), 0);
        assert_ok!(LiquidStaking::force_set_current_era(
            RuntimeOrigin::root(),
            12
        ));
        assert_eq!(CurrentEra::<Test>::get(), 12);
    })
}

#[test]
fn test_force_notification_received_work() {
    new_test_ext().execute_with(|| {
        let derivative_index = 0u16;
        let bond_amount = ksm(10f64);
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(20f64),
        ));

        assert_ok!(LiquidStaking::bond(
            RuntimeOrigin::signed(ALICE),
            derivative_index,
            bond_amount,
            RewardDestination::Staked
        ));

        let query_id = 0;
        assert_eq!(
            XcmRequests::<Test>::get(query_id),
            Some(XcmRequest::Bond {
                index: derivative_index,
                amount: bond_amount,
            })
        );
        assert_noop!(
            LiquidStaking::notification_received(
                RuntimeOrigin::signed(ALICE),
                query_id,
                Response::ExecutionResult(None),
            ),
            BadOrigin
        );
        assert_ok!(LiquidStaking::notification_received(
            RuntimeOrigin::root(),
            query_id,
            Response::ExecutionResult(None),
        ));
        assert_eq!(XcmRequests::<Test>::get(query_id), None);
    })
}

#[test]
fn test_storage_proof_approach_should_work() {
    let relay_root = sp_core::hash::H256::from_slice(&hex::decode(ROOT_HASH).unwrap());
    let key = hex::decode(MOCK_KEY).unwrap();
    let value = hex::decode(MOCK_DATA).unwrap();
    let relay_proof = StorageProof::new(get_mock_proof_bytes());
    let result = sp_state_machine::read_proof_check::<BlakeTwo256, _>(
        relay_root,
        relay_proof.clone(),
        [key.clone()],
    )
    .unwrap();
    assert_eq!(
        result.into_iter().collect::<Vec<_>>(),
        vec![(key, Some(value))],
    );
}

#[test]
fn test_verify_trie_proof_work() {
    type LayoutV1 = sp_trie::LayoutV1<BlakeTwo256>;
    let relay_root = sp_core::hash::H256::from_slice(&hex::decode(ROOT_HASH).unwrap());
    let key = hex::decode(MOCK_KEY).unwrap();
    let value = hex::decode(MOCK_DATA).unwrap();
    let relay_proof = StorageProof::new(get_mock_proof_bytes());
    let db = relay_proof.into_memory_db();
    let result = sp_trie::read_trie_value::<LayoutV1, _>(&db, &relay_root, &key, None, None)
        .unwrap()
        .unwrap();
    assert_eq!(result, value);
}

#[test]
fn test_verify_merkle_proof_work() {
    new_test_ext().execute_with(|| {
        use codec::Encode;
        let derivative_index = 0u16;
        let staking_ledger = get_mock_staking_ledger(derivative_index);
        let key = LiquidStaking::get_staking_ledger_key(derivative_index);
        let value = staking_ledger.encode();
        assert_eq!(hex::encode(&value), MOCK_DATA);
        LiquidStaking::on_finalize(1);
        assert!(LiquidStaking::verify_merkle_proof(
            key,
            value,
            get_mock_proof_bytes()
        ));
    })
}

#[test]
fn reduce_reserves_works() {
    new_test_ext().execute_with(|| {
        // Stake 1000 KSM, 0.5% for reserves
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(100f64)
        ));
        assert_eq!(LiquidStaking::total_reserves(), ksm(0.5f64));
        // Reduce 20 KSM reserves
        assert_ok!(LiquidStaking::reduce_reserves(
            RuntimeOrigin::root(),
            Id(ALICE),
            ksm(0.2f64)
        ));
        assert_eq!(LiquidStaking::total_reserves(), ksm(0.3f64));

        // should failed if exceed the cap
        assert_noop!(
            LiquidStaking::reduce_reserves(RuntimeOrigin::root(), Id(ALICE), ksm(0.31f64)),
            Underflow
        );
    })
}

#[test]
fn cancel_unstake_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(10f64)
        ));
        assert_ok!(LiquidStaking::unstake(
            RuntimeOrigin::signed(ALICE),
            ksm(6f64),
            UnstakeProvider::MatchingPool
        ));

        assert_eq!(LiquidStaking::fast_unstake_requests(&ALICE), ksm(6f64));

        // Check storage is correct
        assert_eq!(ExchangeRate::<Test>::get(), Rate::one());
        assert_eq!(
            MatchingPool::<Test>::get(),
            MatchingLedger {
                total_stake_amount: ReservableAmount {
                    total: ksm(9.95f64),
                    reserved: 0
                },
                total_unstake_amount: ReservableAmount {
                    total: 0,
                    reserved: 0
                }
            }
        );

        assert_ok!(LiquidStaking::cancel_unstake(
            RuntimeOrigin::signed(ALICE),
            ksm(6f64)
        ));
        assert_eq!(
            MatchingPool::<Test>::get(),
            MatchingLedger {
                total_stake_amount: ReservableAmount {
                    total: ksm(9.95f64),
                    reserved: 0
                },
                total_unstake_amount: ReservableAmount {
                    total: 0,
                    reserved: 0
                }
            }
        );

        assert_eq!(LiquidStaking::fast_unstake_requests(&ALICE), 0);
    })
}

#[test]
fn fast_unstake_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            ksm(10f64)
        ));
        assert_ok!(Loans::mint(RuntimeOrigin::signed(BOB), KSM, ksm(100f64)));
        assert_ok!(Loans::collateral_asset(
            RuntimeOrigin::signed(BOB),
            KSM,
            true
        ));
        assert_ok!(LiquidStaking::unstake(
            RuntimeOrigin::signed(ALICE),
            ksm(6f64),
            UnstakeProvider::Loans
        ));
        assert_eq!(
            Unlockings::<Test>::get(LiquidStaking::loans_account_id()).unwrap(),
            vec![UnlockChunk {
                value: ksm(6f64),
                era: 4
            },]
        );
        // 90 * 1e12 + (6 * (1 - 8/1000) * 1e12)
        assert_eq!(
            <Test as Config>::Assets::balance(KSM, &ALICE),
            95952000000000u128
        );

        let derivative_index = 0u16;
        assert_ok!(with_transaction(
            || -> TransactionOutcome<DispatchResult> {
                assert_ok!(LiquidStaking::do_matching());
                assert_ok!(LiquidStaking::do_advance_era(4));
                TransactionOutcome::Commit(Ok(()))
            }
        ));
        assert_ok!(LiquidStaking::notification_received(
            pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
            0,
            Response::ExecutionResult(None),
        ));
        assert_ok!(LiquidStaking::withdraw_unbonded(
            RuntimeOrigin::root(),
            derivative_index,
            0
        ));
        assert_ok!(LiquidStaking::notification_received(
            pallet_xcm::Origin::Response(MultiLocation::parent()).into(),
            1,
            Response::ExecutionResult(None),
        ));

        assert_ok!(LiquidStaking::claim_for(
            RuntimeOrigin::signed(BOB),
            Id(LiquidStaking::loans_account_id())
        ));
        assert_eq!(
            Unlockings::<Test>::get(LiquidStaking::loans_account_id()),
            None
        );
    })
}

#[test]
fn test_charge_commission_work() {
    new_test_ext().execute_with(|| {
        let derivative_index = 0u16;
        let bond_amount = ksm(200f64);
        let staking_ledger = <StakingLedger<AccountId, BalanceOf<Test>>>::new(
            LiquidStaking::derivative_sovereign_account_id(derivative_index),
            bond_amount,
        );
        StakingLedgers::<Test>::insert(derivative_index, staking_ledger.clone());
        assert_ok!(LiquidStaking::update_commission_rate(
            RuntimeOrigin::root(),
            Rate::from_rational(1, 100)
        ));
        LiquidStaking::on_finalize(1);

        // liquid_amount_to_fee=TotalLiquidCurrency * (commission_rate*total_rewards/(TotalStakeCurrency+(1-commission_rate)*total_rewards))
        let commission_rate = CommissionRate::<Test>::get();
        let total_rewards = MOCK_LEDGER_AMOUNT - bond_amount;
        let commission_staking_amount = commission_rate.saturating_mul_int(total_rewards);
        let issurance = <Test as Config>::Assets::total_issuance(SKSM);
        let matching_ledger = LiquidStaking::matching_pool();
        let total_active_bonded: u128 = StakingLedgers::<Test>::iter_values()
            .fold(Zero::zero(), |acc, ledger| {
                acc.saturating_add(ledger.active)
            });
        let total_bonded = total_active_bonded + matching_ledger.total_stake_amount.total
            - matching_ledger.total_unstake_amount.total;
        let inflate_rate = Rate::checked_from_rational(
            commission_staking_amount,
            total_bonded + total_rewards - commission_staking_amount,
        )
        .unwrap();

        let inflate_liquid_amount = inflate_rate.saturating_mul_int(issurance);

        assert_ok!(LiquidStaking::set_staking_ledger(
            RuntimeOrigin::signed(ALICE),
            derivative_index,
            get_mock_staking_ledger(derivative_index),
            get_mock_proof_bytes()
        ));

        assert_eq!(
            LiquidStaking::staking_ledger(derivative_index)
                .unwrap()
                .total,
            MOCK_LEDGER_AMOUNT
        );

        assert_eq!(
            <Test as Config>::Assets::balance(SKSM, &DefaultProtocolFeeReceiver::get()),
            inflate_liquid_amount
        )
    })
}

#[test]
fn test_complete_fast_match_unstake_work() {
    new_test_ext().execute_with(|| {
        let reserve_factor = LiquidStaking::reserve_factor();
        let xcm_fees = XcmFees::get();
        let bond_amount = ksm(10f64);
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(BOB),
            bond_amount
        ));
        let total_stake_amount = bond_amount - xcm_fees - reserve_factor.mul_floor(bond_amount);

        let fast_unstake_amount = ksm(3f64);
        assert_ok!(LiquidStaking::unstake(
            RuntimeOrigin::signed(BOB),
            fast_unstake_amount,
            UnstakeProvider::MatchingPool
        ));
        assert_ok!(LiquidStaking::fast_match_unstake(
            RuntimeOrigin::signed(BOB),
            [BOB].to_vec(),
        ));

        assert_eq!(
            <Test as Config>::Assets::balance(SKSM, &DefaultProtocolFeeReceiver::get()),
            MatchingPoolFastUnstakeFee::get().saturating_mul_int(fast_unstake_amount)
        );

        assert_eq!(
            <Test as Config>::Assets::balance(SKSM, &BOB),
            total_stake_amount - fast_unstake_amount
        );
        let pool_stake_amount = total_stake_amount
            - Rate::one()
                .saturating_sub(MatchingPoolFastUnstakeFee::get())
                .saturating_mul_int(fast_unstake_amount);
        assert_eq!(
            LiquidStaking::matching_pool(),
            MatchingLedger {
                total_stake_amount: ReservableAmount {
                    total: pool_stake_amount,
                    reserved: 0
                },
                total_unstake_amount: Default::default(),
            }
        );
    })
}

#[test]
fn test_partial_fast_match_unstake_work() {
    new_test_ext().execute_with(|| {
        let reserve_factor = LiquidStaking::reserve_factor();
        let xcm_fees = XcmFees::get();
        let bond_amount = ksm(5f64);
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(ALICE),
            bond_amount
        ));
        assert_ok!(LiquidStaking::stake(
            RuntimeOrigin::signed(BOB),
            bond_amount
        ));

        let alice_stake_amount = bond_amount - xcm_fees - reserve_factor.mul_floor(bond_amount);
        let bob_stake_amount = alice_stake_amount;

        // default exchange_rate is 1
        let alice_fast_unstake_amount = ksm(10f64);
        let bob_fast_unstake_amount = ksm(1f64);
        assert_ok!(LiquidStaking::unstake(
            RuntimeOrigin::signed(ALICE),
            alice_fast_unstake_amount,
            UnstakeProvider::MatchingPool
        ));
        assert_ok!(LiquidStaking::unstake(
            RuntimeOrigin::signed(BOB),
            bob_fast_unstake_amount,
            UnstakeProvider::MatchingPool
        ));
        assert_ok!(LiquidStaking::fast_match_unstake(
            RuntimeOrigin::signed(BOB),
            [BOB, ALICE].to_vec(),
        ));

        assert_eq!(
            <Test as Config>::Assets::balance(SKSM, &BOB),
            bob_stake_amount - bob_fast_unstake_amount
        );

        let bob_matched_amount = Rate::one()
            .saturating_sub(MatchingPoolFastUnstakeFee::get())
            .saturating_mul_int(bob_fast_unstake_amount);

        let available_amount = (alice_stake_amount + bob_stake_amount - bob_matched_amount)
            .min(alice_fast_unstake_amount);
        let alice_matched_amount = Rate::one()
            .saturating_sub(MatchingPoolFastUnstakeFee::get())
            .saturating_mul_int(available_amount);

        // mint in mock
        let alice_initial_amount = ksm(100f64);
        assert_eq!(
            <Test as Config>::Assets::balance(SKSM, &ALICE),
            alice_initial_amount + alice_stake_amount - available_amount
        );

        assert_eq!(
            LiquidStaking::matching_pool(),
            MatchingLedger {
                total_stake_amount: ReservableAmount {
                    total: alice_stake_amount + bob_stake_amount
                        - bob_matched_amount
                        - alice_matched_amount,
                    reserved: 0
                },
                total_unstake_amount: Default::default(),
            }
        );
        assert_eq!(
            LiquidStaking::fast_unstake_requests(&ALICE),
            alice_fast_unstake_amount - available_amount
        );

        assert_ok!(with_transaction(
            || -> TransactionOutcome<DispatchResult> {
                assert_ok!(LiquidStaking::do_matching());
                TransactionOutcome::Commit(Ok(()))
            }
        ));
    })
}
