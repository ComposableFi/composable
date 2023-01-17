use super::prelude::*;
use crate::tests::process_and_progress_blocks;
use composable_traits::lending::TotalDebtWithInterest;

#[test]
fn test_repay_partial_amount() {
	new_test_ext().execute_with(|| {
		type COLLATERAL = BTC;
		type BORROW = USDT;

		// accounts have 1 unit of collateral
		let alice_balance = COLLATERAL::ONE;

		let (market_index, vault_id) = create_simple_market();

		mint_and_deposit_collateral::<Runtime>(*ALICE, alice_balance, market_index, COLLATERAL::ID);

		let borrow_asset_deposit = BORROW::units(1_000_000);
		assert_ok!(Tokens::mint_into(BORROW::ID, &CHARLIE, borrow_asset_deposit));
		assert_extrinsic_event::<Runtime>(
			Vault::deposit(RuntimeOrigin::signed(*CHARLIE), vault_id, borrow_asset_deposit),
			RuntimeEvent::Vault(pallet_vault::Event::<Runtime>::Deposited {
				account: *CHARLIE,
				asset_amount: borrow_asset_deposit,
				lp_amount: borrow_asset_deposit,
			}),
		);

		process_and_progress_blocks::<Lending, Runtime>(1_000);

		let get_collateral_borrow_limit_for_account = |account| {
			// `limit_normalized` is the limit in USDT
			// `limit` is the limit in BTC
			// BTC is worth 50_000 times more than USDT (see `create_simple_market()`)

			// borrow_limit * COLLATERAL::ONE / price_of(COLLATERAL::ONE)
			// REVIEW: I'm still not sure if this makes sense
			let limit_normalized = Lending::get_borrow_limit(&market_index, &account).unwrap();
			let limit = limit_normalized
				.mul(COLLATERAL::ONE)
				.div(get_price(COLLATERAL::ID, COLLATERAL::ONE));
			limit
		};

		let alice_limit = get_collateral_borrow_limit_for_account(*ALICE);
		assert_extrinsic_event::<Runtime>(
			// partial borrow
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market_index, alice_limit / 2),
			RuntimeEvent::Lending(crate::Event::<Runtime>::Borrowed {
				sender: *ALICE,
				market_id: market_index,
				amount: alice_limit / 2,
			}),
		);

		process_and_progress_blocks::<Lending, Runtime>(1_000);

		// pay off a small amount
		assert_extrinsic_event::<Runtime>(
			Lending::repay_borrow(
				RuntimeOrigin::signed(*ALICE),
				market_index,
				*ALICE,
				RepayStrategy::PartialAmount(BORROW::units(1) / 10_000),
				false,
			),
			RuntimeEvent::Lending(crate::Event::<Runtime>::BorrowRepaid {
				sender: *ALICE,
				market_id: market_index,
				beneficiary: *ALICE,
				amount: BORROW::units(1) / 10_000,
			}),
		);

		// wait a few blocks
		process_and_progress_blocks::<Lending, Runtime>(3);

		// pay off a small amount
		assert_extrinsic_event::<Runtime>(
			Lending::repay_borrow(
				RuntimeOrigin::signed(*ALICE),
				market_index,
				*ALICE,
				RepayStrategy::PartialAmount(BORROW::units(1) / 10_000),
				false,
			),
			RuntimeEvent::Lending(crate::Event::<Runtime>::BorrowRepaid {
				sender: *ALICE,
				market_id: market_index,
				beneficiary: *ALICE,
				amount: BORROW::units(1) / 10_000,
			}),
		);

		// wait a few blocks
		process_and_progress_blocks::<Lending, Runtime>(10);

		let alice_total_debt_with_interest =
			Lending::total_debt_with_interest(&market_index, &ALICE)
				.unwrap()
				.unwrap_or_zero();

		dbg!(&alice_total_debt_with_interest);

		assert_ok!(Tokens::mint_into(BORROW::ID, &ALICE, alice_total_debt_with_interest));

		// can't repay more than is owed
		assert_err!(
			Lending::repay_borrow(
				RuntimeOrigin::signed(*ALICE),
				market_index,
				*ALICE,
				RepayStrategy::PartialAmount(alice_total_debt_with_interest + 1),
				false,
			),
			DispatchErrorWithPostInfo {
				post_info: PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },
				error: DispatchError::Module(ModuleError {
					index: 8,
					error: [16, 0, 0, 0],
					message: Some(Error::<Runtime>::CannotRepayMoreThanTotalDebt.into(),),
				}),
			},
		);

		assert_no_event::<Runtime>(RuntimeEvent::Lending(crate::Event::BorrowRepaid {
			sender: *ALICE,
			market_id: market_index,
			beneficiary: *ALICE,
			amount: alice_total_debt_with_interest + 1,
		}));

		assert_extrinsic_event::<Runtime>(
			Lending::repay_borrow(
				RuntimeOrigin::signed(*ALICE),
				market_index,
				*ALICE,
				RepayStrategy::PartialAmount(alice_total_debt_with_interest),
				false,
			),
			RuntimeEvent::Lending(crate::Event::<Runtime>::BorrowRepaid {
				sender: *ALICE,
				market_id: market_index,
				beneficiary: *ALICE,
				amount: alice_total_debt_with_interest,
			}),
		);

		assert_eq!(Lending::collateral_of_account(&market_index, &*ALICE), Ok(alice_balance));
	});
}

#[test]
fn test_repay_total_debt() {
	new_test_ext().execute_with(|| {
		// accounts have 1 BTC of collateral
		let alice_original_btc_balance = BTC::ONE;
		let bob_original_btc_balance = BTC::ONE;

		let (market_index, vault_id) = create_simple_market();

		let deposit_collateral = |account, balance| {
			assert_ok!(Tokens::mint_into(BTC::ID, account, balance));
			assert_extrinsic_event::<Runtime>(
				Lending::deposit_collateral(RuntimeOrigin::signed(*account), market_index, balance, false),
				RuntimeEvent::Lending(crate::Event::<Runtime>::CollateralDeposited {
					market_id: market_index,
					amount: BTC::ONE,
					sender: *account,
				}),
			);
		};

		deposit_collateral(&*ALICE, alice_original_btc_balance);
		deposit_collateral(&*BOB, bob_original_btc_balance);

		// CHARLIE is the lender
		let borrow_asset_deposit = USDT::units(1_000_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &CHARLIE, borrow_asset_deposit));
		assert_ok!(Vault::deposit(RuntimeOrigin::signed(*CHARLIE), vault_id, borrow_asset_deposit));

		// processes one block
		process_and_progress_blocks::<Lending, Runtime>(1);

		let get_btc_borrow_limit_for_account = |account| {
			// `limit_normalized` is the limit in USDT
			// `limit` is the limit in BTC
			// BTC is worth 50_000 times more than USDT (see `create_simple_market()`)

			// REVIEW: I'm still not sure if this makes sense
			let limit_normalized = Lending::get_borrow_limit(&market_index, &account).unwrap();
			let limit = limit_normalized.mul(BTC::ONE).div(get_price(BTC::ID, BTC::ONE));
			limit
		};

		let alice_borrow_limit = get_btc_borrow_limit_for_account(*ALICE);
		assert_extrinsic_event::<Runtime>(
			Lending::borrow(RuntimeOrigin::signed(*ALICE), market_index, alice_borrow_limit),
			RuntimeEvent::Lending(crate::Event::<Runtime>::Borrowed {
				sender: *ALICE,
				market_id: market_index,
				amount: alice_borrow_limit,
			}),
		);

		process_and_progress_blocks::<Lending, Runtime>(1000);

		let bob_limit_after_blocks = get_btc_borrow_limit_for_account(*BOB);
		assert_extrinsic_event::<Runtime>(
			Lending::borrow(RuntimeOrigin::signed(*BOB), market_index, bob_limit_after_blocks),
			RuntimeEvent::Lending(crate::Event::<Runtime>::Borrowed {
				sender: *BOB,
				market_id: market_index,
				amount: bob_limit_after_blocks,
			}),
		);

		process_and_progress_blocks::<Lending, Runtime>(100);

		let alice_total_debt_with_interest =
			Lending::total_debt_with_interest(&market_index, &ALICE)
				.unwrap()
				.unwrap_amount();
		let bob_total_debt_with_interest =
			Lending::total_debt_with_interest(&market_index, &BOB).unwrap().unwrap_amount();

		assert_ok!(Tokens::mint_into(USDT::ID, &ALICE, alice_total_debt_with_interest));
		assert_ok!(Tokens::mint_into(USDT::ID, &BOB, bob_total_debt_with_interest));

		// repay ALICE and check state
		{
			assert_extrinsic_event::<Runtime>(
				Lending::repay_borrow(
					RuntimeOrigin::signed(*ALICE),
					market_index,
					*ALICE,
					RepayStrategy::TotalDebt,
					false,
				),
				RuntimeEvent::Lending(crate::Event::<Runtime>::BorrowRepaid {
					sender: *ALICE,
					market_id: market_index,
					beneficiary: *ALICE,
					amount: alice_total_debt_with_interest,
				}),
			);

			assert_eq!(
				Lending::total_debt_with_interest(&market_index, &ALICE).unwrap(),
				TotalDebtWithInterest::NoDebt
			);
		}

		// repay BOB and check state
		{
			assert_extrinsic_event::<Runtime>(
				Lending::repay_borrow(
					RuntimeOrigin::signed(*BOB),
					market_index,
					*BOB,
					RepayStrategy::TotalDebt,
					false,
				),
				RuntimeEvent::Lending(crate::Event::<Runtime>::BorrowRepaid {
					sender: *BOB,
					market_id: market_index,
					beneficiary: *BOB,
					amount: bob_total_debt_with_interest,
				}),
			);

			assert_eq!(
				Lending::total_debt_with_interest(&market_index, &BOB).unwrap(),
				TotalDebtWithInterest::NoDebt
			);
		}
	});
}
