use crate::{
	mock,
	mock::{Pablo, *},
	Config,
	PoolConfiguration::ConstantProduct,
	PoolInitConfiguration,
};
use frame_support::{
	assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use frame_system::EventRecord;
use sp_core::H256;
use sp_runtime::TokenError;

/// `expected_lp_check` takes base_amount, quote_amount and lp_tokens in order and returns
/// true if lp_tokens are expected for given base_amount, quote_amount.
pub fn common_add_remove_lp(
	init_config: PoolInitConfiguration<AccountId, AssetId>,
	init_base_amount: Balance,
	init_quote_amount: Balance,
	next_base_amount: Balance,
	next_quote_amount: Balance,
	expected_lp_check: impl Fn(Balance, Balance, Balance) -> bool,
) {
	System::set_block_number(System::block_number() + 1);
	let actual_pool_id = Pablo::do_create_pool(init_config.clone()).expect("pool creation failed");
	assert_has_event::<Test, _>(
		|e| matches!(e.event, mock::Event::Pablo(crate::Event::PoolCreated { pool_id, .. }) if pool_id == actual_pool_id),
	);
	let pair = match init_config {
		PoolInitConfiguration::ConstantProduct { pair, .. } => pair,
	};
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair.base, &ALICE, init_base_amount));
	assert_ok!(Tokens::mint_into(pair.quote, &ALICE, init_quote_amount));

	System::set_block_number(System::block_number() + 1);
	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(ALICE),
		actual_pool_id,
		init_base_amount,
		init_quote_amount,
		0,
		false
	));
	assert_last_event::<Test, _>(|e| {
		matches!(e.event,
            mock::Event::Pablo(crate::Event::LiquidityAdded { who, pool_id, base_amount, quote_amount, .. })
            if who == ALICE
            && pool_id == actual_pool_id
            && base_amount == init_base_amount
            && quote_amount == init_quote_amount)
	});

	let pool = Pablo::pools(actual_pool_id).expect("pool not found");
	let lp_token = match pool {
		ConstantProduct(pool) => pool.lp_token,
	};
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair.base, &BOB, next_base_amount));
	assert_ok!(Tokens::mint_into(pair.quote, &BOB, next_quote_amount));

	let lp = Tokens::balance(lp_token, &BOB);
	assert_eq!(lp, 0_u128);

	System::set_block_number(System::block_number() + 1);
	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(BOB),
		actual_pool_id,
		next_base_amount,
		next_quote_amount,
		0,
		false
	));
	assert_last_event::<Test, _>(|e| {
		matches!(e.event,
            mock::Event::Pablo(crate::Event::LiquidityAdded { who, pool_id, base_amount, quote_amount, .. })
			if who == BOB
				&& pool_id == actual_pool_id
				&& base_amount == next_base_amount
				&& quote_amount == next_quote_amount)
	});
	let lp = Tokens::balance(lp_token, &BOB);
	assert!(expected_lp_check(next_base_amount, next_quote_amount, lp));
	assert_ok!(Pablo::remove_liquidity(Origin::signed(BOB), actual_pool_id, lp, 0, 0));
	let lp = Tokens::balance(lp_token, &BOB);
	// all lp tokens must have been burnt
	assert_eq!(lp, 0_u128);
}
/// `expected_lp` is a function with `base_amount`, `quote_amount`, `lp_total_issuance`,
/// `pool_base_amount` and `pool_quote_amount` parameters and returns amount of expected new
/// lp_tokens.
pub fn common_add_lp_with_min_mint_amount(
	init_config: PoolInitConfiguration<AccountId, AssetId>,
	init_base_amount: Balance,
	init_quote_amount: Balance,
	base_amount: Balance,
	quote_amount: Balance,
	expected_lp: impl Fn(Balance, Balance, Balance, Balance, Balance) -> Balance,
) {
	let pool_id = Pablo::do_create_pool(init_config.clone()).expect("pool creation failed");
	let pair = match init_config {
		PoolInitConfiguration::ConstantProduct { pair, .. } => pair,
	};
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair.base, &ALICE, init_base_amount));
	assert_ok!(Tokens::mint_into(pair.quote, &ALICE, init_quote_amount));

	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(ALICE),
		pool_id,
		init_base_amount,
		init_quote_amount,
		0,
		false
	));

	let pool = Pablo::pools(pool_id).expect("pool not found");
	let lp_token = match pool {
		ConstantProduct(pool) => pool.lp_token,
	};
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair.base, &BOB, base_amount));
	assert_ok!(Tokens::mint_into(pair.quote, &BOB, quote_amount));

	let alice_lp = Tokens::balance(lp_token, &ALICE);
	let bob_lp = Tokens::balance(lp_token, &BOB);
	assert_eq!(bob_lp, 0_u128);
	let min_mint_amount = expected_lp(
		base_amount,
		quote_amount,
		bob_lp + alice_lp,
		init_base_amount,
		init_quote_amount,
	);
	// Add the liquidity, but expect more lp tokens, hence errors
	assert_noop!(
		Pablo::add_liquidity(
			Origin::signed(BOB),
			pool_id,
			base_amount,
			quote_amount,
			min_mint_amount + 1,
			false
		),
		crate::Error::<Test>::CannotRespectMinimumRequested
	);
	// Add liquidity with min_mint_amount
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(BOB),
		pool_id,
		base_amount,
		quote_amount,
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
) {
	let pool_id = Pablo::do_create_pool(init_config.clone()).expect("pool creation failed");
	let pair = match init_config {
		PoolInitConfiguration::ConstantProduct { pair, .. } => pair,
	};
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair.base, &ALICE, init_base_amount));
	assert_ok!(Tokens::mint_into(pair.quote, &ALICE, init_quote_amount));

	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(ALICE),
		pool_id,
		init_base_amount,
		init_quote_amount,
		0,
		false
	));

	let pool = Pablo::pools(pool_id).expect("pool not found");
	let lp_token = match pool {
		ConstantProduct(pool) => pool.lp_token,
	};
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair.base, &BOB, base_amount));
	assert_ok!(Tokens::mint_into(pair.quote, &BOB, quote_amount));

	let lp = Tokens::balance(lp_token, &BOB);
	assert_eq!(lp, 0_u128);
	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(BOB),
		pool_id,
		base_amount,
		quote_amount,
		0,
		false
	));
	let lp = Tokens::balance(lp_token, &BOB);
	// error as trying to redeem more tokens than lp
	assert_noop!(
		Pablo::remove_liquidity(Origin::signed(BOB), pool_id, lp + 1, 0, 0),
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
			min_expected_base_amount,
			min_expected_quote_amount,
		),
		crate::Error::<Test>::CannotRespectMinimumRequested
	);
}

pub fn common_exchange_failure(
	init_config: PoolInitConfiguration<AccountId, AssetId>,
	init_base_amount: Balance,
	init_quote_amount: Balance,
	exchange_base_amount: Balance,
) {
	let pool_id = Pablo::do_create_pool(init_config.clone()).expect("pool creation failed");
	let pair = match init_config {
		PoolInitConfiguration::ConstantProduct { pair, .. } => pair,
	};
	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair.base, &ALICE, init_base_amount));
	assert_ok!(Tokens::mint_into(pair.quote, &ALICE, init_quote_amount));

	// Add the liquidity
	assert_ok!(Pablo::add_liquidity(
		Origin::signed(ALICE),
		pool_id,
		init_base_amount,
		init_quote_amount,
		0,
		false
	));

	// Mint the tokens
	assert_ok!(Tokens::mint_into(pair.base, &BOB, exchange_base_amount));
	// error as trying to swap more value than balance
	assert_noop!(
		Pablo::swap(Origin::signed(BOB), pool_id, pair.swap(), exchange_base_amount + 1, 0, false),
		orml_tokens::Error::<Test>::BalanceTooLow
	);
	let expected_value = exchange_base_amount + 1;
	// error as expected_value is more that input
	assert_noop!(
		Pablo::swap(
			Origin::signed(BOB),
			pool_id,
			pair.swap(),
			exchange_base_amount,
			expected_value,
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
	use composable_traits::defi::CurrencyPair;
	use sp_runtime::Permill;

	#[test]
	fn signed_user_can_create() {
		new_test_ext().execute_with(|| {
			System::set_block_number(1);
			assert_ok!(
				Pablo::create(
					Origin::signed(ALICE),
					PoolInitConfiguration::ConstantProduct {
						owner: ALICE,
						pair: CurrencyPair::new(BTC, USDT),
						fee: Permill::zero(),
						base_weight: Permill::from_percent(50),
					}
				)
			);
			assert_has_event::<Test, _>(
				|e|
					matches!(e.event, mock::Event::Pablo(crate::Event::PoolCreated { pool_id, .. }) if pool_id == 0)
			);
		});
	}
}
