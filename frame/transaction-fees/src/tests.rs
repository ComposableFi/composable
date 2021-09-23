use sp_runtime::Perbill;

use support::weights::{DispatchInfo, PostDispatchInfo};

use crate::mock::*;
use crate::ChargeTransactionFee;
use orml_traits::MultiCurrency;
use primitives::currency::CurrencyId;
use sp_runtime::traits::SignedExtension;

const CALL: Call = Call::Tokens(orml_tokens::Call::transfer(2, CurrencyId::PICA, 10));

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
			let _pre =
				ChargeTransactionFee::<Runtime>::from(0, Perbill::zero(), None)
					.pre_dispatch(&1, &CALL, &pre_dispatch, 10)
					.unwrap();
			let fees = 10 + 10 + 5; // length fee (1:1) + weight fee (1:1) + base weight (1:1)
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 100 - fees);

			assert_eq!(TIP_UNBALANCED_AMOUNT.with(|val| val.borrow().clone()), 0);
			assert_eq!(FEE_UNBALANCED_AMOUNT.with(|val| val.borrow().clone()), 0);
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
			let pre =
				ChargeTransactionFee::<Runtime>::from(0, Perbill::zero(), None)
					.pre_dispatch(&1, &CALL, &info, 10)
					.unwrap();
			let fees = 10 + 10 + 5; // length fee (1:1) + weight fee (1:1) + base weight (1:1)
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 100 - fees);

			assert_eq!(TIP_UNBALANCED_AMOUNT.with(|val| val.borrow().clone()), 0);
			assert_eq!(FEE_UNBALANCED_AMOUNT.with(|val| val.borrow().clone()), 0);

			assert_eq!(
				ChargeTransactionFee::<Runtime>::post_dispatch(pre, &info, &post_info, 10, &Ok(())),
				Ok(())
			);

			assert_eq!(TIP_UNBALANCED_AMOUNT.with(|val| val.borrow().clone()), 0);
			assert_eq!(FEE_UNBALANCED_AMOUNT.with(|val| val.borrow().clone()), 20);
			// actual weight is 5, so 5 should be refunded
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 100 - (fees - 5));
		});
}

#[test]
fn can_swap_to_pay_fees() {
	let info = DispatchInfo { weight: 10, ..Default::default() };
	let post_info = PostDispatchInfo { actual_weight: Some(5), ..Default::default() };

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
			let fees = 10 + 10 + 5; // length fee (1:1) + weight fee (1:1) + base weight (1:1)

			assert_eq!(TIP_UNBALANCED_AMOUNT.with(|val| val.borrow().clone()), 0);
			assert_eq!(FEE_UNBALANCED_AMOUNT.with(|val| val.borrow().clone()), 0);

			assert_eq!(
				ChargeTransactionFee::<Runtime>::post_dispatch(pre, &info, &post_info, 10, &Ok(())),
				Ok(())
			);

			assert_eq!(TIP_UNBALANCED_AMOUNT.with(|val| val.borrow().clone()), 0);
			assert_eq!(FEE_UNBALANCED_AMOUNT.with(|val| val.borrow().clone()), 20);
			// actual weight is 5, so 5 should be refunded
			assert_eq!(Tokens::free_balance(CurrencyId::LAYR, &1), 100 - (fees - 5));
		});
}
