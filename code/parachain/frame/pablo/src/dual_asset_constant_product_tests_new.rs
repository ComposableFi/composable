use composable_tests_helpers::test::{
	block::{next_block, process_and_progress_blocks},
	currency::{BTC, USDT},
	helper::RuntimeTrait,
};
use frame_support::{
	assert_ok,
	traits::fungibles::{Inspect, Mutate},
};
use sp_runtime::Permill;
use sp_std::collections::btree_map::BTreeMap;

//- test lp mint/burn
use crate::{
	common_test_functions::dual_asset_pool_weights,
	dual_asset_constant_product_tests::{create_pool_from_config, lp_token_of_pool},
	mock::*,
	PoolInitConfiguration,
};

#[test]
fn add_remove_lp() {
	new_test_ext().execute_with(|| {
		next_block::<Pablo, Test>();

		let first_asset = BTC::ID;
		let second_asset = USDT::ID;

		let init_config = PoolInitConfiguration::DualAssetConstantProduct {
			owner: ALICE,
			assets_weights: dual_asset_pool_weights(
				first_asset,
				Permill::from_percent(50_u32),
				second_asset,
			),
			fee: Permill::zero(),
		};

		let first_asset_amount = BTC::units(100);
		let second_asset_amount = USDT::units(100);
		let next_first_asset_amount = BTC::units(10);
		let next_second_asset_amount = USDT::units(10);

		process_and_progress_blocks::<Pablo, Test>(1);

		let pool_id = create_pool_from_config(init_config);
		let lp_token = lp_token_of_pool(pool_id);

		let assets_with_amounts = BTreeMap::from([
			(first_asset, first_asset_amount),
			(second_asset, second_asset_amount),
		]);

		let assets_with_next_amounts = BTreeMap::from([
			(first_asset, next_first_asset_amount),
			(second_asset, next_second_asset_amount),
		]);

		// Mint the tokens
		assert_ok!(Tokens::mint_into(first_asset, &ALICE, first_asset_amount));
		assert_ok!(Tokens::mint_into(second_asset, &ALICE, second_asset_amount));

		process_and_progress_blocks::<Pablo, Test>(1);

		Test::assert_extrinsic_event(
			Pablo::add_liquidity(Origin::signed(ALICE), pool_id, assets_with_amounts, 0, false),
			crate::Event::LiquidityAdded { who: ALICE, pool_id, minted_lp: 199_999_999_814_806 },
		);

		// Mint the tokens
		assert_ok!(Tokens::mint_into(first_asset, &BOB, next_first_asset_amount));
		assert_ok!(Tokens::mint_into(second_asset, &BOB, next_second_asset_amount));

		assert_eq!(Tokens::balance(lp_token, &BOB), 0_u128);

		process_and_progress_blocks::<Pablo, Test>(1);

		// Add the liquidity
		Test::assert_extrinsic_event(
			Pablo::add_liquidity(Origin::signed(BOB), pool_id, assets_with_next_amounts, 0, false),
			crate::Event::LiquidityAdded { who: BOB, pool_id, minted_lp: 39999999962960 },
		);

		assert!(dbg!(Tokens::balance(lp_token, &BOB)) > 0);

		assert_ok!(Pablo::remove_liquidity(
			Origin::signed(BOB),
			pool_id,
			Tokens::balance(lp_token, &BOB),
			BTreeMap::from([(first_asset, 0_u128), (second_asset, 0_u128)]),
		));

		assert_eq!(Tokens::balance(lp_token, &BOB), 0_u128, "all lp tokens must have been burnt");
	});
}
