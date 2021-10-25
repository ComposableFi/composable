use crate::{
	mock::{AccountId, Call, Extrinsic, *},
	AssetInfo, Error, PrePrice, Price, Withdraw, *,
};
use codec::Decode;
use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, OnInitialize},
};
use pallet_balances::Error as BalancesError;
use parking_lot::RwLock;
use sp_core::offchain::{testing, OffchainDbExt, OffchainWorkerExt, TransactionPoolExt};
use sp_io::TestExternalities;
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use sp_runtime::{Percent, RuntimeAppPublic};
use std::sync::Arc;

use sp_runtime::traits::BadOrigin;

#[test]
fn add_asset_and_info() {
	new_test_ext().execute_with(|| {
		const ASSET_ID: u128 = 1;
		const MIN_ANSWERS: u32 = 3;
		const MAX_ANSWERS: u32 = 5;
		const THRESHOLD: Percent = Percent::from_percent(80);

		// passes
		let account_2 = get_account_2();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			ASSET_ID,
			THRESHOLD,
			MIN_ANSWERS,
			MAX_ANSWERS,
		));
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			ASSET_ID + 1,
			THRESHOLD,
			MIN_ANSWERS,
			MAX_ANSWERS,
		));

		let asset_info =
			AssetInfo { threshold: THRESHOLD, min_answers: MIN_ANSWERS, max_answers: MAX_ANSWERS };
		// id now activated and count incremented
		assert_eq!(Oracle::accuracy_threshold(1), asset_info);
		assert_eq!(Oracle::assets_count(), 2);
		// fails with non permission
		let account_1: AccountId = Default::default();
		assert_noop!(
			Oracle::add_asset_and_info(
				Origin::signed(account_1),
				ASSET_ID,
				THRESHOLD,
				MAX_ANSWERS,
				MAX_ANSWERS
			),
			BadOrigin
		);

		assert_noop!(
			Oracle::add_asset_and_info(
				Origin::signed(account_2),
				ASSET_ID,
				THRESHOLD,
				MAX_ANSWERS,
				MIN_ANSWERS,
			),
			Error::<Test>::MaxAnswersLessThanMinAnswers
		);

		assert_noop!(
			Oracle::add_asset_and_info(
				Origin::signed(account_2),
				ASSET_ID,
				Percent::from_percent(100),
				MIN_ANSWERS,
				MAX_ANSWERS,
			),
			Error::<Test>::ExceedThreshold
		);

		assert_noop!(
			Oracle::add_asset_and_info(
				Origin::signed(account_2),
				ASSET_ID,
				THRESHOLD,
				MIN_ANSWERS,
				MAX_ANSWERS + 1,
			),
			Error::<Test>::ExceedMaxAnswers
		);

		assert_noop!(
			Oracle::add_asset_and_info(
				Origin::signed(account_2),
				ASSET_ID,
				THRESHOLD,
				0,
				MAX_ANSWERS,
			),
			Error::<Test>::InvalidMinAnswers
		);

		// passes
		assert_noop!(
			Oracle::add_asset_and_info(
				Origin::signed(account_2),
				ASSET_ID + 2,
				THRESHOLD,
				MIN_ANSWERS,
				MAX_ANSWERS,
			),
			Error::<Test>::ExceedAssetsCount
		);
	});
}

#[test]
fn set_signer() {
	new_test_ext().execute_with(|| {
		let account_1: AccountId = Default::default();
		let account_2 = get_account_2();
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
fn do_request_price() {
	new_test_ext().execute_with(|| {
		// sets ID to true first
		let account_2 = get_account_2();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			1,
			Percent::from_percent(80),
			3,
			5,
		));

		let account_3 = get_account_3();
		// fails on not enough funds
		assert_noop!(Oracle::do_request_price(&account_3, 1), Error::<Test>::NotEnoughFunds);
		// passes
		let account_1: AccountId = Default::default();
		assert_ok!(Oracle::do_request_price(&account_1, 1));
		// takes the cost
		assert_eq!(Balances::free_balance(account_1), 99);
		// True and request ID incremented
		assert!(Oracle::requested(1));

		// fails on invalid ID
		assert_noop!(Oracle::do_request_price(&account_1, 2), Error::<Test>::InvalidAssetId);

		// no funds taken if already true
		let account_3 = get_account_3();
		assert_ok!(Oracle::do_request_price(&account_3, 1));

		// True and request ID not incremented
		assert!(Oracle::requested(1));
	});
}

#[test]
fn add_stake() {
	new_test_ext().execute_with(|| {
		let account_1: AccountId = Default::default();
		let account_2 = get_account_2();
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
		let account_1: AccountId = Default::default();
		let account_2 = get_account_2();
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
		let account_1: AccountId = Default::default();
		let account_2 = get_account_2();
		let account_4 = get_account_4();
		let account_5 = get_account_5();

		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Percent::from_percent(80),
			3,
			3,
		));
		// fails price not requested
		assert_noop!(
			Oracle::submit_price(Origin::signed(account_1), 100u128, 0u128),
			Error::<Test>::PriceNotRequested
		);

		assert_ok!(Oracle::do_request_price(&account_1, 0));
		// fails no stake
		assert_noop!(
			Oracle::submit_price(Origin::signed(account_1), 100u128, 0u128),
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

		assert_ok!(Oracle::submit_price(Origin::signed(account_1), 100u128, 0u128));
		assert_ok!(Oracle::submit_price(Origin::signed(account_2), 100u128, 0u128));
		assert_noop!(
			Oracle::submit_price(Origin::signed(account_2), 100u128, 0u128),
			Error::<Test>::AlreadySubmitted
		);
		assert_ok!(Oracle::submit_price(Origin::signed(account_4), 100u128, 0u128));

		assert_noop!(
			Oracle::submit_price(Origin::signed(account_5), 100u128, 0u128),
			Error::<Test>::MaxPrices
		);

		let price = PrePrice { price: 100u128, block: 0, who: account_1 };

		let price2 = PrePrice { price: 100u128, block: 0, who: account_2 };

		let price4 = PrePrice { price: 100u128, block: 0, who: account_4 };

		assert_eq!(Oracle::pre_prices(0), vec![price, price2, price4]);
	});
}

#[test]
fn medianize_price() {
	new_test_ext().execute_with(|| {
		let account_1: AccountId = Default::default();
		// should not panic
		Oracle::get_median_price(&Oracle::pre_prices(0));
		for i in 0..3 {
			let price = i as u128 + 100u128;
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
		let account_2 = get_account_2();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Percent::from_percent(80),
			3,
			5,
		));
		let account_1: AccountId = Default::default();
		assert_ok!(Oracle::do_request_price(&account_1, 0));
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
fn test_payout_slash() {
	new_test_ext().execute_with(|| {
		let account_1 = Default::default();
		let account_2 = get_account_2();
		let account_3 = get_account_3();
		let account_4 = get_account_4();
		let account_5 = get_account_5();
		assert_ok!(Oracle::set_signer(Origin::signed(account_5), account_2));

		let one = PrePrice { price: 79, block: 0, who: account_1 };
		let two = PrePrice { price: 100, block: 0, who: account_2 };
		let three = PrePrice { price: 151, block: 0, who: account_3 };
		let four = PrePrice { price: 400, block: 0, who: account_4 };

		let five = PrePrice { price: 100, block: 0, who: account_5 };
		// doesn't panic when percent not set
		Oracle::handle_payout(&vec![one, two, three, four, five], 100, 0);
		assert_eq!(Balances::free_balance(account_1), 105);

		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Percent::from_percent(80),
			3,
			5,
		));

		Oracle::handle_payout(&vec![one, two, three, four, five], 100, 0);
		// account 1 and 4 gets slashed 2 and 5 gets rewarded
		assert_eq!(Balances::free_balance(account_1), 100);
		// 5 gets 2's reward and its own
		assert_eq!(Balances::free_balance(account_5), 119);
		assert_eq!(Balances::free_balance(account_2), 100);

		assert_eq!(Balances::free_balance(account_3), 0);
		assert_eq!(Balances::free_balance(account_4), 100);

		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Percent::from_percent(90),
			3,
			5,
		));
		Oracle::handle_payout(&vec![one, two, three, four, five], 100, 0);

		// account 4 gets slashed 2 5 and 1 gets rewarded
		assert_eq!(Balances::free_balance(account_1), 95);
		// 5 gets 2's reward and its own
		assert_eq!(Balances::free_balance(account_5), 129);
		assert_eq!(Balances::free_balance(account_2), 100);

		assert_eq!(Balances::free_balance(account_3), 0);
		assert_eq!(Balances::free_balance(account_4), 95);
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
		let account_2 = get_account_2();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Percent::from_percent(80),
			3,
			5,
		));
		let account_1: AccountId = Default::default();
		assert_ok!(Oracle::do_request_price(&account_1, 0));
		// set prices into storage
		let account_1: AccountId = Default::default();
		for i in 0..3 {
			let price = i as u128 + 100u128;
			add_price_storage(price, 0, account_1, 2);
		}

		Oracle::on_initialize(2);
		let price = Price { price: 101, block: 2 };

		assert_eq!(Oracle::prices(0), price);
		// prunes state
		assert_eq!(Oracle::pre_prices(0), vec![]);
		assert!(!Oracle::requested(0));

		// doesn't prune state if under min prices
		assert_ok!(Oracle::do_request_price(&account_1, 0));

		for i in 0..2 {
			let price = i as u128 + 100u128;
			add_price_storage(price, 0, account_1, 3);
		}

		Oracle::on_initialize(3);
		assert_eq!(Oracle::pre_prices(0).len(), 2);
		assert!(Oracle::requested(0));
	});
}

#[test]
fn on_init_prune_scenerios() {
	new_test_ext().execute_with(|| {
		// add and request oracle id
		let account_2 = get_account_2();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Percent::from_percent(80),
			3,
			5,
		));
		let account_1: AccountId = Default::default();
		assert_ok!(Oracle::do_request_price(&account_1, 0));
		// set prices into storage
		let account_1: AccountId = Default::default();
		for i in 0..3 {
			let price = i as u128 + 100u128;
			add_price_storage(price, 0, account_1, 0);
		}
		// all pruned
		Oracle::on_initialize(3);
		let price = Price { price: 0, block: 0 };
		assert_eq!(Oracle::prices(0), price);
		assert_eq!(Oracle::pre_prices(0).len(), 0);

		for i in 0..5 {
			let price = i as u128 + 1u128;
			add_price_storage(price, 0, account_1, 0);
		}

		for i in 0..3 {
			let price = i as u128 + 100u128;
			add_price_storage(price, 0, account_1, 3);
		}

		// more than half pruned
		Oracle::on_initialize(3);
		let price = Price { price: 101, block: 3 };
		assert_eq!(Oracle::prices(0), price);

		for i in 0..5 {
			let price = i as u128 + 1u128;
			add_price_storage(price, 0, account_1, 0);
		}

		for i in 0..2 {
			let price = i as u128 + 300u128;
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
		let account_2 = get_account_2();
		assert_ok!(Oracle::add_asset_and_info(
			Origin::signed(account_2),
			0,
			Percent::from_percent(80),
			1,
			2,
		));
		let account_1: AccountId = Default::default();
		assert_ok!(Oracle::do_request_price(&account_1, 0));
		// set prices into storage
		let account_1: AccountId = Default::default();
		for i in 0..5 {
			let price = i as u128 + 100u128;
			add_price_storage(price, 0, account_1, 0);
		}
		// all pruned
		Oracle::on_initialize(0);
		// price prunes all but first 2 answers, median went from 102 to 100
		let price = Price { price: 100, block: 0 };
		assert_eq!(Oracle::prices(0), price);
		assert_eq!(Oracle::pre_prices(0).len(), 0);
	});
}

#[test]
fn prune_old_pre_prices_edgecase() {
	new_test_ext().execute_with(|| {
		let asset_info =
			AssetInfo { threshold: Percent::from_percent(80), min_answers: 3, max_answers: 5 };
		Oracle::prune_old_pre_prices(asset_info, vec![], 0);
	});
}

#[test]
fn should_make_http_call_and_parse_result() {
	let (mut t, _) = offchain_worker_env(|state| price_oracle_response(state, "0"));

	t.execute_with(|| {
		// when
		let price = Oracle::fetch_price(&0).unwrap();
		// then
		assert_eq!(price, 15523);
	});
}

#[test]
fn knows_how_to_mock_several_http_calls() {
	let (mut t, _) = offchain_worker_env(|state| {
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
	let (mut t, pool_state) = offchain_worker_env(|state| price_oracle_response(state, "0"));

	t.execute_with(|| {
		// when
		Oracle::fetch_price_and_send_signed(&0).unwrap();
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
	let (mut t, _) = offchain_worker_env(|state| price_oracle_response(state, "0"));

	t.execute_with(|| {
		let account_2 = get_account_2();

		add_price_storage(100u128, 0, account_2, 0);
		// when
		Oracle::fetch_price_and_send_signed(&0).unwrap();
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
	PrePrices::<Test>::mutate(asset_id, |current_prices| current_prices.push(price));
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
) -> (TestExternalities, Arc<RwLock<testing::PoolState>>) {
	const PHRASE: &str =
		"news slush supreme milk chapter athlete soap sausage put clutch what kitten";

	let (offchain, offchain_state) = testing::TestOffchainExt::new();
	let (pool, pool_state) = testing::TestTransactionPoolExt::new();
	let keystore = KeyStore::new();
	SyncCryptoStore::sr25519_generate_new(
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

	(t, pool_state)
}
