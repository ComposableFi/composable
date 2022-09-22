use crate::{
	self as pallet_liquidations,
	mock::{currency::*, runtime::*},
};
use codec::Encode;
use composable_traits::{
	defi::{Ratio, Sell},
	liquidation::Liquidation,
};
use frame_support::{
	assert_noop,
	traits::{fungible::Mutate as NativeMutate, fungibles::Mutate},
};
use sp_runtime::{FixedPointNumber, FixedU128};

// ensure that we take extra for sell, at least amount to remove
#[test]
fn successful_liquidate() {
	new_test_externalities().execute_with(|| {
		Tokens::mint_into(PICA, &ALICE, 1_000_000_000_000_000_000_000).unwrap();
		Balances::mint_into(&ALICE, NativeExistentialDeposit::get() * 3).unwrap();
		<Balances as NativeMutate<_>>::mint_into(&ALICE, NativeExistentialDeposit::get() * 3)
			.unwrap();
		Tokens::mint_into(KUSD, &ALICE, 100000000000).unwrap();
		let who = AccountId::from_raw(ALICE.0);
		let amount = 100;
		let order = <Liquidations as Liquidation>::liquidate(
			&who,
			Sell::new(KUSD, PICA, 100, Ratio::saturating_from_integer(1)),
			vec![],
		)
		.expect("can creator order for existing currencies if enough of amounts");
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
			Sell::new(KUSD, PICA, amount, Ratio::saturating_from_integer(1)),
			vec![],
		)
		.is_err());
	});
}

/// This is used if we will hard code TX for each network.
#[derive(Encode)]
pub enum LiquidationsCall {
	#[codec(index = 1)]
	Sell(Sell<CurrencyId, u128>, Vec<u128>),
}

#[derive(Encode)]
pub enum ComposableCall {
	#[codec(index = 7)]
	Liquidations(LiquidationsCall),
}

#[test]
fn serde_call() {
	let order = Sell::new(PICA, KUSD, 100, FixedU128::saturating_from_integer(42_u64));
	let sell_typed = Call::Liquidations(pallet_liquidations::Call::<Runtime>::sell {
		order: order.clone(),
		configuration: Default::default(),
	});
	let sell_binary = ComposableCall::Liquidations(LiquidationsCall::Sell(order.clone(), vec![]));
	let sell_binary_flat = composable_traits::liquidation::XcmLiquidation::new(7, 1, order, vec![]);
	assert_eq!(sell_typed.encode(), sell_binary.encode());
	assert_eq!(sell_typed.encode(), sell_binary_flat.encode());
}

#[test]
fn pallet_do_not_treat_infinitely_large_strategies_vector() {
	new_test_externalities().execute_with(|| {
		let manager = ALICE;
		let order = Sell::new(PICA, KUSD, 100, FixedU128::saturating_from_integer(1));
		assert_noop!(
			crate::Pallet::<Runtime>::sell(
				SystemOriginOf::<Runtime>::signed(manager),
				order,
				vec![42; 100_000_000]
			),
			crate::Error::<Runtime>::InvalidLiquidationStrategiesVector,
		);
	});
}

// TODO: add XCM end to end tests with callbacks
