use composable_traits::{
	time::{TimeReleaseFunction, LinearDecrease},
	defi::{Sell, Take, LiftedFixedBalance},
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
use sp_runtime::{FixedPointNumber, traits::AccountIdConversion};

use crate::mock::{currency::CurrencyId, runtime::*};

fn fixed(n:u64) -> LiftedFixedBalance {
	LiftedFixedBalance::saturating_from_integer(n)
}

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
		let sell = Sell::new(CurrencyId::BTC, CurrencyId::USDT, 1, fixed(1000));
		let invalid = crate::OrdersIndex::<Runtime>::get();
		let configuration = TimeReleaseFunction::LinearDecrease(LinearDecrease { total: 42 });
		let not_reserved = Assets::reserved_balance(CurrencyId::BTC, &ALICE);
		let gas = Assets::balance(CurrencyId::PICA, &ALICE);
		let treasury = Assets::balance(CurrencyId::PICA, &DutchAuctionPalletId::get().into_account());
		DutchAuction::ask(Origin::signed(seller), sell, configuration).unwrap();
		let treasury_added = Assets::balance(CurrencyId::PICA, &DutchAuctionPalletId::get().into_account()) - treasury;
		assert!(treasury_added > 0);
		assert!(treasury_added >= <() as crate::weights::WeightInfo>::liquidate());
		let reserved = Assets::reserved_balance(CurrencyId::BTC, &ALICE);
		assert!(not_reserved < reserved && reserved == 1);
		let order_id = crate::OrdersIndex::<Runtime>::get();
		assert_ne!(invalid, order_id);
		let ask_gas = <() as crate::weights::WeightInfo>::ask();	
		let remaining_gas = Assets::balance(CurrencyId::PICA, &ALICE);
		assert!(gas < remaining_gas +  ask_gas + treasury_added);
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
		let sell = Sell::new(CurrencyId::BTC, CurrencyId::USDT, sell_amount, fixed(take_amount));
		let configuration = TimeReleaseFunction::LinearDecrease(LinearDecrease { total: 42 });
		DutchAuction::ask(Origin::signed(seller), sell, configuration).unwrap();
		let order_id = crate::OrdersIndex::<Runtime>::get();
		let result = DutchAuction::take(Origin::signed(buyer), order_id, Take::new(1, fixed(999)));
		assert!(!result.is_ok());
		let not_reserved =
			<Assets as MultiReservableCurrency<_>>::reserved_balance(CurrencyId::USDT, &BOB);
		let result = DutchAuction::take(Origin::signed(buyer), order_id, Take::new(1, fixed(1000)));
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
		let configuration = TimeReleaseFunction::LinearDecrease(LinearDecrease { total: 42 });

		let sell = Sell::new(CurrencyId::BTC, CurrencyId::USDT, sell_amount, fixed(take_amount));
		DutchAuction::ask(Origin::signed(seller), sell, configuration).unwrap();
		let order_id = crate::OrdersIndex::<Runtime>::get();
		assert_ok!(DutchAuction::take(Origin::signed(buyer), order_id, Take::new(1, fixed(1001))));
		assert_ok!(DutchAuction::take(Origin::signed(buyer), order_id, Take::new(1, fixed(1002))));

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
		let sell = Sell::new(CurrencyId::BTC, CurrencyId::USDT, 1, fixed(1000));
		let configuration = TimeReleaseFunction::LinearDecrease(LinearDecrease { total: 42 });
		DutchAuction::ask(Origin::signed(seller), sell, configuration).unwrap();
		let order_id = crate::OrdersIndex::<Runtime>::get();
		let _balance_before = <Balances as fungible::Inspect<_>>::balance(&ALICE);
		DutchAuction::liquidate(Origin::signed(seller), order_id).unwrap();

		// TODO: Fix the check below
		// let balance_after = <Balances as fungible::Inspect<_>>::balance(&ALICE);
		// assert!(
		// 	balance_before - <() as crate::weights::WeightInfo>::liquidate() as u128 ==
		// 		balance_after
		// );

		let not_found = crate::SellOrders::<Runtime>::get(order_id);
		assert!(not_found.is_none());
		let reserved =
			<Assets as MultiReservableCurrency<_>>::reserved_balance(CurrencyId::BTC, &ALICE);
		assert_eq!(reserved, 0);
	});
}
