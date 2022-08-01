use super::{create_test_market_input_config, prelude::*};
use crate::currency::*;
use composable_traits::undercollateralized_loans::LoanInput;

#[test]
fn can_create_market() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let manager = *ALICE;
		let origin = Origin::signed(manager);
		let input = create_test_market_input_config();
		let borrow_asset = input.currency_pair.quote;
		let collateral_asset = input.currency_pair.base;
		set_price(borrow_asset, NORMALIZED::ONE);
		set_price(collateral_asset, NORMALIZED::units(50000));
		orml_tokens::Pallet::<Runtime>::mint_into(borrow_asset, &manager, NORMALIZED::units(1000));
		orml_tokens::Pallet::<Runtime>::mint_into(
			collateral_asset,
			&manager,
			NORMALIZED::units(1000),
		);
		assert_ok!(pallet_undercollateralized_loans::Pallet::<Runtime>::create_market(
			origin, input, true
		));
	})
}
