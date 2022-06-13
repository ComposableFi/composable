use crate::{currency::*, mocks::general::*, MarketIndex};
use composable_support::validation::TryIntoValidated;
use composable_traits::{
	defi::{CurrencyPair, DeFiComposableConfig, MoreThanOneFixedU128, Rate},
	lending::{math::*, CreateInput, UpdateInput},
	oracle,
	vault::{Deposit, VaultConfig},
};
use frame_support::{
	assert_ok,
	dispatch::DispatchResultWithPostInfo,
	traits::{fungibles::Mutate, OriginTrait},
	BoundedVec,
};
use pallet_vault::models::VaultInfo;
use sp_runtime::{FixedPointNumber, Percent, Perquintill};

pub mod borrow;
pub mod interest;
pub mod liquidation;
pub mod market;
pub mod offchain;
pub mod prelude;
pub mod repay;
pub mod vault;

pub const DEFAULT_MARKET_VAULT_RESERVE: Perquintill = Perquintill::from_percent(10);
pub const DEFAULT_COLLATERAL_FACTOR: u128 = 2;
pub const DEFAULT_MAX_PRICE_AGE: u64 = 1020;
pub const DEFAULT_MARKET_VAULT_STRATEGY_SHARE: Perquintill = Perquintill::from_percent(90);

type SystemAccountIdOf<T> = <T as frame_system::Config>::AccountId;
type SystemOriginOf<T> = <T as frame_system::Config>::Origin;
type SystemEventOf<T> = <T as frame_system::Config>::Event;
pub type TestBoundedVec = BoundedVec<AccountId, MaxLiquidationBatchSize>;

// Bounds for configuration generic type, used in create market helpers.
pub trait ConfigBound:
	frame_system::Config<BlockNumber = u64>
	+ crate::Config
	+ DeFiComposableConfig<MayBeAssetId = u128>
	+ orml_tokens::Config<CurrencyId = u128, Balance = u128>
{
}
impl ConfigBound for Runtime {}

// HELPERS
/// Creates a "default" [`CreateInput`], with the specified [`CurrencyPair`].
fn default_create_input<AssetId, BlockNumber: sp_runtime::traits::Bounded>(
	currency_pair: CurrencyPair<AssetId>,
) -> CreateInput<u32, AssetId, BlockNumber> {
	CreateInput {
		updatable: UpdateInput {
			collateral_factor: default_collateral_factor(),
			under_collateralized_warn_percent: default_under_collateralized_warn_percent(),
			liquidators: vec![],
			max_price_age: BlockNumber::max_value(),
		},
		interest_rate_model: InterestRateModel::default(),
		reserved_factor: DEFAULT_MARKET_VAULT_RESERVE,
		currency_pair,
	}
}

/// Returns a "default" value (`10%`) for the under collateralized warn percentage.
pub fn default_under_collateralized_warn_percent() -> Percent {
	Percent::from_float(0.10)
}

/// Creates a "default" [`MoreThanOneFixedU128`], equal to [`DEFAULT_COLLATERAL_FACTOR`].
pub fn default_collateral_factor() -> sp_runtime::FixedU128 {
	MoreThanOneFixedU128::saturating_from_integer(DEFAULT_COLLATERAL_FACTOR)
}

/// Helper to get the price of an asset from the Oracle, in USDT cents.
pub fn get_price(asset_id: CurrencyId, amount: Balance) -> Balance {
	<Oracle as oracle::Oracle>::get_price(asset_id, amount).unwrap().price
}

/// Creates a very simple vault for the given currency. 100% is reserved.
///
/// Values used:
///
/// - `reserved`: `Perquintill::from_percent(100)`
/// - `strategies`: Empty [`BTreeMap`][std::collection::BTreeMap]
///
/// # Panics
///
/// Panics on any errors. Only for use in testing.
pub fn create_simple_vault(
	asset: RuntimeCurrency,
	manager: AccountId,
) -> (VaultId, VaultInfo<AccountId, Balance, CurrencyId, BlockNumber>) {
	let config = VaultConfig {
		asset_id: asset.id(),
		manager,
		reserved: Perquintill::from_percent(100),
		strategies: Default::default(),
	};

	Vault::do_create_vault(Deposit::Existential, config.try_into_validated().unwrap()).unwrap()
}

/// Creates a market with the given values and initializes some state.
//
/// State initialized:
///
/// - Price of the `borrow_asset` is set to `NORMALIZED::ONE`
/// - Price of the `collateral_asset` is set to `NORMALIZED::units(NORMALIZED_PRICE)`
/// - `1000` units of `borrow_asset` are minted into the `manager`
/// - `100` units of `collateral_asset` are minted into the `manager`
///
/// Values used:
///
/// - `interest_rate_model`: [`Default`] implementation of [`InterestRateModel`]
/// - `liquidators`: empty [`Vec`]
/// - `under_collateralized_warn_percent`: [`default_under_collateralized_warn_percent()`]
///
/// # Panics
///
/// Panics on any errors. Only for use in testing.
/// some model with sane parameter
pub fn create_market<T, const NORMALIZED_PRICE: u128>(
	borrow_asset: RuntimeCurrency,
	collateral_asset: RuntimeCurrency,
	manager: SystemAccountIdOf<T>,
	reserved_factor: Perquintill,
	collateral_factor: MoreThanOneFixedU128,
) -> (MarketIndex, T::VaultId)
where
	T: ConfigBound,
	SystemOriginOf<T>: OriginTrait<AccountId = SystemAccountIdOf<T>>,
	SystemEventOf<T>: TryInto<crate::Event<T>>,
	<SystemEventOf<T> as TryInto<crate::Event<T>>>::Error: std::fmt::Debug,
{
	set_price(borrow_asset.id(), NORMALIZED::ONE);
	set_price(collateral_asset.id(), NORMALIZED::units(NORMALIZED_PRICE));

	orml_tokens::Pallet::<T>::mint_into(borrow_asset.id(), &manager, borrow_asset.units(1000))
		.unwrap();
	orml_tokens::Pallet::<T>::mint_into(
		collateral_asset.id(),
		&manager,
		collateral_asset.units(100),
	)
	.unwrap();

	let config = CreateInput {
		updatable: UpdateInput {
			collateral_factor,
			under_collateralized_warn_percent: default_under_collateralized_warn_percent(),
			liquidators: vec![],
			max_price_age: DEFAULT_MAX_PRICE_AGE,
		},
		interest_rate_model: InterestRateModel::default(),
		reserved_factor,
		currency_pair: CurrencyPair::new(collateral_asset.id(), borrow_asset.id()),
	};

	crate::Pallet::<T>::create_market(SystemOriginOf::<T>::signed(manager), config, false).unwrap();
	let system_events = frame_system::Pallet::<T>::events();
	let last_system_event = system_events.last().expect("There are no events in System::events()");
	let pallet_event: crate::Event<T> = last_system_event
		.event
		.clone()
		.try_into()
		.expect("Market was not created due to System::Event => crate::Event conversion error");
	if let crate::Event::<T>::MarketCreated { market_id, vault_id, .. } = pallet_event {
		(market_id, vault_id)
	} else {
		panic!(
			"There is no market creation event in System::events(). Found: {:#?}",
			system_events
		);
	}
}

fn new_jump_model() -> (Percent, InterestRateModel) {
	let base_rate = Rate::saturating_from_rational(2, 100);
	let jump_rate = Rate::saturating_from_rational(10, 100);
	let full_rate = Rate::saturating_from_rational(32, 100);
	let optimal = Percent::from_percent(80);
	let interest_rate_model =
		InterestRateModel::Jump(JumpModel::new(base_rate, jump_rate, full_rate, optimal).unwrap());
	(optimal, interest_rate_model)
}

/// Create a market with a USDT vault LP token as collateral.
pub fn create_simple_vaulted_market(
	borrow_asset: RuntimeCurrency,
	manager: AccountId,
) -> ((MarketIndex, VaultId), CurrencyId) {
	let (_, VaultInfo { lp_token_id, .. }) = create_simple_vault(borrow_asset, manager);

	let market = create_market::<Runtime, 50_000>(
		borrow_asset,
		RuntimeCurrency::new(lp_token_id, 12),
		manager,
		DEFAULT_MARKET_VAULT_RESERVE,
		MoreThanOneFixedU128::saturating_from_integer(2_u128),
	);

	(market, lp_token_id)
}

/// Create a simple  market with USDT as borrow and BTC as collateral.
///
/// `NORMALIZED_PRICE` is set to `50_000`.
///
/// See [`create_market()`] for more information.
pub fn create_simple_market() -> (MarketIndex, VaultId) {
	create_market_with_specific_collateral_factor::<Runtime>(DEFAULT_COLLATERAL_FACTOR, *ALICE)
}

/// Create a market with BTC as collateral asset and USDT as borrow asset.
/// Initial collateral asset price is `50_000` USDT. Market's collateral factor equals two.
/// It means that borrow supposed to be undercolateraized when
/// borrowed amount is higher then one half of collateral amount in terms of USDT.
pub fn create_market_for_liquidation_test<T>(
	manager: T::AccountId,
) -> (crate::MarketIndex, T::VaultId)
where
	T: ConfigBound,
	SystemOriginOf<T>: OriginTrait<AccountId = SystemAccountIdOf<T>>,
	SystemEventOf<T>: TryInto<crate::Event<T>>,
	<SystemEventOf<T> as TryInto<crate::Event<T>>>::Error: std::fmt::Debug,
{
	create_market_with_specific_collateral_factor::<T>(2, manager)
}

/// Create a  market with USDT as borrow and BTC as collateral.
/// Collateral factor should be specified
pub fn create_market_with_specific_collateral_factor<T>(
	collateral_factor: u128,
	manager: T::AccountId,
) -> (crate::MarketIndex, T::VaultId)
where
	T: ConfigBound,
	SystemOriginOf<T>: OriginTrait<AccountId = SystemAccountIdOf<T>>,
	SystemEventOf<T>: TryInto<crate::Event<T>>,
	<SystemEventOf<T> as TryInto<crate::Event<T>>>::Error: std::fmt::Debug,
{
	create_market::<T, 50_000>(
		USDT::instance(),
		BTC::instance(),
		manager,
		DEFAULT_MARKET_VAULT_RESERVE,
		MoreThanOneFixedU128::saturating_from_integer(collateral_factor),
	)
}

/// Mints `amount` of `collateral` into `account`, and then deposits that same `amount` into
/// `market_index`.
///
/// Panics on any errors and checks that the last event was `CollateralDeposited` with the correct/
/// expected values.
pub fn mint_and_deposit_collateral<T>(
	account: SystemAccountIdOf<T>,
	balance: u128,
	market_index: MarketIndex,
	asset_id: u128,
) where
	T: frame_system::Config
		+ crate::Config
		+ orml_tokens::Config<CurrencyId = u128, Balance = u128>
		+ DeFiComposableConfig<Balance = u128>,
	SystemAccountIdOf<T>: Copy,
	SystemOriginOf<T>: OriginTrait<AccountId = T::AccountId>,
	SystemEventOf<T>: From<crate::Event<T>>,
{
	assert_ok!(orml_tokens::Pallet::<T>::mint_into(asset_id, &account, balance));
	assert_ok!(crate::Pallet::<T>::deposit_collateral(
		SystemOriginOf::<T>::signed(account),
		market_index,
		balance,
		false,
	));
	let event = crate::Event::<T>::CollateralDeposited {
		market_id: market_index,
		amount: balance,
		sender: account,
	};
	frame_system::Pallet::<T>::assert_last_event(event.into());
}

/// Borrows amount of tokens from the market for particular account.
/// Checks if corresponded event was emitted.
pub fn borrow<T>(
	borrower: T::AccountId,
	market_id: crate::MarketIndex,
	amount: <T as DeFiComposableConfig>::Balance,
) where
	T: ConfigBound,
	SystemOriginOf<T>: OriginTrait<AccountId = SystemAccountIdOf<T>>,
	SystemEventOf<T>: TryInto<crate::Event<T>>,
	<SystemEventOf<T> as TryInto<crate::Event<T>>>::Error: std::fmt::Debug,
{
	crate::tests::assert_extrinsic_event::<T>(
		crate::Pallet::<T>::borrow(
			SystemOriginOf::<T>::signed(borrower.clone()),
			market_id,
			amount,
		),
		crate::Event::<T>::Borrowed { sender: borrower, amount, market_id }
			.try_into()
			.unwrap(),
	);
}

/// Asserts that the outcome of an extrinsic is `Ok`, and that the last event is the specified
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
