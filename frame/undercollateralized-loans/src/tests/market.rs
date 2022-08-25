use super::{create_test_market_input_config, prelude::*};
use crate::currency::*;
use crate::validation::MarketInputIsValid;
use composable_traits::undercollateralized_loans::{
    UndercollateralizedLoans as UndercollateralizedLoansTrait,
    MarketInput,
};

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
		Tokens::mint_into(borrow_asset, &manager, NORMALIZED::units(1000)).unwrap();
		Tokens::mint_into(collateral_asset, &manager, NORMALIZED::units(1000)).unwrap();
	    // Check if market was created.	
        assert_ok!(UndercollateralizedLoans::create_market(origin, input, true));
        // Check if corresponded event was emitted.
        let market_account_id = UndercollateralizedLoans::market_account_id(1);
        let event = crate::Event::MarketCreated { market_account_id };
        System::assert_has_event(Event::UndercollateralizedLoans(event));
	})
}

#[test] 
fn test_do_create_market_input_validation() {
    let market_input_configuration = create_test_market_input_config();
    // Original market input configuration is valid. 
    market_input_configuration.try_into_validated::<MarketInputIsValid<Oracle, UndercollateralizedLoans>>();
    
   
}
