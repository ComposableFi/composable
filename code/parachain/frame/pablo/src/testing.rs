#![allow(clippy::unwrap_used, clippy::disallowed_methods)]

use std::collections::BTreeMap;

use composable_maths::dex::constant_product::compute_first_deposit_lp;
use composable_support::math::safe::SafeAdd;
use composable_tests_helpers::{
	test::{
		block::next_block,
		currency::{Currency, PICA, USDT},
		helper::RuntimeTrait,
	},
	ALICE, BOB,
};
use composable_traits::currency::BalanceLike;
use frame_support::{
	assert_noop, bounded_btree_map,
	traits::{fungibles::Mutate, OriginTrait, TryCollect},
};
use frame_system::{pallet_prelude::OriginFor, Config as SystemConfig};
use sp_arithmetic::Permill;
use sp_runtime::{traits::Zero, AccountId32};

use crate::{Config, Event, Pallet, PoolInitConfiguration};

#[allow(clippy::upper_case_acronyms)]
type KSM = Currency<4, 12>;

// copied from runtime/primitives/src/currency.rs
// TODO(benluelo): pool creation helpers for these hardcoded pools
pub const KSM_USDT_LPT: u128 = 105;
pub const PICA_USDT_LPT: u128 = 106;
pub const PICA_KSM_LPT: u128 = 107;

// NOTE/FIXME(benluelo): These trait bounds can be simplified quite a bit once this issue is
// resolved: https://github.com/rust-lang/rust/issues/20671#issuecomment-529752828
//
// Associated type constraints currently aren't resolved at the use site, but super traits (and
// their associated type constraints) are! To work with this, we write some extra boilerplate here
// to make the call site only require a simple `<Runtime: Trait>` constraint as opposed to repeating
// the trait bounds on every function.

/// This trait defines the constraints that pablo assumes from the runtime, allowing tests using
/// this trait to bound the runtime to be run with any runtime thatr fits within these criteria.
pub trait PabloRuntimeConstraints:
	RuntimeTrait<Event<Self>>
	+ SystemConfig<BlockNumber = Self::__SystemBlockNumber, AccountId = Self::__SystemAccountId>
	+ Config<Balance = Self::__PabloBalance> // , AssetId = Self::__AssetId
	+ pallet_timestamp::Config<Moment = Self::__TimestampMoment>
{
	// these associated types aren't intended to be used, they only exist for the reasons
	// described above.
	type __SystemBlockNumber: From<u32> + Into<u64> + SafeAdd;
	type __SystemAccountId: From<AccountId32> + Clone;
	type __TimestampMoment: From<u64>;
	type __PabloBalance: BalanceLike + From<u128> + Into<u128> + Zero;
}

impl<T> PabloRuntimeConstraints for T
where
	T: RuntimeTrait<Event<Self>>,
	Event<Self>: Into<<Self as SystemConfig>::Event>,
	T: SystemConfig + Config + pallet_timestamp::Config,
	<T as SystemConfig>::BlockNumber: From<u32> + Into<u64> + SafeAdd,
	<T as SystemConfig>::AccountId: From<AccountId32> + Clone,
	<T as Config>::Balance: BalanceLike + From<u128> + Into<u128> + Zero,
	<T as pallet_timestamp::Config>::Moment: From<u64>,
{
	type __SystemAccountId = <T as SystemConfig>::AccountId;
	type __SystemBlockNumber = <T as SystemConfig>::BlockNumber;
	type __TimestampMoment = <T as pallet_timestamp::Config>::Moment;
	type __PabloBalance = <T as Config>::Balance;
}

fn mint_assets<Runtime: PabloRuntimeConstraints>(
	asset_id: impl Into<Runtime::AssetId>,
	who: impl Into<Runtime::AccountId>,
	amount: impl Into<<Runtime as Config>::Balance>,
) {
	<Runtime as Config>::Assets::mint_into(asset_id.into(), &who.into(), amount.into()).unwrap();
}

pub mod pool_creation {
	use super::*;

	pub fn create_new_constant_product_pool_1_1<Runtime: PabloRuntimeConstraints>(
	) -> Runtime::PoolId {
		next_block::<Pallet<Runtime>, Runtime>();

		let asset_1_id = PICA::ID;
		let asset_2_id = USDT::ID;

		Runtime::assert_extrinsic_event_with(
			Pallet::<Runtime>::do_create_pool(
				PoolInitConfiguration::DualAssetConstantProduct {
					owner: ALICE.into(),
					assets_weights: bounded_btree_map! {
						asset_1_id.into() => Permill::from_parts(500_000),
						asset_2_id.into() => Permill::from_parts(500_000),
					},
					fee: Permill::from_parts(10_000),
				},
				Some(PICA_USDT_LPT.into()),
			),
			|event| match event {
				Event::PoolCreated { pool_id, .. } => Some(pool_id),
				_ => None,
			},
		)
	}

	pub fn zero_fees_pool_1_4<Runtime: PabloRuntimeConstraints>() {
		next_block::<Pallet<Runtime>, Runtime>();

		let asset_1_id = PICA::ID;
		let asset_2_id = USDT::ID;

		let _pool_id = Runtime::assert_extrinsic_event_with(
			Pallet::<Runtime>::do_create_pool(
				PoolInitConfiguration::DualAssetConstantProduct {
					owner: ALICE.into(),
					assets_weights: bounded_btree_map! {
						asset_1_id.into() => Permill::from_parts(500_000),
						asset_2_id.into() => Permill::from_parts(500_000),
					},
					fee: Permill::zero(),
				},
				Some(PICA_USDT_LPT.into()),
			),
			|event| match event {
				Event::PoolCreated { pool_id, .. } => Some(pool_id),
				_ => None,
			},
		);
	}

	pub fn pool_assets_cannot_be_the_same_1_6<Runtime: PabloRuntimeConstraints>() {
		next_block::<Pallet<Runtime>, Runtime>();

		let asset_1_id = PICA::ID;
		let asset_2_id = PICA::ID;

		assert_noop!(
			// don't need to use do_create_pool here as we're checking pool creation validations
			// and never adding liquidity
			Pallet::<Runtime>::create(
				OriginFor::<Runtime>::root(),
				PoolInitConfiguration::DualAssetConstantProduct {
					owner: ALICE.into(),
					assets_weights: bounded_btree_map! {
						asset_1_id.into() => Permill::from_parts(500_000),
						asset_2_id.into() => Permill::from_parts(500_000),
					},
					fee: Permill::zero(),
				},
			),
			crate::Error::<Runtime>::InvalidPair,
		);
	}
}

pub mod providing_liquidity {
	use super::*;

	pub fn add_liquidity_to_1_1_pool_2_1<Runtime: PabloRuntimeConstraints>() {
		next_block::<Pallet<Runtime>, Runtime>();

		let pool_id = super::pool_creation::create_new_constant_product_pool_1_1::<Runtime>();

		mint_assets::<Runtime>(PICA::ID, BOB, PICA::units(1_100_000));
		mint_assets::<Runtime>(USDT::ID, BOB, USDT::units(1_100_000));

		let assets = [
			(PICA::ID.into(), PICA::units(10_000).into()),
			(USDT::ID.into(), USDT::units(10_000).into()),
		]
		.into_iter()
		.collect::<BTreeMap<_, _>>();

		Runtime::assert_extrinsic_event(
			Pallet::<Runtime>::add_liquidity(
				OriginFor::<Runtime>::signed(BOB.into()),
				pool_id,
				assets.clone(),
				Zero::zero(),
				true,
			),
			Event::<Runtime>::LiquidityAdded {
				who: BOB.into(),
				pool_id,
				asset_amounts: assets.clone(),
				// minted_lp: 19_999_999_993_470_955_u128.into(),
				minted_lp: compute_first_deposit_lp(
					assets.into_iter().map(|(id, deposit)| {
						(id.into(), deposit.into(), Permill::from_rational::<u32>(1, 2))
					}),
					Zero::zero(),
				)
				.unwrap()
				.value
				.into(),
			},
		);
	}
}

pub fn add_liquidity<Runtime: PabloRuntimeConstraints>() {
	next_block::<Pallet<Runtime>, Runtime>();

	let asset_1_id: <Runtime as Config>::AssetId = PICA::ID.into();
	let asset_2_id: <Runtime as Config>::AssetId = USDT::ID.into();

	let pool_id = Runtime::assert_extrinsic_event_with(
		Pallet::<Runtime>::do_create_pool(
			PoolInitConfiguration::DualAssetConstantProduct {
				owner: ALICE.into(),
				assets_weights: [
					(asset_1_id, Permill::from_parts(500_000)),
					(asset_2_id, Permill::from_parts(500_000)),
				]
				.into_iter()
				.try_collect()
				.unwrap(),
				fee: Permill::from_parts(10_000),
			},
			Some(PICA_USDT_LPT.into()),
		),
		|event| match event {
			Event::PoolCreated { pool_id, .. } => Some(pool_id),
			_ => None,
		},
	);

	mint_assets::<Runtime>(asset_1_id, BOB, PICA::units(1_100_000));
	mint_assets::<Runtime>(asset_2_id, BOB, PICA::units(1_100_000));

	let assets =
		[(asset_1_id, PICA::units(1_000_000).into()), (asset_2_id, USDT::units(1_000_000).into())]
			.into_iter()
			.collect::<BTreeMap<_, _>>();

	Runtime::assert_extrinsic_event(
		Pallet::<Runtime>::add_liquidity(
			OriginFor::<Runtime>::signed(BOB.into()),
			pool_id,
			assets.clone(),
			Zero::zero(),
			true,
		),
		Event::<Runtime>::LiquidityAdded {
			who: BOB.into(),
			pool_id,
			asset_amounts: assets.clone(),
			minted_lp: compute_first_deposit_lp(
				assets.into_iter().map(|(id, deposit)| {
					(id.into(), deposit.into(), Permill::from_rational::<u32>(1, 2))
				}),
				Zero::zero(),
			)
			.unwrap()
			.value
			.into(),
		},
	);
}

pub fn ksm_usdt<Runtime: PabloRuntimeConstraints>() {
	next_block::<Pallet<Runtime>, Runtime>();

	let asset_1_id: <Runtime as Config>::AssetId = KSM::ID.into();
	let asset_2_id: <Runtime as Config>::AssetId = USDT::ID.into();

	let pool_id = Runtime::assert_extrinsic_event_with(
		Pallet::<Runtime>::do_create_pool(
			PoolInitConfiguration::DualAssetConstantProduct {
				owner: ALICE.into(),
				assets_weights: [
					(asset_1_id, Permill::from_parts(500_000)),
					(asset_2_id, Permill::from_parts(500_000)),
				]
				.into_iter()
				.try_collect()
				.unwrap(),
				fee: Permill::from_parts(10_000),
			},
			Some(PICA_USDT_LPT.into()),
		),
		|event| match event {
			Event::PoolCreated { pool_id, .. } => Some(pool_id),
			_ => None,
		},
	);

	mint_assets::<Runtime>(asset_1_id, BOB, KSM::units(1_000_000));
	mint_assets::<Runtime>(asset_2_id, BOB, USDT::units(1_000_000));

	let assets = [(asset_1_id, KSM::units(10).into()), (asset_2_id, USDT::units(100).into())]
		.into_iter()
		.collect::<BTreeMap<_, _>>();

	Runtime::assert_extrinsic_event(
		Pallet::<Runtime>::add_liquidity(
			OriginFor::<Runtime>::signed(BOB.into()),
			pool_id,
			assets.clone(),
			Zero::zero(),
			true,
		),
		Event::<Runtime>::LiquidityAdded {
			who: BOB.into(),
			pool_id,
			asset_amounts: assets.clone(),
			// TODO(benluelo): Figure out where this number comes from
			// minted_lp: 63_245_552_925_824_u128.into(),
			minted_lp: compute_first_deposit_lp(
				assets.into_iter().map(|(id, deposit)| {
					(id.into(), deposit.into(), Permill::from_rational::<u32>(1, 2))
				}),
				Zero::zero(),
			)
			.unwrap()
			.value
			.into(),
		},
	);
}
