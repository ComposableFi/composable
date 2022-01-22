/// TODO
///
/// 1. Test each extrinsic
/// 2. Make sure unlocks etc are respected (timing)
/// 3. Add tests for linear decay.
///
///
/// grouping tests
///
/// test every failure case
/// every error that an extrinsic can return
///
/// all the happy path cases
///
///
/// interaction logic between extrinsics
/// such as:
///  transfer_to -> waiting for a block (til lock_time expires) -> claiming
///         check if the funds are correctly moved to the user's account
///
///  transfer_to -> waiting for a block -> relayer accepts transfer -> (til lock_time expires) -> we should no longer be able to claim
///
///  incoming -> waiting til lock_time expires -> claiming
///
///  incoming -> wainting for a block -> relayer cancels transfer (finality issue) -> we should no longer be able to claim
///
///
///  For every test, make sure that you check wether the funds moved to the correct (sub) accounts.
///
///
///
use crate::{decay::*, mock::*, *};
use frame_support::{
	assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use sp_runtime::DispatchError;

pub trait OriginExt {
	fn relayer() -> Origin {
		Origin::signed(RELAYER)
	}

	fn alice() -> Origin {
		Origin::signed(ALICE)
	}
}

impl OriginExt for Origin {}


mod set_relayer {
    use super::*;

    #[test]
    fn set_relayer() {
        new_test_ext().execute_with(|| {
            assert_ok!(Mosaic::set_relayer(Origin::root(), RELAYER));
            assert_eq!(Mosaic::relayer_account_id(), Some(RELAYER));
        })
    }

    #[test]
    fn relayer_cannot_set_relayer() {
        new_test_ext().execute_with(|| {
            assert_noop!(Mosaic::set_relayer(Origin::relayer(), ALICE), DispatchError::BadOrigin);
        })
    }

    #[test]
    fn none_cannot_set_relayer() {
        new_test_ext().execute_with(|| {
            assert_noop!(Mosaic::set_relayer(Origin::none(), ALICE), DispatchError::BadOrigin);
        })
    }

    #[test]
    fn alice_cannot_set_relayer() {
        new_test_ext().execute_with(|| {
            assert_noop!(Mosaic::set_relayer(Origin::signed(ALICE), ALICE), DispatchError::BadOrigin);
        })
    }
}

mod rotate_relayer {
    use super::*;

    #[test]
    fn relayer_can_rotate_relayer() {
        new_test_ext().execute_with(|| {
            let ttl = 500;
            let current_block = System::block_number();
            assert_ok!(Mosaic::set_relayer(Origin::root(), RELAYER));

            // first rotation
            assert_ok!(Mosaic::rotate_relayer(Origin::relayer(), BOB, ttl));
            System::set_block_number(current_block + ttl);
            assert_eq!(Mosaic::relayer_account_id(), Some(BOB));

            // second rotation
            assert_ok!(Mosaic::rotate_relayer(Origin::signed(BOB), CHARLIE, ttl));
            System::set_block_number(current_block + 2 * ttl);
            assert_eq!(Mosaic::relayer_account_id(), Some(CHARLIE));
        })
    }

    #[test]
    fn relayer_must_not_rotate_early() {
        new_test_ext().execute_with(|| {
            let ttl = 500;
            let current_block = System::block_number();
            assert_ok!(Mosaic::set_relayer(Origin::root(), RELAYER));
            assert_ok!(Mosaic::rotate_relayer(Origin::relayer(), BOB, ttl));
            System::set_block_number(current_block + ttl - 1); // just before the ttl
            assert_eq!(Mosaic::relayer_account_id(), Some(RELAYER)); // not BOB
        })
    }

    #[test]
    fn arbitrary_account_cannot_rotate_relayer() {
        new_test_ext().execute_with(|| {
            let ttl = 500;
            assert_ok!(Mosaic::set_relayer(Origin::root(), RELAYER));
            assert_noop!(
                Mosaic::rotate_relayer(Origin::signed(ALICE), BOB, ttl),
                DispatchError::BadOrigin
            );
        })
    }

    #[test]
    fn none_cannot_rotate_relayer() {
        new_test_ext().execute_with(|| {
            let ttl = 500;
            assert_ok!(Mosaic::set_relayer(Origin::root(), RELAYER));
            assert_noop!(
                Mosaic::rotate_relayer(Origin::none(), BOB, ttl),
                DispatchError::BadOrigin
            );
        })
    }
}

mod set_network {
    use super::*;

    #[test]
    fn relayer_can_set_network() {
        let network_id = 3;
        let network_info = 	NetworkInfo { enabled: false, max_transfer_size: 100000 };
        new_test_ext().execute_with(|| {
            assert_ok!(Mosaic::set_relayer(Origin::root(), RELAYER));

            assert_ok!(Mosaic::set_network(Origin::relayer(), network_id, network_info.clone()));
            assert_eq!(Mosaic::network_infos(network_id), Some(network_info));
        })
    }

    #[test]
    fn root_cannot_set_network() {
        let network_id = 3;
        let network_info = 	NetworkInfo { enabled: false, max_transfer_size: 100000 };
        new_test_ext().execute_with(|| {
            assert_ok!(Mosaic::set_relayer(Origin::root(), RELAYER));

            assert_noop!(Mosaic::set_network(Origin::root(), network_id, network_info.clone()), DispatchError::BadOrigin);
        })
    }

    #[test]
    fn none_cannot_set_network() {
        let network_id = 3;
        let network_info = 	NetworkInfo { enabled: false, max_transfer_size: 100000 };
        new_test_ext().execute_with(|| {
            assert_ok!(Mosaic::set_relayer(Origin::root(), RELAYER));

            assert_noop!(Mosaic::set_network(Origin::none(), network_id, network_info.clone()), DispatchError::BadOrigin);
        })
    }

}





mod budget {
    use super::*;

    mod set_budget {
        use super::*;

        #[test]
        fn root_can_set_budget() {
            new_test_ext().execute_with(|| {
                assert_ok!(Mosaic::set_budget(Origin::root(), 1, 1, BudgetDecay::linear(5)));
            })
        }

        #[test]
        fn arbitrary_user_cannot_set_budget() {
            new_test_ext().execute_with(|| {
                assert_noop!(
                Mosaic::set_budget(Origin::signed(ALICE), 1, 1, BudgetDecay::linear(5)),
                DispatchError::BadOrigin);
            })
        }

        #[test]
        fn none_cannot_set_budget() {
            new_test_ext().execute_with(|| {
                assert_noop!(
                Mosaic::set_budget(Origin::none(), 1, 1, BudgetDecay::linear(5)),
                DispatchError::BadOrigin);
            })
        }
    }

    #[test]
    fn budget_are_isolated() {
        new_test_ext().execute_with(|| {
            assert_ok!(Mosaic::set_budget(Origin::root(), 1, 0xCAFEBABE, BudgetDecay::linear(10)));
            assert_ok!(Mosaic::set_budget(Origin::root(), 2, 0xDEADC0DE, BudgetDecay::linear(5)));
            assert_eq!(Mosaic::asset_infos(1).expect("budget must exists").budget, 0xCAFEBABE);
            assert_eq!(Mosaic::asset_infos(2).expect("budget must exists").budget, 0xDEADC0DE);
        })
    }


    #[test]
    fn last_deposit_does_not_change_after_updating_budget() {
        new_test_ext().execute_with(|| {
            let initial_block = System::block_number();
            assert_ok!(Mosaic::set_budget(Origin::root(), 1, 0xCAFEBABE, BudgetDecay::linear(10)));
            assert_eq!(Mosaic::asset_infos(1).expect("budget must exists").last_deposit, initial_block);

            System::set_block_number(initial_block + 1);
            assert_ok!(Mosaic::set_budget(Origin::root(), 1, 0xDEADC0DE, BudgetDecay::linear(10)));
            assert_eq!(Mosaic::asset_infos(1).expect("budget must exists").last_deposit, initial_block);
        })
    }
}



#[test]
fn incoming_outgoing_accounts_are_isolated() {
	ExtBuilder { balances: Default::default() }.build().execute_with(|| {
		initialize();

		let amount = 100;
		let network_id = 1;
		let asset_id = 1;

		assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));
		let account_balance = || Tokens::balance(asset_id, &ALICE);
		let balance_of = |t| Tokens::balance(asset_id, &Mosaic::sub_account_id(t));
		assert_eq!(account_balance(), amount);
		assert_eq!(balance_of(SubAccount::outgoing(ALICE)), 0);
		assert_eq!(balance_of(SubAccount::incoming(ALICE)), 0);
		assert_ok!(Mosaic::transfer_to(
			Origin::signed(ALICE),
			network_id,
			asset_id,
			[0; 20],
			amount,
			true
		));
		assert_eq!(account_balance(), 0);
		assert_eq!(balance_of(SubAccount::outgoing(ALICE)), amount);
		assert_eq!(balance_of(SubAccount::incoming(ALICE)), 0);
	})
}



fn initialize() {
	System::set_block_number(1);

	Mosaic::set_relayer(Origin::root(), RELAYER).expect("root may call set_relayer");
	Mosaic::set_network(
		Origin::relayer(),
		1,
		NetworkInfo { enabled: true, max_transfer_size: 100000 },
	)
	.expect("relayer may set network info");
	Mosaic::set_budget(Origin::root(), 1, 10000, BudgetDecay::linear(10))
		.expect("root may set budget");
}



fn do_timelocked_mint(to: AccountId, asset_id: AssetId, amount: Balance, lock_time: u64) {
	let initial_block = System::block_number();

	Mosaic::timelocked_mint(Origin::relayer(), asset_id, to, amount, lock_time, Default::default())
		.expect("relayer should be able to mint");

	assert_eq!(
		Mosaic::incoming_transactions(to, asset_id),
		Some((amount, initial_block + lock_time))
	);
}


mod transfers {
    use super::*;

    #[test]
    fn transfer_to() {
        new_test_ext().execute_with(|| {
            initialize();
            do_transfer_to();
        })
    }

    #[test]
    fn accept_transfer() {
        new_test_ext().execute_with(|| {
            initialize();
            do_transfer_to();
            Mosaic::accept_transfer(Origin::relayer(), ALICE, 1, 100)
                .expect("accepting transfer should work");
        })
    }

    #[test]
    fn cannot_accept_transfer_larger_than_balance() {
        new_test_ext().execute_with(|| {
            initialize();
            do_transfer_to();
            assert_noop!(
                Mosaic::accept_transfer(Origin::relayer(), ALICE, 1, 101),
                Error::<Test>::AmountMismatch
            );
        })
    }

    #[test]
    fn claim_stale_to() {
        new_test_ext().execute_with(|| {
            initialize();
            do_transfer_to();
            let current_block = System::block_number();
            System::set_block_number(current_block + Mosaic::timelock_period() + 1);
            Mosaic::claim_stale_to(Origin::signed(ALICE), 1, ALICE)
                .expect("claiming an outgoing transaction should work after the timelock period");
        })
    }

    #[test]
    fn cannot_claim_stale_to_early() {
        new_test_ext().execute_with(|| {
            initialize();
            do_transfer_to();
            let current_block = System::block_number();
            System::set_block_number(current_block + Mosaic::timelock_period() - 1);
            assert_noop!(
                Mosaic::claim_stale_to(Origin::signed(ALICE), 1, ALICE),
                Error::<Test>::NoStaleTransactions
            );
        })
    }

    #[test]
    fn cannot_claim_after_relayer_accepts_transfer() {
        new_test_ext().execute_with(|| {
            initialize();
            do_transfer_to();
            assert_ok!(Mosaic::accept_transfer(Origin::relayer(), ALICE, 1, 100));
            let current_block = System::block_number();
            System::set_block_number(current_block + Mosaic::timelock_period() + 1);
            assert_noop!(
                Mosaic::claim_stale_to(Origin::signed(ALICE), 1, ALICE),
                Error::<Test>::NoStaleTransactions
            );
        })
    }

    #[test]
    fn relayer_cannot_accept_transfer_after_claim() {
        new_test_ext().execute_with(|| {
            initialize();
            do_transfer_to();
            let current_block = System::block_number();
            System::set_block_number(current_block + Mosaic::timelock_period() + 1);
            assert_ok!(Mosaic::claim_stale_to(Origin::signed(ALICE), 1, ALICE));
            assert_noop!(
                Mosaic::accept_transfer(Origin::relayer(), ALICE, 1, 100),
                Error::<Test>::NoOutgoingTx
            );
        })
    }

    #[test]
    fn can_claim_stale_after_partial_accept_transfer() {
        new_test_ext().execute_with(|| {
            initialize();
            do_transfer_to();
            let current_block = System::block_number();
            System::set_block_number(current_block + Mosaic::timelock_period() + 1);
            assert_ok!(Mosaic::accept_transfer(Origin::relayer(), ALICE, 1, 20));
            // System::set_block_number(current_block + Mosaic::timelock_period() + 1);
            assert_ok!(Mosaic::claim_stale_to(Origin::signed(ALICE), 1, ALICE));
        })
    }

    #[test]
    fn transfer_to_exceeds_max_transfer_size() {
        ExtBuilder { balances: Default::default() }.build().execute_with(|| {
            let max_transfer_size = 100000;

            assert_ok!(Mosaic::set_relayer(Origin::root(), RELAYER));

            let network_id = 1;
            assert_ok!(Mosaic::set_network(
                Origin::relayer(),
                network_id,
                NetworkInfo { enabled: true, max_transfer_size },
            ));

            let asset_id = 1;
            assert_ok!(Mosaic::set_budget(Origin::root(), asset_id, 10000, BudgetDecay::linear(10)));

            // We exceed the max transfer size
            let amount = max_transfer_size + 1;
            assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));
            assert_noop!(
                Mosaic::transfer_to(Origin::signed(ALICE), network_id, asset_id, [0; 20], amount, true),
                Error::<Test>::ExceedsMaxTransferSize
            );
        })
    }


    #[test]
    fn transfer_to_move_funds_to_outgoing() {
        ExtBuilder { balances: Default::default() }.build().execute_with(|| {
            initialize();

            let amount = 100;
            let network_id = 1;
            let asset_id = 1;

            assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));
            let account_balance = || Tokens::balance(asset_id, &ALICE);
            let outgoing_balance =
                || Tokens::balance(asset_id, &Mosaic::sub_account_id(SubAccount::outgoing(ALICE)));
            assert_eq!(account_balance(), amount);
            assert_eq!(outgoing_balance(), 0);
            assert_ok!(Mosaic::transfer_to(
			Origin::signed(ALICE),
			network_id,
			asset_id,
			[0; 20],
			amount,
			true
		));
            assert_eq!(account_balance(), 0);
            assert_eq!(outgoing_balance(), amount);
        })
    }

    #[test]
    fn transfer_to_unsupported_asset() {
        ExtBuilder { balances: Default::default() }.build().execute_with(|| {
            assert_ok!(Mosaic::set_relayer(Origin::root(), RELAYER));
            assert_ok!(Mosaic::set_network(
			Origin::relayer(),
			1,
			NetworkInfo { enabled: true, max_transfer_size: 100000 },
		));

            // We don't register the asset

            let amount = 100;
            let network_id = 1;
            let asset_id = 1;

            assert_ok!(Tokens::mint_into(asset_id, &ALICE, amount));
            assert_noop!(
			Mosaic::transfer_to(Origin::signed(ALICE), network_id, asset_id, [0; 20], amount, true),
			Error::<Test>::UnsupportedAsset
		);
        })
    }

    fn do_transfer_to() {
        let ethereum_address = [0; 20];
        let amount = 100;
        let network_id = 1;
        let asset_id = 1;

        Mosaic::transfer_to(
            Origin::signed(ALICE),
            network_id,
            asset_id,
            ethereum_address,
            amount,
            true,
        )
            .expect("transfer_to should work");
        assert_eq!(
            Mosaic::outgoing_transactions(&ALICE, 1),
            Some((100, MinimumTimeLockPeriod::get() + System::block_number()))
        );

        // normally we don't unit test events being emitted, but in this case it is very crucial for the
        // relayer to observe the events.

        // When a transfer is made, the nonce is incremented. However, nonce is one of the dependencies
        // for `generate_id`, we want to check if the events match, so we decrement the nonce and
        // increment it back when we're done
        // TODO: this is a hack, cfr: CU-1ubrf2y
        Nonce::<Test>::mutate(|nonce| {
            *nonce = nonce.wrapping_sub(1);
            *nonce
        });

        let id = generate_id::<Test>(
            &ALICE,
            &network_id,
            &asset_id,
            &ethereum_address,
            &amount,
            &System::block_number(),
        );
        Nonce::<Test>::mutate(|nonce| {
            *nonce = nonce.wrapping_add(1);
            *nonce
        });

        System::assert_last_event(mock::Event::Mosaic(crate::Event::TransferOut {
            id,
            to: ethereum_address,
            amount,
            network_id,
        }));
    }
}

mod timelocked_mint {
    use super::*;

    #[test]
    fn timelocked_mint() {
        new_test_ext().execute_with(|| {
            initialize();
            do_timelocked_mint(ALICE, 1, 50, 10);
        })
    }


    #[test]
    fn cannot_mint_unsupported_assets() {
        new_test_ext().execute_with(|| {
            initialize();
            let unsupported_asset_id = 42;
            assert_noop!(
                Mosaic::timelocked_mint(Origin::relayer(), unsupported_asset_id, ALICE, 50, 10,  Default::default()),
                Error::<Test>::UnsupportedAsset
            );
        })
    }

    #[test]
    fn cannot_mint_more_than_budget() {
        new_test_ext().execute_with(|| {
            initialize();
            assert_noop!(
                Mosaic::timelocked_mint(Origin::relayer(), 1, ALICE, 10001, 10,  Default::default()),
                Error::<Test>::InsufficientBudget
            );
        })
    }

    #[test]
    fn only_relayer_can_timelocked_mint() {
        new_test_ext().execute_with(|| {
            initialize();
            assert_noop!(
                Mosaic::timelocked_mint(Origin::signed(ALICE), 1, ALICE, 50, 10,  Default::default()),
                DispatchError::BadOrigin
            );
        })
    }

    #[test]
    fn none_cannot_timelocked_mint() {
        new_test_ext().execute_with(|| {
            initialize();
            assert_noop!(
                Mosaic::timelocked_mint(Origin::none(), 1, ALICE, 50, 10,  Default::default()),
                DispatchError::BadOrigin
            );
        })
    }

    #[test]
    fn timelocked_mint_adds_to_incoming_transactions() {
        new_test_ext().execute_with(|| {
            initialize();
            let amount = 50;
            let lock_time = 10;
            Mosaic::timelocked_mint(Origin::relayer(), 1, ALICE, amount, lock_time,  Default::default())
                .expect("timelocked_mint should work");
            assert_eq!(
                Mosaic::incoming_transactions(ALICE, 1),
                Some((amount, lock_time + System::block_number()))
            );
        })
    }

    #[test]
    fn timelocked_mint_updates_incoming_transactions() {
        new_test_ext().execute_with(|| {
            initialize();
            let amount = 50;
            let lock_time = 10;

            Mosaic::timelocked_mint(Origin::relayer(), 1, ALICE, amount, lock_time,  Default::default())
                .expect("timelocked_mint should work");
            assert_eq!(
                Mosaic::incoming_transactions(ALICE, 1),
                Some((amount, lock_time + System::block_number()))
            );

            let amount_2 = 100;
            let new_lock_time = 20;

            Mosaic::timelocked_mint(Origin::relayer(), 1, ALICE, amount_2, new_lock_time,  Default::default())
                .expect("timelocked_mint should work");

            assert_eq!(
                Mosaic::incoming_transactions(ALICE, 1),
                Some((amount + amount_2, new_lock_time + System::block_number()))
            );

        })
    }


    #[test]
    fn rescind_timelocked_mint() {
        new_test_ext().execute_with(|| {
            initialize();
            let lock_time = 10;
            do_timelocked_mint(ALICE, 1, 50, lock_time);

            let initial_block = System::block_number();

            Mosaic::rescind_timelocked_mint(Origin::relayer(), 1, ALICE, 40)
                .expect("relayer should be able to rescind transactions");
            assert_eq!(Mosaic::incoming_transactions(ALICE, 1), Some((10, initial_block + lock_time)));
            let transfer_amount = 9;
            Mosaic::rescind_timelocked_mint(Origin::relayer(), 1, ALICE, transfer_amount)
                .expect("relayer should be able to rescind transactions");
            assert_eq!(Mosaic::incoming_transactions(ALICE, 1), Some((1, 11)));
        })
    }
}

mod set_timelock_duration {
    use super::*;

    #[test]
    fn set_timelock_duration() {
        new_test_ext().execute_with(|| {
            Mosaic::set_timelock_duration(Origin::root(), MinimumTimeLockPeriod::get() + 1)
                .expect("root may set the timelock period");
        })
    }

    #[test]
    fn set_timelock_duration_with_non_root() {
        new_test_ext().execute_with(|| {
            assert_noop!(
                Mosaic::set_timelock_duration(Origin::signed(ALICE), MinimumTimeLockPeriod::get() + 1),
                DispatchError::BadOrigin
            );
        })
    }

    #[test]
    fn set_timelock_duration_with_origin_none() {
        new_test_ext().execute_with(|| {
            assert_noop!(
                Mosaic::set_timelock_duration(Origin::none(), MinimumTimeLockPeriod::get() + 1),
                DispatchError::BadOrigin
            );
        })
    }

    #[test]
    fn set_timelock_duration_with_invalid_period() {
        new_test_ext().execute_with(|| {
            assert_noop!(
                Mosaic::set_timelock_duration(Origin::root(), 0),
                Error::<Test>::BadTimelockPeriod
            );
        })
    }

    #[test]
    fn set_timelock_duration_with_invalid_period_2() {
        new_test_ext().execute_with(|| {
            assert_noop!(
                Mosaic::set_timelock_duration(Origin::root(), MinimumTimeLockPeriod::get() - 1),
                Error::<Test>::BadTimelockPeriod
            );
        })
    }
}

#[test]
fn claim_to() {
	new_test_ext().execute_with(|| {
		initialize();
		let lock_time = 10;
		do_timelocked_mint(ALICE, 1, 50, lock_time);
		let current_block = System::block_number();
		Mosaic::claim_to(Origin::alice(), 1, ALICE).expect_err(
			"received funds should only be claimable after waiting for the relayer mandated time",
		);
		System::set_block_number(current_block + lock_time + 1);
		Mosaic::claim_to(Origin::alice(), 1, ALICE)
			.expect("received funds should be claimable after time has passed");
	})
}
