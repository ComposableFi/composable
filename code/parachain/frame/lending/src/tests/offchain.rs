use crate::{
	currency::*,
	mocks::{authority_id_wrapper, offchain::*},
	tests::{
		assert_no_event, borrow, create_market_for_liquidation_test, mint_and_deposit_collateral,
		ConfigBound,
	},
};
use codec::Decode;
use frame_support::{assert_ok, traits::fungibles::Mutate, BoundedVec};
use sp_core::{
	offchain::{testing, TransactionPoolExt},
	H256,
};
use sp_runtime::{testing::Digest, traits::Header as HeaderTrait};

type TestBoundedVec = BoundedVec<AccountId, MaxLiquidationBatchSize>;
impl ConfigBound for Runtime {}

#[test]
fn test_liquidation_offchain_worker() {
	let account_id = *ALICE;
	let authority_id = authority_id_wrapper::UintAuthorityIdWrapper::from(account_id);
	authority_id_wrapper::UintAuthorityIdWrapper::set_all_keys(vec![authority_id]);
	// Create externalities, register transaction pool and key store in the externalities.
	let mut ext = new_test_ext();
	let (pool, pool_state) = testing::TestTransactionPoolExt::new();
	ext.register_extension(TransactionPoolExt::new(pool));
	ext.execute_with(|| {
		let manager = *ALICE;
		let lender = *CHARLIE;
		// ALICE is also borrower who's borrow going to be liquidated
		let risky_borrower = *ALICE;
		// BOB is reliable borrower who's borrow should not be liquidated
		let reliable_borrower = *BOB;
		// Create a market with BTC as collateral asset and USDT as borrow asset.
		// Initial collateral asset price is 50_000 USDT. Market's collateral factor equals two.
		// It means that borrow supposed to be undercollateralized when
		// borrowed amount is higher then one half of collateral amount in terms of USDT.
		let (market_id, vault_id) = create_market_for_liquidation_test::<Runtime>(manager);
		// Deposit USDT in the vault.
		let vault_value = USDT::units(100_000_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &lender, vault_value));
		assert_ok!(Vault::deposit(Origin::signed(lender), vault_id, vault_value));

		//test::block::process_and_progress_blocks::<Lending, Runtime>(1);
		crate::Markets::<Runtime>::iter().for_each(|x| println!("{:?}", x));
		crate::tests::process_and_progress_blocks::<Lending, Runtime>(1);
		// Deposit 1 BTC collateral from risky borrower account.
		mint_and_deposit_collateral::<Runtime>(risky_borrower, BTC::units(1), market_id, BTC::ID);
		// Risky borrower borrows 20_000 USDT.
		borrow::<Runtime>(risky_borrower, market_id, USDT::units(20_000));
		// Deposit 100 BTC collateral from reliable borrower account.
		mint_and_deposit_collateral::<Runtime>(
			reliable_borrower,
			BTC::units(100),
			market_id,
			BTC::ID,
		);
		// Reliable borrower borrows 20_000 USDT.
		borrow::<Runtime>(reliable_borrower, market_id, USDT::units(20_000));
		// Emulate situation when collateral price has fallen down
		// from 50_000 USDT to 38_000 USDT.
		// Now the risky borrow is undercollateralized since market's collateral factor equals two.
		// Therefore, one BTC can cover only 19_000 of 20_0000 borrowed USDT.
		set_price(BTC::ID, NORMALIZED::units(38_000));
		// Header for the fake block to execute off-chain worker
		let header =
			Header::new(2, H256::default(), H256::default(), [69u8; 32].into(), Digest::default());
		// Execute off-chain worker
		Executive::offchain_worker(&header);
		let tx = pool_state.write().transactions.pop().unwrap();
		// Check that we have had only one transaction since reliable borrow should not be
		// liquidated
		assert!(pool_state.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		// Check that it is transaction which leads to the risky borrow liquidation.
		assert_eq!(
			tx.call,
			Call::Lending(crate::Call::liquidate {
				market_id,
				borrowers: TestBoundedVec::try_from(vec![risky_borrower]).unwrap()
			})
		);
		process_block_with_execution(tx);
		// Check that events for the risky borrow were emitted
		// Check event from Lending pallet
		let event =
			crate::Event::LiquidationInitiated { market_id, borrowers: vec![risky_borrower] };
		System::assert_has_event(Event::Lending(event));
		// Check event from Liquidations pallet
		let event = pallet_liquidations::Event::PositionWasSentToLiquidation {};
		System::assert_has_event(Event::Liquidations(event));
		// Check that events for the reliable borrow were not emitted
		// Check event from Lending pallet
		let event = crate::Event::<Runtime>::LiquidationInitiated {
			market_id,
			borrowers: vec![reliable_borrower],
		};
		assert_no_event::<Runtime>(Event::Lending(event));
		// Check that Liquidations pallet emitted only one event
		let event =
			Event::Liquidations(pallet_liquidations::Event::PositionWasSentToLiquidation {});
		assert!(System::events().iter().filter(|record| record.event == event).count() == 1);
	});
}
