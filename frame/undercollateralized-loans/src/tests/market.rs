use super::{create_test_market_input_config, prelude::*};
use crate::{currency::*, validation::MarketInputIsValid};

// Check that test market can be created successfully.
#[test]
fn can_create_market() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let manager = *ALICE;
		let origin = Origin::signed(manager);
		let input = create_test_market_input_config();
		let borrow_asset = input.borrow_asset;
		let collateral_asset = input.collateral_asset;
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
		// Check if loan's info and config were added to the storage.
		assert_eq!(
			*crate::MarketsStorage::<Runtime>::get(market_account_id)
				.unwrap()
				.config()
				.account_id(),
			market_account_id
		);
		// Check if markets counter has adequate value.
		assert_eq!(crate::MarketsCounterStorage::<Runtime>::get(), 1)
	})
}

// Check if input market config validation works properly.
#[test]
fn test_do_create_market_input_validation() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let valid_market_input_configuration = create_test_market_input_config();
		let borrow_asset = valid_market_input_configuration.borrow_asset;
		let collateral_asset = valid_market_input_configuration.collateral_asset;
		set_price(borrow_asset, NORMALIZED::ONE);
		set_price(collateral_asset, NORMALIZED::units(50000));

		// Original market input configuration is valid.
		assert_ok!(valid_market_input_configuration
			.clone()
			.try_into_validated::<MarketInputIsValid<Oracle, UndercollateralizedLoans>>());

		// Check validation of assets supporting.
	    let invalid_borrow_asset = INVALID::instance().id();	
        let invalid_market_input_configuration =
			MarketInput { borrow_asset: invalid_borrow_asset, ..valid_market_input_configuration.clone() };
		assert_err!(
			invalid_market_input_configuration
				.clone()
				.try_into_validated::<MarketInputIsValid<Oracle, UndercollateralizedLoans>>(),
			"Borrow asset is not supported by oracle."
		);
	    let invalid_collateral_asset = INVALID::instance().id();	
        let invalid_market_input_configuration =
			MarketInput { collateral_asset: invalid_collateral_asset, ..valid_market_input_configuration.clone() };
		assert_err!(
			invalid_market_input_configuration
				.clone()
				.try_into_validated::<MarketInputIsValid<Oracle, UndercollateralizedLoans>>(),
			"Collateral asset is not supported by oracle."
		);

		// Currencies should be different.
	    let invalid_borrow_asset = USDT::instance().id();	
        let invalid_collateral_asset = invalid_borrow_asset;
        let invalid_market_input_configuration =
			MarketInput { borrow_asset: invalid_borrow_asset, collateral_asset: invalid_collateral_asset, ..valid_market_input_configuration.clone() };
		assert_err!(
			invalid_market_input_configuration
				.clone()
				.try_into_validated::<MarketInputIsValid<Oracle, UndercollateralizedLoans>>(),
			"Borrow and collateral currencies are supposed to be different."
		);

		// Check borrowers white-list bound validation.
		// Used market_account_id() method to generate users ids for testing purposes.
		let whitelist = (0..<WhiteListBound as Get<u32>>::get() + 100)
			.map(|num| UndercollateralizedLoans::market_account_id(num))
			.collect();
		let invalid_market_input_configuration =
			MarketInput { whitelist, ..valid_market_input_configuration };
		assert_err!(
			invalid_market_input_configuration
				.try_into_validated::<MarketInputIsValid<Oracle, UndercollateralizedLoans>>(),
			"Borrowers white-list exceeded maximum size."
		);
	});
}
