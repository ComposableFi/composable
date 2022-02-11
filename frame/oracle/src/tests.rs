use crate::{
	mock::{AccountId, Call, Extrinsic, *},
	AssetInfo, Error, PrePrice, Withdraw, *,
};
use codec::Decode;
use composable_traits::{defi::CurrencyPair, oracle::Price};
use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, OnInitialize},
	BoundedVec,
};
use pallet_balances::Error as BalancesError;
use parking_lot::RwLock;
use sp_core::offchain::{testing, OffchainDbExt, OffchainWorkerExt, TransactionPoolExt};
use sp_io::TestExternalities;
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use sp_runtime::{
	traits::{BadOrigin, Zero},
	FixedPointNumber, FixedU128, Percent, RuntimeAppPublic,
};
use std::sync::Arc;

use crate::validation::{ValidBlockInterval, ValidMaxAnswer, ValidMinAnswers, ValidThreshhold};
use composable_support::validation::{Validate, Validated};
use composable_tests_helpers::{prop_assert_noop, prop_assert_ok};
use core::{fmt, marker::PhantomData};
use proptest::prelude::*;

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10_000))]

use sp_core::{H256, sr25519};

prop_compose! {
    fn asset_info()
        (
            MIN_ANSWERS in 1..MaxAnswerBound::get(),
            MAX_ANSWERS in 1..MaxAnswerBound::get(),
            BLOCK_INTERVAL in (StalePrice::get()+1)..(BlockNumber::MAX/16),
            threshold in 0..100u8,
            REWARD in 0..u64::MAX,
            SLASH in 0..u64::MAX,
        ) -> AssetInfo<Percent, BlockNumber, Balance> {
            let MIN_ANSWERS = MAX_ANSWERS.saturating_sub(MIN_ANSWERS) + 1;
            let THRESHOLD: Percent = Percent::from_percent(threshold);

            AssetInfo {
                threshold: THRESHOLD,
                min_answers: MIN_ANSWERS,
                max_answers: MAX_ANSWERS,
                block_interval: BLOCK_INTERVAL,
                reward: REWARD,
                slash: SLASH,
            }
        }
}

prop_compose! {
    fn asset_id()
        (x in 0..AssetId::MAX) -> AssetId {
            x
        }
}

prop_compose! {
	fn account_id()
		(x in 0..u64::MAX) -> AccountId {
            let h256 = H256::from_low_u64_be(x);
			AccountId::from_h256(h256)
		}
}


mod add_asset_and_info {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10_000))]

        #[test]
        fn normal_asset_and_info_assert(
            asset_id in asset_id(),
            asset_info in asset_info(),
        ) {
            new_test_ext().execute_with(|| -> Result<(), TestCaseError> {
                let root_account = get_root_account();

                prop_assert_ok!(Oracle::add_asset_and_info(
                    Origin::signed(root_account),
                    asset_id,
                    asset_info.threshold,
                    asset_info.min_answers,
                    asset_info.max_answers,
                    asset_info.block_interval,
                    asset_info.reward,
                    asset_info.slash,
                ));

                Ok(())
            })?;
        }

        #[test]
        fn asset_count_should_not_increase_when_updating_asset_info(
            asset_id in asset_id(),
            asset_info_1 in asset_info(),
            asset_info_2 in asset_info(),
        ) {
            new_test_ext().execute_with(|| {
                let root_account = get_root_account();

                prop_assert_ok!(Oracle::add_asset_and_info(
                    Origin::signed(root_account),
                    asset_id,
                    asset_info_1.threshold,
                    asset_info_1.min_answers,
                    asset_info_1.max_answers,
                    asset_info_1.block_interval,
                    asset_info_1.reward,
                    asset_info_1.slash,
                ));

                // does not increment asset_count because we have info for the same asset_id
                prop_assert_ok!(Oracle::add_asset_and_info(
                    Origin::signed(root_account),
                    asset_id,
                    asset_info_2.threshold,
                    asset_info_2.min_answers,
                    asset_info_2.max_answers,
                    asset_info_2.block_interval,
                    asset_info_2.reward,
                    asset_info_2.slash,
                ));
                prop_assert_eq!(Oracle::assets_count(), 1);

                Ok(())
            })?;
        }

        #[test]
        fn fails_when_non_root_account(
            asset_id in asset_id(),
            asset_info in asset_info(),
            account_id in account_id(),
        ) {
            // very small chance of happening, but for correctness' sake ;)
            prop_assume!(account_id != get_root_account());

            new_test_ext().execute_with(|| {
                prop_assert_noop!(
                    Oracle::add_asset_and_info(
                        Origin::signed(account_id),
                        asset_id,
                        asset_info.threshold,
                        asset_info.min_answers,
                        asset_info.max_answers,
                        asset_info.block_interval,
                        asset_info.reward,
                        asset_info.slash,
                    ),
                    BadOrigin
                );
                Ok(())
            })?;
        }






    //
    //     #[test]
    //     fn add_asset_and_info(
    //         ASSET_ID in 1..u128::MAX, // When ASSET_ID = 0, we get an error: Module { index: 2, error: 20, message: Some("ExceedAssetsCount") }
    //         MIN_ANSWERS in 1..u32::MAX,
    //         MAX_ANSWERS in 1..u32::MAX,
    //         BLOCK_INTERVAL in 5..20u64, // TODO(cor): find suitable range. The minimum is Oracle::Config::StalePrice, which is currently configured to be 5.
    //         threshold in 0..100u8,
    //         REWARD in 0..u64::MAX,
    //         SLASH in 0..u64::MAX,
    //     ) {
    //         new_test_ext().execute_with(|| {
    //             prop_assume!(MIN_ANSWERS < MAX_ANSWERS);
    //             // const ASSET_ID: u128 = 1;
    //             // const MIN_ANSWERS: u32 = 3;
    //             // const MAX_ANSWERS: u32 = 5;
    //             let THRESHOLD: Percent = Percent::from_percent(threshold);
    //             // const BLOCK_INTERVAL: u64 = 5;
    //             // const REWARD: u64 = 5;
    //             // const SLASH: u64 = 5;
    //
    //             // passes
    //             let account_2 = get_account_2();
    //             prop_assert_ok!(Oracle::add_asset_and_info(
    //                 Origin::signed(account_2),
    //                 ASSET_ID,
    //                 THRESHOLD,
    //                 MIN_ANSWERS,
    //                 MAX_ANSWERS,
    //                 BLOCK_INTERVAL,
    //                 REWARD,
    //                 SLASH
    //             ));
    //
    //             // does not increment if exists
    //             prop_assert_ok!(Oracle::add_asset_and_info(
    //                 Origin::signed(account_2),
    //                 ASSET_ID,
    //                 THRESHOLD,
    //                 MIN_ANSWERS,
    //                 MAX_ANSWERS,
    //                 BLOCK_INTERVAL,
    //                 REWARD,
    //                 SLASH
    //             ));
    //             prop_assert_eq!(Oracle::assets_count(), 1);
    //
    //             prop_assert_ok!(Oracle::add_asset_and_info(
    //                 Origin::signed(account_2),
    //                 ASSET_ID + 1,
    //                 THRESHOLD,
    //                 MIN_ANSWERS,
    //                 MAX_ANSWERS,
    //                 BLOCK_INTERVAL,
    //                 REWARD,
    //                 SLASH
    //             ));
    //
    //             let asset_info = AssetInfo {
    //                 threshold: THRESHOLD,
    //                 min_answers: MIN_ANSWERS,
    //                 max_answers: MAX_ANSWERS,
    //                 block_interval: BLOCK_INTERVAL,
    //                 reward: REWARD,
    //                 slash: SLASH,
    //             };
    //             // id now activated and count incremented
    //             prop_assert_eq!(Oracle::asset_info(1), Some(asset_info));
    //             prop_assert_eq!(Oracle::assets_count(), 2);
    //
    //             // fails with non permission
    //             let account_1 = get_account_1();
    //             prop_assert_noop!(
    //                 Oracle::add_asset_and_info(
    //                     Origin::signed(account_1),
    //                     ASSET_ID,
    //                     THRESHOLD,
    //                     MAX_ANSWERS,
    //                     MAX_ANSWERS,
    //                     BLOCK_INTERVAL,
    //                     REWARD,
    //                     SLASH
    //                 ),
    //                 BadOrigin
    //             );
    //
    //             prop_assert_noop!(
    //                 Oracle::add_asset_and_info(
    //                     Origin::signed(account_2),
    //                     ASSET_ID,
    //                     THRESHOLD,
    //                     MAX_ANSWERS,
    //                     MIN_ANSWERS,
    //                     BLOCK_INTERVAL,
    //                     REWARD,
    //                     SLASH
    //                 ),
    //                 Error::<Test>::MaxAnswersLessThanMinAnswers
    //             );
    //
    //             prop_assert_noop!(
    //                 Oracle::add_asset_and_info(
    //                     Origin::signed(account_2),
    //                     ASSET_ID,
    //                     Percent::from_percent(100),
    //                     MIN_ANSWERS,
    //                     MAX_ANSWERS,
    //                     BLOCK_INTERVAL,
    //                     REWARD,
    //                     SLASH
    //                 ),
    //                 Error::<Test>::ExceedThreshold
    //             );
    //
    //             prop_assert_noop!(
    //                 Oracle::add_asset_and_info(
    //                     Origin::signed(account_2),
    //                     ASSET_ID,
    //                     THRESHOLD,
    //                     MIN_ANSWERS,
    //                     MAX_ANSWERS + 1,
    //                     BLOCK_INTERVAL,
    //                     REWARD,
    //                     SLASH
    //                 ),
    //                 Error::<Test>::ExceedMaxAnswers
    //             );
    //
    //             prop_assert_noop!(
    //                 Oracle::add_asset_and_info(
    //                     Origin::signed(account_2),
    //                     ASSET_ID,
    //                     THRESHOLD,
    //                     0,
    //                     MAX_ANSWERS,
    //                     BLOCK_INTERVAL,
    //                     REWARD,
    //                     SLASH
    //                 ),
    //                 Error::<Test>::InvalidMinAnswers
    //             );
    //
    //             prop_assert_noop!(
    //                 Oracle::add_asset_and_info(
    //                     Origin::signed(account_2),
    //                     ASSET_ID + 2,
    //                     THRESHOLD,
    //                     MIN_ANSWERS,
    //                     MAX_ANSWERS,
    //                     BLOCK_INTERVAL,
    //                     REWARD,
    //                     SLASH
    //                 ),
    //                 Error::<Test>::ExceedAssetsCount
    //             );
    //
    //             prop_assert_noop!(
    //                 Oracle::add_asset_and_info(
    //                     Origin::signed(account_2),
    //                     ASSET_ID,
    //                     THRESHOLD,
    //                     MIN_ANSWERS,
    //                     MAX_ANSWERS,
    //                     BLOCK_INTERVAL - 4,
    //                     REWARD,
    //                     SLASH
    //                 ),
    //                 Error::<Test>::BlockIntervalLength
    //             );
    //             Ok(())
    //         })?;
    //     }
    }
}

#[test]
fn set_signer() {
	new_test_ext().execute_with(|| {
		let account_1 = get_account_1();
		let account_2 = get_root_account();
		let account_3 = get_account_3();
		let account_4 = get_account_4();
		let account_5 = get_account_5();

		assert_ok!(Oracle::set_signer(Origin::signed(account_2), account_1));
		assert_eq!(Oracle::controller_to_signer(account_2), Some(account_1));
		assert_eq!(Oracle::signer_to_controller(account_1), Some(account_2));

		assert_ok!(Oracle::set_signer(Origin::signed(account_1), account_5));
		assert_eq!(Oracle::controller_to_signer(account_1), Some(account_5));
		assert_eq!(Oracle::signer_to_controller(account_5), Some(account_1));

		assert_noop!(
			Oracle::set_signer(Origin::signed(account_3), account_4),
			BalancesError::<Test>::InsufficientBalance
		);
		assert_noop!(
			Oracle::set_signer(Origin::signed(account_4), account_1),
			Error::<Test>::SignerUsed
		);
		assert_noop!(
			Oracle::set_signer(Origin::signed(account_1), account_2),
			Error::<Test>::ControllerUsed
		);
	});
}

#[test]
fn add_stake() {
	new_test_ext().execute_with(|| {
		let account_1 = get_account_1();
		let account_2 = get_root_account();
		// fails no controller set
		assert_noop!(Oracle::add_stake(Origin::signed(account_1), 50), Error::<Test>::UnsetSigner);

		assert_ok!(Oracle::set_signer(Origin::signed(account_1), account_2));

		assert_eq!(Balances::free_balance(account_2), 100);
		assert_eq!(Balances::free_balance(account_1), 99);
		assert_ok!(Oracle::add_stake(Origin::signed(account_1), 50));
		assert_eq!(Balances::free_balance(account_1), 49);
		assert_eq!(Balances::total_balance(&account_1), 49);
		// funds were transferred to signer and locked
		assert_eq!(Balances::free_balance(account_2), 100);
		assert_eq!(Balances::total_balance(&account_2), 151);

		assert_eq!(Oracle::oracle_stake(account_2), Some(51));
		assert_eq!(Oracle::oracle_stake(account_1), None);

		assert_ok!(Oracle::add_stake(Origin::signed(account_1), 39));
		assert_eq!(Balances::free_balance(account_1), 10);
		assert_eq!(Balances::total_balance(&account_1), 10);
		assert_eq!(Balances::free_balance(account_2), 100);
		assert_eq!(Balances::total_balance(&account_2), 190);

		assert_eq!(Oracle::oracle_stake(account_2), Some(90));
		assert_eq!(Oracle::oracle_stake(account_1), None);

		assert_noop!(
			Oracle::add_stake(Origin::signed(account_1), 10),
			BalancesError::<Test>::KeepAlive
		);
	});
}

#[test]
fn remove_and_reclaim_stake() {
	new_test_ext().execute_with(|| {
		let account_1 = get_account_1();
		let account_2 = get_root_account();
		let account_3 = get_account_3();

		assert_ok!(Oracle::set_signer(Origin::signed(account_1), account_2));

		assert_ok!(Oracle::add_stake(Origin::signed(account_1), 50));

		assert_noop!(Oracle::reclaim_stake(Origin::signed(account_1)), Error::<Test>::Unknown);

		assert_ok!(Oracle::remove_stake(Origin::signed(account_1)));
		let withdrawal = Withdraw { stake: 51, unlock_block: 1 };
		assert_eq!(Oracle::declared_withdraws(account_2), Some(withdrawal));
		assert_eq!(Oracle::oracle_stake(account_2), None);
		assert_noop!(Oracle::remove_stake(Origin::signed(account_1)), Error::<Test>::NoStake);

		assert_noop!(Oracle::reclaim_stake(Origin::signed(account_1)), Error::<Test>::StakeLocked);

		System::set_block_number(2);
		assert_ok!(Oracle::reclaim_stake(Origin::signed(account_1)));
		// everyone gets their funds back
		assert_eq!(Balances::free_balance(account_1), 100);
		assert_eq!(Balances::total_balance(&account_1), 100);
		assert_eq!(Balances::free_balance(account_2), 100);
		assert_eq!(Balances::total_balance(&account_2), 100);

		// signer controller pruned
		assert_eq!(Oracle::controller_to_signer(account_1), None);
		assert_eq!(Oracle::signer_to_controller(account_2), None);

		assert_noop!(Oracle::reclaim_stake(Origin::signed(account_3)), Error::<Test>::UnsetSigner);
	});
}

#[test]
fn add_price() {
	new_test_ext().execute_with(|| {
		let account_1 = get_account_1();
		let account_2 = get_root_account();
		let account_4 = get_account_4();
		let account_5 = get_account_5();

		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Validated::new(Percent::from_percent(80)).unwrap(),
			Validated::new(3).unwrap(),
			Validated::new(3).unwrap(),
			Validated::<BlockNumber, ValidBlockInterval<StalePrice>>::new(5).unwrap(),
			5,
			5
		));

		System::set_block_number(6);
		// fails no stake
		assert_noop!(
			Oracle::submit_price(Origin::signed(account_1), 100_u128, 0_u128),
			Error::<Test>::NotEnoughStake
		);

		assert_ok!(Oracle::set_signer(Origin::signed(account_2), account_1));
		assert_ok!(Oracle::set_signer(Origin::signed(account_1), account_2));
		assert_ok!(Oracle::set_signer(Origin::signed(account_5), account_4));
		assert_ok!(Oracle::set_signer(Origin::signed(account_4), account_5));

		assert_ok!(Oracle::add_stake(Origin::signed(account_1), 50));
		assert_ok!(Oracle::add_stake(Origin::signed(account_2), 50));
		assert_ok!(Oracle::add_stake(Origin::signed(account_4), 50));
		assert_ok!(Oracle::add_stake(Origin::signed(account_5), 50));

		assert_ok!(Oracle::submit_price(Origin::signed(account_1), 100_u128, 0_u128));
		assert_ok!(Oracle::submit_price(Origin::signed(account_2), 100_u128, 0_u128));
		assert_noop!(
			Oracle::submit_price(Origin::signed(account_2), 100_u128, 0_u128),
			Error::<Test>::AlreadySubmitted
		);
		assert_ok!(Oracle::submit_price(Origin::signed(account_4), 100_u128, 0_u128));

		assert_eq!(Oracle::answer_in_transit(account_1), Some(5));
		assert_eq!(Oracle::answer_in_transit(account_2), Some(5));
		assert_eq!(Oracle::answer_in_transit(account_4), Some(5));

		assert_noop!(
			Oracle::submit_price(Origin::signed(account_5), 100_u128, 0_u128),
			Error::<Test>::MaxPrices
		);

		let price = PrePrice { price: 100_u128, block: 6, who: account_1 };

		let price2 = PrePrice { price: 100_u128, block: 6, who: account_2 };

		let price4 = PrePrice { price: 100_u128, block: 6, who: account_4 };

		assert_eq!(Oracle::pre_prices(0), vec![price, price2, price4]);
		System::set_block_number(2);
		Oracle::on_initialize(2);

		// fails price not requested
		assert_noop!(
			Oracle::submit_price(Origin::signed(account_1), 100_u128, 0_u128),
			Error::<Test>::PriceNotRequested
		);

		// non existent asset_id
		assert_noop!(
			Oracle::submit_price(Origin::signed(account_1), 100_u128, 10_u128),
			Error::<Test>::PriceNotRequested
		);
	});
}

#[test]
fn medianize_price() {
	new_test_ext().execute_with(|| {
		let account_1 = get_account_1();
		// should not panic
		Oracle::get_median_price(&Oracle::pre_prices(0));
		for i in 0..3 {
			let price = i as u128 + 100_u128;
			add_price_storage(price, 0, account_1, 0);
		}
		let price = Oracle::get_median_price(&Oracle::pre_prices(0));
		assert_eq!(price, Some(101));
	});
}

#[test]
#[should_panic = "No `keystore` associated for the current context!"]
fn check_request() {
	new_test_ext().execute_with(|| {
		let account_2 = get_root_account();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Validated::new(Percent::from_percent(80)).unwrap(),
			Validated::new(3).unwrap(),
			Validated::new(5).unwrap(),
			Validated::<BlockNumber, ValidBlockInterval<StalePrice>>::new(5).unwrap(),
			5,
			5
		));
		System::set_block_number(6);
		Oracle::check_requests();
	});
}

#[test]
fn not_check_request() {
	new_test_ext().execute_with(|| {
		Oracle::check_requests();
	});
}

#[test]
fn is_requested() {
	new_test_ext().execute_with(|| {
		let account_2 = get_root_account();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Validated::new(Percent::from_percent(80)).unwrap(),
			Validated::new(3).unwrap(),
			Validated::new(5).unwrap(),
			Validated::<BlockNumber, ValidBlockInterval<StalePrice>>::new(5).unwrap(),
			5,
			5
		));
		System::set_block_number(6);
		assert!(Oracle::is_requested(&0));

		let price = Price { price: 0, block: 6 };
		Prices::<Test>::insert(0, price);

		assert!(!Oracle::is_requested(&0));

		System::set_block_number(11);
		assert!(!Oracle::is_requested(&0));
	});
}

#[test]
fn test_payout_slash() {
	new_test_ext().execute_with(|| {
		let account_1 = get_account_1();
		let account_2 = get_root_account();
		let account_3 = get_account_3();
		let account_4 = get_account_4();
		let account_5 = get_account_5();
		assert_ok!(Oracle::set_signer(Origin::signed(account_5), account_2));

		let one = PrePrice { price: 79, block: 0, who: account_1 };
		let two = PrePrice { price: 100, block: 0, who: account_2 };
		let three = PrePrice { price: 151, block: 0, who: account_3 };
		let four = PrePrice { price: 400, block: 0, who: account_4 };

		let five = PrePrice { price: 100, block: 0, who: account_5 };

		let asset_info = AssetInfo {
			threshold: Percent::from_percent(0),
			min_answers: 0,
			max_answers: 0,
			block_interval: 0,
			reward: 0,
			slash: 0,
		};
		// doesn't panic when percent not set
		Oracle::handle_payout(&vec![one, two, three, four, five], 100, 0, &asset_info);
		assert_eq!(Balances::free_balance(account_1), 100);

		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Validated::new(Percent::from_percent(80)).unwrap(),
			Validated::new(3).unwrap(),
			Validated::new(5).unwrap(),
			Validated::<BlockNumber, ValidBlockInterval<StalePrice>>::new(5).unwrap(),
			5,
			5
		));

		add_price_storage(79, 0, account_1, 0);
		add_price_storage(100, 0, account_2, 0);

		assert_eq!(Oracle::answer_in_transit(account_1), Some(5));
		assert_eq!(Oracle::answer_in_transit(account_2), Some(5));

		Oracle::handle_payout(
			&vec![one, two, three, four, five],
			100,
			0,
			&Oracle::asset_info(0).unwrap(),
		);

		assert_eq!(Oracle::answer_in_transit(account_1), Some(0));
		assert_eq!(Oracle::answer_in_transit(account_2), Some(0));
		// account 1 and 4 gets slashed 2 and 5 gets rewarded
		assert_eq!(Balances::free_balance(account_1), 95);
		// 5 gets 2's reward and its own
		assert_eq!(Balances::free_balance(account_5), 109);
		assert_eq!(Balances::free_balance(account_2), 100);

		assert_eq!(Balances::free_balance(account_3), 0);
		assert_eq!(Balances::free_balance(account_4), 95);

		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Validated::new(Percent::from_percent(90)).unwrap(),
			Validated::new(3).unwrap(),
			Validated::new(5).unwrap(),
			Validated::<BlockNumber, ValidBlockInterval<StalePrice>>::new(5).unwrap(),
			4,
			5
		));
		Oracle::handle_payout(
			&vec![one, two, three, four, five],
			100,
			0,
			&Oracle::asset_info(0).unwrap(),
		);

		// account 4 gets slashed 2 5 and 1 gets rewarded
		assert_eq!(Balances::free_balance(account_1), 90);
		// 5 gets 2's reward and its own
		assert_eq!(Balances::free_balance(account_5), 117);
		assert_eq!(Balances::free_balance(account_2), 100);

		assert_eq!(Balances::free_balance(account_3), 0);
		assert_eq!(Balances::free_balance(account_4), 90);
	});
}

#[test]
fn on_init() {
	new_test_ext().execute_with(|| {
		// no price fetch
		Oracle::on_initialize(1);
		let price = Price { price: 0, block: 0 };

		assert_eq!(Oracle::prices(0), price);

		// add and request oracle id
		let account_2 = get_root_account();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Validated::new(Percent::from_percent(80)).unwrap(),
			Validated::new(3).unwrap(),
			Validated::new(5).unwrap(),
			Validated::<BlockNumber, ValidBlockInterval<StalePrice>>::new(5).unwrap(),
			5,
			5
		));
		// set prices into storage
		let account_1 = get_account_1();
		for i in 0..3 {
			let price = i as u128 + 100_u128;
			add_price_storage(price, 0, account_1, 2);
		}

		Oracle::on_initialize(2);
		let price = Price { price: 101, block: 2 };

		assert_eq!(Oracle::prices(0), price);
		// prunes state
		assert_eq!(Oracle::pre_prices(0), vec![]);

		// doesn't prune state if under min prices
		for i in 0..2 {
			let price = i as u128 + 100_u128;
			add_price_storage(price, 0, account_1, 3);
		}

		// does not fire under min answers
		Oracle::on_initialize(3);
		assert_eq!(Oracle::pre_prices(0).len(), 2);
		assert_eq!(Oracle::prices(0), price);
	});
}

#[test]
fn historic_pricing() {
	new_test_ext().execute_with(|| {
		// add and request oracle id
		let account_2 = get_root_account();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Validated::new(Percent::from_percent(80)).unwrap(),
			Validated::new(3).unwrap(),
			Validated::new(5).unwrap(),
			Validated::<BlockNumber, ValidBlockInterval<StalePrice>>::new(5).unwrap(),
			5,
			5
		));

		let mut price_history = vec![];

		do_price_update(0, 0);

		assert_eq!(Oracle::price_history(0).len(), 0);
		assert_eq!(Oracle::price_history(0), price_history);

		do_price_update(0, 5);

		let price_5 = Price { price: 101, block: 5 };
		price_history = vec![price_5.clone()];

		assert_eq!(Oracle::price_history(0), price_history);
		assert_eq!(Oracle::price_history(0).len(), 1);

		do_price_update(0, 10);
		let price_10 = Price { price: 101, block: 10 };
		price_history = vec![price_5.clone(), price_10.clone()];

		assert_eq!(Oracle::price_history(0), price_history);
		assert_eq!(Oracle::price_history(0).len(), 2);

		do_price_update(0, 15);
		let price_15 = Price { price: 101, block: 15 };
		price_history = vec![price_5, price_10.clone(), price_15.clone()];

		assert_eq!(Oracle::price_history(0), price_history);
		assert_eq!(Oracle::price_history(0).len(), 3);

		do_price_update(0, 20);
		let price_20 = Price { price: 101, block: 20 };
		price_history = vec![price_10, price_15, price_20];

		assert_eq!(Oracle::price_history(0), price_history);
		assert_eq!(Oracle::price_history(0).len(), 3);
	});
}

#[test]
fn price_of_amount() {
	new_test_ext().execute_with(|| {
		let value = 100500;
		let id = 42;
		let amount = 10000;

		let price = Price { price: value, block: System::block_number() };
		Prices::<Test>::insert(id, price);
		let total_price =
			<Oracle as composable_traits::oracle::Oracle>::get_price(id, amount).unwrap();

		assert_eq!(total_price.price, value * amount)
	});
}

#[test]
fn ratio_human_case() {
	new_test_ext().execute_with(|| {
		let price = Price { price: 10000, block: System::block_number() };
		Prices::<Test>::insert(13, price);
		let price = Price { price: 100, block: System::block_number() };
		Prices::<Test>::insert(42, price);
		let mut pair = CurrencyPair::new(13, 42);

		let ratio = <Oracle as composable_traits::oracle::Oracle>::get_ratio(pair).unwrap();
		assert_eq!(ratio, FixedU128::saturating_from_integer(100));
		pair.reverse();
		let ratio = <Oracle as composable_traits::oracle::Oracle>::get_ratio(pair).unwrap();

		assert_eq!(ratio, FixedU128::saturating_from_rational(1_u32, 100_u32));
	})
}

#[test]
fn inverses() {
	new_test_ext().execute_with(|| {
		let price = Price { price: 1, block: System::block_number() };
		Prices::<Test>::insert(13, price);
		let inverse =
			<Oracle as composable_traits::oracle::Oracle>::get_price_inverse(13, 1).unwrap();
		assert_eq!(inverse, 1);

		let price = Price { price: 1, block: System::block_number() };
		Prices::<Test>::insert(13, price);
		let inverse =
			<Oracle as composable_traits::oracle::Oracle>::get_price_inverse(13, 2).unwrap();
		assert_eq!(inverse, 2);
	})
}

#[test]
fn ratio_base_is_way_less_smaller() {
	new_test_ext().execute_with(|| {
		let price = Price { price: 1, block: System::block_number() };
		Prices::<Test>::insert(13, price);
		let price = Price { price: 10_u128.pow(12), block: System::block_number() };
		Prices::<Test>::insert(42, price);
		let pair = CurrencyPair::new(13, 42);

		let ratio = <Oracle as composable_traits::oracle::Oracle>::get_ratio(pair).unwrap();

		assert_eq!(ratio, FixedU128::saturating_from_rational(1, 1000000000000_u64));
	})
}

#[test]
fn get_twap() {
	new_test_ext().execute_with(|| {
		// add and request oracle id
		let account_2 = get_root_account();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Validated::new(Percent::from_percent(80)).unwrap(),
			Validated::new(3).unwrap(),
			Validated::new(5).unwrap(),
			Validated::<BlockNumber, ValidBlockInterval<StalePrice>>::new(5).unwrap(),
			5,
			5
		));

		do_price_update(0, 0);
		let price_1 = Price { price: 100, block: 20 };
		let price_2 = Price { price: 100, block: 20 };
		let price_3 = Price { price: 120, block: 20 };
		let historic_prices = [price_1, price_2, price_3].to_vec();
		set_historic_prices(0, historic_prices);

		let twap = Oracle::get_twap(0, vec![20, 30, 50]);
		// twap should be (0.2 * 100) + (0.3 * 120) + (0.5 * 101)
		assert_eq!(twap, Ok(106));
		let err_twap = Oracle::get_twap(0, vec![21, 30, 50]);
		assert_eq!(err_twap, Err(Error::<Test>::MustSumTo100.into()));

		let err_2_twap = Oracle::get_twap(0, vec![10, 10, 10, 10, 60]);
		assert_eq!(err_2_twap, Err(Error::<Test>::DepthTooLarge.into()));
	});
}

#[test]
fn on_init_prune_scenerios() {
	new_test_ext().execute_with(|| {
		// add and request oracle id
		let account_2 = get_root_account();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Validated::new(Percent::from_percent(80)).unwrap(),
			Validated::new(3).unwrap(),
			Validated::new(5).unwrap(),
			Validated::<BlockNumber, ValidBlockInterval<StalePrice>>::new(5).unwrap(),
			5,
			5
		));
		// set prices into storage
		let account_1 = get_account_1();
		for i in 0..3 {
			let price = i as u128 + 100_u128;
			add_price_storage(price, 0, account_1, 0);
		}
		// all pruned
		Oracle::on_initialize(3);
		let price = Price { price: 0, block: 0 };
		assert_eq!(Oracle::prices(0), price);
		assert_eq!(Oracle::pre_prices(0).len(), 0);

		for i in 0..5 {
			let price = i as u128 + 1_u128;
			add_price_storage(price, 0, account_1, 0);
		}

		for i in 0..3 {
			let price = i as u128 + 100_u128;
			add_price_storage(price, 0, account_1, 3);
		}

		// more than half pruned
		Oracle::on_initialize(3);
		let price = Price { price: 101, block: 3 };
		assert_eq!(Oracle::prices(0), price);

		for i in 0..5 {
			let price = i as u128 + 1_u128;
			add_price_storage(price, 0, account_1, 0);
		}

		for i in 0..2 {
			let price = i as u128 + 300_u128;
			add_price_storage(price, 0, account_1, 3);
		}

		// more than half pruned not enough for a price call, same as previous
		Oracle::on_initialize(5);
		let price = Price { price: 101, block: 3 };
		assert_eq!(Oracle::pre_prices(0).len(), 2);
		assert_eq!(Oracle::prices(0), price);
	});
}

#[test]
fn on_init_over_max_answers() {
	new_test_ext().execute_with(|| {
		// add and request oracle id
		let account_2 = get_root_account();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Validated::new(Percent::from_percent(80)).unwrap(),
			Validated::new(1).unwrap(),
			Validated::new(2).unwrap(),
			Validated::<BlockNumber, ValidBlockInterval<StalePrice>>::new(5).unwrap(),
			5,
			5
		));
		// set prices into storage
		let account_1 = get_account_1();
		for i in 0..5 {
			let price = i as u128 + 100_u128;
			add_price_storage(price, 0, account_1, 0);
		}

		assert_eq!(Oracle::answer_in_transit(account_1), Some(25));

		// all pruned
		Oracle::on_initialize(0);
		// price prunes all but first 2 answers, median went from 102 to 100
		let price = Price { price: 100, block: 0 };
		assert_eq!(Oracle::prices(0), price);
		assert_eq!(Oracle::pre_prices(0).len(), 0);

		assert_eq!(Oracle::answer_in_transit(account_1), Some(0));
	});
}

#[test]
fn prune_old_pre_prices_edgecase() {
	new_test_ext().execute_with(|| {
		let asset_info = AssetInfo {
			threshold: Percent::from_percent(80),
			min_answers: 3,
			max_answers: 5,
			block_interval: 5,
			reward: 5,
			slash: 5,
		};
		Oracle::prune_old_pre_prices(&asset_info, vec![], 0);
	});
}

#[test]
fn should_make_http_call_and_parse_result() {
	let (mut t, _, _) = offchain_worker_env(|state| price_oracle_response(state, "0"));

	t.execute_with(|| {
		// when
		let price = Oracle::fetch_price(&0).unwrap();
		// then
		assert_eq!(price, 15523);
	});
}

#[test]
fn knows_how_to_mock_several_http_calls() {
	let (mut t, _, _) = offchain_worker_env(|state| {
		state.expect_request(testing::PendingRequest {
			method: "GET".into(),
			uri: "http://localhost:3001/price/0".into(),
			response: Some(br#"{"0": 100}"#.to_vec()),
			sent: true,
			..Default::default()
		});

		state.expect_request(testing::PendingRequest {
			method: "GET".into(),
			uri: "http://localhost:3001/price/0".into(),
			response: Some(br#"{"0": 200}"#.to_vec()),
			sent: true,
			..Default::default()
		});

		state.expect_request(testing::PendingRequest {
			method: "GET".into(),
			uri: "http://localhost:3001/price/0".into(),
			response: Some(br#"{"0": 300}"#.to_vec()),
			sent: true,
			..Default::default()
		});
	});

	t.execute_with(|| {
		let price1 = Oracle::fetch_price(&0).unwrap();
		let price2 = Oracle::fetch_price(&0).unwrap();
		let price3 = Oracle::fetch_price(&0).unwrap();

		assert_eq!(price1, 100);
		assert_eq!(price2, 200);
		assert_eq!(price3, 300);
	})
}

#[test]
fn should_submit_signed_transaction_on_chain() {
	let (mut t, _, pool_state) = offchain_worker_env(|state| price_oracle_response(state, "0"));

	t.execute_with(|| {
		let account_2 = get_root_account();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Validated::new(Percent::from_percent(80)).unwrap(),
			Validated::new(3).unwrap(),
			Validated::new(5).unwrap(),
			Validated::<BlockNumber, ValidBlockInterval<StalePrice>>::new(5).unwrap(),
			5,
			5
		));

		// when
		Oracle::fetch_price_and_send_signed(&0, Oracle::asset_info(0).unwrap()).unwrap();
		// then
		let tx = pool_state.write().transactions.pop().unwrap();
		assert!(pool_state.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(tx.signature.unwrap().0, 0);
		assert_eq!(tx.call, Call::Oracle(crate::Call::submit_price { price: 15523, asset_id: 0 }));
	});
}

#[test]
#[should_panic = "Tx already submitted"]
fn should_check_oracles_submitted_price() {
	let (mut t, oracle_account_id, _) = offchain_worker_env(|state| price_oracle_response(state, "0"));

	t.execute_with(|| {
		let account_2 = get_root_account();

		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Validated::new(Percent::from_percent(80)).unwrap(),
			Validated::new(3).unwrap(),
			Validated::new(5).unwrap(),
			Validated::<BlockNumber, ValidBlockInterval<StalePrice>>::new(5).unwrap(),
			5,
			5
		));

		add_price_storage(100_u128, 0, oracle_account_id, 0);
		// when
		Oracle::fetch_price_and_send_signed(&0, Oracle::asset_info(0).unwrap()).unwrap();
	});
}

#[test]
#[should_panic = "Max answers reached"]
fn should_check_oracles_max_answer() {
	let (mut t, _, _) = offchain_worker_env(|state| price_oracle_response(state, "0"));
	let asset_info = AssetInfo {
		threshold: Percent::from_percent(0),
		min_answers: 0,
		max_answers: 0,
		block_interval: 0,
		reward: 0,
		slash: 0,
	};
	t.execute_with(|| {
		Oracle::fetch_price_and_send_signed(&0, asset_info).unwrap();
	});
}

#[test]
fn parse_price_works() {
	let test_data = vec![
		("{\"1\":6536.92}", Some(6536)),
		("{\"1\":650000000}", Some(650000000)),
		("{\"2\":6536}", None),
		("{\"0\":\"6432\"}", None),
	];

	for (json, expected) in test_data {
		assert_eq!(expected, Oracle::parse_price(json, "1"));
	}
}

fn add_price_storage(price: u128, asset_id: u128, who: AccountId, block: u64) {
	let price = PrePrice { price, block, who };
	PrePrices::<Test>::mutate(asset_id, |current_prices| current_prices.try_push(price).unwrap());
	AnswerInTransit::<Test>::mutate(who, |transit| {
		*transit = Some(transit.unwrap_or_else(Zero::zero) + 5)
	});
}

fn do_price_update(asset_id: u128, block: u64) {
	let account_1 = get_account_1();
	for i in 0..3 {
		let price = i as u128 + 100_u128;
		add_price_storage(price, asset_id, account_1, block);
	}

	System::set_block_number(block);
	Oracle::on_initialize(block);
	let price = Price { price: 101, block };
	assert_eq!(Oracle::prices(asset_id), price);
}

fn set_historic_prices(asset_id: u128, historic_prices: Vec<Price<u128, u64>>) {
	PriceHistory::<Test>::insert(asset_id, BoundedVec::try_from(historic_prices).unwrap());
}

fn price_oracle_response(state: &mut testing::OffchainState, price_id: &str) {
	let base: String = "http://localhost:3001/price/".to_owned();
	let url = base + price_id;

	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: url,
		response: Some(br#"{"0": 15523}"#.to_vec()),
		sent: true,
		..Default::default()
	});
}

fn offchain_worker_env(
	state_updater: fn(&mut testing::OffchainState),
) -> (TestExternalities, AccountId, Arc<RwLock<testing::PoolState>>) {
	const PHRASE: &str =
		"news slush supreme milk chapter athlete soap sausage put clutch what kitten";

	let (offchain, offchain_state) = testing::TestOffchainExt::new();
	let (pool, pool_state) = testing::TestTransactionPoolExt::new();
	let keystore = KeyStore::new();
	let account_id = SyncCryptoStore::sr25519_generate_new(
		&keystore,
		crate::crypto::Public::ID,
		Some(&format!("{}/hunter1", PHRASE)),
	)
	.unwrap();

	let mut t = sp_io::TestExternalities::default();
	t.register_extension(OffchainDbExt::new(offchain.clone()));
	t.register_extension(OffchainWorkerExt::new(offchain));
	t.register_extension(TransactionPoolExt::new(pool));
	t.register_extension(KeystoreExt(Arc::new(keystore)));

	state_updater(&mut offchain_state.write());

	(t, account_id, pool_state)
}

#[cfg(test)]
mod test {
	use super::*;
	use composable_support::validation::Validate;
	use frame_support::assert_ok;
	use mock::Test;
	use validation::{ValidBlockInterval, ValidMaxAnswer, ValidMinAnswers, ValidThreshhold};

	#[test]
	fn test_threshold_valid_case() {
		assert_ok!(<ValidThreshhold as Validate<Percent, ValidThreshhold>>::validate(
			Percent::from_percent(99)
		));
	}

	#[test]
	fn test_threshold_invalid_case() {
		assert!(<ValidThreshhold as Validate<Percent, ValidThreshhold>>::validate(
			Percent::from_percent(100)
		)
		.is_err());

		assert!(<ValidThreshhold as Validate<Percent, ValidThreshhold>>::validate(
			Percent::from_percent(110)
		)
		.is_err());
	}

	#[test]
	fn test_threshold() {
		assert!(<ValidThreshhold as Validate<Percent, ValidThreshhold>>::validate(
			Percent::from_percent(100)
		)
		.is_err());

		assert!(<ValidThreshhold as Validate<Percent, ValidThreshhold>>::validate(
			Percent::from_percent(110)
		)
		.is_err());
	}

	#[test]
	fn test_max_answer_valid_case() {
		assert_ok!(<ValidMaxAnswer<MaxAnswerBound> as Validate<
			u32,
			ValidMaxAnswer<MaxAnswerBound>,
		>>::validate(2_u32));
	}

	#[test]
	fn test_max_answer_invalid_case() {
		assert!(<ValidMaxAnswer<MaxAnswerBound> as Validate<
			u32,
			ValidMaxAnswer<MaxAnswerBound>,
		>>::validate(10_u32)
		.is_err());
	}

	#[test]
	fn test_min_answer_valid_case() {
		assert!(<ValidMinAnswers as Validate<u32, ValidMinAnswers>>::validate(0_u32).is_err());
	}

	#[test]
	fn test_min_answer_invalid_case() {
		assert_ok!(<ValidMinAnswers as Validate<u32, ValidMinAnswers>>::validate(1_u32));
	}

	#[test]
	fn test_block_interval_valid_case() {
		assert_ok!(<ValidBlockInterval<StalePrice> as Validate<
			BlockNumber,
			ValidBlockInterval<StalePrice>,
		>>::validate(100_u64));
	}

	#[test]
	fn test_block_interval_invalid_case() {
		assert!(<ValidBlockInterval<StalePrice> as Validate<
			BlockNumber,
			ValidBlockInterval<StalePrice>,
		>>::validate(2_u64)
		.is_err());
	}
}
