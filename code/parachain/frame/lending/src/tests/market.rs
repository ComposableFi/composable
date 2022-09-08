use super::prelude::*;
use crate::{
	tests::{default_create_input, process_and_progress_blocks},
	validation::UpdateInputValid,
	MarketId,
};
use composable_traits::{defi::CurrencyPair, oracle, vault};
use frame_system::{EventRecord, Phase};

#[test]
fn can_update_market() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let manager = *ALICE;
		let origin = Origin::signed(manager);
		// Create a market
		let ((market_id, _), _) = create_simple_vaulted_market(BTC::instance(), manager);
		// Get the market from the storage via market id
		let market = crate::Markets::<Runtime>::get(market_id).unwrap();
		println!("collateral factor: {:?}", market.collateral_factor);
		let update_input = UpdateInput {
			collateral_factor: market.collateral_factor,
			under_collateralized_warn_percent: market.under_collateralized_warn_percent,
			liquidators: market.liquidators.clone(),
			max_price_age: market.max_price_age,
		};
		let updated = Lending::update_market(origin, market_id, update_input.clone());
		// check if the market was successfully updated
		assert_ok!(updated);
		let market_updated_event: crate::Event<Runtime> =
			crate::Event::MarketUpdated { market_id, input: update_input };
		// check if the event was emitted
		System::assert_has_event(Event::Lending(market_updated_event));

		// validation on input fails as it has collateral_factor less than one
		let update_input = UpdateInput {
			collateral_factor: FixedU128::from_float(0.5),
			under_collateralized_warn_percent: market.under_collateralized_warn_percent,
			liquidators: market.liquidators,
			max_price_age: market.max_price_age,
		};
		assert_err!(
			update_input.try_into_validated::<UpdateInputValid>(),
			"Collateral factor must be more than one."
		);
	})
}

#[test]
/// Tests market creation and the associated event(s).
fn can_create_valid_market() {
	new_test_ext().execute_with(|| {
            System::set_block_number(1); // ensure block is non-zero

            /// The amount of the borrow asset to mint into ALICE.
            const INITIAL_BORROW_ASSET_AMOUNT: u128 = 10_u128.pow(30);

            const BORROW_ASSET_ID: u128 = BTC::ID;
            const COLLATERAL_ASSET_ID: u128 = USDT::ID;
            const EXPECTED_AMOUNT_OF_BORROW_ASSET: u128 = 50_000 * USDT::ONE;

            let config = default_create_input(CurrencyPair::new(COLLATERAL_ASSET_ID, BORROW_ASSET_ID));

            set_price(BORROW_ASSET_ID, EXPECTED_AMOUNT_OF_BORROW_ASSET);
            set_price(COLLATERAL_ASSET_ID, USDT::ONE);

            let price = <Oracle as oracle::Oracle>::get_price(BORROW_ASSET_ID, BTC::ONE)
                .expect("impossible")
                .price;

            assert_eq!(price, EXPECTED_AMOUNT_OF_BORROW_ASSET);

            let should_have_failed = Lending::create_market(
                Origin::signed(*ALICE),
                config.clone(),
                false,
            );

            assert!(
                matches!(
                    should_have_failed,
                    Err(DispatchErrorWithPostInfo {
                        post_info: PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },
                        error: DispatchError::Module(ModuleError {
                            index: _, // not important in mock runtime
                            error: _, // not important in mock runtime
                            message: Some(error)
                        }),
                    }) if Into::<&'static str>::into(orml_tokens::Error::<Runtime>::BalanceTooLow) == error
                ),
                "Creating a market with insufficient funds should fail, with the error message being \"BalanceTooLow\".
                The other fields are also checked to make sure any changes are tested and accounted for, perhaps one of those fields changed?
                Market creation result was {:#?}",
                should_have_failed
            );

            Tokens::mint_into(BORROW_ASSET_ID, &*ALICE, INITIAL_BORROW_ASSET_AMOUNT).unwrap();
            let manager = *ALICE;
            let origin = Origin::signed(manager);
            let input = config.clone();

            let should_be_created = Lending::create_market(origin, config, false);

            assert!(
                matches!(should_be_created, Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },)),
                "Market creation should have succeeded, since ALICE now has BTC.
                Market creation result was {:#?}",
                should_be_created,
            );

            //  Check if corresponded event was emitted
            let currency_pair = input.currency_pair;
            // Market id and vault id values are defined via previous logic.
            let market_id = pallet_lending::pallet::MarketId::new(1);
            let vault_id = 1;
            let market_created_event = crate::Event::MarketCreated {market_id, vault_id, manager, currency_pair};
            System::assert_has_event(Event::Lending(market_created_event));

            let initial_market_volume = Lending::calculate_initial_market_volume(BORROW_ASSET_ID).unwrap();
            let alice_balance_after_market_creation = Tokens::balance(BORROW_ASSET_ID, &*ALICE);

            assert_eq!(
                alice_balance_after_market_creation,
                INITIAL_BORROW_ASSET_AMOUNT - initial_market_volume,
                "ALICE should have 'paid' the initial_pool_size into the market vault.
                alice_balance_after_market_creation: {alice_balance_after_market_creation}
                initial_market_volume: {initial_market_volume}",
                alice_balance_after_market_creation = alice_balance_after_market_creation,
                initial_market_volume = initial_market_volume,
            );

            let system_events = System::events();

            match &*system_events {
                [_, _, _, _, _, EventRecord {
                    topics: event_topics,
                    phase: Phase::Initialization,
                    event:
                        Event::Lending(crate::Event::MarketCreated {
                            currency_pair:
                                CurrencyPair { base: COLLATERAL_ASSET_ID, quote: BORROW_ASSET_ID },
                            market_id: created_market_id @ MarketId(1),
                            vault_id: created_vault_id @ 1,
                            manager: event_manager,
                        }),
                }] if event_manager == &*ALICE && event_topics.is_empty() => {
                    assert_eq!(
                        Lending::total_available_to_be_borrowed(&created_market_id).unwrap(),
                        initial_market_volume,
                        "The market should have {} in it.",
                        initial_market_volume,
                    );

                    assert_eq!(
                        <Vault as vault::Vault>::asset_id(&created_vault_id).unwrap(),
                        BORROW_ASSET_ID,
                        "The created market vault should be backed by the borrow asset"
                    );

                    // REVIEW: Review this test
                    let alice_total_debt_with_interest = Tokens::balance(Lending::get_assets_for_market(&created_market_id).unwrap().debt_asset, &ALICE);
                    assert_eq!(
                        alice_total_debt_with_interest,
                        0,
                        "The borrowed balance of ALICE should be 0. Found {:#?}",
                        alice_total_debt_with_interest
                    );
                },
                _ => panic!(
                    "Unexpected value for System::events(); found {:#?}",
                    system_events
                ),
            }
        });
}

#[test]
fn can_create_valid_market_with_keep_alive() {
	new_test_ext().execute_with(|| {
            System::set_block_number(1); // ensure block is non-zero

            /// The amount of the borrow asset to mint into ALICE.
            const INITIAL_BORROW_ASSET_AMOUNT: u128 = 10_u128.pow(30);

            const BORROW_ASSET_ID: u128 = BTC::ID;
            const COLLATERAL_ASSET_ID: u128 = USDT::ID;
            const EXPECTED_AMOUNT_OF_BORROW_ASSET: u128 = 50_000 * USDT::ONE;

            let config = default_create_input(CurrencyPair::new(COLLATERAL_ASSET_ID, BORROW_ASSET_ID));

            set_price(BORROW_ASSET_ID, EXPECTED_AMOUNT_OF_BORROW_ASSET);
            set_price(COLLATERAL_ASSET_ID, USDT::ONE);

            let price = <Oracle as oracle::Oracle>::get_price(BORROW_ASSET_ID, BTC::ONE)
                .expect("impossible")
                .price;

            assert_eq!(price, EXPECTED_AMOUNT_OF_BORROW_ASSET);

            let should_have_failed = Lending::create_market(
                Origin::signed(*ALICE),
                config.clone(),
                true,
            );

            assert!(
                matches!(
                    should_have_failed,
                    Err(DispatchErrorWithPostInfo {
                        post_info: PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },
                        error: DispatchError::Module(ModuleError {
                            index: _, // not important in mock runtime
                            error: _, // not important in mock runtime
                            message: Some(error)
                        }),
                    }) if Into::<&'static str>::into(orml_tokens::Error::<Runtime>::BalanceTooLow) == error
                ),
                "Creating a market with insufficient funds should fail, with the error message being \"BalanceTooLow\".
                The other fields are also checked to make sure any changes are tested and accounted for, perhaps one of those fields changed?
                Market creation result was {:#?}",
                should_have_failed
            );

            Tokens::mint_into(BORROW_ASSET_ID, &*ALICE, INITIAL_BORROW_ASSET_AMOUNT).unwrap();
            let manager = *ALICE;
            let origin = Origin::signed(manager);
            let input = config.clone();

            let should_be_created = Lending::create_market(origin, config, true);

            assert!(
                matches!(should_be_created, Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },)),
                "Market creation should have succeeded, since ALICE now has BTC.
                Market creation result was {:#?}",
                should_be_created,
            );

            //  Check if corresponded event was emitted
            let currency_pair = input.currency_pair;
            // Market id and vault id values are defined via previous logic.
            let market_id = pallet_lending::pallet::MarketId::new(1);
            let vault_id = 1;
            let market_created_event = crate::Event::MarketCreated {market_id, vault_id, manager, currency_pair};
            System::assert_has_event(Event::Lending(market_created_event));

            let initial_market_volume = Lending::calculate_initial_market_volume(BORROW_ASSET_ID).unwrap();
            let alice_balance_after_market_creation = Tokens::balance(BORROW_ASSET_ID, &*ALICE);

            assert_eq!(
                alice_balance_after_market_creation,
                INITIAL_BORROW_ASSET_AMOUNT - initial_market_volume,
                "ALICE should have 'paid' the initial_pool_size into the market vault.
                alice_balance_after_market_creation: {alice_balance_after_market_creation}
                initial_market_volume: {initial_market_volume}",
                alice_balance_after_market_creation = alice_balance_after_market_creation,
                initial_market_volume = initial_market_volume,
            );

            let system_events = System::events();

            match &*system_events {
                [_, _, _, _, _, EventRecord {
                    topics: event_topics,
                    phase: Phase::Initialization,
                    event:
                        Event::Lending(crate::Event::MarketCreated {
                            currency_pair:
                                CurrencyPair { base: COLLATERAL_ASSET_ID, quote: BORROW_ASSET_ID },
                            market_id: created_market_id @ MarketId(1),
                            vault_id: created_vault_id @ 1,
                            manager: event_manager,
                        }),
                }] if event_manager == &*ALICE && event_topics.is_empty() => {
                    assert_eq!(
                        Lending::total_available_to_be_borrowed(&created_market_id).unwrap(),
                        initial_market_volume,
                        "The market should have {} in it.",
                        initial_market_volume,
                    );

                    assert_eq!(
                        <Vault as vault::Vault>::asset_id(&created_vault_id).unwrap(),
                        BORROW_ASSET_ID,
                        "The created market vault should be backed by the borrow asset"
                    );

                    // REVIEW: Review this test
                    let alice_total_debt_with_interest = Tokens::balance(Lending::get_assets_for_market(&created_market_id).unwrap().debt_asset, &ALICE);
                    assert_eq!(
                        alice_total_debt_with_interest,
                        0,
                        "The borrowed balance of ALICE should be 0. Found {:#?}",
                        alice_total_debt_with_interest
                    );
                },
                _ => panic!(
                    "Unexpected value for System::events(); found {:#?}",
                    system_events
                ),
            }
        });
}

prop_compose! {
	fn valid_amount_without_overflow()
		(x in MINIMUM_BALANCE..u64::MAX as Balance) -> Balance {
		x
	}
}

prop_compose! {
	fn valid_amounts_without_overflow_2()
		(x in MINIMUM_BALANCE..u64::MAX as Balance / 2,
		y in MINIMUM_BALANCE..u64::MAX as Balance / 2) -> (Balance, Balance) {
			(x, y)
	}
}

prop_compose! {
	fn valid_amounts_without_overflow_3()
		(x in MINIMUM_BALANCE..u64::MAX as Balance / 3,
		y in MINIMUM_BALANCE..u64::MAX as Balance / 3,
		z in MINIMUM_BALANCE..u64::MAX as Balance / 3) -> (Balance, Balance, Balance) {
			(x, y, z)
		}
}

prop_compose! {
	fn valid_amounts_without_overflow_k
		(max_accounts: usize, limit: Balance)
		(balances in prop::collection::vec(MINIMUM_BALANCE..limit, 3..max_accounts))
		-> Vec<(AccountId, Balance)> {
			let mut result = Vec::with_capacity(balances.len());
			let mut account = U256::from_little_endian(UNRESERVED.as_ref());
			let mut buffer = [0; 32];
			for balance in balances {
				account += U256::one();
				account.to_little_endian(&mut buffer);
				result.push((AccountId::from_raw(buffer), balance))
			};
			result
		}
}

prop_compose! {
	fn valid_amounts_without_overflow_k_with_random_index(max_accounts: usize, limit: Balance)
		(accounts in valid_amounts_without_overflow_k(max_accounts, limit),
		index in 1..max_accounts) -> (usize, Vec<(AccountId, Balance)>) {
			(usize::max(1, index % usize::max(1, accounts.len())), accounts)
		}
}

prop_compose! {
	fn strategy_account()
		(x in u128::from(U256::from_little_endian(UNRESERVED.as_ref()).low_u64())..u128::MAX) -> AccountId {
			let mut account = U256::from_little_endian(UNRESERVED.as_ref());
			account += x.into();
			let mut buffer = [0; 32];
			account.to_little_endian(&mut buffer);
			AccountId::from_raw(buffer)
		}
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(10_000))]

	#[test]
	fn market_collateral_deposit_withdraw_identity(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let (market, _) = create_simple_market();
			let before = Tokens::balance( BTC::ID, &ALICE);
			prop_assert_ok!(Tokens::mint_into( BTC::ID, &ALICE, amount));
			prop_assert_ok!(Lending::deposit_collateral(Origin::signed(*ALICE), market, amount, false));
			let event =
				Event::Lending(crate::Event::CollateralDeposited {
					sender: *ALICE,
					amount,
					market_id: market,
				});
			System::assert_last_event(event);

			prop_assert_ok!(Lending::withdraw_collateral(Origin::signed(*ALICE), market, amount));
			let event =
				Event::Lending(crate::Event::CollateralWithdrawn {
					sender: *ALICE,
					amount,
					market_id: market,
				});
			System::assert_last_event(event);
			prop_assert_eq!(Tokens::balance( BTC::ID, &ALICE) - before, amount);

			Ok(())
		})?;
	}

	#[test]
	fn market_collateral_deposit_withdraw_higher_amount_fails(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let (market, _vault) = create_simple_market();
			prop_assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, amount));
			prop_assert_ok!(Lending::deposit_collateral(Origin::signed(*ALICE), market, amount, false));
			let event =
				Event::Lending(crate::Event::CollateralDeposited {
					sender: *ALICE,
					amount,
					market_id: market,
				});
			System::assert_last_event(event);

			prop_assert_eq!(
				Lending::withdraw_collateral(Origin::signed(*ALICE), market, amount + 1),
				Err(Error::<Runtime>::NotEnoughCollateralToWithdraw.into())
			);
			let event =
				Event::Lending(crate::Event::CollateralWithdrawn {
					sender: *ALICE,
					amount: amount + 1,
					market_id: market,
				});
			assert_no_event::<Runtime>(event);

			Ok(())
		})?;
	}

	#[test]
	fn market_collateral_vaulted_deposit_withdraw_identity(amount in valid_amount_without_overflow()) {
		new_test_ext().execute_with(|| {
			let ((market, _), collateral_asset) = create_simple_vaulted_market(BTC::instance(), *ALICE);
			let before = Tokens::balance(collateral_asset, &ALICE);
			prop_assert_ok!(Tokens::mint_into(collateral_asset, &ALICE, amount));
			prop_assert_ok!(Lending::deposit_collateral(Origin::signed(*ALICE), market, amount , false));
			let event =
				Event::Lending(crate::Event::CollateralDeposited {
					sender: *ALICE,
					amount,
					market_id: market,
				});
			System::assert_last_event(event);
			prop_assert_ok!(Lending::withdraw_collateral(Origin::signed(*ALICE), market, amount));
			let event =
				Event::Lending(crate::Event::CollateralWithdrawn {
					sender: *ALICE,
					amount,
					market_id: market,
				});
			System::assert_last_event(event);
			prop_assert_eq!(Tokens::balance(collateral_asset, &ALICE) - before, amount);

			Ok(())
		})?;
	}

	#[test]
	fn market_are_isolated(
		(amount1, amount2) in valid_amounts_without_overflow_2()
	) {
		new_test_ext().execute_with(|| {
			let (market_id1, vault_id1) = create_simple_market();
			let m1 = Tokens::balance(USDT::ID, &Lending::account_id(&market_id1));
			let (market_id2, vault_id2) = create_simple_market();
			let m2 = Tokens::balance(USDT::ID, &Lending::account_id(&market_id2));

			prop_assert_ne!(market_id1, market_id2);
			prop_assert_ne!(Lending::account_id(&market_id1), Lending::account_id(&market_id2));

			prop_assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, amount1));
			prop_assert_ok!(Vault::deposit(Origin::signed(*ALICE), vault_id1, amount1));

			prop_assert_ok!(Tokens::mint_into(USDT::ID, &BOB, 10*amount2));
			prop_assert_ok!(Vault::deposit(Origin::signed(*BOB), vault_id2, 10*amount2));

		process_and_progress_blocks::<Lending, Runtime>(1);

			let expected_market1_balance = DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(amount1);
			let expected_market2_balance = DEFAULT_MARKET_VAULT_STRATEGY_SHARE.mul(10*amount2);

			prop_assert_acceptable_computation_error!(
				Tokens::balance(USDT::ID, &Lending::account_id(&market_id1)) - m1,
				expected_market1_balance
			);
			prop_assert_acceptable_computation_error!(
				Tokens::balance(USDT::ID, &Lending::account_id(&market_id2)) - m2,
				expected_market2_balance
			);

			Ok(())
		})?;
	}
}
