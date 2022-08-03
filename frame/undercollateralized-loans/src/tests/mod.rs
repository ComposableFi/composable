use crate::{
	currency::{BTC, NORMALIZED, USDT},
	mocks::{general as runtime, general::*},
	strategies::repayment_strategies::RepaymentStrategy,
	types::{LoanConfigOf, MarketInfoOf, MarketInputOf},
};
use composable_support::validation::TryIntoValidated;
use composable_traits::{
	defi::{CurrencyPair, DeFiComposableConfig, DeFiEngine, MoreThanOneFixedU128, Rate},
	oracle,
	undercollateralized_loans::{LoanInput, MarketInput},
};
use frame_support::{
	assert_ok,
	dispatch::DispatchResultWithPostInfo,
	traits::{fungibles::Mutate},
	BoundedVec,
};
use pallet_vault::models::VaultInfo;
use sp_runtime::{FixedPointNumber, Percent, Perquintill};
use sp_std::collections::btree_set::BTreeSet;
pub mod loan;
pub mod market;
pub mod prelude;

pub const DEFAULT_MARKET_VAULT_RESERVE: Perquintill = Perquintill::from_percent(10);
pub const DEFAULT_MAX_PRICE_AGE: u64 = 1020;
pub const DEFAULT_MARKET_VAULT_STRATEGY_SHARE: Perquintill = Perquintill::from_percent(90);

// Bounds for configuration generic type, used in create market helpers.
pub trait ConfigBound:
	frame_system::Config<BlockNumber = runtime::BlockNumber>
	+ crate::Config
	+ DeFiComposableConfig<MayBeAssetId = runtime::CurrencyId>
	+ orml_tokens::Config<CurrencyId = runtime::CurrencyId, Balance = runtime::Balance>
{
}
impl ConfigBound for Runtime {}

// HELPERS
/// Helper to get the price of an asset from the Oracle, in USDT cents.
pub fn get_price(asset_id: CurrencyId, amount: Balance) -> Balance {
	<Oracle as oracle::Oracle>::get_price(asset_id, amount).unwrap().price
}

pub fn create_market<T, const NORMALIZED_PRICE: CurrencyId>(
	manager: <T as frame_system::Config>::AccountId,
	input: MarketInputOf<T>,
) -> MarketInfoOf<T>
where
	T: ConfigBound,
{
	let borrow_asset = input.currency_pair.quote;
	let collateral_asset = input.currency_pair.base;
	set_price(borrow_asset, NORMALIZED::ONE);
	set_price(collateral_asset, NORMALIZED::units(NORMALIZED_PRICE));

	orml_tokens::Pallet::<T>::mint_into(borrow_asset, &manager, NORMALIZED::units(1000));
	orml_tokens::Pallet::<T>::mint_into(collateral_asset, &manager, NORMALIZED::units(100));
	crate::Pallet::<T>::do_create_market(manager, input.try_into_validated().unwrap(), true)
		.unwrap()
}

pub fn create_market_input_config<T>(
	borrow_asset: T::CurrencyId,
	collateral_asset: T::CurrencyId,
	reserved_factor: Perquintill,
	whitelist: BTreeSet<T::AccountId>,
) -> MarketInputOf<T>
where
	T: ConfigBound,
{
	MarketInput {
		currency_pair: CurrencyPair::new(collateral_asset, borrow_asset),
		reserved_factor,
		whitelist,
		liquidation_strategies: vec![],
		max_price_age: DEFAULT_MAX_PRICE_AGE,
	}
}

pub fn create_test_market_input_config() -> MarketInputOf<Runtime> {
	let mut borrowers_whitelist: BTreeSet<runtime::AccountId> = BTreeSet::new();
	borrowers_whitelist.insert(*BOB);
	borrowers_whitelist.insert(*CHARLIE);
	create_market_input_config::<Runtime>(
		USDT::instance().id(),
		BTC::instance().id(),
		DEFAULT_MARKET_VAULT_RESERVE,
		borrowers_whitelist,
	)
}

pub fn create_test_market() -> MarketInfoOf<Runtime> {
	let input = create_test_market_input_config();
	let manager = *ALICE;
	let info = create_market::<Runtime, 50000>(manager, input);
	info
}

pub fn create_test_loan() -> LoanConfigOf<Runtime> {
	let market_info = create_test_market();
	let market_account_id = market_info.config().account_id().clone();
	let loan_input = LoanInput {
		market_account_id,
		borrower_account_id: *BOB,
		principal: 1000,
		collateral: 5,
		interest: Percent::from_percent(5),
        payment_schedule: vec![], 
		loan_maturity: 100,
		repayment_strategy: RepaymentStrategy::InterestPeriodicallyPrincipalWhenMature,
	};
	crate::Pallet::<Runtime>::do_create_loan(loan_input.try_into_validated().unwrap()).unwrap()
}

// Asserts that the outcome of an extrinsic is `Ok`, and that the last event is the specified
/// event.
pub fn assert_extrinsic_event<T: crate::Config>(
	result: DispatchResultWithPostInfo,
	event: <T as crate::Config>::Event,
) {
	assert_ok!(result);
	frame_system::Pallet::<T>::assert_last_event(event.into());
}

/// Asserts the event wasn't dispatched.
pub fn assert_no_event<T>(event: T::Event)
where
	T: frame_system::Config,
{
	assert!(frame_system::Pallet::<T>::events().iter().all(|record| record.event != event));
}
