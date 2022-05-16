#![allow(unused_imports)]
use composable_traits::lending::{Lending as LendingTrait, RepayStrategy, TotalDebtWithInterest};
use frame_benchmarking::Zero;
use sp_application_crypto::Pair;
use std::ops::{Div, Mul};

use crate::{
	self as pallet_lending, accrue_interest_internal, currency::*, mocks_offchain::*,
	models::borrower_data::BorrowerData, setup::assert_last_event, AccruedInterest, Error,
	MarketIndex,
};
use codec::{Decode, Encode};
use composable_support::validation::{TryIntoValidated, Validated};
use composable_tests_helpers::{prop_assert_acceptable_computation_error, prop_assert_ok};
use composable_traits::{
	defi::{CurrencyPair, LiftedFixedBalance, MoreThanOneFixedU128, Rate, ZeroToOneFixedU128},
	lending::{self, math::*, CreateInput, UpdateInput},
	oracle,
	time::SECONDS_PER_YEAR_NAIVE,
	vault::{self, Deposit, VaultConfig},
};
use frame_support::{
	assert_err, assert_noop, assert_ok,
	dispatch::{DispatchErrorWithPostInfo, DispatchResultWithPostInfo, PostDispatchInfo},
	traits::{
		fungibles::{Inspect, Mutate},
		OffchainWorker, OnFinalize, OnInitialize,
	},
	weights::Pays,
};
use frame_system::{EventRecord, Phase};
use pallet_liquidations;
use pallet_vault::models::VaultInfo;
use proptest::{prelude::*, test_runner::TestRunner};
use sp_arithmetic::assert_eq_error_rate;
use sp_core::{
	crypto::Dummy,
	offchain::{testing, OffchainWorkerExt, TransactionPoolExt},
	H256, U256,
};
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use sp_runtime::{
	testing::{Digest, Header as HeaderType, UintAuthorityId},
	traits::{Block, Header as HeaderTrait},
	ArithmeticError, DispatchError, FixedPointNumber, FixedU128, ModuleError, Percent, Perquintill,
	RuntimeAppPublic,
};
use std::sync::Arc;

const DEFAULT_MARKET_VAULT_RESERVE: Perquintill = Perquintill::from_percent(10);

#[test]
fn test_liquidation_executed_offchain_worker() {
	let account_id = *ALICE;
	let authority_id = wrapper::UintAuthorityIdWrapper::from(account_id);
	wrapper::UintAuthorityIdWrapper::set_all_keys(vec![authority_id]);
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
		let (market_id, vault) = create_market::<50_000>(
			USDT::instance(),
			BTC::instance(),
			manager,
			DEFAULT_MARKET_VAULT_RESERVE,
			MoreThanOneFixedU128::saturating_from_integer::<u128>(2),
		);

		// Deposit collateral.
		let collateral_value = BTC::units(1);
		assert_ok!(Tokens::mint_into(BTC::ID, &manager, collateral_value));
		assert_extrinsic_event::<Runtime>(
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
		process_and_progress_blocks(1);
		// Borrow 20_000 USDT.
		let borrowed_value = USDT::units(20_000);
		assert_extrinsic_event::<Runtime>(
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
		// Check if corresponded liquidation transaction has been placed in the pool
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

// HELPERS
fn default_under_collateralized_warn_percent() -> Percent {
	Percent::from_float(0.10)
}

fn create_market<const NORMALIZED_PRICE: u128>(
	borrow_asset: RuntimeCurrency,
	collateral_asset: RuntimeCurrency,
	manager: AccountId,
	reserved_factor: Perquintill,
	collateral_factor: MoreThanOneFixedU128,
) -> (MarketIndex, VaultId) {
	set_price(borrow_asset.id(), NORMALIZED::ONE);
	set_price(collateral_asset.id(), NORMALIZED::units(NORMALIZED_PRICE));

	Tokens::mint_into(borrow_asset.id(), &manager, borrow_asset.units(1000)).unwrap();
	Tokens::mint_into(collateral_asset.id(), &manager, collateral_asset.units(100)).unwrap();

	let config = CreateInput {
		updatable: UpdateInput {
			collateral_factor,
			under_collateralized_warn_percent: default_under_collateralized_warn_percent(),
			liquidators: vec![],
			interest_rate_model: InterestRateModel::default(),
		},
		reserved_factor,
		currency_pair: CurrencyPair::new(collateral_asset.id(), borrow_asset.id()),
	};

	Lending::create_market(Origin::signed(manager), config.try_into_validated().unwrap()).unwrap();

	let system_events = System::events();
	if let Some(EventRecord {
		event:
			Event::Lending(crate::Event::<Runtime>::MarketCreated {
				market_id,
				vault_id,
				manager: _,
				currency_pair: _,
			}),
		..
	}) = system_events.last()
	{
		(*market_id, *vault_id)
	} else {
		panic!(
			"System::events() did not contain the market creation event. Found {:#?}",
			system_events
		)
	}
}

fn assert_extrinsic_event<T: crate::Config>(
	result: DispatchResultWithPostInfo,
	event: <T as crate::Config>::Event,
) {
	assert_ok!(result);
	assert_last_event::<T>(event);
}
