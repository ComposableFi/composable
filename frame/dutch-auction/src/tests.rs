use composable_traits::{
	auction::{AuctionStepFunction, LinearDecrease},
	defi::{Sell, Take},
};
use orml_traits::MultiReservableCurrency;

use frame_support::{
	assert_ok,
	traits::{
		fungible::{self, Mutate as NativeMutate},
		fungibles::{Inspect, Mutate},
		Hooks,
	},
};

use crate::mock::{currency::CurrencyId, runtime::*};

pub fn new_test_externalities() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
	let balances =
		vec![(ALICE, 1_000_000_000_000_000_000_000_000), (BOB, 1_000_000_000_000_000_000_000_000)];

	pallet_balances::GenesisConfig::<Runtime> { balances }
		.assimilate_storage(&mut storage)
		.unwrap();

	let mut externatlities = sp_io::TestExternalities::new(storage);
	externatlities.execute_with(|| {
		System::set_block_number(42);
		Timestamp::set_timestamp(System::block_number() * MILLISECS_PER_BLOCK);
	});
	externatlities
}

#[test]
fn setup_sell() {
	new_test_externalities().execute_with(|| {
		Tokens::mint_into(CurrencyId::PICA, &ALICE, 1_000_000_000_000_000_000_000).unwrap();
		Balances::mint_into(&ALICE, NativeExistentialDeposit::get() * 3).unwrap();
		<Balances as NativeMutate<_>>::mint_into(&ALICE, NativeExistentialDeposit::get() * 3)
			.unwrap();
		Tokens::mint_into(CurrencyId::BTC, &ALICE, 100000000000).unwrap();
		let seller = AccountId::from_raw(ALICE.0);
		let sell = Sell::new(CurrencyId::BTC, CurrencyId::USDT, 1, 1000);
		let invalid = crate::OrdersIndex::<Runtime>::get();
		let configuration = AuctionStepFunction::LinearDecrease(LinearDecrease { total: 42 });
		let not_reserved = Assets::reserved_balance(CurrencyId::BTC, &ALICE);
		DutchAuction::ask(Origin::signed(seller), sell, configuration).unwrap();
		let reserved = Assets::reserved_balance(CurrencyId::BTC, &ALICE);
		assert!(not_reserved < reserved && reserved == 1);
		let order_id = crate::OrdersIndex::<Runtime>::get();
		assert_ne!(invalid, order_id);
		let initiative: u128 = Assets::reserved_balance(CurrencyId::PICA, &ALICE);
		let taken = <() as crate::weights::WeightInfo>::liquidate();
		assert!(initiative == taken.into());
	});
}

#[test]
fn with_immediate_exact_buy() {
	new_test_externalities().execute_with(|| {
		let a = 1_000_000_000_000_000_000_000;
		let b = 10;
		Tokens::mint_into(CurrencyId::USDT, &BOB, a).unwrap();
		Tokens::mint_into(CurrencyId::BTC, &ALICE, b).unwrap();
		let seller = AccountId::from_raw(ALICE.0);
		let buyer = AccountId::from_raw(BOB.0);
		let sell_amount = 1;
		let take_amount = 1000;
		let sell = Sell::new(CurrencyId::BTC, CurrencyId::USDT, sell_amount, take_amount);
		let configuration = AuctionStepFunction::LinearDecrease(LinearDecrease { total: 42 });
		DutchAuction::ask(Origin::signed(seller), sell, configuration).unwrap();
		let order_id = crate::OrdersIndex::<Runtime>::get();
		let result = DutchAuction::take(Origin::signed(buyer), order_id, Take::new(1, 999));
		assert!(!result.is_ok());
		let not_reserved =
			<Assets as MultiReservableCurrency<_>>::reserved_balance(CurrencyId::USDT, &BOB);
		let result = DutchAuction::take(Origin::signed(buyer), order_id, Take::new(1, 1000));
		assert_ok!(result);
		let reserved = Assets::reserved_balance(CurrencyId::USDT, &BOB);
		assert!(not_reserved < reserved && reserved == take_amount);
		DutchAuction::on_finalize(42);
		let not_found = crate::SellOrders::<Runtime>::get(order_id);
		assert!(not_found.is_none());
		assert_eq!(Tokens::balance(CurrencyId::USDT, &ALICE), 1000);
		assert_eq!(Tokens::balance(CurrencyId::BTC, &BOB), 1);
	});
}

#[test]
fn with_two_takes_higher_than_limit_and_not_enough_for_all() {
	new_test_externalities().execute_with(|| {
		let a = 1_000_000_000_000_000_000_000;
		let b = 1_000_000_000_000_000_000_000;
		Tokens::mint_into(CurrencyId::USDT, &BOB, a).unwrap();
		Tokens::mint_into(CurrencyId::BTC, &ALICE, b).unwrap();
		let seller = AccountId::from_raw(ALICE.0);
		let buyer = AccountId::from_raw(BOB.0);
		let sell_amount = 3;
		let take_amount = 1000;
		let configuration = AuctionStepFunction::LinearDecrease(LinearDecrease { total: 42 });

		let sell = Sell::new(CurrencyId::BTC, CurrencyId::USDT, sell_amount, take_amount);
		DutchAuction::ask(Origin::signed(seller), sell, configuration).unwrap();
		let order_id = crate::OrdersIndex::<Runtime>::get();
		assert_ok!(DutchAuction::take(Origin::signed(buyer), order_id, Take::new(1, 1001)));
		assert_ok!(DutchAuction::take(Origin::signed(buyer), order_id, Take::new(1, 1002)));

		DutchAuction::on_finalize(42);

		let order = crate::SellOrders::<Runtime>::get(order_id);
		assert!(order.is_some(), "not filled order exists");
	});
}

#[test]
fn liquidation() {
	new_test_externalities().execute_with(|| {
		Tokens::mint_into(CurrencyId::BTC, &ALICE, 10).unwrap();
		let seller = AccountId::from_raw(ALICE.0);
		let sell = Sell::new(CurrencyId::BTC, CurrencyId::USDT, 1, 1000);
		let configuration = AuctionStepFunction::LinearDecrease(LinearDecrease { total: 42 });
		DutchAuction::ask(Origin::signed(seller), sell, configuration).unwrap();
		let order_id = crate::OrdersIndex::<Runtime>::get();
		let balance_before = <Balances as fungible::Inspect<_>>::balance(&ALICE);
		DutchAuction::liquidate(Origin::signed(seller), order_id).unwrap();
		let balance_after = <Balances as fungible::Inspect<_>>::balance(&ALICE);
		assert!(
			balance_before - <() as crate::weights::WeightInfo>::liquidate() as u128 ==
				balance_after
		);
		let not_found = crate::SellOrders::<Runtime>::get(order_id);
		assert!(not_found.is_none());
		let reserved =
			<Assets as MultiReservableCurrency<_>>::reserved_balance(CurrencyId::BTC, &ALICE);
		assert_eq!(reserved, 0);
	});
}
