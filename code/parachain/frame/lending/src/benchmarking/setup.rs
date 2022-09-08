//! most usable things, 20% of all , for tests

/// test should be freely use external API
pub use codec::{Codec, Decode, Encode, FullCodec, MaxEncodedLen};

use composable_traits::{
	defi::{CurrencyPair, DeFiComposableConfig, MoreThanOneFixedU128},
	lending::{math::InterestRateModel, CreateInput, UpdateInput},
	oracle::Price,
};
use frame_benchmarking::whitelisted_caller;
use frame_support::traits::Hooks;
use frame_system::{EventRecord, RawOrigin};
use sp_runtime::{FixedPointNumber, Percent, Perquintill};

use crate::{currency::*, Config};

/// Creates a [`RawOrigin::Signed`] from [`whitelisted_caller`].
pub(crate) fn whitelisted_origin<T: frame_system::Config>() -> RawOrigin<T::AccountId> {
	let caller: T::AccountId = whitelisted_caller();
	RawOrigin::Signed(caller)
}

pub(crate) type AssetIdOf<T> = <T as DeFiComposableConfig>::MayBeAssetId;

/// Creates a new [`CurrencyPair`] with [`USDT`] as collateral (base) and [`BTC`] as borrow (quote)
pub(crate) fn create_currency_pair<T: crate::Config>() -> CurrencyPair<AssetIdOf<T>> {
	// fancy encode/ decode shenanigans because there's aren't any `From` bounds on
	// `DeFiComposableConfig::MayBeAssetId`
	CurrencyPair::new(encode_decode(USDT::ID), encode_decode(BTC::ID))
}

pub(crate) fn produce_block<T: crate::Config + pallet_timestamp::Config>(
	n: T::BlockNumber,
	time: T::Moment,
) {
	frame_system::Pallet::<T>::set_block_number(n);
	<crate::Pallet<T> as Hooks<T::BlockNumber>>::on_initialize(n);
	<pallet_timestamp::Pallet<T>>::set_timestamp(time);
}

#[allow(dead_code)]
pub(crate) fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

pub(crate) fn set_price<T: Config + pallet_oracle::Config>(asset_id: T::MayBeAssetId, price: u64) {
	let asset_id: T::AssetId = encode_decode(asset_id);
	pallet_oracle::Prices::<T>::insert(
		asset_id,
		Price { price: <T as pallet_oracle::Config>::PriceValue::from(price), block: 0_u32.into() },
	);
}

/// Round-trip encode/ decode a value into a different type.
///
/// Panics if the input value cannot be decoded into the specified type.
fn encode_decode<D: Decode, E: Encode>(value: E) -> D {
	let asset_id = value.encode();
	let asset_id = D::decode(&mut &asset_id[..]).unwrap();
	asset_id
}

/// Creates a new [`CurrencyPair`] with [`USDT`] as collateral and [`BTC`] as borrow.
///
/// Mints `amount` of both currencies into `account`.
pub(crate) fn setup_currency_pair<T: Config + pallet_oracle::Config + DeFiComposableConfig>(
	account: &<T as frame_system::Config>::AccountId,
	amount: <T as DeFiComposableConfig>::Balance,
) -> CurrencyPair<AssetIdOf<T>> {
	use frame_support::traits::tokens::fungibles::Mutate;

	let pair = create_currency_pair::<T>();

	<T as Config>::MultiCurrency::mint_into(pair.base, &account, amount).unwrap();
	<T as Config>::MultiCurrency::mint_into(pair.quote, &account, amount).unwrap();

	set_price::<T>(pair.base, 48_000_000_000_u64);
	set_price::<T>(pair.quote, 1_000_000_000_u64);

	pair
}

pub(crate) fn create_market_config<T: Config>(
	collateral_asset: <T as DeFiComposableConfig>::MayBeAssetId,
	borrow_asset: <T as DeFiComposableConfig>::MayBeAssetId,
	max_price_age: <T as frame_system::Config>::BlockNumber,
) -> CreateInput<
	<T as Config>::LiquidationStrategyId,
	<T as DeFiComposableConfig>::MayBeAssetId,
	<T as frame_system::Config>::BlockNumber,
> {
	CreateInput {
		updatable: UpdateInput {
			collateral_factor: MoreThanOneFixedU128::saturating_from_rational(200_u128, 100_u128),
			under_collateralized_warn_percent: Percent::from_percent(10),
			liquidators: Default::default(),
			max_price_age,
		},
		reserved_factor: Perquintill::from_percent(10),
		currency_pair: CurrencyPair::new(collateral_asset, borrow_asset),
		interest_rate_model: InterestRateModel::default(),
	}
}
