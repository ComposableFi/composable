use crate::{currency::*, mocks_offchain::*};
use codec::Decode;
use composable_tests_helpers::test;
use composable_traits::defi::MoreThanOneFixedU128;
use frame_support::{assert_ok, traits::fungibles::Mutate};
use sp_core::{
	offchain::{testing, TransactionPoolExt},
	H256,
};
use sp_runtime::{testing::Digest, traits::Header as HeaderTrait, FixedPointNumber, Perquintill};
const DEFAULT_MARKET_VAULT_RESERVE: Perquintill = Perquintill::from_percent(10);

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
		// Create a market with BTC as collateral asset and USDT as borrow asset.
		// Initial collateral asset price is 50_000 USDT. Market's collateral factor equals two.
		// It means that borrow supposed to be undercolateraized when
		// borrowed amount is higher then one half of collateral amount in terms of USDT.
		let (market_id, vault) = crate::tests::create_market::<50_000, Lending, Tokens>(
			USDT::instance(),
			BTC::instance(),
			manager,
			DEFAULT_MARKET_VAULT_RESERVE,
			MoreThanOneFixedU128::saturating_from_integer::<u128>(2),
		);

		// Deposit collateral.
		let collateral_value = BTC::units(1);
		assert_ok!(Tokens::mint_into(BTC::ID, &manager, collateral_value));
		crate::tests::assert_extrinsic_event::<Runtime>(
			Lending::deposit_collateral(Origin::signed(manager), market_id, collateral_value),
			Event::Lending(crate::Event::CollateralDeposited {
				sender: manager,
				amount: collateral_value,
				market_id,
			}),
		);
		// Deposit USDT in the vault.
		let vault_value = USDT::units(100_000_000);
		assert_ok!(Tokens::mint_into(USDT::ID, &lender, vault_value));
		assert_ok!(Vault::deposit(Origin::signed(lender), vault, vault_value));
		test::block::process_and_progress_blocks::<Lending, Runtime>(1);
		// Borrow 20_000 USDT.
		let borrowed_value = USDT::units(20_000);
		crate::tests::assert_extrinsic_event::<Runtime>(
			Lending::borrow(Origin::signed(manager), market_id, borrowed_value),
			Event::Lending(crate::Event::Borrowed {
				sender: manager,
				amount: borrowed_value,
				market_id,
			}),
		);
		// Emulate situation when collateral price has fallen down
		// from 50_000 USDT to 38_000 USDT.
		// Now the borrow is undercolateraized since market's collateral factor equals two.
		// Therefore, one BTC can cover only 19_000 of 20_0000 borrowed USDT.
		set_price(BTC::ID, NORMALIZED::units(38_000));
		// Header for the fake block to execute off-chain worker
		let header =
			Header::new(2, H256::default(), H256::default(), [69u8; 32].into(), Digest::default());
		// Execute off-chain worker
		Executive::offchain_worker(&header);
		let tx = pool_state.write().transactions.pop().unwrap();
		assert!(pool_state.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(
			tx.call,
			Call::Lending(crate::Call::liquidate { market_id, borrowers: vec![manager.clone()] })
		);
		process_block_with_execution(tx);
		// Check event from Lending pallet
		let event =
			crate::Event::LiquidationInitiated { market_id, borrowers: vec![manager.clone()] };
		System::assert_has_event(Event::Lending(event));
		// Check event from Liquidations pallet
		let event = pallet_liquidations::Event::PositionWasSentToLiquidation {};
		System::assert_has_event(Event::Liquidations(event));
	});
}
