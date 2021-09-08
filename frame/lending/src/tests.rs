use crate::{
	mock::{new_test_ext, AccountId, Lending},
	MarketIndex,
};
use hex_literal::hex;
use composable_traits::rate_model::*;
use sp_runtime::FixedPointNumber;
use sp_runtime::traits::Zero;

#[test]
fn account_id_should_work() {
	new_test_ext().execute_with(|| {
		let market_id = MarketIndex::new(0);
		assert_eq!(
			Lending::account_id(&market_id),
			AccountId::from_raw(hex!(
				"6d6f646c4c656e64696e67210000000000000000000000000000000000000000"
			))
		);
	})
}

#[test]
fn test_calc_utilization_ratio() {
	 // 50% borrow
     assert_eq!(
         Lending::calc_utilization_ratio(&1, &1, &0).unwrap(),
         Ratio::saturating_from_rational(50, 100)
     );
     assert_eq!(
         Lending::calc_utilization_ratio(&100, &100, &0).unwrap(),
         Ratio::saturating_from_rational(50, 100)
     );
     // no borrow
     assert_eq!(
         Lending::calc_utilization_ratio(&1, &0, &0).unwrap(),
         Ratio::zero()
     );
     // full borrow
     assert_eq!(
         Lending::calc_utilization_ratio(&0, &1, &0).unwrap(),
         Ratio::saturating_from_rational(100, 100)
     );
}
