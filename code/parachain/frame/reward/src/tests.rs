/// Tests for Reward
use crate::mock::*;
use frame_support::{assert_err, assert_ok};
use rand::Rng;

// type Event = crate::Event<Test>;

use crate::mock::CurrencyId;

const PICA : CurrencyId = 1;
const LP : CurrencyId = 10000;
const KSM : CurrencyId = 4;
const USDT : CurrencyId = 140;

macro_rules! fixed {
    ($amount:expr) => {
        sp_arithmetic::FixedI128::from($amount)
    };
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn reproduce_live_state() {
    // This function is most useful for debugging. Keeping this test here for convenience
    // and to function as an additional regression test
    run_test(|| {
        let f = |x: i128| SignedFixedPoint::from_inner(x);
        let currency = PICA;

        // state for a3eFe9M2HbAgrQrShEDH2CEvXACtzLhSf4JGkwuT9SQ1EV4ti at block 0xb47ed0e773e25c81da2cc606495ab6f716c3c2024f9beb361605860912fee652
        crate::RewardPerToken::<Test>::insert(currency, (), f(1_699_249_738_518_636_122_154_288_694));
        crate::RewardTally::<Test>::insert(currency, ((), ALICE), f(164_605_943_476_265_834_062_592_062_507_811_208));
        crate::Stake::<Test>::insert(((), ALICE), f(97_679_889_000_000_000_000_000_000));
        crate::TotalRewards::<Test>::insert(currency, f(8_763_982_459_262_268_000_000_000_000_000_000));
        crate::TotalStake::<Test>::insert((), f(2_253_803_217_000_000_000_000_000_000));

        assert_ok!(Reward::compute_reward(&(), &ALICE, currency), 1376582365513566);
    })
}

#[test]
fn should_distribute_rewards_equally() {
    run_test(|| {
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(50)));
        assert_ok!(Reward::deposit_stake(&(), &BOB, fixed!(50)));
        assert_ok!(Reward::distribute_reward(&(), LP, fixed!(100)));
        assert_ok!(Reward::compute_reward(&(), &ALICE, LP), 50);
        assert_ok!(Reward::compute_reward(&(), &BOB, LP), 50);
    })
}

#[test]
fn should_distribute_uneven_rewards_equally() {
    run_test(|| {
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(50)));
        assert_ok!(Reward::deposit_stake(&(), &BOB, fixed!(50)));
        assert_ok!(Reward::distribute_reward(&(), LP, fixed!(451)));
        assert_ok!(Reward::compute_reward(&(), &ALICE, LP), 225);
        assert_ok!(Reward::compute_reward(&(), &BOB, LP), 225);
    })
}

#[test]
fn should_not_update_previous_rewards() {
    run_test(|| {
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(40)));
        assert_ok!(Reward::distribute_reward(&(), LP, fixed!(1000)));
        assert_ok!(Reward::compute_reward(&(), &ALICE, LP), 1000);

        assert_ok!(Reward::deposit_stake(&(), &BOB, fixed!(20)));
        assert_ok!(Reward::compute_reward(&(), &ALICE, LP), 1000);
        assert_ok!(Reward::compute_reward(&(), &BOB, LP), 0);
    })
}

#[test]
fn should_withdraw_reward() {
    run_test(|| {
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(45)));
        assert_ok!(Reward::deposit_stake(&(), &BOB, fixed!(55)));
        assert_ok!(Reward::distribute_reward(&(), LP, fixed!(2344)));
        assert_ok!(Reward::compute_reward(&(), &BOB, LP), 1289);
        assert_ok!(Reward::withdraw_reward(&(), &ALICE, LP), 1054);
        assert_ok!(Reward::compute_reward(&(), &BOB, LP), 1289);
    })
}

#[test]
fn should_withdraw_stake() {
    run_test(|| {
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(1312)));
        assert_ok!(Reward::distribute_reward(&(), LP, fixed!(4242)));
        // rounding in `CheckedDiv` loses some precision
        assert_ok!(Reward::compute_reward(&(), &ALICE, LP), 4241);
        assert_ok!(Reward::withdraw_stake(&(), &ALICE, fixed!(1312)));
        assert_ok!(Reward::compute_reward(&(), &ALICE, LP), 4241);
    })
}

#[test]
fn should_not_withdraw_stake_if_balance_insufficient() {
    run_test(|| {
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(100)));
        assert_ok!(Reward::distribute_reward(&(), LP, fixed!(2000)));
        assert_ok!(Reward::compute_reward(&(), &ALICE, LP), 2000);
        assert_err!(
            Reward::withdraw_stake(&(), &ALICE, fixed!(200)),
            TestError::InsufficientFunds
        );
    })
}

#[test]
fn should_deposit_stake() {
    run_test(|| {
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(25)));
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(25)));
        assert_eq!(Reward::stake(&(), &ALICE), fixed!(50));
        assert_ok!(Reward::deposit_stake(&(), &BOB, fixed!(50)));
        assert_ok!(Reward::distribute_reward(&(), LP, fixed!(1000)));
        assert_ok!(Reward::compute_reward(&(), &ALICE, LP), 500);
    })
}

#[test]
fn should_not_distribute_rewards_without_stake() {
    run_test(|| {
        assert_err!(
            Reward::distribute_reward(&(), LP, fixed!(1000)),
            TestError::ZeroTotalStake
        );
        assert_eq!(Reward::total_rewards(LP), fixed!(0));
    })
}

#[test]
fn should_distribute_with_many_rewards() {
    // test that reward tally doesn't overflow
    run_test(|| {
        let mut rng = rand::thread_rng();
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(9230404)));
        assert_ok!(Reward::deposit_stake(&(), &BOB, fixed!(234234444)));
        for _ in 0..30 {
            // NOTE: this will overflow compute_reward with > u32
            assert_ok!(Reward::distribute_reward(
                &(),
                LP,
                fixed!(rng.gen::<u32>() as i128)
            ));
        }
        let alice_reward = Reward::compute_reward(&(), &ALICE, LP).unwrap();
        assert_ok!(Reward::withdraw_reward(&(), &ALICE, LP), alice_reward);
        let bob_reward = Reward::compute_reward(&(), &BOB, LP).unwrap();
        assert_ok!(Reward::withdraw_reward(&(), &BOB, LP), bob_reward);
    })
}

macro_rules! assert_approx_eq {
    ($left:expr, $right:expr, $delta:expr) => {
        assert!(if $left > $right { $left - $right } else { $right - $left } <= $delta)
    };
}

#[test]
fn should_distribute_with_different_rewards() {
    run_test(|| {
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(100)));
        assert_ok!(Reward::distribute_reward(&(), LP, fixed!(1000)));
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(100)));
        assert_ok!(Reward::distribute_reward(&(), PICA, fixed!(1000)));
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(100)));
        assert_ok!(Reward::distribute_reward(&(), KSM, fixed!(1000)));
        assert_ok!(Reward::deposit_stake(&(), &ALICE, fixed!(100)));
        assert_ok!(Reward::distribute_reward(&(), USDT, fixed!(1000)));

        assert_ok!(Reward::withdraw_stake(&(), &ALICE, fixed!(300)));

        assert_approx_eq!(Reward::compute_reward(&(), &ALICE, LP).unwrap(), 1000, 1);
        assert_approx_eq!(Reward::compute_reward(&(), &ALICE, PICA).unwrap(), 1000, 1);
        assert_approx_eq!(Reward::compute_reward(&(), &ALICE, KSM).unwrap(), 1000, 1);
        assert_approx_eq!(Reward::compute_reward(&(), &ALICE, USDT).unwrap(), 1000, 1);
    })
}
