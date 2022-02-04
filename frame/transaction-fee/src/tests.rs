use sp_runtime::{FixedPointNumber, Perbill};

use support::{
	assert_noop, assert_ok,
	weights::{DispatchInfo, GetDispatchInfo, PostDispatchInfo},
};

use crate::{fee_adjustment::Multiplier, mock::*, ChargeTransactionFee, NextFeeMultiplier, Pallet};
use orml_traits::MultiCurrency;
use pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo;
use primitives::currency::CurrencyId;
use sp_runtime::{
	testing::TestXt,
	traits::{One, SignedExtension},
};
use support::{dispatch::Weight, pallet_prelude::*};

const CALL: Call = Call::Tokens(orml_tokens::Call::transfer {
	dest: 2,
	currency_id: CurrencyId::PICA,
	amount: 10,
});

#[test]
fn can_pay_fees_easily() {
	let pre_dispatch = DispatchInfo { weight: 10, ..Default::default() };
	ExtBuilder::default()
		// all accounts start with 100 units
		.balance_factor((CurrencyId::LAYR, 100))
		// our configured extrinsic base weight
		.base_weight(5)
		.build()
		.execute_with(|| {
			let _pre = ChargeTransactionFee::<Runtime>::from(0, Perbill::zero(), None)
				.pre_dispatch(&1, &CALL, &pre_dispatch, 10)
				.unwrap();
			let fees = 10 + 10 + 5; // length fee (1:1) + weight fee (1:1) + base weight (1:1)
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 100 - fees);

			assert_eq!(TIP_UNBALANCED_AMOUNT.with(|val| *val.borrow()), 0);
			assert_eq!(FEE_UNBALANCED_AMOUNT.with(|val| *val.borrow()), 0);
		});
}

#[test]
fn refund_on_post_dispatch() {
	let info = DispatchInfo { weight: 10, ..Default::default() };
	let post_info = PostDispatchInfo { actual_weight: Some(5), ..Default::default() };

	ExtBuilder::default()
		// all accounts start with 100 units
		.balance_factor((CurrencyId::LAYR, 100))
		// our configured extrinsic base weight
		.base_weight(5)
		.build()
		.execute_with(|| {
			let pre = ChargeTransactionFee::<Runtime>::from(0, Perbill::zero(), None)
				.pre_dispatch(&1, &CALL, &info, 10)
				.unwrap();
			let fees = 10 + 10 + 5; // length fee (1:1) + weight fee (1:1) + base weight (1:1)
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 100 - fees);

			assert_eq!(TIP_UNBALANCED_AMOUNT.with(|val| *val.borrow()), 0);
			assert_eq!(FEE_UNBALANCED_AMOUNT.with(|val| *val.borrow()), 0);

			assert_eq!(
				ChargeTransactionFee::<Runtime>::post_dispatch(
					Some(pre),
					&info,
					&post_info,
					10,
					&Ok(())
				),
				Ok(())
			);

			assert_eq!(TIP_UNBALANCED_AMOUNT.with(|val| *val.borrow()), 0);
			assert_eq!(FEE_UNBALANCED_AMOUNT.with(|val| *val.borrow()), 20);
			// actual weight is 5, so 5 should be refunded
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 100 - (fees - 5));
		});
}

#[test]
fn can_swap_to_pay_fees() {
	let info = DispatchInfo { weight: 10, ..Default::default() };
	let post_info = PostDispatchInfo { actual_weight: Some(10), ..Default::default() };

	ExtBuilder::default()
		// all accounts start with 100 units
		.balance_factor((CurrencyId::PICA, 100))
		// our configured extrinsic base weight
		.base_weight(5)
		.build()
		.execute_with(|| {
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 0);
			let pre =
				ChargeTransactionFee::<Runtime>::from(0, Perbill::zero(), Some(CurrencyId::PICA))
					.pre_dispatch(&1, &CALL, &info, 10)
					.unwrap();

			assert_eq!(TIP_UNBALANCED_AMOUNT.with(|val| *val.borrow()), 0);
			assert_eq!(FEE_UNBALANCED_AMOUNT.with(|val| *val.borrow()), 0);

			assert_eq!(
				ChargeTransactionFee::<Runtime>::post_dispatch(
					Some(pre),
					&info,
					&post_info,
					10,
					&Ok(())
				),
				Ok(())
			);

			// assert that swap succeeded and was used to pay fees
			assert_eq!(TIP_UNBALANCED_AMOUNT.with(|val| *val.borrow()), 0);
			assert_eq!(FEE_UNBALANCED_AMOUNT.with(|val| *val.borrow()), 25);
			// assert that user now has minimum layr deposit
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 1);
		});
}

#[test]
fn compute_fee_does_not_overflow() {
	ExtBuilder::default().base_weight(100).byte_fee(10).build().execute_with(|| {
		// Overflow is handled
		let dispatch_info = DispatchInfo {
			weight: Weight::MAX,
			class: DispatchClass::Operational,
			pays_fee: Pays::Yes,
		};
		assert_eq!(
			Pallet::<Runtime>::compute_fee(<u32>::MAX, &dispatch_info, <u64>::MAX),
			<u64>::MAX
		);
	});
}

#[test]
fn signed_extension_transaction_payment_is_bounded() {
	let info = DispatchInfo { weight: Weight::MAX, ..Default::default() };
	ExtBuilder::default()
		.balance_factor((CurrencyId::LAYR, 10000))
		.byte_fee(0)
		.build()
		.execute_with(|| {
			// maximum weight possible
			assert!(matches!(
				ChargeTransactionFee::<Runtime>::from(0, Perbill::zero(), None)
					.pre_dispatch(&1, &CALL, &info, 10),
				Ok(_)
			));
			// fee will be proportional to what is the actual maximum weight in the runtime.
			assert_eq!(
				Tokens::free_balance(CurrencyId::LAYR, &1),
				(10000 - <Runtime as system::Config>::BlockWeights::get().max_block) as u64
			);
		});
}

#[test]
#[ignore]
fn signed_extension_allows_free_transactions() {
	ExtBuilder::default()
		.balance_factor((CurrencyId::LAYR, 0))
		.byte_fee(0)
		.build()
		.execute_with(|| {
			// 1 ain't have a penny.
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 0);

			let len = 100;

			// This is a completely free (and thus wholly insecure/DoS-ridden) transaction.
			let operational =
				DispatchInfo { weight: 0, class: DispatchClass::Operational, pays_fee: Pays::No };
			assert_ok!(ChargeTransactionFee::<Runtime>::from(0, Perbill::zero(), None).validate(
				&1,
				&CALL,
				&operational,
				len
			));
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 0);

			// like a InsecureFreeNormal
			let free =
				DispatchInfo { weight: 0, class: DispatchClass::Normal, pays_fee: Pays::Yes };
			assert_noop!(
				ChargeTransactionFee::<Runtime>::from(0, Perbill::zero(), None)
					.validate(&1, &CALL, &free, len),
				TransactionValidityError::Invalid(InvalidTransaction::Payment),
			);
		});
}

#[test]
fn signed_ext_length_fee_is_also_updated_per_congestion() {
	ExtBuilder::default()
		.base_weight(5)
		.balance_factor((CurrencyId::LAYR, 100))
		.build()
		.execute_with(|| {
			// all fees should be x1.5
			<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(3, 2));
			let len = 10;

			assert!(matches!(
				ChargeTransactionFee::<Runtime>::from(10, Perbill::zero(), None) // tipped
					.pre_dispatch(&1, &CALL, &info_from_weight(3), len),
				Ok(_)
			));
			assert_eq!(
				Tokens::free_balance(CurrencyId::LAYR, &1),
				100 // original
					- 10 // tip
					- 5 // base
					- 10 // len
					- (3 * 3 / 2) // adjusted weight
			);
		})
}

#[test]
fn query_info_works() {
	let origin = 111111;
	let extra = ();
	let xt = TestXt::new(CALL, Some((origin, extra)));
	let info = xt.get_dispatch_info();
	let ext = xt.encode();
	let len = ext.len() as u32;
	ExtBuilder::default().base_weight(5).weight_fee(2).build().execute_with(|| {
		// all fees should be x1.5
		<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(3, 2));

		assert_eq!(
			TransactionPayment::query_info(xt, len),
			RuntimeDispatchInfo {
				weight: info.weight,
				class: info.class,
				partial_fee: 5 * 2 /* base * weight_fee */
					+ len as u64  /* len * 1 */
					+ info.weight.min(BlockWeights::get().max_block) as u64 * 2 * 3 / 2 /* weight */
			},
		);
	});
}

#[test]
fn compute_fee_works_without_multiplier() {
	ExtBuilder::default().base_weight(100).byte_fee(10).build().execute_with(|| {
		// Next fee multiplier is zero
		assert_eq!(<NextFeeMultiplier<Runtime>>::get(), Multiplier::one());

		// Tip only, no fees works
		let dispatch_info =
			DispatchInfo { weight: 0, class: DispatchClass::Operational, pays_fee: Pays::No };
		assert_eq!(Pallet::<Runtime>::compute_fee(0, &dispatch_info, 10), 10);
		// No tip, only base fee works
		let dispatch_info =
			DispatchInfo { weight: 0, class: DispatchClass::Operational, pays_fee: Pays::Yes };
		assert_eq!(Pallet::<Runtime>::compute_fee(0, &dispatch_info, 0), 100);
		// Tip + base fee works
		assert_eq!(Pallet::<Runtime>::compute_fee(0, &dispatch_info, 69), 169);
		// Len (byte fee) + base fee works
		assert_eq!(Pallet::<Runtime>::compute_fee(42, &dispatch_info, 0), 520);
		// Weight fee + base fee works
		let dispatch_info =
			DispatchInfo { weight: 1000, class: DispatchClass::Operational, pays_fee: Pays::Yes };
		assert_eq!(Pallet::<Runtime>::compute_fee(0, &dispatch_info, 0), 1100);
	});
}

#[test]
fn compute_fee_works_with_multiplier() {
	ExtBuilder::default().base_weight(100).byte_fee(10).build().execute_with(|| {
		// Add a next fee multiplier. Fees will be x3/2.
		<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(3, 2));
		// Base fee is unaffected by multiplier
		let dispatch_info =
			DispatchInfo { weight: 0, class: DispatchClass::Operational, pays_fee: Pays::Yes };
		assert_eq!(Pallet::<Runtime>::compute_fee(0, &dispatch_info, 0), 100);

		// Everything works together :)
		let dispatch_info =
			DispatchInfo { weight: 123, class: DispatchClass::Operational, pays_fee: Pays::Yes };
		// 123 weight, 456 length, 100 base
		assert_eq!(
			Pallet::<Runtime>::compute_fee(456, &dispatch_info, 789),
			100 + (3 * 123 / 2) + 4560 + 789,
		);
	});
}

#[test]
fn compute_fee_works_with_negative_multiplier() {
	ExtBuilder::default().base_weight(100).byte_fee(10).build().execute_with(|| {
		// Add a next fee multiplier. All fees will be x1/2.
		<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(1, 2));

		// Base fee is unaffected by multiplier.
		let dispatch_info =
			DispatchInfo { weight: 0, class: DispatchClass::Operational, pays_fee: Pays::Yes };
		assert_eq!(Pallet::<Runtime>::compute_fee(0, &dispatch_info, 0), 100);

		// Everything works together.
		let dispatch_info =
			DispatchInfo { weight: 123, class: DispatchClass::Operational, pays_fee: Pays::Yes };
		// 123 weight, 456 length, 100 base
		assert_eq!(
			Pallet::<Runtime>::compute_fee(456, &dispatch_info, 789),
			100 + (123 / 2) + 4560 + 789,
		);
	});
}

#[test]
fn refund_does_not_recreate_account() {
	ExtBuilder::default()
		.balance_factor((CurrencyId::LAYR, 200))
		.base_weight(5)
		.build()
		.execute_with(|| {
			// So events are emitted
			System::set_block_number(10);
			let len = 10;
			let info = DispatchInfo { weight: 100, ..Default::default() };
			let post = PostDispatchInfo { actual_weight: Some(50), ..Default::default() };
			let pre =
				ChargeTransactionFee::<Runtime>::from(5 /* tipped */, Perbill::zero(), None)
					.pre_dispatch(&1, &CALL, &info, len)
					.unwrap();
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 200 - 5 - 10 - 100 - 5);

			// kill the account between pre and post dispatch
			assert_ok!(Tokens::transfer(
				Some(1).into(),
				2,
				CurrencyId::LAYR,
				Tokens::free_balance(CurrencyId::LAYR, &1)
			));
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 0);

			assert_ok!(ChargeTransactionFee::<Runtime>::post_dispatch(
				Some(pre),
				&info,
				&post,
				len,
				&Ok(())
			));
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 0);
			// Transfer Event
			System::assert_has_event(Event::Tokens(orml_tokens::Event::Transfer {
				currency_id: CurrencyId::LAYR,
				from: 1,
				to: 2,
				amount: 80,
			}));
			// Killed Event
			System::assert_has_event(Event::System(system::Event::KilledAccount { account: 1 }));
		});
}

#[test]
fn actual_weight_higher_than_max_refunds_nothing() {
	ExtBuilder::default()
		.balance_factor((CurrencyId::LAYR, 200))
		.base_weight(5)
		.build()
		.execute_with(|| {
			let len = 10;
			let info = DispatchInfo { weight: 100, ..Default::default() };
			let post = PostDispatchInfo { actual_weight: Some(101), ..Default::default() };
			let pre =
				ChargeTransactionFee::<Runtime>::from(5 /* tipped */, Perbill::zero(), None)
					.pre_dispatch(&2, &CALL, &info, len)
					.unwrap();
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &2), 200 - 5 - 10 - 100 - 5);

			assert_ok!(ChargeTransactionFee::<Runtime>::post_dispatch(
				Some(pre),
				&info,
				&post,
				len,
				&Ok(())
			));
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &2), 200 - 5 - 10 - 100 - 5);
		});
}

#[test]
fn zero_transfer_on_free_transaction() {
	ExtBuilder::default()
		.balance_factor((CurrencyId::LAYR, 100))
		.base_weight(5)
		.build()
		.execute_with(|| {
			// So events are emitted
			System::set_block_number(10);
			let len = 10;
			let dispatch_info =
				DispatchInfo { weight: 100, pays_fee: Pays::No, class: DispatchClass::Normal };
			let user = 69;
			let pre = ChargeTransactionFee::<Runtime>::from(0, Default::default(), None)
				.pre_dispatch(&user, &CALL, &dispatch_info, len)
				.unwrap();
			assert_eq!(Tokens::total_balance(CurrencyId::LAYR, &user), 0);
			assert_ok!(ChargeTransactionFee::<Runtime>::post_dispatch(
				Some(pre),
				&dispatch_info,
				&Default::default(),
				len,
				&Ok(())
			));
			assert_eq!(Tokens::total_balance(CurrencyId::LAYR, &user), 0);
			// No events for such a scenario
			assert_eq!(System::events().len(), 0);
		});
}

#[test]
fn refund_consistent_with_actual_weight() {
	ExtBuilder::default()
		.balance_factor((CurrencyId::LAYR, 1000))
		.base_weight(7)
		.build()
		.execute_with(|| {
			let info = DispatchInfo { weight: 100, ..Default::default() };
			let post_info = PostDispatchInfo { actual_weight: Some(33), ..Default::default() };
			let prev_balance = Tokens::free_balance(CurrencyId::LAYR, &2);
			let len = 10;
			let tip = 5;

			<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(5, 4));

			let pre = ChargeTransactionFee::<Runtime>::from(tip, Default::default(), None)
				.pre_dispatch(&2, &CALL, &info, len)
				.unwrap();

			ChargeTransactionFee::<Runtime>::post_dispatch(
				Some(pre),
				&info,
				&post_info,
				len,
				&Ok(()),
			)
			.unwrap();

			let refund_based_fee = prev_balance - Tokens::free_balance(CurrencyId::LAYR, &2);
			let actual_fee =
				Pallet::<Runtime>::compute_actual_fee(len as u32, &info, &post_info, tip);

			// 33 weight, 10 length, 7 base, 5 tip
			assert_eq!(actual_fee, 7 + 10 + (33 * 5 / 4) + 5);
			assert_eq!(refund_based_fee, actual_fee);
		});
}

#[test]
fn post_info_can_change_pays_fee() {
	ExtBuilder::default()
		.balance_factor((CurrencyId::LAYR, 1000))
		.base_weight(7)
		.build()
		.execute_with(|| {
			let info = DispatchInfo { weight: 100, ..Default::default() };
			let post_info = PostDispatchInfo { pays_fee: Pays::No, ..Default::default() };
			let prev_balance = Tokens::free_balance(CurrencyId::LAYR, &2);
			let len = 10;
			let tip = 5;

			<NextFeeMultiplier<Runtime>>::put(Multiplier::saturating_from_rational(5, 4));

			let pre = ChargeTransactionFee::<Runtime>::from(tip, Default::default(), None)
				.pre_dispatch(&2, &CALL, &info, len)
				.unwrap();

			ChargeTransactionFee::<Runtime>::post_dispatch(
				Some(pre),
				&info,
				&post_info,
				len,
				&Ok(()),
			)
			.unwrap();

			let refund_based_fee = prev_balance - Tokens::free_balance(CurrencyId::LAYR, &2);
			let actual_fee =
				Pallet::<Runtime>::compute_actual_fee(len as u32, &info, &post_info, tip);

			// Only 5 tip is paid
			assert_eq!(actual_fee, 5);
			assert_eq!(refund_based_fee, actual_fee);
		});
}
