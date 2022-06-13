use super::prelude::*;

#[test]
fn vault_takes_part_of_borrow_so_cannot_withdraw() {
	new_test_ext().execute_with(|| {
		let (market_id, vault_id) = create_simple_market();
		let initial_total_cash = Lending::total_available_to_be_borrowed(&market_id).unwrap();
		let deposit_usdt = 1_000_000_000;
		let deposit_btc = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, deposit_usdt));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, deposit_btc));

		assert_ok!(Vault::deposit(Origin::signed(*ALICE), vault_id, deposit_btc));
		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(Origin::signed(*ALICE), market_id, deposit_usdt, false),
			Event::Lending(pallet_lending::Event::<Runtime>::CollateralDeposited {
				sender: *ALICE,
				market_id,
				amount: deposit_usdt,
			}),
		);
		assert_noop!(
			Lending::borrow(
				Origin::signed(*ALICE),
				market_id.clone(),
				deposit_btc + initial_total_cash
			),
			Error::<Runtime>::NotEnoughBorrowAsset
		);
		assert_no_event::<Runtime>(Event::Lending(pallet_lending::Event::<Runtime>::Borrowed {
			sender: *ALICE,
			market_id,
			amount: deposit_btc + initial_total_cash,
		}));
	});
}

#[test]
fn test_vault_market_can_withdraw() {
	new_test_ext().execute_with(|| {
		let (market, vault_id) = create_simple_market();

		let collateral = 1_000_000_000_000;
		let borrow = 10;
		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, collateral));
		assert_ok!(Tokens::mint_into(BTC::ID, &ALICE, borrow));

		assert_ok!(Vault::deposit(Origin::signed(*ALICE), vault_id, borrow));

		assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(Origin::signed(*ALICE), market, collateral, false),
			Event::Lending(crate::Event::CollateralDeposited {
				sender: *ALICE,
				amount: collateral,
				market_id: market,
			}),
		);
		test::block::process_and_progress_blocks::<Lending, Runtime>(1);
		// We waited 1 block, the market should have withdraw the funds
		assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(*ALICE), market, borrow - 1),
			Event::Lending(crate::Event::Borrowed {
				sender: *ALICE,
				amount: borrow - 1, // DEFAULT_MARKET_VAULT_RESERVE
				market_id: market,
			}),
		);
	});
}
