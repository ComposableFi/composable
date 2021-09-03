use crate::{
	mock::{new_test_ext, AccountId, Lending},
	MarketIndex,
};
use hex_literal::hex;

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
