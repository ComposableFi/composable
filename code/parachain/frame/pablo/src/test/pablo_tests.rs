#![allow(clippy::disallowed_methods, clippy::unwrap_used)]

use crate::{
	mock::{Pablo, *},
	test::common_test_functions::*,
};

use composable_tests_helpers::test::helper::RuntimeTrait;
use frame_support::assert_ok;
use sp_runtime::Permill;

mod create {

	use crate::PoolInitConfigurationOf;

	use super::*;

	#[test]
	fn should_successfully_create_50_50_pool() {
		new_test_ext().execute_with(|| {
			let owner = ALICE;
			let assets_weights = dual_asset_pool_weights(USDC, Permill::from_percent(50), USDT);
			let fee = Permill::from_percent(1);
			let pool_config = PoolInitConfigurationOf::<Test>::DualAssetConstantProduct {
				owner,
				assets_weights,
				fee,
			};

			assert_ok!(Pablo::do_create_pool(pool_config, Some(LP_TOKEN_ID)));
		});
	}
}

mod simulate {
	use super::*;

	use composable_traits::dex::Amm;
	use frame_support::{bounded_btree_map, traits::fungibles::Mutate};
	use sp_runtime::Permill;

	use crate::{
		mock::{new_test_ext, RuntimeOrigin, Pablo, System, Test, ALICE},
		Event, PoolInitConfiguration,
	};

	#[test]
	fn simulation_same_as_actual() {
		new_test_ext().execute_with(|| {
			System::set_block_number(1);
			let pool_id = Test::assert_extrinsic_event_with(
				Pablo::create(
					RuntimeOrigin::signed(ALICE),
					PoolInitConfiguration::DualAssetConstantProduct {
						owner: ALICE,
						assets_weights: bounded_btree_map! {
							USDT => Permill::from_percent(50),
							USDC => Permill::from_percent(50),
						},
						fee: Permill::from_percent(1),
					},
				),
				|e| match e {
					Event::PoolCreated { pool_id, .. } => Some(pool_id),
					_ => None,
				},
			);

			Tokens::mint_into(USDT, &BOB, 1_000_000_000_000_000).unwrap();
			Tokens::mint_into(USDC, &BOB, 1_000_000_000_000_000).unwrap();

			let add_simulation_result = <Pablo as Amm>::simulate_add_liquidity(
				&BOB,
				pool_id,
				[(USDT, 100_000_000), (USDC, 100_000_000)].into_iter().collect(),
			)
			.unwrap();

			let add_result = Test::assert_extrinsic_event_with(
				Pablo::add_liquidity(
					RuntimeOrigin::signed(BOB),
					pool_id,
					[(USDT, 100_000_000), (USDC, 100_000_000)].into_iter().collect(),
					0,
					false,
				),
				|e| match e {
					Event::LiquidityAdded { minted_lp, .. } => Some(minted_lp),
					_ => None,
				},
			);

			assert_eq!(add_simulation_result, add_result);

			let remove_simulation_result = <Pablo as Amm>::simulate_remove_liquidity(
				&BOB,
				pool_id,
				add_result,
				[(USDT, 0), (USDC, 0)].into_iter().collect(),
			)
			.unwrap();

			let remove_result = Test::assert_extrinsic_event_with(
				Pablo::remove_liquidity(
					RuntimeOrigin::signed(BOB),
					pool_id,
					add_result,
					[(USDT, 0), (USDC, 0)].into_iter().collect(),
				),
				|e| match e {
					Event::LiquidityRemoved { asset_amounts, .. } => Some(asset_amounts),
					_ => None,
				},
			);

			assert_eq!(remove_simulation_result, remove_result);
		})
	}
}
