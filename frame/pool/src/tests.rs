use crate as pallet_pool;

use crate::{
	pallet::{PoolCount, Pools as PoolStorage, PoolAssets, PoolAssetBalance, 
		PoolAssetTotalBalance, PoolAssetWeight, PoolAssetVault, 
	},
	mocks::{
		currency_factory::{MockCurrencyId, strategy_pick_random_mock_currency},
		tests::{
			ALICE, AccountId, Balance, BOB, ExtBuilder, MAXIMUM_DEPOSIT,
			MINIMUM_DEPOSIT, Pools, Test, Tokens, Vaults,
		},
	},
};
use composable_traits::{
	pool::{
		Bound, ConstantMeanMarket, Deposit, PoolConfig, Weight,
	},
	vault::Vault,
};

use crate::{Error};

use frame_support::{
	assert_noop, assert_ok,
	traits::{
		fungibles::{Inspect, Mutate},
	},
	sp_runtime::Perquintill,
};

use fixed::{
	traits::LossyInto,
	types::U110F18
};
use hydra_dx_math::transcendental::pow;

use std::collections::BTreeSet;

use proptest::prelude::*;
use approx::assert_relative_eq;

const MAX_POOL_SIZE: u8 = 26;

// ----------------------------------------------------------------------------------------------------
//                                             Prop Compose                                            
// ----------------------------------------------------------------------------------------------------

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

// TEMP: (Nevin)
//  - this prop_compose gets around the issue of creating native-asset vaults without using the 
//		  vault create extrinsic. in the extrinsic the creation deposit is moved to two different
//		  accounts (deletion reward and rent accounts). without calling the extrinsic the creation
//		  deposit remains in the vaults account. if the vault is made to hold the native asset
//		  the held creation deposit is assumed to be apart of the vaults usable reserves - this
//		  funciton removes the possibility of the native asset being included in the pools 
//		  underlying assets
prop_compose! {
	fn generate_initial_assets_without_duplicates_or_native_asset() 
		(
			initial_assets in prop::collection::vec(strategy_pick_random_mock_currency(), 2usize..26usize),
		) -> Vec<MockCurrencyId>{
			BTreeSet::<MockCurrencyId>::from_iter(
				initial_assets
				.iter()
				.copied()
				.map(|asset| match asset {
					MockCurrencyId::A => MockCurrencyId::B,
					_ => asset
				})
			).iter()
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

prop_compose! {
	fn generate_equal_weighted_pool_config()
		(
			initial_assets in generate_initial_assets_without_duplicates()
		) -> PoolConfig<AccountId, MockCurrencyId> {
			PoolConfig {
				owner: ALICE,
				fee: Perquintill::zero(),

				assets: initial_assets.clone(),
				asset_bounds: Bound {
					minimum: 0, 
					maximum: MAX_POOL_SIZE,
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
			}
		}
}

prop_compose! {
	fn generate_n_initial_deposits(n: usize)
		(
			initial_deposits in prop::collection::vec(MINIMUM_DEPOSIT..MAXIMUM_DEPOSIT, n * 26usize),
		) -> Vec<Balance>{
			initial_deposits
		}
}

prop_compose! {
	fn generate_equal_weighted_pool_config_and_n_all_asset_deposits(n: usize) 
		(
			pool_config in generate_equal_weighted_pool_config(),
			mut initial_deposits in generate_n_initial_deposits(n),
		) -> (PoolConfig<AccountId, MockCurrencyId>, Vec<Vec<Deposit<MockCurrencyId, Balance>>>){
			let mut all_deposits = Vec::new();
			
			for _ in 0..n {
				let mut deposit = Vec::new();

				for asset in &pool_config.assets {
					deposit.push(
						Deposit {
							asset_id: *asset,
							amount: initial_deposits.pop().unwrap()
						}
					);
				}

				all_deposits.push(deposit);
			}

			(pool_config, all_deposits)
		}
}

// ----------------------------------------------------------------------------------------------------
//                                           Helper Functions                                          
// ----------------------------------------------------------------------------------------------------

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

fn create_pool_with(config: &PoolConfig<AccountId, MockCurrencyId>) -> u64 {
	let creation_fee = Deposit {
		asset_id: MockCurrencyId::A,
		amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
	};
	assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));

	<Pools as ConstantMeanMarket>::create(ALICE, config.clone(), creation_fee).unwrap()
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
				owner: ALICE,
				fee: Perquintill::zero(),

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
				owner: ALICE,
				fee: Perquintill::zero(),

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
				owner: ALICE,
				fee: Perquintill::zero(),

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
				owner: ALICE,
				fee: Perquintill::zero(),

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
	fn creating_a_pool_with_updates_all_runtime_storage_objects(
		initial_assets in generate_initial_assets_without_duplicates(),
	) {
		// Tests that when a pool is created all related runtime storage objects 
		//  |   are all updated with the Pools values
		//  '-> Conditions:
		//       i.   PoolCount is incremented
		//		 ii.  Pools stores the created pools PoolInfoObject
		//		 iii. PoolAssets stores a vector of the pools underlying assets
		//		 iv.  PoolAssetWeight stores the weight that each underlying asset holds
		//				  in the pool
		//		 v.   PoolAssetVault stores a reference to each assets corresponding vault
		//       vi.  PoolAssetBalance is empty (doesn't include fee)
		//       vii. PoolAssetTotalBalance is empty (includes fee)

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfig {
				owner: ALICE,
				fee: Perquintill::zero(),

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
			};

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));
		
			let pool_id = <Pools as ConstantMeanMarket>::create(ALICE, config.clone(), creation_fee).unwrap();

			// Condition i
			assert_eq!(pool_id, PoolCount::<Test>::get());

			// Condition ii
			assert!(PoolStorage::<Test>::contains_key(pool_id));

			let pool_info = PoolStorage::<Test>::get(pool_id);
			assert_eq!(pool_info.owner, config.owner);
			assert_eq!(pool_info.fee, config.fee);
			assert_eq!(pool_info.weight_bounds, config.weight_bounds);
			assert_eq!(pool_info.deposit_bounds, config.deposit_bounds);
			assert_eq!(pool_info.withdraw_bounds, config.withdraw_bounds);

			// Condition iii
			assert!(PoolAssets::<Test>::contains_key(pool_id));
			assert_eq!(PoolAssets::<Test>::get(pool_id).unwrap(), config.assets);

			// Condition iv
			for weight in config.weights {
				assert!(PoolAssetWeight::<Test>::contains_key(pool_id, weight.asset_id));
				
				assert_eq!(PoolAssetWeight::<Test>::get(pool_id, weight.asset_id), weight.weight);
			}

			for asset_id in initial_assets {
				// Condition v
				assert!(PoolAssetVault::<Test>::contains_key(pool_id, asset_id));

				let vault = PoolAssetVault::<Test>::get(pool_id, asset_id);
				assert_eq!(Vaults::asset_id(&vault).unwrap(), asset_id);
			
				// Condition vi
				assert_eq!(PoolAssetBalance::<Test>::get(&pool_id, asset_id), 0);
				// Condition vii
				assert_eq!(PoolAssetTotalBalance::<Test>::get(&pool_id, asset_id), 0);
				
			}
		});
	}

	#[test]
	fn creating_a_pool_with_n_underlying_assets_tracks_n_seperate_vaults(
		initial_assets in generate_initial_assets_without_duplicates(),
	) {
		// Tests that when a pool is created to track n different assets, 
		//  |   PoolAssetVault maintains n different
		//  |   key (pool_id, asset_id) -> value (vault_id) entries, one for each
		//  |   asset in the pool.
		//  '-> Conditions:
		//       i. a pool with n different (unique) assets must have n different
		//              (unique) underlying vaults

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfig {
				owner: ALICE,
				fee: Perquintill::zero(),

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
			};

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));
		
			let pool_id = <Pools as ConstantMeanMarket>::create(ALICE, config, creation_fee).unwrap();

			// Condition i
			for asset_id in initial_assets {
				assert_eq!(PoolAssetVault::<Test>::contains_key(pool_id, asset_id), true);
			}
		});
	}

	#[test]
	fn creating_a_pool_transfers_creation_fee_into_pools_account(
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
		//        iii. pool (P) has △ native tokens in its account

		// guarantee the user has enough native assets to create the pool
		let required_creation_fee = Pools::required_creation_deposit_for(initial_assets.len()).unwrap();

		let creation_fee_amount = creation_fee + required_creation_fee;
		let user_balance = user_balance + creation_fee_amount;

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfig {
				owner: ALICE,
				fee: Perquintill::zero(),

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

			// Post-Condition iii
			assert_eq!(
				Tokens::balance(MockCurrencyId::A, &<Pools as ConstantMeanMarket>::account_id(&pool_id)), 
				creation_fee_amount
			);
		});
	}
}

// ----------------------------------------------------------------------------------------------------
//                                               Deposit                                              
// ----------------------------------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]

	#[test]
	fn depositing_into_an_empty_pool_correctly_mints_lp_tokens(
		(config, all_deposits) in generate_equal_weighted_pool_config_and_n_all_asset_deposits(1),
	) {
		// Tests that if a user is the first to deposit into a newly created pool they are 
		//  |  rewarded a number of LP tokens equal to the weighted geometric mean of the
		//  |  users deposits and Pool's weights
		//  |-> Pre-Conditions:
		//  |     i.   Pool P is empty
		//  '-> Post-Conditions:
		//        ii.  ∀ initial deposits D into P : User U receives LP tokens, where LP = Π D_i ^ (w_i)
		//                 D_i refers to the deposited amount of asset i and w_i refers the Pool's
		//                 weight of asset i

		// Only tests the state after one deposit
		let deposits = (&all_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			// Condition i - Pool is newly created
			let pool_id = create_pool_with(&config);
			
			// Make the first deposit
			for deposit in &deposits {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &ALICE, deposit.amount));
			}
			assert_ok!(<Pools as ConstantMeanMarket>::deposit(&ALICE, &pool_id, deposits.clone()));

			// Condition ii
			let weighted_geometric_mean: FixedBalance = invariant(deposits, config.weights).unwrap();
			let weighted_geometric_mean: f64 = weighted_geometric_mean.to_num::<f64>();

			let lp_token = <Pools as ConstantMeanMarket>::lp_token_id(&pool_id).unwrap();
			let lp_tokens_minted: f64 = Tokens::balance(lp_token, &ALICE) as f64;

			assert_relative_eq!(lp_tokens_minted, weighted_geometric_mean, epsilon = 1.0);
		});
	}

	#[test]
	fn depositing_into_an_empty_pool_correctly_updates_runtime_storage_objects(
		(config, all_deposits) in generate_equal_weighted_pool_config_and_n_all_asset_deposits(2),
	) {
		// Tests that if a user is the first to deposit into a newly created pool they are 
		//  |  rewarded a number of LP tokens equal to the weighted geometric mean of the
		//  |  users deposits and Pool's weights
		//  |-> Pre-Conditions:
		//  |     i.   PoolAssetBalance is empty (doesn't include fee)
		//  |     ii.  PoolAssetTotalBalance is empty (includes fee)
		//  '-> Post-Conditions:
		//        iii. PoolAssetBalance adds the deposited amount (fees aren't taken out of all-asset deposits)
		//		  iv.  PoolAssetTotalBalance adds the deposited amount

		// Only tests the state after one deposit
		let deposit_1 = (&all_deposits[0]).to_vec();
		let deposit_2 = (&all_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			
			for asset_id in &config.assets {
				// Condition i
				assert_eq!(PoolAssetBalance::<Test>::get(&pool_id, asset_id), 0);
				// Condition ii
				assert_eq!(PoolAssetTotalBalance::<Test>::get(&pool_id, asset_id), 0);
			}

			// Make the first deposit
			for deposit in &deposit_1 {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &ALICE, deposit.amount));
			}
			assert_ok!(<Pools as ConstantMeanMarket>::deposit(&ALICE, &pool_id, deposit_1.clone()));

			for deposit in &deposit_1 {
				// Condition iii
				assert_eq!(PoolAssetBalance::<Test>::get(&pool_id, deposit.asset_id), deposit.amount);
				// Condition iv
				assert_eq!(PoolAssetTotalBalance::<Test>::get(&pool_id, deposit.asset_id), deposit.amount);
			}

			// Make the second deposit
			for deposit in &deposit_2 {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &BOB, deposit.amount));
			}
			assert_ok!(<Pools as ConstantMeanMarket>::deposit(&BOB, &pool_id, deposit_2.clone()));

			for (deposit_1, deposit_2) in deposit_1.iter().zip(deposit_2.iter()) {
				let deposit_amount = deposit_1.amount + deposit_2.amount;

				// Condition iii
				assert_eq!(PoolAssetBalance::<Test>::get(&pool_id, deposit_1.asset_id), deposit_amount);
				// Condition iv
				assert_eq!(PoolAssetTotalBalance::<Test>::get(&pool_id, deposit_1.asset_id), deposit_amount);
			}
		});
	}

	#[test]
	fn depositing_into_a_non_empty_pool_with_duplicate_deposits_correctly_mints_lp_tokens(
		(config, all_deposits) in generate_equal_weighted_pool_config_and_n_all_asset_deposits(1),
	) {
		// Tests that if a user is the first to deposit into a newly created pool they are 
		//  |  rewarded a number of LP tokens equal to the weighted geometric mean of the
		//  |  users deposits and Pool's weights
		//  |-> Pre-Conditions:
		//  |     i.   Pool P is nonempty
		//  '-> Post-Conditions:
		//        ii.  ∀ secondary deposits D into P : User U receives LP_minted tokens, where 
		//				   LP_minted = LP_supply * (w_i * (D_i / B_i)) and D_i and B_i refer to the
		//                 deposited amount and pool's reserve  of asset i, respectively, and w_i 
		//				   refers the Pool's weight of asset i

		// use the same deposit struct for both deposits
		let deposits = (&all_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			let lp_token = <Pools as ConstantMeanMarket>::lp_token_id(&pool_id).unwrap();
			
			// Make the first deposit
			for deposit in &deposits {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &ALICE, deposit.amount));
			}
			assert_ok!(<Pools as ConstantMeanMarket>::deposit(&ALICE, &pool_id, deposits.clone()));

			// Condition i
			let alice_lp_tokens: Balance = Tokens::balance(lp_token, &ALICE);

			// Make the second deposit
			for deposit in &deposits {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &BOB, deposit.amount));
			}
			assert_ok!(<Pools as ConstantMeanMarket>::deposit(&BOB, &pool_id, deposits.clone()));

			// Condition ii
			let bob_lp_tokens: Balance = Tokens::balance(lp_token, &BOB);
			assert_relative_eq!(bob_lp_tokens as f64, alice_lp_tokens as f64, epsilon = 1.0);

			let lp_circulating_supply: Balance = <Pools as ConstantMeanMarket>::lp_circulating_supply(&pool_id).unwrap();
			assert_eq!(lp_circulating_supply, alice_lp_tokens + bob_lp_tokens);
		});
	}

	#[test]
	fn depositing_into_a_non_empty_pool_correctly_mints_lp_tokens(
		(config, all_deposits) in generate_equal_weighted_pool_config_and_n_all_asset_deposits(2),
	) {
		// Tests that if a user is the first to deposit into a newly created pool they are 
		//  |  rewarded a number of LP tokens equal to the weighted geometric mean of the
		//  |  users deposits and Pool's weights
		//  |-> Pre-Conditions:
		//  |     i.   Pool P is nonempty
		//  '-> Post-Conditions:
		//        ii.  ∀ secondary deposits D into P : User U receives LP_minted tokens, where 
		//				   LP_minted = LP_supply * (w_i * (D_i / B_i)) and D_i and B_i refer to the
		//                 deposited amount and pool's reserve  of asset i, respectively, and w_i 
		//				   refers the Pool's weight of asset i

		let deposit_1 = (&all_deposits[0]).to_vec();
		let deposit_2 = (&all_deposits[1]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			let lp_token = <Pools as ConstantMeanMarket>::lp_token_id(&pool_id).unwrap();
			
			// Make the first deposit
			for deposit in &deposit_1 {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &ALICE, deposit.amount));
			}
			assert_ok!(<Pools as ConstantMeanMarket>::deposit(&ALICE, &pool_id, deposit_1.clone()));

			// Condition i
			let alice_lp_tokens: Balance = Tokens::balance(lp_token, &ALICE);

			// Make the second deposit
			for deposit in &deposit_2 {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &BOB, deposit.amount));
			}
			assert_ok!(<Pools as ConstantMeanMarket>::deposit(&BOB, &pool_id, deposit_2.clone()));

			// Condition ii
			let bob_lp_tokens: Balance = Tokens::balance(lp_token, &BOB);

			let lp_circulating_supply: Balance = <Pools as ConstantMeanMarket>::lp_circulating_supply(&pool_id).unwrap();
			assert_eq!(lp_circulating_supply, alice_lp_tokens + bob_lp_tokens);
		});
	}
}

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

// 			let vault_id = PoolAssetVault::<Test>::get(pool_id, *asset_id);
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

// 			let vault_id = PoolAssetVault::<Test>::get(pool_id, *asset_id);
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
// 			let vault_id = PoolAssetVault::<Test>::get(pool_id, deposit.asset_id);
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
// 			let vault_id = PoolAssetVault::<Test>::get(pool_id, deposit.asset_id);
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
// 			let vault_id = PoolAssetVault::<Test>::get(pool_id, *asset_id);
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

pub type FixedBalance = U110F18;

#[test]
fn fixed_u128() {
	type S = FixedBalance;
	type D = FixedBalance;

	let zero = S::from_num(0u8);
	let one_half = S::from_num(0.5f64);
	let one = S::from_num(1u8);
	let two = S::from_num(2u8);
	let three = S::from_num(3u8);
	let four = S::from_num(4u8);
	let ten = S::from_num(4u8);
	let one_hundred = S::from_num(4u8);

	assert_eq!(pow::<S, D>(two, zero), Ok(one.into()));
	assert_eq!(pow::<S, D>(zero, two), Ok(zero.into()));
	assert_eq!(pow::<S, D>(ten, two), Ok(one_hundred.into()));
	assert_eq!(pow::<S, D>(one_hundred, one_half), Ok(ten.into()));

	let result: f64 = pow::<S, D>(two, three).unwrap().lossy_into();
	assert_relative_eq!(result, 8.0f64, epsilon = 1.0e-6);

	let result: f64 = pow::<S, D>(one / four, two).unwrap().lossy_into();
	assert_relative_eq!(result, 0.0625f64, epsilon = 1.0e-6);

	assert_eq!(pow::<S, D>(two, one), Ok(two.into()));

	let result: f64 = pow::<S, D>(one / four, one / two).unwrap().lossy_into();
	assert_relative_eq!(result, 0.5f64, epsilon = 1.0e-6);

	assert_eq!(
		pow(S::from_num(22.1234f64), S::from_num(2.1f64)),
		Ok(D::from_num(667.097035126091f64))
	);

	assert_eq!(
		pow(S::from_num(0.986069911074f64), S::from_num(1.541748732743f64)),
		Ok(D::from_num(0.978604513883f64))
	);
}

fn invariant(
	reserves: Vec<Deposit<MockCurrencyId, Balance>>, 
	weights: Vec<Weight<MockCurrencyId>>
) -> Result<FixedBalance, ()> {
	let mut invariant_constant: FixedBalance = FixedBalance::from_num(1u8);

	for (reserve, weight) in reserves.iter().zip(weights.iter()) {

		let reserve: FixedBalance = FixedBalance::from_num(reserve.amount);
		let weight: FixedBalance = FixedBalance::from_num(
			weight.weight.deconstruct() as f64 / Perquintill::one().deconstruct() as f64
		);

		let result = pow(
			reserve,
			weight
		).unwrap();
		
		invariant_constant = invariant_constant.checked_mul(
			result
		).unwrap();
	}

	Ok(invariant_constant)
}

#[test]
fn test_invariant() {
	let reserves = vec![
		Deposit {
			asset_id: MockCurrencyId::A,
			amount: 100
		},
		Deposit {
			asset_id: MockCurrencyId::B,
			amount: 100
		},
	];

	let weights = vec![
		Weight {
			asset_id: MockCurrencyId::A,
			weight: Perquintill::from_percent(50)
		},
		Weight {
			asset_id: MockCurrencyId::B,
			weight: Perquintill::from_percent(50)
		},
	];

	let result: f64 = invariant(reserves, weights).unwrap().lossy_into();
	assert_relative_eq!(result, 100.0f64, epsilon = 1.0e-2);
}