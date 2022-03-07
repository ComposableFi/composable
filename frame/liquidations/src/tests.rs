use crate::mock::{currency::*, runtime::*};
use composable_traits::{
	defi::{Ratio, Sell},
	liquidation::Liquidation,
};
use frame_support::traits::{fungible::Mutate as NativeMutate, fungibles::Mutate};
use sp_runtime::FixedPointNumber;

// ensure that we take extra for sell, at least amount to remove
#[test]
fn successfull_liquidate() {
	new_test_externalities().execute_with(|| {
		Tokens::mint_into(PICA, &ALICE, 1_000_000_000_000_000_000_000).unwrap();
		Balances::mint_into(&ALICE, NativeExistentialDeposit::get() * 3).unwrap();
		<Balances as NativeMutate<_>>::mint_into(&ALICE, NativeExistentialDeposit::get() * 3)
			.unwrap();
		Tokens::mint_into(BTC, &ALICE, 100000000000).unwrap();
		let who = AccountId::from_raw(ALICE.0);
		let amount = 100;
		let order = <Liquidations as Liquidation>::liquidate(
			&who,
			Sell::new(BTC, PICA, 100, Ratio::saturating_from_integer(1)),
			vec![],
		)
		.expect("can creater order for existign currencies if enough of amounts");
		let order =
			pallet_dutch_auction::SellOrders::<Runtime>::get(order).expect("order was placed");
		assert_eq!(order.from_to, who);
		assert_eq!(order.order.take.amount, amount);
	});
}

#[test]
fn do_not_have_amount_to_liquidate() {
	new_test_externalities().execute_with(|| {
		let who = AccountId::from_raw(CHARLIE.0);
		let amount = 100;
		assert!(<Liquidations as Liquidation>::liquidate(
			&who,
			Sell::new(BTC, PICA, 100, Ratio::saturating_from_integer(1)),
			vec![],
		)
		.is_err());
	});
}
