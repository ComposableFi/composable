use crate as pallet_pool;

use crate::{
	pallet::{PoolIdAndAssetIdToVaultId/*, Pools as PoolIdToPoolInfo*/},
	mocks::{
		currency_factory::{MockCurrencyId, strategy_pick_random_mock_currency},
		tests::{
			ALICE, Balance, /*BOB, */ExtBuilder, Pools, Test, Tokens, Vaults,
		},
	},
	// *,
};
use composable_traits::{
	defi::{LiftedFixedBalance, ZeroToOneFixedU128},
	pool::{
		Bound, ConstantMeanMarket, Deposit, PoolConfig, Weight,
	},
	vault::Vault,
};

use crate::{Error};

use frame_support::{
	assert_noop, assert_ok,
	traits::fungibles::{Inspect, Mutate},
	sp_runtime::Perquintill,
};
use num_traits::Zero;

// use num_integer::Roots;
use sp_runtime::{
	FixedPointNumber,
	/*FixedU128, */traits::Saturating,
	// traits::{
	// 	One,
	// },
};
use fixed::{
	traits::{Fixed, FixedSigned, LossyFrom, ToFixed},
	types::{
		I110F18, I9F23, I32F32,
		extra::U7 as Frac,
		extra::Unsigned,
	},
	FixedU128, FixedI128,
	transcendental::{exp, ln, pow},
};
use std::collections::BTreeSet;

use proptest::prelude::*;

const MAX_POOL_SIZE: u8 = 26;

prop_compose! {
	fn generate_creation_fee(number_of_assets: usize) 
		(x in 0..Pools::required_creation_deposit_for(number_of_assets).unwrap()) -> Balance{
			x
	}
}

prop_compose! {
	fn generate_native_balance(number_of_assets: usize) 
		(x in 0..Pools::required_creation_deposit_for(number_of_assets).unwrap()*2) -> Balance{
			x
	}
}

prop_compose! {
	fn generate_initial_assets() 
		(
			initial_assets in prop::collection::vec(strategy_pick_random_mock_currency(), 2usize..26usize),
		) -> Vec<MockCurrencyId>{
			initial_assets
	}
}

prop_compose! {
	fn generate_initial_assets_without_duplicates() 
		(
			initial_assets in prop::collection::vec(strategy_pick_random_mock_currency(), 2usize..26usize),
		) -> Vec<MockCurrencyId>{
			BTreeSet::<MockCurrencyId>::from_iter(initial_assets.iter().copied())
				.iter()
				.copied()
				.collect()
	}
}

prop_compose! {
	fn generate_pool_bounds() 
		(
			x in 0..5, 
		 	y in 4..20,
		) -> (u8, u8){
			(x as u8, y as u8)
	}
}

fn generate_random_weights() -> impl proptest::strategy::Strategy<Value = Vec<Perquintill>> {
	generate_random_weights_with_size_bounds(MAX_POOL_SIZE as usize, MAX_POOL_SIZE as usize)
}

prop_compose! {
	fn generate_random_weights_with_size_bounds(min: usize, max: usize)
		(
			nonnormalized_weights in prop::collection::vec(1u64..100u64, min..=max),
		) -> Vec<Perquintill> {

			let mut weights_as_perquintill: Vec<Perquintill> = Vec::new();
			for weight in nonnormalized_weights {
				weights_as_perquintill.push(Perquintill::from_percent(weight));
			}

			weights_as_perquintill
	}
}

prop_compose! {
	fn generate_weight_bounds() 
		(
			minimim in 0u64..30u64, 
		 	maximum in 25u64..100u64,
		) -> (Perquintill, Perquintill){
			(Perquintill::from_percent(minimim), Perquintill::from_percent(maximum))
	}
}


fn equal_weight_vector_for(assets: &Vec<MockCurrencyId>) -> Vec<Weight<MockCurrencyId>>{
	let mut weights = Vec::new();

	for asset in assets {
		weights.push(Weight {
			asset_id: *asset,
			weight: Perquintill::from_rational(1, assets.len() as u64),
		});
	}

	weights
}

fn normalize_weights(non_normalized_weights: &mut Vec<Perquintill>) -> Vec<Perquintill> {
	let sum = non_normalized_weights.iter().map(|weight| weight.deconstruct()).sum();
	
	// println!("sum: {:?}", sum);
	let mut weights: Vec<Perquintill> = Vec::new();
	for weight in non_normalized_weights.iter_mut() {
		weights.push(Perquintill::from_rational(weight.deconstruct(), sum));
	}
	// println!("normalized {:?}", weights);
	// println!("one {:?}", Perquintill::one());
	// let sum: u64 = weights.iter().map(|weight| weight.deconstruct()).sum();
	// println!("sum {:?}", sum);
	weights
}

fn construct_weight_vector_from(assets: &Vec<MockCurrencyId>, perquintills: &Vec<Perquintill>) -> Vec<Weight<MockCurrencyId>> {
	let mut weights: Vec<Weight<MockCurrencyId>> = Vec::new();

	for (asset, weight) in assets.iter().zip(perquintills.iter()) {
		weights.push(Weight{
			asset_id: *asset,
			weight: *weight,
		});
	}

	weights
}

// ----------------------------------------------------------------------------------------------------
//                                               Create                                              
// ----------------------------------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]

	#[test]
	fn creating_a_pool_that_meets_all_requirements_does_not_raise_an_error(
		initial_assets in generate_initial_assets_without_duplicates(),
	) {
		// Tests that if all the conditions are met to create a pool it will be created 
		//  |  successfully
		//  '-> Conditions:
		//		  i.	∀(i, j) a_i != a_j -- no duplicate assets
		//		  ii.	min_underlying_tokens ≤ max_underlying_tokens
		//        iii.	min_underlying_tokens ≤ n ≤ max_underlying_tokens, where n is the number
		//					of tokens in the pool
		//        iv.	∀ assets a_i ⇒ ∃ weight w_i
		//        v.	Σ w_i = 1 & w_i ≥ 0
		//		  vi.	min_weight ≤ max_weight
		//        vii.	min_weight ≤ w_i ≤ max_weight
		//        viii.	creation_fee ≥ (asset_ids.len() + 1) * (creation_deposit + existential_deposit)
		//        ix.	user_balance ≥ creation_fee

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfig {
				manager: ALICE,

				assets: initial_assets.clone(),
				// Condition ii && Condition iii
				asset_bounds: Bound {
					minimum: 0, 
					maximum: MAX_POOL_SIZE,
				},

				// Condition iv && Condition v
				weights: equal_weight_vector_for(&initial_assets),
				// Condition vi && Condition vii
				weight_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},

				deposit_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},
				withdraw_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},

				transaction_fee: Perquintill::zero(),
			};

			// Condition viii
			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
			};

			// Condition ix
			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));
		
			assert_ok!(<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee));
		});
	}

	#[test]
	fn creating_a_pool_that_does_not_meet_the_asset_requirements_raises_an_error(
		initial_assets in generate_initial_assets(),
		(minimum_asset_bound, maximum_asset_bound) in generate_pool_bounds(),
	) {
		// Tests that if not all asset conditions are met to create a pool it will not
		//  |  be created
		//  '-> Conditions:
		//		  i.    ∀(i, j) a_i != a_j -- no duplicate assets
		//		  ii.	min_underlying_tokens ≤ max_underlying_tokens
		//        iii.  min_underlying_tokens ≤ n ≤ max_underlying_tokens, where n is the number
		//                 of tokens in the pool		
		//        ...

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfig {
				manager: ALICE,

				assets: initial_assets.clone(),
				asset_bounds: Bound {
					minimum: minimum_asset_bound, 
					maximum: maximum_asset_bound,
				},

				weights: equal_weight_vector_for(&initial_assets),
				weight_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},

				deposit_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},
				withdraw_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},

				transaction_fee: Perquintill::zero(),
			};

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));

			let pool_size = config.assets.len();
			// Condition i
			if BTreeSet::<MockCurrencyId>::from_iter(initial_assets.iter().copied()).len() != pool_size {
				assert_noop!(
					<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee), 
					Error::<Test>::DuplicateAssets
				);
			// Condition ii
			} else if config.asset_bounds.maximum < config.asset_bounds.minimum {
				assert_noop!(
					<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee), 
					Error::<Test>::InvalidAssetBounds
				);
			// Condition iii
			} else if pool_size < config.asset_bounds.minimum as usize || (config.asset_bounds.maximum as usize) < pool_size {
				assert_noop!(
					<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee), 
					Error::<Test>::PoolSizeIsOutsideOfAssetBounds
				);
			} else {
				assert_ok!(<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee));
			}
		});
	}

	#[test]
	fn creating_a_pool_that_does_not_meet_the_weight_requirements_raises_an_error(
		initial_assets in generate_initial_assets(),
		mut weights in generate_random_weights(),
		(weight_minimum, weight_maximum) in generate_weight_bounds(),
	) {
		// Tests that if not all weight conditions are met to create a pool it will not
		//  |  be created
		//  '-> Conditions:
		//        ...
		//        iv.  ∀ assets a_i ⇒ ∃ weight w_i
		//		  v.   Σ w_i = 1 & w_i ≥ 0
		//		  vi.  min_weight ≤ max_weight
		// 		  vii. min_weight ≤ w_i ≤ max_weight	
		//        ...

		// create a random weight vector for the generated initial assets,
		//     however, at this stage there might be duplicate assets so these 
		//     assets will have multiple weights in the weight vector
		weights.resize(initial_assets.len(), Perquintill::zero());
		let weights: Vec<Perquintill> = normalize_weights(&mut weights);
		let weights: Vec<Weight<MockCurrencyId>> = construct_weight_vector_from(&initial_assets, &weights);

		// remove duplicate assets
		let initial_assets: Vec<MockCurrencyId> = BTreeSet::<MockCurrencyId>::from_iter(initial_assets.iter().copied())
			.iter()
			.copied()
			.collect();

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfig {
				manager: ALICE,

				assets: initial_assets.clone(),
				asset_bounds: Bound {
					minimum: 0, 
					maximum: MAX_POOL_SIZE,
				},

				weights: weights,
				weight_bounds: Bound {
					minimum: weight_minimum, 
					maximum: weight_maximum
				},

				deposit_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},
				withdraw_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},

				transaction_fee: Perquintill::zero(),
			};

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));
			
			// Condition iv
			if !Pools::each_asset_has_a_corresponding_weight(&config.assets, &config.weights) {
				assert_noop!(
					<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee), 
					Error::<Test>::ThereMustBeOneWeightForEachAssetInThePool
				);
			// Condition v
			} else if !Pools::weights_are_normalized_and_nonnegative(&config.weights) {
				assert_noop!(
					<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee), 
					Error::<Test>::PoolWeightsMustBeNormalized
				);
			// Condition vi
			} else if weight_maximum < weight_minimum {
				assert_noop!(
					<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee), 
					Error::<Test>::InvalidWeightBounds
				);
			// Condition vii
			} else if !Pools::weights_are_in_weight_bounds(&config.weights, &config.weight_bounds) {
				assert_noop!(
					<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee), 
					Error::<Test>::PoolWeightsAreOutsideOfWeightBounds
				);
			} else {
				assert_ok!(<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee));
			}
		});
	}

	#[test]
	fn creating_a_pool_that_does_not_meet_the_user_requirements_raises_an_error(
		initial_assets in generate_initial_assets_without_duplicates(),
		user_balance in generate_native_balance(26usize),
		creation_fee in generate_creation_fee(26usize),
	) {
		// Tests that if all the conditions are met to create a pool it will be created 
		//  |  successfully
		//  '-> Conditions:
		//		  ...
		//        viii.	user_balance ≥ creation_fee
		//        ix.	creation_fee ≥ (asset_ids.len() + 1) * (creation_deposit + existential_deposit)

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfig {
				manager: ALICE,

				assets: initial_assets.clone(),
				asset_bounds: Bound {
					minimum: 0, 
					maximum: 26
				},

				weights: equal_weight_vector_for(&initial_assets),
				weight_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},

				deposit_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},
				withdraw_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},

				transaction_fee: Perquintill::zero(),
			};

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: creation_fee, 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, user_balance));
			
			// Condition viii
			if user_balance < creation_fee.amount {
				assert_noop!(
					<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee), 
					Error::<Test>::IssuerDoesNotHaveBalanceTryingToDeposit
				);
			// Condition ix
			} else if creation_fee.amount < Pools::required_creation_deposit_for(config.assets.len()).unwrap() {
				assert_noop!(
					<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee), 
					Error::<Test>::CreationFeeIsInsufficient
				);
			} else {
				assert_ok!(<Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee));
			}
		});
	}

	#[test]
	fn creating_a_pool_with_n_underlying_assets_tracks_n_seperate_vaults(
		initial_assets in generate_initial_assets_without_duplicates(),
	) {
		// Tests that when a pool is created to track n different assets, 
		//  |   PoolIdAndAssetIdToVaultId maintains n different
		//  |   key (pool_id, asset_id) -> value (vault_id) entries, one for each
		//  |   asset in the pool.
		//  '-> Conditions:
		//       i. a pool with n different (unique) assets must have n different
		//              (unique) underlying vaults

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfig {
				manager: ALICE,

				assets: initial_assets.clone(),
				asset_bounds: Bound {
					minimum: 0, 
					maximum: 26
				},

				weights: equal_weight_vector_for(&initial_assets),
				weight_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},

				deposit_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},
				withdraw_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},

				transaction_fee: Perquintill::zero(),
			};

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));
		
			let pool_id = <Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee).unwrap();

			// Condition i
			for asset_id in initial_assets {
				assert_eq!(PoolIdAndAssetIdToVaultId::<Test>::contains_key(pool_id, asset_id), true);
			}
		});
	}

	#[test]
	fn creating_a_pool_transfers_creation_fee_into_pool_and_vault_accounts(
		initial_assets in generate_initial_assets_without_duplicates(),
		creation_fee in generate_creation_fee(26usize),
		user_balance in generate_native_balance(26usize),
	) {
		// Tests that when a user successfully creates a Pool their Creation fee is transfered 
		//  |  into the Pools account
		//  |-> Pre-Conditions:
		//  |     i.   user (U) has at least n ≥ CreationFee native tokens in their account
		//  '-> Post-Conditions:
		//        ii.  user (U) has n' = n - △ native tokens in their account, where △ = CreationFee
		//        iii. ∀ vaults v_i ⇒ v_i has △', where △' = △/(PoolSize + 1)
		//        iv.  pool (P) has △' native tokens in its account

		// guarantee the user has enough native assets to create the pool
		let required_creation_fee = Pools::required_creation_deposit_for(initial_assets.len()).unwrap();

		let creation_fee_amount = creation_fee + required_creation_fee;
		let user_balance = user_balance + creation_fee_amount;

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfig {
				manager: ALICE,

				assets: initial_assets.clone(),
				asset_bounds: Bound {
					minimum: 0, 
					maximum: 26
				},

				weights: equal_weight_vector_for(&initial_assets),
				weight_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},

				deposit_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},
				withdraw_bounds: Bound {
					minimum: Perquintill::zero(), 
					maximum: Perquintill::one()
				},

				transaction_fee: Perquintill::zero(),
			};

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: creation_fee_amount, 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, user_balance));
			
			// Pre-Condition i
			assert_eq!(Tokens::balance(MockCurrencyId::A, &ALICE) >= creation_fee_amount, true);

			let pool_id = <Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee).unwrap();

			// Post-Condition ii
			assert_eq!(Tokens::balance(MockCurrencyId::A, &ALICE), user_balance - creation_fee_amount);

			let individual_vault_fee = <Test as pallet_pool::Config>::ExistentialDeposit::get() 
				+ <Test as pallet_pool::Config>::CreationFee::get();
			let total_vault_fee      = individual_vault_fee * initial_assets.len() as u128;

			for asset_id in initial_assets {
				let vault_id = PoolIdAndAssetIdToVaultId::<Test>::get(pool_id, asset_id);
				
				// Post-Condition iii
				assert_eq!(
					Tokens::balance(MockCurrencyId::A, &Vaults::account_id(&vault_id)), 
					individual_vault_fee
				);
			}

			// Post-Condition iv
			assert_eq!(
				Tokens::balance(MockCurrencyId::A, &<Pools as ConstantMeanMarket>::account_id(&1)), 
				creation_fee_amount - total_vault_fee
			);

		});
	}
}

// // ----------------------------------------------------------------------------------------------------
// //                                               Deposit                                              
// // ----------------------------------------------------------------------------------------------------

// #[test]
// fn trying_to_deposit_to_a_pool_that_does_not_exist_raises_an_error() {
// 	// Tests that when trying to deposit assets into a pool using a pool id 
// 	//  |  that doesn't correspond to an active pool, then the deposit 
// 	//  |  extrinsic raises an error
// 	//  '-> Condition
// 	//        i. ∀ deposits d ⇒ pool_id must exist

// 	ExtBuilder::default().build().execute_with(|| {
// 		// No pool has been created
// 		let pool_id = 1;

// 		// Condition i
// 		let deposit = vec![Deposit{asset_id: MockCurrencyId::A, amount: 1_010}];
// 		assert_noop!(
// 			Pools::deposit(Origin::signed(ALICE), pool_id, deposit),
// 			Error::<Test>::PoolDoesNotExist
// 		);
// 	});
// }

// #[test]
// fn trying_to_deposit_an_amount_larger_than_issuer_has_raises_an_error() {
// 	// Tests that the deposit extrinsic checks that the issuer has the balance
// 	//  |  they are trying to deposit
// 	//  '-> Conditions:
// 	//        i. ∀ deposits d of assets a1 ... an ⇒ user U has ≥ asset ai

// 	ExtBuilder::default().build().execute_with(|| {
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C],
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(50),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_perthousand(0),
// 			deposit_max: Perquintill::from_perthousand(1_000),
// 			withdraw_min: Perquintill::from_perthousand(0),
// 			withdraw_max: Perquintill::from_perthousand(1_000),
// 		};
// 		let deposit = Deposit {
// 			asset_id: MockCurrencyId::A,
// 			amount: 2_020,
// 		};
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 2_020));

// 		assert_ok!(Pools::create(Origin::signed(ALICE), config.clone(), deposit));
// 		let pool_id = 1;

// 		// don't mint any tokens of type MockCurrencyId::B or MockCurrencyId::C -
// 		//     does not satisfy condition i

// 		// Condition i
// 		let deposit = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 1_010},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 1_010},
// 		];
// 		assert_noop!(
// 			Pools::deposit(Origin::signed(ALICE), pool_id, deposit),
// 			Error::<Test>::IssuerDoesNotHaveBalanceTryingToDeposit
// 		);
// 	});
// }

// #[test]
// fn trying_to_deposit_an_amount_smaller_than_minimum_deposit_raises_an_error() {
// 	// Tests that, when trying to deposit assets into a pool and the amount
// 	//  |  being deposited is smaller than the pools minimum deposit requirement, 
// 	//  |  the deposit extrinsic raises an error
// 	//  '-> Condition
// 	//        i. ∀ deposits d with assets a1 ... an ⇒ ai >= min_deposit, where 1 ≤ i ≤ n

// 	ExtBuilder::default().build().execute_with(|| {
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C],
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(50),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_percent(10),
// 			deposit_max: Perquintill::from_percent(100),
// 			withdraw_min: Perquintill::from_percent(10),
// 			withdraw_max: Perquintill::from_percent(100),
// 		};
// 		let deposit = Deposit {
// 			asset_id: MockCurrencyId::A,
// 			amount: 2_020,
// 		};
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 2_020));

// 		assert_ok!(Pools::create(Origin::signed(ALICE), config.clone(), deposit));
// 		let pool_id = 1;

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_000));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &ALICE, 1_000));

// 		let deposit = vec![
// 			Deposit{asset_id: MockCurrencyId::B, amount: 1_000},
// 			Deposit{asset_id: MockCurrencyId::C, amount: 1_000},
// 		];

// 		assert_ok!(Pools::deposit(Origin::signed(ALICE), pool_id, deposit));

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_000));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &ALICE, 1_000));

// 		// Condition i
// 		let deposit = vec![
// 			Deposit{asset_id: MockCurrencyId::B, amount: 100},
// 			Deposit{asset_id: MockCurrencyId::C, amount: 100},
// 		];
// 		assert_noop!(
// 			Pools::deposit(Origin::signed(ALICE), pool_id, deposit),
// 			Error::<Test>::AmountMustBeGreaterThanMinimumDeposit
// 		);
// 	});
// }

// #[test]
// fn trying_to_deposit_an_amount_that_doesnt_match_weight_metric_raises_an_error() {
// 	// Test that when depositing funds into a pool that the ratio of each asset deposited corresponds
// 	//  |  to the assets weight in the pool
// 	//  |-> Pre-Conditions:
// 	//  |     i.  Pool P is has weights w1 ... wn for assets a1 ... an
// 	//  '-> Post-Conditions:
// 	//        ii. ∀ deposits d consisting of assets a1 ... an ⇒ (ai / total(a1 ... an)) = wi, where 1 ≤ i ≤ n

// 	ExtBuilder::default().build().execute_with(|| {
// 		// Creating Pool
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C],
// 			// Condition i
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(50),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_perthousand(0),
// 			deposit_max: Perquintill::from_perthousand(1_000),
// 			withdraw_min: Perquintill::from_perthousand(0),
// 			withdraw_max: Perquintill::from_perthousand(1_000),
// 		};
// 		let deposit = Deposit {asset_id: MockCurrencyId::A, amount: 2_020};

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 2_020));
// 		assert_ok!(Pools::create(Origin::signed(ALICE), config, deposit));
// 		let pool_id = 1;

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_000));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &ALICE,   999));

// 		let deposit = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 1_000},
// 			Deposit {asset_id: MockCurrencyId::C, amount:   999},
// 		];

// 		// Condition ii
// 		assert_noop!(
// 			Pools::deposit(Origin::signed(ALICE), pool_id, deposit),
// 			Error::<Test>::DepositDoesNotMatchWeightingMetric
// 		);
// 	});
// }

// #[test]
// fn trying_to_deposit_an_amount_that_doesnt_match_weight_metric_raises_an_error_2() {
// 	// Test that when depositing funds into a pool that the ratio of each asset deposited corresponds
// 	//  |  to the assets weight in the pool
// 	//  |-> Pre-Conditions:
// 	//  |     i.  Pool P is has weights w1 ... wn for assets a1 ... an
// 	//  '-> Post-Conditions:
// 	//        ii. ∀ deposits d consisting of assets a1 ... an ⇒ (ai / total(a1 ... an)) = wi, where 1 ≤ i ≤ n

// 	ExtBuilder::default().build().execute_with(|| {
// 		// Pool Creation
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C, MockCurrencyId::D],
// 			// Condition i
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(25),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::D,
// 					weight: Perquintill::from_percent(25),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_perthousand(0),
// 			deposit_max: Perquintill::from_perthousand(1_000),
// 			withdraw_min: Perquintill::from_perthousand(0),
// 			withdraw_max: Perquintill::from_perthousand(1_000),
			
// 		};
// 		let deposit = Deposit {asset_id: MockCurrencyId::A, amount: 3_030};

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 3_030));
// 		assert_ok!(Pools::create(Origin::signed(ALICE), config, deposit));
// 		let pool_id = 1;

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_000));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &ALICE,   501));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::D, &ALICE,   500));

// 		// Alice Deposits
// 		let deposits = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 1_000},
// 			Deposit {asset_id: MockCurrencyId::C, amount:   501},
// 			Deposit {asset_id: MockCurrencyId::D, amount:   500},
// 		];

// 		// Condition ii
// 		assert_noop!(
// 			Pools::deposit(Origin::signed(ALICE), pool_id, deposits),
// 			Error::<Test>::DepositDoesNotMatchWeightingMetric
// 		);
// 	});
// }
	

// #[test]
// fn deposit_adds_amounts_to_underlying_vaults_and_removes_from_depositer() {
// 	// Tests that after the deposit extrinsic has been called, the assets deposited
// 	//  |  have been transfered out of the issuers account and into the underlying 
// 	//  |  vaults account.
// 	//  |-> Pre-Conditions:
// 	//  |     i.   ∀ deposits d of assets a1 ... an ⇒  user  U has ui asset ai before the deposit
// 	//  |     ii.  ∀ deposits d of assets a1 ... an ⇒ vault Vi has pi asset ai before the deposit
// 	//  '-> Post-Conditions:
// 	//        iii. ∀ deposits d of assets a1 ... an ⇒  user  U has ui - △i of asset ai after the deposit
// 	//        iv.  ∀ deposits d of assets a1 ... an ⇒ vault Vi has pi + △i of asset ai after the deposit
	
// 	ExtBuilder::default().build().execute_with(|| {
// 		// Creating Pool
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C],
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(50),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_perthousand(0),
// 			deposit_max: Perquintill::from_perthousand(1_000),
// 			withdraw_min: Perquintill::from_perthousand(0),
// 			withdraw_max: Perquintill::from_perthousand(1_000),
// 		};
// 		let deposit = Deposit {asset_id: MockCurrencyId::A, amount: 2_020};

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 2_020));
// 		assert_ok!(Pools::create(Origin::signed(ALICE), config, deposit));
		
// 		let pool_id = 1;

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_010));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &ALICE, 1_010));

// 		// Pre-Conditions
// 		for asset_id in &vec![MockCurrencyId::B, MockCurrencyId::C] {
// 			// Condition i
// 			assert_eq!(Tokens::balance(*asset_id, &ALICE), 1_010);

// 			let vault_id = PoolIdAndAssetIdToVaultId::<Test>::get(pool_id, *asset_id);
// 			let vault_account = Vaults::account_id(&vault_id);

// 			// Condition ii
// 			assert_eq!(Tokens::balance(*asset_id, &vault_account), 0);
// 		}
	
// 		let deposit = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 1_010},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 1_010},
// 		];

// 		// Depositing into Pool
// 		assert_ok!(Pools::deposit(Origin::signed(ALICE), pool_id, deposit));

// 		// Post-Conditions
// 		for asset_id in &vec![MockCurrencyId::B, MockCurrencyId::C] {
// 			// Condition iii
// 			assert_eq!(Tokens::balance(*asset_id, &ALICE), 0);

// 			let vault_id = PoolIdAndAssetIdToVaultId::<Test>::get(pool_id, *asset_id);
// 			let vault_account = Vaults::account_id(&vault_id);

// 			// Condition iv
// 			assert_eq!(Tokens::balance(*asset_id, &vault_account), 1_010);
// 		}
// 	});
// }

// #[test]
// fn deposit_adds_gemoetric_mean_of_deposits_as_lp_tokens() {
// 	// Tests that when depositing to a newly created pool, an amount of lp tokens
// 	//  |  equivalent to the geometric mean of the deposit amounts is minted into 
// 	//  |  issuers account
// 	//  '-> Conditions:
// 	//        i. ∀ deposits d of assets a1 ... an (into an empty pool) 
// 	//               ⇒ lp_tokens_minted (lp) = nth-√(Π ai), where 1 ≤ i ≤ n
	
// 	ExtBuilder::default().build().execute_with(|| {
// 		// Pool Creation
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::D],
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::D,
// 					weight: Perquintill::from_percent(50),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_perthousand(0),
// 			deposit_max: Perquintill::from_perthousand(1_000),
// 			withdraw_min: Perquintill::from_perthousand(0),
// 			withdraw_max: Perquintill::from_perthousand(1_000),
// 		};
// 		let deposit = Deposit {asset_id: MockCurrencyId::A, amount: 2_020};

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 2_020));
// 		assert_ok!(Pools::create(Origin::signed(ALICE), config, deposit));
// 		let pool_id = 1;

// 		// Mint tokens into Alices account
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_010));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::D, &ALICE, 1_010));

// 		// Depositing balance into pool's underlying vaults
// 		let deposit = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 1_010},
// 			Deposit {asset_id: MockCurrencyId::D, amount: 1_010},
// 		];
// 		assert_ok!(Pools::deposit(Origin::signed(ALICE), pool_id, deposit));

// 		let lp_token_id = PoolIdToPoolInfo::<Test>::get(&pool_id).lp_token_id;
// 		let geometic_mean = 1_010;

// 		// Condition i
// 		assert_eq!(Tokens::balance(lp_token_id, &ALICE), geometic_mean);
// 	});
// }

// #[test]
// fn deposit_into_nonempty_pool_mints_lp_tokens_proportional_to_ratio_of_assets_deposited() {
// 	// Tests that when a user is depositing assets into a non-empty pool (i.e. a pool
// 	//  |  that has already minted lp tokens), the amount of lp tokens minted is
// 	//  |  equivalent to the ratio of deposited assets to the pools balance of 
// 	//  |  each asset.
// 	//  |-> Pre-Conditions:
// 	//  |     i.  circulating supply of lp tokens (c) > 0
// 	//  '-> Post-Conditions:
// 	//        ii. ∀ deposits d of assets a1 ... an (into a non-empty pool) 
// 	//               ⇒ lp_tokens_minted (lp) = c x Σ (wi x (di/bi)),
// 	//                   where <  1 ≤ i ≤ n
// 	//                          | wi = the weight of asset i for the pool P
// 	//                          | di = the balance deposited of asset i
// 	//                          | bi = the balance of asset i in pool P before the deposit

// 	ExtBuilder::default().build().execute_with(|| {
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C],
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(50),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_perthousand(0),
// 			deposit_max: Perquintill::from_perthousand(1_000),
// 			withdraw_min: Perquintill::from_perthousand(0),
// 			withdraw_max: Perquintill::from_perthousand(1_000),
// 		};
//      let deposit = Deposit {asset_id: MockCurrencyId::A, amount: 2_020};

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 2_020));
// 		assert_ok!(Pools::create(Origin::signed(ALICE), config, deposit));
// 		let pool_id = 1;

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_010));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &ALICE, 1_010));

// 		let deposit = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 1_010},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 1_010},
// 		];
// 		// Alice deposits
// 		assert_ok!(Pools::deposit(Origin::signed(ALICE), pool_id, deposit));

// 		// Condition i
// 		let lp_token_id = PoolIdToPoolInfo::<Test>::get(&pool_id).lp_token_id;
// 		let lp_circulating_supply = Tokens::total_issuance(lp_token_id);
// 		assert!(
// 			lp_circulating_supply > 0
// 		);

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &BOB, 1_010));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &BOB, 1_010));

// 		// depositing half as many assets should print half as many lp tokens
// 		let deposit = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 505},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 505},
// 		];
// 		// Bob deposits
// 		assert_ok!(Pools::deposit(Origin::signed(BOB), pool_id, deposit));

// 		let lp_tokens_minted = 505;

// 		// Condition ii
// 		assert_eq!(Tokens::balance(lp_token_id, &BOB), lp_tokens_minted);
// 	});
// }

// #[test]
// fn depositing_funds_into_underlying_vaults_mints_vaults_lp_tokens_into_the_pools_account_and_not_issuers_account() {
// 	// Test that the lp tokens that are minted by the underlying vaults are kept by
// 	//  |  the Pools account and the issuer of the extrinsic never receives them
// 	//  |-> Pre-Conditions:
// 	//  |     i.   ∀ deposits d ⇒ pool (P) has πi of lp tokens from vault Vi before the deposit
// 	//  |     ii.  ∀ deposits d ⇒ user (U) has 0 lp tokens from vault Vi before the deposit
// 	//  '-> Post-Conditions:
// 	//        iii. ∀ deposits d ⇒ pool (P) has πi + △i of lp tokens from vault Vi after the deposit
// 	//                 where △i corresponds to the number of lp tokens minted by vaut Vi
// 	//        iv.  ∀ deposits d ⇒ user (U) has 0 lp tokens from vault Vi after the deposit

// 	ExtBuilder::default().build().execute_with(|| {
// 		// Pool Creation
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C],
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(50),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_perthousand(0),
// 			deposit_max: Perquintill::from_perthousand(1_000),
// 			withdraw_min: Perquintill::from_perthousand(0),
// 			withdraw_max: Perquintill::from_perthousand(1_000),
// 		};
// 		let deposit = Deposit {asset_id: MockCurrencyId::A, amount: 2_020};

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 2_020));
// 		assert_ok!(Pools::create(Origin::signed(ALICE), config, deposit));
// 		let pool_id = 1;

// 		let pool_account = <Pools as Pool>::account_id(&pool_id);

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_010));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &ALICE, 1_010));

// 		let deposits = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 1_010},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 1_010},
// 		];

// 		// Pre-Conditions
// 		for deposit in &deposits {
// 			let vault_id = PoolIdAndAssetIdToVaultId::<Test>::get(pool_id, deposit.asset_id);
// 			let vault_lp_token_id = Vaults::lp_asset_id(&vault_id).unwrap();

// 			// Condition i
// 			assert_eq!(Tokens::balance(vault_lp_token_id, &pool_account), 0);
// 			// Condition ii
// 			assert_eq!(Tokens::balance(vault_lp_token_id, &ALICE), 0);
// 		}

// 		// Depositing into Pool
// 		assert_ok!(Pools::deposit(Origin::signed(ALICE), pool_id, deposits.clone()));

// 		// Post-Conditions
// 		for deposit in &deposits {
// 			let vault_id = PoolIdAndAssetIdToVaultId::<Test>::get(pool_id, deposit.asset_id);
// 			let vault_lp_token_id = Vaults::lp_asset_id(&vault_id).unwrap();

// 			// The pool holds the vaults LP tokens
// 			assert_eq!(Tokens::balance(vault_lp_token_id, &pool_account), deposit.amount);
// 			// Alice never holds the vaults LP tokens
// 			assert_eq!(Tokens::balance(vault_lp_token_id, &ALICE), 0);
// 		}
// 	});
// }

// #[test]
// fn depositing_funds_into_pool_keeps_track_of_circulating_supply() {
// 	// Test that all lp tokens that all are minted by the pool are kept track of
// 	//  |  by the pool
// 	//  |-> Pre-Conditions:
// 	//  |     i.  ∀ deposits d ⇒ pool (P) keeps track of π lp tokens in circulation before the deposit
// 	//  '-> Post-Conditions:
// 	//        ii. ∀ deposits d ⇒ pool (P) keeps track of π + △ lp tokens in circulation after the deposit,
// 	//                where △ = hte number of lp tokens minted from the deposit

// 	ExtBuilder::default().build().execute_with(|| {
// 		// Pool Creation
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C],
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(50),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_perthousand(0),
// 			deposit_max: Perquintill::from_perthousand(1_000),
// 			withdraw_min: Perquintill::from_perthousand(0),
// 			withdraw_max: Perquintill::from_perthousand(1_000),
// 		};
// 		let deposit = Deposit {asset_id: MockCurrencyId::A, amount: 2_020};

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 2_020));
// 		assert_ok!(Pools::create(Origin::signed(ALICE), config, deposit));
// 		let pool_id = 1;

// 		let lp_token_id = PoolIdToPoolInfo::<Test>::get(pool_id).lp_token_id;

// 		// Condition i
// 		assert_eq!(
// 			Tokens::balance(lp_token_id, &ALICE),
// 			0
// 		);

// 		// Alice Deposits
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_010));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &ALICE, 1_010));

// 		let deposits = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 1_010},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 1_010},
// 		];
// 		assert_ok!(Pools::deposit(Origin::signed(ALICE), pool_id, deposits.clone()));

// 		let lp_token_id = PoolIdToPoolInfo::<Test>::get(&pool_id).lp_token_id;

// 		// Condition ii (for Alices deposti) & Condition i (for Bobs deposit)
// 		let lp_circulating_supply = Tokens::total_issuance(lp_token_id);
// 		assert_eq!(
// 			Tokens::balance(lp_token_id, &ALICE),
// 			lp_circulating_supply
// 		);

// 		// Bob Deposits
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &BOB, 1_010));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &BOB, 1_010));

// 		let deposits = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 505},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 505},
// 		];
// 		assert_ok!(Pools::deposit(Origin::signed(BOB), pool_id, deposits.clone()));

// 		// Condition ii
// 		let lp_circulating_supply = Tokens::total_issuance(lp_token_id);
// 		assert_eq!(
// 			Tokens::balance(lp_token_id, &ALICE) + Tokens::balance(lp_token_id, &BOB),
// 			lp_circulating_supply
// 		);
// 	});
// }

// // ----------------------------------------------------------------------------------------------------
// //                                               Withdraw                                              
// // ----------------------------------------------------------------------------------------------------

// #[test]
// fn trying_to_withdraw_from_a_pool_that_does_not_exist_raises_an_error() {
// 	// Tests that when trying to withdraw assets from a pool using a pool id 
// 	//  |  that doesn't correspond to an active pool, then the withdraw 
// 	//  |  extrinsic raises an error
// 	//  '-> Condition
// 	//        i. ∀ withdraws w ⇒ pool_id must exist

// 	ExtBuilder::default().build().execute_with(|| {
// 		// No pool has been created

// 		let pool_id = 1;

// 		// Condition i
// 		let deposit = vec![Deposit{asset_id: MockCurrencyId::A, amount: 1_010}];
// 		assert_noop!(
// 			Pools::deposit(Origin::signed(ALICE), pool_id, deposit),
// 			Error::<Test>::PoolDoesNotExist
// 		);
// 	});
// }

// #[test]
// fn withdrawing_an_amount_of_lp_tokens_greater_than_owned_raises_error() {
// 	// Test that, when a user tries to withdraw by depositing a number of lp tokens 
// 	//  |  greater than the amount they currently own, an error is thrown
// 	//  '-> Condition
// 	//        i. ∀ users U ⇒ lp_tokens_deposited ≥ lp_tokens_owned

// 	ExtBuilder::default().build().execute_with(|| {
// 		// Pool Creation
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C],
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(50),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_perthousand(0),
// 			deposit_max: Perquintill::from_perthousand(1_000),
// 			withdraw_min: Perquintill::from_perthousand(0),
// 			withdraw_max: Perquintill::from_perthousand(1_000),
// 		};
// 		let deposit = Deposit {asset_id: MockCurrencyId::A, amount: 2_020};

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 2_020));
// 		assert_ok!(Pools::create(Origin::signed(ALICE), config.clone(), deposit));
// 		let pool_id = 1;

// 		// ALICE does not deposit anything, and thus does not have any lp tokens

// 		// Condition i
// 		assert_noop!(
// 			Pools::withdraw(Origin::signed(ALICE), pool_id, 1),
// 			Error::<Test>::IssuerDoesNotHaveLpTokensTryingToDeposit
// 		);
// 	});
// }

// #[test]
// fn trying_to_withdraw_an_amount_outside_withdraw_bounds_raises_an_error() {
// 	// Tests that, when trying to withdraw assets from a pool and the amount of lp tokens
// 	//  |  being deposited is smaller than the pools minimum withdraw requirement, 
// 	//  |  the withdraw extrinsic raises an error
// 	//  '-> Condition
// 	//        i.  ∀ withdraw w of lp tokens lp ⇒ lp share of ai ≥ min_withdraw
// 	//        ii. ∀ withdraw w of lp tokens lp ⇒ lp share of ai ≤ max_withdraw

// 	ExtBuilder::default().build().execute_with(|| {
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C],
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(50),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_percent(0),
// 			deposit_max: Perquintill::from_percent(100),
// 			withdraw_min: Perquintill::from_percent(1),
// 			withdraw_max: Perquintill::from_percent(30),
// 		};
// 		let deposit = Deposit {
// 			asset_id: MockCurrencyId::A,
// 			amount: 2_020,
// 		};
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 2_020));

// 		assert_ok!(Pools::create(Origin::signed(ALICE), config.clone(), deposit));
// 		let pool_id = 1;

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_000));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &ALICE, 1_000));

// 		let deposit = vec![
// 			Deposit{asset_id: MockCurrencyId::B, amount: 1_000},
// 			Deposit{asset_id: MockCurrencyId::C, amount: 1_000},
// 		];
// 		assert_ok!(Pools::deposit(Origin::signed(ALICE), pool_id, deposit));

// 		// Condition i
// 		assert_noop!(
// 			Pools::withdraw(Origin::signed(ALICE), pool_id, 9),
// 			Error::<Test>::AmountMustBeGreaterThanMinimumWithdraw
// 		);

// 		// Condition ii
// 		assert_noop!(
// 			Pools::withdraw(Origin::signed(ALICE), pool_id, 301),
// 			Error::<Test>::AmountMustBeLessThanMaximumWithdraw
// 		);
// 	});
// }

// #[test]
// fn withdraw_transfers_users_share_of_underlying_assets_from_pools_account_into_the_users_account() {
// 	// Test that when a user deposits assets into the pool, receives lp tokens and
// 	//  |  deposits those lp tokens back into the pool the user receives their share 
// 	//  |  of the pools assets and it is transfered out of the pools (and vaults) account
// 	//  '-> Conditions:
// 	//        i.   User no longer has pools lp tokens
// 	//        ii.  User receives their share of assets back
// 	//        iii. Pool account burns the received lp tokens
// 	//        iv.  Pool account no longer has vaults lp tokens
// 	//        v.   Underlying vaults hold a balance of Bt - △t tokens t
// 	//        vi.  Underlying vaults burn their native lp tokens

// 	ExtBuilder::default().build().execute_with(|| {
// 		// Pool is created
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C],
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(50),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_perthousand(0),
// 			deposit_max: Perquintill::from_perthousand(1_000),
// 			withdraw_min: Perquintill::from_perthousand(0),
// 			withdraw_max: Perquintill::from_perthousand(1_000),
// 		};
// 		let deposit = Deposit {asset_id: MockCurrencyId::A, amount: 2_020};

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 2_020));
// 		assert_ok!(Pools::create(Origin::signed(ALICE), config.clone(), deposit));
// 		let pool_id = 1;

// 		// Alice Deposits
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_010));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &ALICE, 1_010));

// 		let alices_deposit = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 1_010},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 1_010},
// 		];
// 		assert_ok!(Pools::deposit(Origin::signed(ALICE), pool_id, alices_deposit.clone()));
		
// 		// Bob deposits
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &BOB, 505));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &BOB, 505));

// 		let bobs_deposit = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 505},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 505},
// 		];
// 		assert_ok!(Pools::deposit(Origin::signed(BOB), pool_id, bobs_deposit.clone()));

// 		// ALICE will withdraw half of her assets
// 		let pool_account = <Pools as Pool>::account_id(&pool_id);
// 		let pool_lp_token = PoolIdToPoolInfo::<Test>::get(&pool_id).lp_token_id;

// 		let pool_lp_tokens_received = Tokens::balance(pool_lp_token, &ALICE);
// 		let withdraw = pool_lp_tokens_received/2;
// 		assert_ok!(Pools::withdraw(Origin::signed(ALICE), pool_id, withdraw));

// 		// Vailidity Checking

// 		// Condition i
// 		assert_eq!(Tokens::balance(pool_lp_token, &ALICE), pool_lp_tokens_received - withdraw);
		
// 		// Condition ii
// 		for deposit in &alices_deposit {
// 			assert_eq!(Tokens::balance(deposit.asset_id, &ALICE), deposit.amount/2);
// 		}

// 		// Condition iii
// 		assert_eq!(Tokens::balance(pool_lp_token, &pool_account), 0);
		
// 		for asset_id in &config.asset_ids {
// 			let vault_id = PoolIdAndAssetIdToVaultId::<Test>::get(pool_id, *asset_id);
// 			let vault_account = Vaults::account_id(&vault_id);

// 			let vault_lp_token_id = Vaults::lp_asset_id(&vault_id).unwrap();

// 			// Condition iv
// 			assert_eq!(
// 				Tokens::balance(vault_lp_token_id, &pool_account), 
// 				Tokens::balance(*asset_id, &vault_account) 
// 			);

// 			// Condition v
// 			assert_eq!(
// 				Tokens::balance(*asset_id, &vault_account), 
// 				1_010, 
// 			);

// 			// Condition vi
// 			assert_eq!(Tokens::balance(vault_lp_token_id, &vault_account), 0);
// 		}
// 	});
// }

// #[test]
// fn withdrawing_funds_from_pool_updates_circulating_supply_of_lp_tokens() {
// 	// Test that the pool's counter of circulating supply of lp tokens is updated
// 	//  |  after lp tokens are burned after a withdraw.
// 	//  |-> Pre-Conditions:
// 	//  |     i.  ∀ withdraws w ⇒ pool (P) keeps track of π lp tokens in circulation before the withdraw
// 	//  '-> Post-Conditions:
// 	//        ii. ∀ withdraws w ⇒ pool (P) keeps track of π - △ lp tokens in circulation after the withdraw,
// 	//                where △ = the number of lp tokens burned from the withdraw

// 	ExtBuilder::default().build().execute_with(|| {
// 		// Pool Creation
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C],
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(50),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_perthousand(0),
// 			deposit_max: Perquintill::from_perthousand(1_000),
// 			withdraw_min: Perquintill::from_perthousand(0),
// 			withdraw_max: Perquintill::from_perthousand(1_000),
// 		};
// 		let deposit = Deposit {asset_id: MockCurrencyId::A, amount: 2_020};

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 2_020));
// 		assert_ok!(Pools::create(Origin::signed(ALICE), config, deposit));
// 		let pool_id = 1;

// 		// Alice Deposits
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_010));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &ALICE, 1_010));

// 		let deposits = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 1_010},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 1_010},
// 		];
// 		assert_ok!(Pools::deposit(Origin::signed(ALICE), pool_id, deposits.clone()));

// 		// Bob Deposits
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &BOB, 1_010));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &BOB, 1_010));

// 		let deposits = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 505},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 505},
// 		];
// 		assert_ok!(Pools::deposit(Origin::signed(BOB), pool_id, deposits.clone()));

// 		let lp_token_id = PoolIdToPoolInfo::<Test>::get(pool_id).lp_token_id;
		
// 		// Condition i
// 		let lp_circulating_supply = Tokens::total_issuance(lp_token_id);
// 		assert_eq!(
// 			Tokens::balance(lp_token_id, &ALICE) + Tokens::balance(lp_token_id, &BOB),
// 			lp_circulating_supply
// 		);

// 		// Alice withdraws half of here assets from the pool
// 		let pool_lp_token = PoolIdToPoolInfo::<Test>::get(&pool_id).lp_token_id;
// 		let withdraw = Tokens::balance(pool_lp_token, &ALICE)/2;
// 		assert_ok!(Pools::withdraw(Origin::signed(ALICE), pool_id, withdraw));

// 		// Condition ii
// 		let lp_circulating_supply = Tokens::total_issuance(lp_token_id);
// 		assert_eq!(
// 			Tokens::balance(lp_token_id, &ALICE) + Tokens::balance(lp_token_id, &BOB),
// 			lp_circulating_supply
// 		);
// 	});
// }

// #[test]
// fn withdrawing_funds_from_a_pool_with_uneven_weights_correctly_calculates_asset_amounts_to_withdraw() {
// 	// Test that when withdrawing funds from a pool that has asset weights that aren't equal to each other
// 	//  |  the withdraw extrinsic calculates the proportionate amount of each asset that corresponds to
// 	//  |  the share of lp tokens deposited
// 	//  |-> Pre-Conditions:
// 	//  |     i.  Pool P is not equal-weighted
// 	//  '-> Post-Conditions:                       circulating_supply_lp - deposited_lp
// 	//        ii. ∀ withdraws w ⇒ wi = bi x (1 - --------------------------------------), where 1 ≤ i ≤ n
// 	//                                                    circulating_supply_lp         

// 	ExtBuilder::default().build().execute_with(|| {
// 		// Pool Creation
// 		let config = PoolConfig {
// 			manager: ALICE,
// 			asset_ids: vec![MockCurrencyId::B, MockCurrencyId::C, MockCurrencyId::D],
// 			weights: vec![
// 				Weight {
// 					asset_id: MockCurrencyId::B,
// 					weight: Perquintill::from_percent(50),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::C,
// 					weight: Perquintill::from_percent(25),
// 				},
// 				Weight {
// 					asset_id: MockCurrencyId::D,
// 					weight: Perquintill::from_percent(25),
// 				},
// 			],
// 			min_underlying_tokens: 0,
// 			max_underlying_tokens: 32,
// 			deposit_min: Perquintill::from_perthousand(0),
// 			deposit_max: Perquintill::from_perthousand(1_000),
// 			withdraw_min: Perquintill::from_perthousand(0),
// 			withdraw_max: Perquintill::from_perthousand(1_000),
// 		};

// 		let deposit = Deposit {asset_id: MockCurrencyId::A, amount: 3_030};

// 		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, 3_030));
// 		assert_ok!(Pools::create(Origin::signed(ALICE), config, deposit));
// 		let pool_id = 1;

// 		// Condition i
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::B, &ALICE, 1_000));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::C, &ALICE, 500));
// 		assert_ok!(Tokens::mint_into(MockCurrencyId::D, &ALICE, 500));

// 		// Alice Deposits
// 		let deposits = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 1_000},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 500},
// 			Deposit {asset_id: MockCurrencyId::D, amount: 500},
// 		];
// 		assert_ok!(Pools::deposit(Origin::signed(ALICE), pool_id, deposits.clone()));

// 		// Alice Withdraws
// 		let pool_lp_token = PoolIdToPoolInfo::<Test>::get(&pool_id).lp_token_id;
// 		let withdraw = Tokens::balance(pool_lp_token, &ALICE) /10;
// 		assert_ok!(Pools::withdraw(Origin::signed(ALICE), pool_id, withdraw));

// 		let required_withdrawn = vec![
// 			Deposit {asset_id: MockCurrencyId::B, amount: 100},
// 			Deposit {asset_id: MockCurrencyId::C, amount: 50},
// 			Deposit {asset_id: MockCurrencyId::D, amount: 50},
// 		];

// 		// Condition ii
// 		for asset in &required_withdrawn {
// 			let epsilon = asset.amount / 100;

// 			assert!(
// 				(asset.amount - epsilon) <= Tokens::balance(asset.asset_id, &ALICE),
// 			);

// 			assert!(
// 				Tokens::balance(asset.asset_id, &ALICE) <= (asset.amount - epsilon),
// 			);
// 		}
// 	});
// }

// ----------------------------------------------------------------------------------------------------
//                                                 Math                                                
// ----------------------------------------------------------------------------------------------------

// fn power(x: LiftedFixedBalance, n: Perquintill) -> LiftedFixedBalance {
// 	x * LiftedFixedBalance::from_rational(n.deconstruct(), Perquintill::one().deconstruct())
	
// 	// LiftedFixedBalance::zero()
// }

// #[test]
// fn calculate_number_squared() {
	
// 	ExtBuilder::default().build().execute_with(|| {
		

// 		assert_eq!(
// 			power(LiftedFixedBalance::saturating_from_integer(0u128), Perquintill::zero()),
// 			LiftedFixedBalance::saturating_from_integer(0u128)
// 		);
// 	});
// }

// type ConstType = I110F18;

// fn print_power(x: FixedU128::<U18>, n: FixedU128::<U18>) {
// 	println!("{:?} ^ {:?} = {:?}", x, n, x^n);
// }

// pub fn log2<S, D>(operand: S) -> Result<D, ()>
// where
//     S: FixedUnsigned + PartialOrd<ConstType>,
//     D: FixedUnsigned + PartialOrd<ConstType> + From<S>,
//     D::Bits: Copy + ToFixed + AddAssign + BitOrAssign + ShlAssign,
// {
//     if operand <= S::from_num(0) {
//         return Err(());
//     };

//     let operand = D::from(operand);
//     if operand < D::from_num(1) {
//         let inverse = D::from_num(1).checked_div(operand).unwrap();
//         return Ok(-log2_inner::<D, D>(inverse));
//     };
//     return Ok(log2_inner::<D, D>(operand));
// }


// fn ln(x: FixedU128) -> FixedU128 {

// }

fn invariant(reserves: Vec<Balance>, weights: Vec<f64>) -> f64 {
	let mut result = 1.0;

	for (reserve, weight) in reserves.iter().zip(weights.iter()) {
		result *= power(*reserve, *weight);
	}

	result
}

fn power(x: Balance, n: f64) -> f64 {
	let x = x as f64;

	x.powf(n)
}

// fn fixed_power(x: LiftedFixedBalance, n: ZeroToOneFixedU128) -> LiftedFixedBalance {
// 	x.saturating_root(n)
// }

// #[test]
// fn calculate_fixed_Balance_square_root() {
// 	let one_hundred = LiftedFixedBalance::from(100);
// 	let point_five = ZeroToOneFixedU128::from_float(0.5);

// 	let ten = LiftedFixedBalance::from(10);

// 	ExtBuilder::default().build().execute_with(|| {
// 		assert_eq!(
// 			fixed_power(one_hundred, point_five),
// 			ten
// 		);
// 	});
// }

#[test]
fn calculate_number_squared() {
	let one_hundred = 100;
	let ten = 10;
	let three = 3;
	let two = 2;
	let one = 1;
	let point_five = 0.5;
	let zero = 0;

	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			power(one_hundred, 0.5),
			ten as f64
		);

		assert_eq!(
			power(one_hundred, 0.3) * power(one_hundred, 0.7),
			ten as f64
		);

		assert_eq!(
			power(Balance::MAX, 0.5) * power(Balance::MAX, 0.5),
			Balance::MAX as f64
		);

		assert_eq!(
			power(Balance::MIN, 0.5) * power(Balance::MIN, 0.5),
			Balance::MIN as f64
		);
	});
}

#[test]
fn calculate_invariant() {
	let reserves: Vec<Balance> = vec![750, 200, 50];
	let weights: Vec<f64> = vec![0.75, 0.2, 0.05];

	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			invariant(reserves, weights),
			502.863885685
		);
	});
}

#[test]
fn fixed_u128() {
	let frac = Frac::U32;

	let a: u128 = 10;
	let b: u128 = 2;

	// let a: f64 = 100.0;
	// let b: f64 = 0.5;
	for &(a, b) in &[(a, b), (b, a)] {
		let af: FixedI128::<Frac> = FixedI128::<Frac>::from_num(a);
		let bf: FixedI128::<Frac> = FixedI128::<Frac>::from_num(b);
		
		println!("{:?}", af);
		println!("{:?}", bf);
		println!("{:?}", af ^ bf);
		// println!("{:?}", pow::<FixedI128::<Frac>, FixedI128::<Frac>>(af, bf).unwrap());

		println!("{:?}", bf ^ af);
	}
}

/// zero
pub const ZERO: I9F23 = I9F23::from_bits(0i32 << 23);
/// one
pub const ONE: I9F23 = I9F23::from_bits(1i32 << 23);
/// two
pub const TWO: I9F23 = I9F23::from_bits(2i32 << 23);
/// three
pub const THREE: I9F23 = I9F23::from_bits(3i32 << 23);

#[test]
fn pow_works() {
	type S = I9F23;
	type D = I32F32;

	let result: D = pow(ZERO, TWO).unwrap();
	let result: f64 = result.lossy_into();
	assert_eq!(result, 0.0);

	let result: D = pow(ONE, TWO).unwrap();
	let result: f64 = result.lossy_into();
	assert_eq!(result, 1.0);

	let result: D = pow(TWO, TWO).unwrap();
	let result: f64 = result.lossy_into();
	assert_relative_eq!(result, 4.0, epsilon = 1.0e-3);
	let result: D = pow(TWO, THREE).unwrap();
	let result: f64 = result.lossy_into();
	assert_relative_eq!(result, 8.0, epsilon = 1.0e-3);
	let result: D = pow(S::from_num(2.9), S::from_num(3.1)).unwrap();
	let result: f64 = result.lossy_into();
	assert_relative_eq!(result, 27.129, epsilon = 1.0e-2);
	let result: D = pow(S::from_num(0.0001), S::from_num(2)).unwrap();
	let result: f64 = result.lossy_into();
	assert_relative_eq!(result, 0.00000001, epsilon = 1.0e-9);

	// this would lead a complex result due to computation method
	assert!(pow::<S, D>(S::from_num(-0.0001), S::from_num(2)).is_err());
}



// #[test]
// fn calculate_number_squared() {
// 	type S = I110F18;
// 	type D = I110F18;

// 	let ten: S = S::from_num(10u128);
// 	let three: S = S::from_num(3u128);
// 	let two: S = S::from_num(2u128);
// 	let one: S = S::from_num(1u128);

// 	ExtBuilder::default().build().execute_with(|| {
// 		let result: D = power(ten, one);
// 		let correct_answer: D = I110F18::from_num(100u128);

// 		assert_eq!(
// 			result,
// 			correct_answer
// 		);
// 	});
// }

// fn calculate_invariant(deposits: &Vec<Balance>, weights: Vec<Perquintill>) -> f64 {
// 	let mut result: f64 = 1.0;

// 	let one = Perquintill::one();
// 	println!("one:             {:?}", one.deconstruct());

// 	for index in 0..weights.len() {
// 		let deposit: Balance 	= deposits[index];
// 		let weight: Perquintill = weights[index];

// 		println!("weight:          {:?}", weight.deconstruct());
// 		let weight =  weight.deconstruct() as f64 / one.deconstruct() as f64;
// 		println!("weight/one:      {:?}", weight);
// 		let weight = weight * 100.0;
// 		println!("weight * 100.0:  {:?}", weight);
// 		let weight = weight as u32;
// 		println!("weight as u32:   {:?}", weight);
// 		let weight = 100 / weight;
// 		println!("100/weight:      {:?}", weight);

// 		println!("deposit**(1/weight): {:?}\n", deposit.nth_root(weight));

// 		result *= deposit.nth_root(weight) as f64;
// 		println!("result:          {:?}", result);
// 	}

// 	result
// }

// #[test]
// fn test_calculating_invariant() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		let deposits = vec![
// 			100,
// 			100,
// 		];

// 		let weights = vec![
// 			Perquintill::from_percent(80),
// 			Perquintill::from_perthousand(200)
// 		];

// 		assert_eq!(
// 			calculate_invariant(&deposits, weights),
// 			geometric_mean(deposits) as f64
// 		);
// 	});
// }

// fn geometric_mean(deposits: Vec<Balance>) -> Balance {
// 	let mut result = Balance::one();

// 	for deposit in &deposits {
// 		result = result.checked_mul(*deposit).unwrap();
// 	}

// 	let number_of_assets = deposits.len() as u32;
	
// 	result.nth_root(number_of_assets)
// }

// #[test]
// fn test_calculating_geometric_mean() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		let deposit1: Balance = 1_010;
// 		let deposit2: Balance = 1_010;
// 		let deposit3: Balance =   505;

// 		let deposits = vec![deposit1, deposit2];
// 		assert_eq!(1_010, geometric_mean(deposits));

// 		let deposits = vec![deposit1, deposit2, deposit3];
// 		assert_eq!(801, geometric_mean(deposits));

// 		assert_eq!(2_446, geometric_mean(vec![7, 9_073, 647, 30_579, 69_701]));
// 	});
// }

// #[test]
// fn test_minimum_maximum_deposit_bounds_as_a_percentage() {
// 	let percent = 10;
// 	let minimum_deposit = Perquintill::from_percent(percent);
// 	let maximum_deposit = Perquintill::from_percent(100-percent);
// 	let pool_balance: Vec<Balance> = 
// 		vec![
// 			1_000, 
// 			2_000,
// 			3_000
// 		];

// 	let deposits: Vec<Balance> = 
// 		vec![
// 			100, 
// 			200,
// 			300
// 		]; 

// 	for index in 0..deposits.len() {
// 		assert!(deposits[index] >= minimum_deposit * pool_balance[index]);
// 		assert!(deposits[index] <= maximum_deposit * pool_balance[index]);
// 	}
// }

// #[test]
// fn test_calculating_percentage_of_balance() {
// 	let percentage = Perquintill::from_percent(10);

// 	let balance: Balance = 1000;

// 	assert_eq!(percentage * balance, 100);
// }

// #[test]
// fn test_summing_vector_of_perquintill_values() {
// 	let epsilon = Perquintill::from_float(0.0000000000000001 as f64).deconstruct();
// 	let one = Perquintill::one().deconstruct();

// 	for size in 13..14 {
// 		let percentages = vec![Perquintill::from_float(1 as f64 / size as f64); size as usize];
// 		let sum = percentages
// 			.iter()
// 			.map(|weight| weight.deconstruct())
// 			.sum();

// 		assert!(one - epsilon < sum && sum < one + epsilon);
// 	};
// }
