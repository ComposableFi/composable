use crate::{
	mock,
	mock::{Pablo, *},
	Config,
	PoolConfiguration::DualAssetConstantProduct,
	PoolInitConfiguration,
};
use composable_traits::dex::AssetAmount;
use frame_support::{
	assert_noop, assert_ok,
	traits::{
		fungibles::{Inspect, Mutate},
		TryCollect,
	},
};
use frame_system::EventRecord;
use sp_arithmetic::{PerThing, Permill};
use sp_core::H256;
use sp_runtime::{traits::ConstU32, BoundedBTreeMap, TokenError};
use sp_std::collections::btree_map::BTreeMap;

pub fn dual_asset_pool_weights(
	first_asset: AssetId,
	first_asset_weight: Permill,
	second_asset: AssetId,
) -> BoundedBTreeMap<AssetId, Permill, ConstU32<2>> {
	[(first_asset, first_asset_weight), (second_asset, first_asset_weight.left_from_one())]
		.into_iter()
		.try_collect()
		.expect("only 2 elements present, should not fail; qed;")
}

/// `expected_lp_check` takes base_amount, quote_amount and lp_tokens in order and returns
/// true if lp_tokens are expected for given base_amount, quote_amount.
pub fn common_add_remove_lp(
	init_config: PoolInitConfiguration<AccountId, AssetId>,
	first_asset_amount: Balance,
	second_asset_amount: Balance,
	next_first_asset_amount: Balance,
	next_second_asset_amount: Balance,
	lp_token_id: AssetId,
	expected_lp_check: impl Fn(Balance, Balance, Balance) -> bool,
) {
	System::set_block_number(System::block_number() + 1);
	let actual_pool_id = Pablo::do_create_pool(init_config.clone(), Some(lp_token_id))
		.expect("pool creation failed");
	assert_has_event::<Test, _>(
		|e| matches!(e.event, mock::Event::Pablo(crate::Event::PoolCreated { pool_id, .. }) if pool_id == actual_pool_id),
	);
	let pair = get_pair(init_config);
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair[0], &ALICE, first_asset_amount));
	assert_ok!(Tokens::mint_into(pair[1], &ALICE, second_asset_amount));

	System::set_block_number(System::block_number() + 1);
	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(ALICE),
		actual_pool_id,
		BTreeMap::from([(pair[0], first_asset_amount), (pair[1], second_asset_amount)]),
		0,
		false
	));
	assert_last_event::<Test, _>(|e| {
		matches!(e.event,
			mock::Event::Pablo(crate::Event::LiquidityAdded { who, pool_id, /* base_amount, quote_amount, */ .. })
			if who == ALICE
			&& pool_id == actual_pool_id
			// && base_amount == first_asset_amount
			// && quote_amount == second_asset_amount
		)
	});

	let pool = Pablo::pools(actual_pool_id).expect("pool not found");
	let lp_token = match pool {
		DualAssetConstantProduct(pool) => pool.lp_token,
	};
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair[0], &BOB, next_first_asset_amount));
	assert_ok!(Tokens::mint_into(pair[1], &BOB, next_second_asset_amount));

	let lp = Tokens::balance(lp_token, &BOB);
	assert_eq!(lp, 0_u128);

	System::set_block_number(System::block_number() + 1);
	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(BOB),
		actual_pool_id,
		BTreeMap::from([(pair[0], next_first_asset_amount), (pair[1], next_second_asset_amount)]),
		0,
		false
	));
	assert_last_event::<Test, _>(|e| {
		matches!(e.event,
		mock::Event::Pablo(crate::Event::LiquidityAdded { who, pool_id, /* base_amount, quote_amount, */  .. })
		if who == BOB
			&& pool_id == actual_pool_id
			// && base_amount == next_first_asset_amount
			// && quote_amount == next_second_asset_amount
		)
	});
	let lp = Tokens::balance(lp_token, &BOB);
	assert!(expected_lp_check(next_first_asset_amount, next_second_asset_amount, lp));
	assert_ok!(Pablo::remove_liquidity(
		Origin::signed(BOB),
		actual_pool_id,
		lp,
		BTreeMap::from([(pair[0], 0_u128), (pair[1], 0_u128)]),
	));
	let lp = Tokens::balance(lp_token, &BOB);
	// all lp tokens must have been burnt
	assert_eq!(lp, 0_u128);
}

pub fn get_pair(init_config: PoolInitConfiguration<AccountId, AssetId>) -> [AssetId; 2] {
	match init_config {
		PoolInitConfiguration::DualAssetConstantProduct { assets_weights, .. } => assets_weights
			.keys()
			.copied()
			.collect::<Vec<_>>()
			.try_into()
			.expect("pool should have exactly 2 assets; qed;"),
	}
}

/// `expected_lp` is a function with `base_amount`, `quote_amount`, `lp_total_issuance`,
/// `pool_base_amount` and `pool_quote_amount` parameters and returns amount of expected new
/// lp_tokens.
pub fn common_add_lp_with_min_mint_amount(
	init_config: PoolInitConfiguration<AccountId, AssetId>,
	init_first_asset_amount: Balance,
	init_second_asset_amount: Balance,
	first_asset_amount: Balance,
	second_asset_amount: Balance,
	lp_token_id: AssetId,
	expected_lp: impl Fn(Balance, Balance, Balance, Balance, Balance) -> Balance,
) {
	Pablo::create(Origin::root(), init_config.clone(), Some(lp_token_id))
		.expect("pool creation failed");

	let pool_id = System::events()
		.into_iter()
		.find_map(|event| match event.event {
			Event::Pablo(crate::Event::PoolCreated { pool_id, .. }) => Some(pool_id),
			_ => None,
		})
		.expect("pool creation should emit an event if successful; qed;");

	let pool = Pablo::pools(pool_id).expect("pool not found");

	let lp_token = match pool {
		DualAssetConstantProduct(pool) => pool.lp_token,
	};

	let [first_asset, second_asset] = get_pair(init_config);

	// Mint the tokens
	assert_ok!(Tokens::mint_into(first_asset, &ALICE, init_first_asset_amount));
	assert_ok!(Tokens::mint_into(second_asset, &ALICE, init_second_asset_amount));

	let assets_with_amounts = BTreeMap::from([
		(first_asset, init_first_asset_amount),
		(second_asset, init_second_asset_amount),
	]);

	// Add the liquidity, min amount = 0
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(ALICE),
		pool_id,
		assets_with_amounts.clone(),
		0,
		false
	));

	// Mint the tokens
	assert_ok!(Tokens::mint_into(first_asset, &BOB, first_asset_amount));
	assert_ok!(Tokens::mint_into(second_asset, &BOB, second_asset_amount));

	let alice_lp = Tokens::balance(lp_token, &ALICE);
	let bob_lp = Tokens::balance(lp_token, &BOB);

	assert_eq!(bob_lp, 0_u128, "BOB should not have any LP tokens");

	let min_mint_amount = expected_lp(
		first_asset_amount,
		second_asset_amount,
		bob_lp + alice_lp,
		init_first_asset_amount,
		init_second_asset_amount,
	);

	// Add the liquidity, but expect more lp tokens, hence errors
	assert_noop!(
		Pablo::add_liquidity(
			Origin::signed(BOB),
			pool_id,
			assets_with_amounts.clone(),
			min_mint_amount + 1,
			false
		),
		crate::Error::<Test>::CannotRespectMinimumRequested
	);

	// Add liquidity with min_mint_amount
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(BOB),
		pool_id,
		assets_with_amounts,
		min_mint_amount,
		false
	));
}

pub fn common_remove_lp_failure(
	init_config: PoolInitConfiguration<AccountId, AssetId>,
	init_base_amount: Balance,
	init_quote_amount: Balance,
	base_amount: Balance,
	quote_amount: Balance,
	lp_token_id: AssetId,
) {
	let pool_id = Pablo::do_create_pool(init_config.clone(), Some(lp_token_id))
		.expect("pool creation failed");
	let pair = get_pair(init_config);
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair[0], &ALICE, init_base_amount));
	assert_ok!(Tokens::mint_into(pair[1], &ALICE, init_quote_amount));

	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(ALICE),
		pool_id,
		BTreeMap::from([(pair[0], init_base_amount), (pair[1], init_quote_amount)]),
		0,
		false
	));

	let pool = Pablo::pools(pool_id).expect("pool not found");
	let lp_token = match pool {
		DualAssetConstantProduct(pool) => pool.lp_token,
	};
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair[0], &BOB, base_amount));
	assert_ok!(Tokens::mint_into(pair[1], &BOB, quote_amount));

	let lp = Tokens::balance(lp_token, &BOB);
	assert_eq!(lp, 0_u128);
	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(BOB),
		pool_id,
		BTreeMap::from([(pair[0], base_amount), (pair[1], quote_amount)]),
		0,
		false
	));
	let lp = Tokens::balance(lp_token, &BOB);
	// error as trying to redeem more tokens than lp
	assert_noop!(
		Pablo::remove_liquidity(
			Origin::signed(BOB),
			pool_id,
			lp + 1,
			BTreeMap::from([(pair[0], 0), (pair[1], 0)])
		),
		TokenError::NoFunds
	);
	let min_expected_base_amount = base_amount + 1;
	let min_expected_quote_amount = quote_amount + 1;
	// error as expected values are more than actual redeemed values.
	assert_noop!(
		Pablo::remove_liquidity(
			Origin::signed(BOB),
			pool_id,
			lp,
			BTreeMap::from([
				(pair[0], min_expected_base_amount),
				(pair[1], min_expected_quote_amount)
			])
		),
		crate::Error::<Test>::CannotRespectMinimumRequested
	);
}

pub fn common_exchange_failure(
	init_config: PoolInitConfiguration<AccountId, AssetId>,
	init_first_amount: AssetAmount<AssetId, Balance>,
	init_second_amount: AssetAmount<AssetId, Balance>,
	exchange_first_amount: AssetAmount<AssetId, Balance>,
	lp_token_id: AssetId,
) {
	let pool_id =
		Pablo::do_create_pool(init_config, Some(lp_token_id)).expect("pool creation failed");
	// Mint the tokens
	assert_ok!(Tokens::mint_into(init_first_amount.asset_id, &ALICE, init_first_amount.amount));
	assert_ok!(Tokens::mint_into(init_second_amount.asset_id, &ALICE, init_second_amount.amount));

	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(ALICE),
		pool_id,
		BTreeMap::from([
			(init_first_amount.asset_id, init_first_amount.amount),
			(init_second_amount.asset_id, init_second_amount.amount)
		]),
		0,
		false
	));

	// Mint the tokens
	assert_ok!(Tokens::mint_into(init_first_amount.asset_id, &BOB, exchange_first_amount.amount));
	// error as trying to swap more value than balance
	assert_noop!(
		Pablo::swap(
			Origin::signed(BOB),
			pool_id,
			AssetAmount::new(exchange_first_amount.asset_id, exchange_first_amount.amount + 1),
			AssetAmount::new(init_second_amount.asset_id, 0),
			false
		),
		orml_tokens::Error::<Test>::BalanceTooLow
	);
	let expected_value = exchange_first_amount.amount + 1;
	// error as expected_value is more that input
	assert_noop!(
		Pablo::swap(
			Origin::signed(BOB),
			pool_id,
			AssetAmount::new(exchange_first_amount.asset_id, exchange_first_amount.amount),
			AssetAmount::new(init_second_amount.asset_id, expected_value),
			false
		),
		crate::Error::<Test>::CannotRespectMinimumRequested
	);
}

pub fn assert_has_event<T, F>(matcher: F)
where
	T: Config,
	F: Fn(&EventRecord<mock::Event, H256>) -> bool,
{
	assert!(System::events().iter().any(matcher));
}

pub fn assert_last_event<T, F>(matcher: F)
where
	T: Config,
	F: FnOnce(&EventRecord<mock::Event, H256>) -> bool,
{
	assert!(matcher(System::events().last().expect("events expected")));
}

mod create {
	use super::*;
	use sp_runtime::Permill;

	#[test]
	fn signed_user_can_create() {
		new_test_ext().execute_with(|| {
			System::set_block_number(1);
			assert_ok!(Pablo::create(
				Origin::signed(ALICE),
				PoolInitConfiguration::DualAssetConstantProduct {
					owner: ALICE,
					assets_weights: dual_asset_pool_weights(BTC, Permill::from_percent(50), USDT),
					fee: Permill::zero(),
				},
				Some(LP_TOKEN_ID),
			));
			assert_has_event::<Test, _>(|e| {
				matches!(e.event, mock::Event::Pablo(crate::Event::PoolCreated { pool_id: 0, .. }))
			});
		});
	}
}
