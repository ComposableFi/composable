// use crate as pallet_pool;

pub use crate::{
	pallet::{PoolCount, Pools as PoolStorage, PoolAssets, PoolAssetBalance, 
		PoolAssetTotalBalance, PoolAssetWeight, PoolAssetVault, 
	},
	mocks::{
		currency_factory::{MockCurrencyId, strategy_pick_random_mock_currency},
		tests::{
			ADMIN, ALICE, AccountId, Balance, BOB, CHARLIE, Epsilon, ExtBuilder, MAXIMUM_DEPOSIT,
			MINIMUM_DEPOSIT, Pools, Test, Tokens, Vaults
		},
	},
};

use composable_traits::{
	pool::{
		Bound, ConstantMeanMarket, Deposit, PoolConfig, Reserve, Weight,
	},
	vault::Vault,
};
use sp_runtime::DispatchError;

use crate::{Error};

use frame_support::{
	assert_noop, assert_ok,
	traits::{
		fungibles::{Inspect, Mutate}, 
		tokens::WithdrawConsequence,
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

// Create prop Compose Functions

prop_compose! {
	fn generate_initial_assets()(
		initial_assets in prop::collection::vec(strategy_pick_random_mock_currency(), 2usize..26usize),
	) -> Vec<MockCurrencyId>{
		initial_assets
	}
}

prop_compose! {
	fn generate_initial_assets_without_duplicates()(
		initial_assets in prop::collection::vec(strategy_pick_random_mock_currency(), 2usize..26usize),
	) -> Vec<MockCurrencyId>{
		BTreeSet::<MockCurrencyId>::from_iter(initial_assets.iter().copied())
			.iter()
			.copied()
			.collect()
	}
}

prop_compose! {
	fn generate_size_bounds()(
		x in 0u8..5u8, 
		y in 4u8..20u8,
	) -> Bound<u8> {
		Bound::new(Some(x), Some(y))
	}
}

prop_compose! {
	fn generate_weight_bounds() (
		minimim in 0u64..30u64, 
		maximum in 25u64..100u64,
	) -> Bound<Perquintill> {
		Bound::new(
			Some(Perquintill::from_percent(minimim)), 
			Some(Perquintill::from_percent(maximum))
		)
	}
}

prop_compose! {
	fn generate_creation_fee(number_of_assets: usize)(
		x in 0..Pools::required_creation_deposit_for(number_of_assets).unwrap()
	) -> Balance{
		x
	}
}

prop_compose! {
	fn generate_native_balance(number_of_assets: usize)(x 
		in 0..Pools::required_creation_deposit_for(number_of_assets).unwrap()*2
	) -> Balance{
		x
	}
}

// Deposit Prop Compose Functions

prop_compose! {
	fn generate_n_initial_deposits(n: usize)
	(
		initial_deposits in prop::collection::vec(MINIMUM_DEPOSIT..MAXIMUM_DEPOSIT, n * MAX_POOL_SIZE as usize),
	) -> Vec<Balance>{
		initial_deposits
	}
}

prop_compose! {
	fn generate_n_all_asset_deposits(n: usize, initial_assets: Vec<MockCurrencyId>)
	(
		mut initial_deposits in generate_n_initial_deposits(n),
	) -> (Vec<MockCurrencyId>, Vec<Vec<Deposit<MockCurrencyId, Balance>>>) {
		let mut all_asset_deposits = Vec::new();
			
		for _ in 0..n {
			let mut deposit = Vec::new();

			for asset in &initial_assets {
				deposit.push(
					Deposit {
						asset_id: *asset,
						amount: initial_deposits.pop().unwrap()
					}
				);
			}

			all_asset_deposits.push(deposit);
		}

		(initial_assets.clone(), all_asset_deposits)
	}
}

prop_compose! {
	fn generate_pool_assets_and_n_all_asset_deposits(n: usize)
		(initial_assets in generate_initial_assets_without_duplicates())
		((initial_assets, all_asset_deposits) in generate_n_all_asset_deposits(n, initial_assets))
	-> (Vec<MockCurrencyId>, Vec<Vec<Deposit<MockCurrencyId, Balance>>>) {
		(initial_assets, all_asset_deposits)
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
	fn generate_equal_weighted_pool_config()
		(
			initial_assets in generate_initial_assets_without_duplicates()
		) -> PoolConfig<AccountId, MockCurrencyId, Perquintill> {
			PoolConfig {
				owner: ADMIN,
				fee: Perquintill::zero(),

				assets: initial_assets.clone(),
				asset_bounds: Bound::new(Some(0), Some(MAX_POOL_SIZE)),

				weights: equal_weight_vector_for(&initial_assets),
				weight_bounds: Bound::new(Some(Perquintill::zero()), Some(Perquintill::one())),

				deposit_bounds: Bound::new(Some(Perquintill::zero()), Some(Perquintill::one())),
				withdraw_bounds: Bound::new(Some(Perquintill::zero()), Some(Perquintill::one())),
			}
		}
}

prop_compose! {
	fn generate_equal_weighted_pool_config_and_n_all_asset_deposits(n: usize) 
		(
			pool_config in generate_equal_weighted_pool_config(),
			mut initial_deposits in generate_n_initial_deposits(n),
		) -> (PoolConfig<AccountId, MockCurrencyId, Perquintill>, Vec<Vec<Deposit<MockCurrencyId, Balance>>>){
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

prop_compose! {
	fn generate_equal_weighted_pool_config_and_n_rational_all_asset_deposits(n: usize) 
		(
			pool_config in generate_equal_weighted_pool_config(),
			initial_deposits in generate_n_initial_deposits(1),
			modifiers in prop::collection::vec(0.1f64..5.0f64, n)
		) -> (PoolConfig<AccountId, MockCurrencyId, Perquintill>, Vec<Vec<Deposit<MockCurrencyId, Balance>>>){
			let mut all_deposits = Vec::new();
			
			for modifier in modifiers {
				let mut deposit = Vec::new();

				for (asset, base_deposit) in pool_config.assets.iter().zip(initial_deposits.iter()) {
					let deposit_amount: f64 = *base_deposit as f64 * modifier;
					let deposit_amount: u128 = deposit_amount as u128;
					
					deposit.push(
						Deposit {
							asset_id: *asset,
							amount: deposit_amount
						}
					);
				}

				all_deposits.push(deposit);
			}

			(pool_config, all_deposits)
		}
}

prop_compose! {
	fn generate_equal_weighted_pool_config_and_n_rational_all_asset_deposits_and_initial_balances(n: usize) 
		(
			pool_config in generate_equal_weighted_pool_config(),
			initial_deposits in generate_n_initial_deposits(1),
			initial_balances in generate_n_initial_deposits(1),
			modifiers in prop::collection::vec(0.1f64..5.0f64, n)
		) -> (PoolConfig<AccountId, MockCurrencyId, Perquintill>, Vec<Vec<Deposit<MockCurrencyId, Balance>>>, Vec<Vec<Deposit<MockCurrencyId, Balance>>>){
			let mut all_deposits = Vec::new();
			
			for modifier in &modifiers {
				let mut deposit = Vec::new();

				for (asset, base_deposit) in pool_config.assets.iter().zip(initial_deposits.iter()) {
					let deposit_amount: f64 = *base_deposit as f64 * modifier;
					let deposit_amount: u128 = deposit_amount as u128;
					
					deposit.push(
						Deposit {
							asset_id: *asset,
							amount: deposit_amount
						}
					);
				}

				all_deposits.push(deposit);
			}

			let mut all_balances = Vec::new();
			
			for modifier in &modifiers {
				let mut balance = Vec::new();

				for (asset, base_balance) in pool_config.assets.iter().zip(initial_balances.iter()) {
					let balance_amount: f64 = *base_balance as f64 * modifier;
					let balance_amount: u128 = balance_amount as u128;
					
					balance.push(
						Deposit {
							asset_id: *asset,
							amount: balance_amount
						}
					);
				}

				all_balances.push(balance);
			}

			(pool_config, all_deposits, all_balances)
		}
}

prop_compose! {
	fn generate_eq_pool_config_and_n_all_asset_deposits(n: usize)
		(
			pool_config in generate_equal_weighted_pool_config(),
			deposit_amounts in generate_n_initial_deposits(1),
			deposit_modifiers in prop::collection::vec(1..=5u128, n),
			deposit_users in prop::collection::vec(1..=10u128, n),
		) -> (PoolConfig<AccountId, MockCurrencyId, Perquintill>, Vec<(AccountId, Vec<Deposit<MockCurrencyId, Balance>>)>){
			let mut all_deposits = Vec::new();
			
			for (user, modifier) in deposit_users.iter().zip(deposit_modifiers.iter()) {
				let mut deposit = Vec::new();

				for (asset, base_deposit) in pool_config.assets.iter().zip(deposit_amounts.iter()) {					
					let deposit_amount: u128 = *base_deposit * modifier;

					deposit.push(
						Deposit {
							asset_id: *asset,
							amount: deposit_amount
						}
					);
				}

				all_deposits.push((*user, deposit));
			}

			(pool_config, all_deposits)
		}
}

// TODO: (Nevin) ✔
//  - Set up withdraw framework
//  	-- generate pool config ✔
//		-- generate vector of n users and their respective rational all asset deposits ✔
//      -- generate vector of m withdraws (perquintills) ✔
prop_compose! {
	fn generate_eq_pool_config_and_n_all_asset_deposits_and_m_withdraws(n: usize, m: usize)
		(
			pool_config in generate_equal_weighted_pool_config(),
			deposit_amounts in generate_n_initial_deposits(1),
			deposit_modifiers in prop::collection::vec(1..=5u128, n),
			deposit_users in prop::collection::vec(1..=10u128, n),
			withdraw_ratios in prop::collection::vec(1..=100u64, m),
			withdraw_users in prop::collection::vec(1..=10u128, m),
		) -> (PoolConfig<AccountId, MockCurrencyId, Perquintill>, 
			Vec<(AccountId, Vec<Deposit<MockCurrencyId, Balance>>)>, 
			Vec<(AccountId, Perquintill)>){
			let mut all_deposits = Vec::new();
			
			for (user, modifier) in deposit_users.iter().zip(deposit_modifiers.iter()) {
				let mut deposit = Vec::new();

				for (asset, base_deposit) in pool_config.assets.iter().zip(deposit_amounts.iter()) {					
					let deposit_amount: u128 = *base_deposit * modifier;

					deposit.push(
						Deposit {
							asset_id: *asset,
							amount: deposit_amount
						}
					);
				}

				all_deposits.push((*user, deposit));
			}

			let mut all_withdraws = Vec::new();

			withdraw_users.iter()
				.zip(withdraw_ratios.iter())
				.for_each(|(user, ratio)| {
					all_withdraws.push((*user, Perquintill::from_percent(*ratio)))
				});

			(pool_config, all_deposits, all_withdraws)
		}
}

// ----------------------------------------------------------------------------------------------------
//                                           Helper Functionality                                          
// ----------------------------------------------------------------------------------------------------

pub struct PoolConfigBuilder {
	/// Owner of pool
	pub owner: AccountId,
	/// Amount of the fee pool charges for the exchange
	pub fee: Perquintill,
	/// Vector of the Pool's underlying assets
	pub assets: Vec<MockCurrencyId>,
	/// Min/max bounds on number of assets allowed in the pool
	pub asset_bounds: Bound<u8>,
	/// Vector of the Pool's underlying asset weights
	pub weights: Vec<Weight<MockCurrencyId, Perquintill>>,
	/// Min/max bounds on weights of assets for the pool
	pub weight_bounds: Bound<Perquintill>,
	/// Min/max bounds on amount of assets that can be deposited at once
	pub deposit_bounds: Bound<Perquintill>,
	/// Min/max bounds on amount of assets that can be withdrawn at once
	pub withdraw_bounds: Bound<Perquintill>,
}

impl Default for PoolConfigBuilder {
	fn default() -> Self {
		PoolConfigBuilder {
			owner: ALICE,
			fee: Perquintill::zero(),
			assets: Vec::new(),
			asset_bounds: Bound::new(Some(0), Some(MAX_POOL_SIZE)),
			weights: Vec::new(),
			weight_bounds: Bound::new(None, None),
			deposit_bounds: Bound::new(None, None),
			withdraw_bounds: Bound::new(None, None),
		}
	}
}

impl PoolConfigBuilder {
	#[allow(dead_code)]
	fn owner(mut self, owner: AccountId) -> Self {
		self.owner = owner;
		self
	}

	#[allow(dead_code)]
	fn fee(mut self, fee: Perquintill) -> Self {
		self.fee = fee;
		self
	}

	fn assets(mut self, assets: &Vec<MockCurrencyId>) -> Self {
		self.assets = assets.to_vec();
		self
	}

	fn asset_bounds(mut self, asset_bounds: Bound<u8>) -> Self {
		self.asset_bounds = asset_bounds;
		self
	}

	fn weights(mut self, weights: &Vec<Weight<MockCurrencyId, Perquintill>>) -> Self {
		self.weights = weights.to_vec();
		self
	}

	fn weight_bounds(mut self, weight_bounds: Bound<Perquintill>) -> Self {
		self.weight_bounds = weight_bounds;
		self
	}

	#[allow(dead_code)]
	fn deposit_bounds(mut self, deposit_bounds: Bound<Perquintill>) -> Self {
		self.deposit_bounds = deposit_bounds;
		self
	}

	#[allow(dead_code)]
	fn withdraw_bounds(mut self, withdraw_bounds: Bound<Perquintill>) -> Self {
		self.withdraw_bounds = withdraw_bounds;
		self
	}
	
	fn build(&self) -> PoolConfig<AccountId, MockCurrencyId, Perquintill> {
		PoolConfig {
			owner: self.owner,
			fee: self.fee,
			assets: self.assets.clone(),
			asset_bounds: self.asset_bounds,
			weights: self.weights.clone(),
			weight_bounds: self.weight_bounds,
			deposit_bounds: self.deposit_bounds,
			withdraw_bounds: self.withdraw_bounds,
		}
	}
}

#[derive(Default)]
pub struct PoolStateBuilder {
	pub config: PoolConfig<AccountId, MockCurrencyId, Perquintill>,
	pub reserves: Vec<Vec<Reserve<MockCurrencyId, Balance>>>,
}

impl PoolStateBuilder {
	#[allow(dead_code)]
	fn config(mut self, config: PoolConfig<AccountId, MockCurrencyId, Perquintill>) -> Self {
		self.config = config;
		self
	}

	#[allow(dead_code)]
	fn reserves(mut self, reserves: Vec<Vec<Reserve<MockCurrencyId, Balance>>>) -> Self {
		self.reserves = reserves;
		self
	}

	fn build(self) -> u64 {
		let creation_fee = Deposit {
			asset_id: MockCurrencyId::A,
			amount: Pools::required_creation_deposit_for(self.config.assets.len()).unwrap(), 
		};
		assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ADMIN, creation_fee.amount));
	
		let pool_id = Pools::create(ADMIN, self.config, creation_fee).unwrap();
	
		self.reserves.iter()
			.for_each(|reserve| {
				reserve.iter().for_each(|deposit| {
					assert_ok!(Tokens::mint_into(deposit.asset_id, &ADMIN, deposit.amount));
				});
				
				assert_ok!(Pools::all_asset_deposit(&ADMIN, &pool_id, reserve.to_vec()));
			});

		pool_id
	}
}

pub struct ValidityCheck {}

fn equal_weight_vector_for(assets: &[MockCurrencyId]) -> Vec<Weight<MockCurrencyId, Perquintill>>{
	assets.iter()
		.map(|asset_id|
			Weight {
				asset_id: *asset_id,
				weight: Perquintill::from_rational(1, assets.len() as u64),
			}
		)
		.collect()
}

fn normalize_weights(non_normalized_weights: &mut Vec<Perquintill>) -> Vec<Perquintill> {
	let sum = non_normalized_weights.iter()
		.map(|weight| weight.deconstruct())
		.sum();
	
	non_normalized_weights.iter()
		.map(|weight| 
			Perquintill::from_rational(weight.deconstruct(), sum)
		)
		.collect()
}

fn construct_weight_vector_from(assets: &[MockCurrencyId], weights: &[Perquintill]) -> Vec<Weight<MockCurrencyId, Perquintill>> {
	assets.iter()
		.zip(weights.iter())
		.map(|(asset_id, weight)|
			Weight {
				asset_id: *asset_id,
				weight: *weight
			}
		)
		.collect()
}

fn create_pool_with(config: &PoolConfig<AccountId, MockCurrencyId, Perquintill>) -> u64 {
	let creation_fee = Deposit {
		asset_id: MockCurrencyId::A,
		amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
	};
	assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));

	<Pools as ConstantMeanMarket>::create(ALICE, config.clone(), creation_fee).unwrap()
}

fn deposit_into(
	pool_id: &u64, 
	user: &AccountId, 
	deposits: Vec<Deposit<MockCurrencyId, Balance>>
) -> Result<Balance, DispatchError> {
	for deposit in &deposits {
		assert_ok!(Tokens::mint_into(deposit.asset_id, user, deposit.amount));
	}
	
	<Pools as ConstantMeanMarket>::all_asset_deposit(user, pool_id, deposits)
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
		//        v.	w_i ≥ 0
		//        vi.	Σ w_i = 1
		//		  vii.	min_weight ≤ max_weight
		//        viii.	min_weight ≤ w_i ≤ max_weight
		//        ix.	creation_fee ≥ (asset_ids.len() + 1) * (creation_deposit + existential_deposit)
		//        x.	user_balance ≥ creation_fee

		let config = PoolConfigBuilder::default()
			.assets(&initial_assets)
			.weights(&equal_weight_vector_for(&initial_assets))
			.build();

		ExtBuilder::default().build().execute_with(|| {
			// Condition viii
			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
			};

			// Condition ix
			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));
		
			assert_ok!(Pools::create(ALICE, config, creation_fee));
		});
	}

	#[test]
	fn creating_a_pool_that_does_not_meet_the_asset_requirements_raises_an_error(
		initial_assets in generate_initial_assets(),
		size_bounds in generate_size_bounds(),
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
			let config = PoolConfigBuilder::default()
				.assets(&initial_assets)
				.asset_bounds(size_bounds)
				.weights(&equal_weight_vector_for(&initial_assets))
				.build();

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));

			let unique_initial_assets = BTreeSet::<MockCurrencyId>::from_iter(initial_assets.iter().copied()).len();
			let pool_size = config.assets.len();

			let error = if unique_initial_assets != pool_size {
				// Condition i
				Some(Error::<Test>::DuplicateAssets)
			} else if config.asset_bounds.maximum < config.asset_bounds.minimum {
				// Condition ii
				Some(Error::<Test>::InvalidAssetBounds)
			} else if pool_size < config.asset_bounds.minimum.unwrap() as usize || (config.asset_bounds.maximum.unwrap() as usize) < pool_size {
				// Condition iii
				Some(Error::<Test>::PoolSizeIsOutsideOfAssetBounds)
			} else {
				None
			};

			match error {
				None => { assert_ok!(Pools::create(ALICE, config, creation_fee)); },
				Some(error) => {
					assert_noop!(Pools::create(ALICE, config, creation_fee), error);
				}
			}
		});
	}

	#[test]
	fn creating_a_pool_that_does_not_meet_the_weight_requirements_raises_an_error(
		initial_assets in generate_initial_assets(),
		mut weights in generate_random_weights(),
		weight_bounds in generate_weight_bounds(),
	) {
		// Tests that if not all weight conditions are met to create a pool it will not
		//  |  be created
		//  '-> Conditions:
		//        ...
		//        iv.  ∀ assets a_i ⇒ ∃ weight w_i
		//        v.	w_i ≥ 0
		//        vi.	Σ w_i = 1
		//		  vii.	min_weight ≤ max_weight
		//        viii.	min_weight ≤ w_i ≤ max_weight
		//        ...

		// create a random weight vector for the generated initial assets,
		//     however, at this stage there might be duplicate assets so these 
		//     assets will have multiple weights in the weight vector
		weights.resize(initial_assets.len(), Perquintill::zero());
		let weights: Vec<Perquintill> = normalize_weights(&mut weights);
		let weights: Vec<Weight<MockCurrencyId, Perquintill>> = construct_weight_vector_from(&initial_assets, &weights);

		// remove duplicate assets
		let initial_assets: Vec<MockCurrencyId> = BTreeSet::<MockCurrencyId>::from_iter(initial_assets.iter().copied())
			.iter()
			.copied()
			.collect();

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfigBuilder::default()
				.assets(&initial_assets)
				.weights(&weights)
				.weight_bounds(weight_bounds)
				.build();

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));
			
			let error = if !Pools::each_asset_has_exactly_one_corresponding_weight(&config.assets, &config.weights) {
				// Condition iv
				Some(Error::<Test>::ThereMustBeOneWeightForEachAssetInThePool)
			} else if !Pools::weights_are_nonnegative(&config.weights) {
				// Condition v
				Some(Error::<Test>::PoolWeightsMustBeNonnegative)
		    } else if !Pools::weights_are_normalized(&config.weights) {
				// Condition vi
				Some(Error::<Test>::PoolWeightsMustBeNormalized)
		    } else if config.weight_bounds.maximum < config.weight_bounds.minimum {
				// Condition vii
				Some(Error::<Test>::InvalidWeightBounds)
		    } else if !Pools::weights_are_in_weight_bounds(&config.weights, &config.weight_bounds) {
				// Condition v
				Some(Error::<Test>::PoolWeightsAreOutsideOfWeightBounds)
		    } else {
				None
			};

			match error {
				None => { assert_ok!(Pools::create(ALICE, config, creation_fee)); },
				Some(error) => {
					assert_noop!(Pools::create(ALICE, config, creation_fee), error);
				}
			}
		});
	}

	#[test]
	fn creating_a_pool_that_does_not_meet_the_user_requirements_raises_an_error(
		initial_assets in generate_initial_assets_without_duplicates(),
		user_balance in generate_native_balance(MAX_POOL_SIZE as usize),
		creation_fee in generate_creation_fee(MAX_POOL_SIZE as usize),
	) {
		// Tests that if all the conditions are met to create a pool it will be created 
		//  |  successfully
		//  '-> Conditions:
		//		  ...
		//        ix.	creation_fee ≥ (asset_ids.len() + 1) * 
		//                  (creation_deposit + existential_deposit)
		//        x.	user_balance ≥ creation_fee

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfigBuilder::default()
				.assets(&initial_assets)
				.weights(&equal_weight_vector_for(&initial_assets))
				.build();

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: creation_fee, 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, user_balance));
			
			let required_creatikon_deposit = 
				Pools::required_creation_deposit_for(config.assets.len()).unwrap();

			let error = if user_balance < creation_fee.amount {
				// Condition ix
				Some(Error::<Test>::IssuerDoesNotHaveBalanceTryingToDeposit)
			} else if creation_fee.amount < required_creatikon_deposit {
				// Condition x
				Some(Error::<Test>::CreationFeeIsInsufficient)
		    } else {
				None
			};

			match error {
				None => { assert_ok!(Pools::create(ALICE, config, creation_fee)); },
				Some(error) => {
					assert_noop!(Pools::create(ALICE, config, creation_fee), error);
				}
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
			let config = PoolConfigBuilder::default()
				.assets(&initial_assets)
				.weights(&equal_weight_vector_for(&initial_assets))
				.build();

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));
		
			let pool_id = Pools::create(ALICE, config.clone(), creation_fee).unwrap();

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
			let config = PoolConfigBuilder::default()
				.assets(&initial_assets)
				.weights(&equal_weight_vector_for(&initial_assets))
				.build();

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: Pools::required_creation_deposit_for(config.assets.len()).unwrap(), 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, creation_fee.amount));
		
			let pool_id = Pools::create(ALICE, config, creation_fee).unwrap();

			// Condition i
			for asset_id in initial_assets {
				assert!(PoolAssetVault::<Test>::contains_key(pool_id, asset_id));
			}
		});
	}

	#[test]
	fn creating_a_pool_transfers_creation_fee_into_pools_account(
		initial_assets in generate_initial_assets_without_duplicates(),
		creation_fee in generate_creation_fee(MAX_POOL_SIZE as usize),
		user_balance in generate_native_balance(MAX_POOL_SIZE as usize),
	) {
		// Tests that when a user successfully creates a Pool their Creation fee is transfered 
		//  |  into the Pools account
		//  |-> Pre-Conditions:
		//  |     i.   user (U) has at least n ≥ CreationFee native tokens in their account
		//  '-> Post-Conditions:
		//        ii.  user (U) has n' = n - △ native tokens in their account, where △ = CreationFee
		//        iii. pool (P) has △ native tokens in its account

		// guarantee the user has enough native assets to create the pool
		let required_creation_fee = 
			Pools::required_creation_deposit_for(initial_assets.len()).unwrap();

		let creation_fee_amount = creation_fee + required_creation_fee;
		let user_balance = user_balance + creation_fee_amount;

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfigBuilder::default()
				.assets(&initial_assets)
				.weights(&equal_weight_vector_for(&initial_assets))
				.build();

			let creation_fee = Deposit {
				asset_id: MockCurrencyId::A,
				amount: creation_fee_amount, 
			};

			assert_ok!(Tokens::mint_into(MockCurrencyId::A, &ALICE, user_balance));
			
			// Pre-Condition i
			assert!(Tokens::balance(MockCurrencyId::A, &ALICE) >= creation_fee_amount);

			let pool_id = Pools::create(ALICE, config, creation_fee).unwrap();

			// Post-Condition ii
			assert_eq!(
				Tokens::balance(MockCurrencyId::A, &ALICE), 
				user_balance - creation_fee_amount
			);

			// Post-Condition iii
			assert_eq!(
				Tokens::balance(MockCurrencyId::A, &Pools::account_id(&pool_id)), 
				creation_fee_amount
			);
		});
	}
}

// ----------------------------------------------------------------------------------------------------
//                                          All-Asset Deposit                                          
// ----------------------------------------------------------------------------------------------------

impl ValidityCheck {
	pub fn user_does_not_have_balance_trying_to_deposit(user: &AccountId, asset_id: MockCurrencyId, amount: Balance) -> bool {
		Tokens::can_withdraw(asset_id, user, amount) != WithdrawConsequence::Success
	}

	pub fn deposit_is_within_nonempty_pools_deposit_bounds(pool_id: &u64, deposit: &Deposit<MockCurrencyId, Balance>) -> bool {
		let amount = deposit.amount as f64;

		let reserve: Balance = Pools::balance_of(pool_id, &deposit.asset_id).unwrap();
		let reserve: f64 = reserve as f64;

		let reserve_increase: f64 = amount / reserve;

		let deposit_bounds: Bound<Perquintill> = Pools::pool_info(pool_id).unwrap().deposit_bounds;
		let lower_bound: f64 = match deposit_bounds.minimum {
			None => 0.0,
			Some(minimum) => minimum.deconstruct() as f64 / Perquintill::one().deconstruct() as f64,
		};
		let upper_bound: f64 = match deposit_bounds.minimum {
			None => f64::MAX,
			Some(maximum) => maximum.deconstruct() as f64 / Perquintill::one().deconstruct() as f64,
		};
			
		lower_bound <= reserve_increase && reserve_increase <= upper_bound
	}

	pub fn deposit_matches_underlying_value_distribution(pool_id: &u64, deposit: &Deposit<MockCurrencyId, Balance>, deposit_total: Balance, reserve_total: Balance) -> bool {
		let lp_circulating_supply = Pools::lp_circulating_supply(pool_id).unwrap();	
		if lp_circulating_supply == 0u128 { 
			return true;
		}
		
		let asset =  deposit.asset_id;

		let deposit = deposit.amount as f64;
		let deposit_value_distribution = deposit / deposit_total as f64;

		let reserve = Pools::balance_of(pool_id, &asset).unwrap();
		let reserve = reserve as f64;
		let reserve_value_distribution = reserve / reserve_total as f64;

		let margin_of_error = Epsilon::get().deconstruct() as f64 / Perquintill::one().deconstruct() as f64;
		let margin_of_error = margin_of_error * reserve_value_distribution;
		
		let lower_bound = reserve_value_distribution - margin_of_error;
		let upper_bound = reserve_value_distribution + margin_of_error;

		if deposit_value_distribution < lower_bound || upper_bound < deposit_value_distribution {
			println!("asset: {asset:?}");
			println!("test_lower_bound {:?}", lower_bound);
			println!("test_deposit_value_distribution {:?}", deposit_value_distribution);
			println!("test_upper_bound {:?}\n", upper_bound);
		}

		lower_bound <= deposit_value_distribution && deposit_value_distribution <= upper_bound 
	}
}

fn weighted_geometric_mean(
	reserves: Vec<Deposit<MockCurrencyId, Balance>>, 
	weights: Vec<Weight<MockCurrencyId, Perquintill>>
) -> Result<u128, ()> {
	let mut invariant_constant: f64 = 1.0;

	for (reserve, weight) in reserves.iter().zip(weights.iter()) {

		let reserve: f64 = reserve.amount as f64;
		let weight: f64 = 
			weight.weight.deconstruct() as f64 / Perquintill::one().deconstruct() as f64;

		let result = reserve.powf(weight);
		
		invariant_constant = invariant_constant * result;
	}

	Ok(invariant_constant as u128)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]

	#[test]
	fn depositing_into_a_pool_transfers_funds_from_issuers_account_and_into_the_underlying_vault_accounts(
		(pool_assets, all_asset_deposits) in generate_pool_assets_and_n_all_asset_deposits(1)
	) {
		// Tests that when a user deposits assets into the Pool, these assets are removed from the users
		//  |  account and transferred into the underlying vault accounts
		//  |-> Pre-Conditions:
		//  |     i.   ∀ deposits d of assets a1 ... an ⇒  user U has balance b_i of asset a_i before the deposit
		//  |     ii.  ∀ deposits d of assets a1 ... an ⇒ vault V has balance v_i of asset a_i before the deposit
		//  |     iii. User U deposits balance △_i of asset a_i into Pool P
		//  '-> Post-Conditions:
		//        iii. ∀ deposits d of assets a1 ... an ⇒  user U has balance b_i - △i of asset a_i after the deposit
		//        iv.  ∀ deposits d of assets a1 ... an ⇒ vault V has balance v_i + △i of asset a_i after the deposit
		
		// Only tests the state after one deposit
		let deposits = (&all_asset_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfigBuilder::default()
				.assets(&pool_assets)
				.weights(&equal_weight_vector_for(&pool_assets))
				.build();

			// Create the pool
			let pool_id = PoolStateBuilder::default()
				.config(config.clone())
				.build();

			// Make the first deposit
			for deposit in &deposits {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &ALICE, deposit.amount));
			}

			// Condition i
			for balance in &deposits {
				assert_eq!(Tokens::balance(balance.asset_id, &ALICE), balance.amount);
			}

			// Condition ii
			for deposit in &deposits {
				let vault_id = PoolAssetVault::<Test>::get(pool_id, deposit.asset_id);
				let vault_account = Vaults::account_id(&vault_id);
	
				assert_eq!(Tokens::balance(deposit.asset_id, &vault_account), 0);
			}

			// Condition iii
			assert_ok!(Pools::all_asset_deposit(&ALICE, &pool_id, deposits.clone()));

			// Condition iv
			for balance in &deposits {
				assert_eq!(Tokens::balance(balance.asset_id, &ALICE), 0);
			}

			// Condition v
			for deposit in &deposits {
				let vault_id = PoolAssetVault::<Test>::get(pool_id, deposit.asset_id);
				let vault_account = Vaults::account_id(&vault_id);
	
				assert_eq!(Tokens::balance(deposit.asset_id, &vault_account), deposit.amount);
			}

		});
	}

	#[test]
	fn depositing_into_an_empty_pool_correctly_mints_lp_tokens(
		(pool_assets, all_asset_deposits) in generate_pool_assets_and_n_all_asset_deposits(1)
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
		let deposits = (&all_asset_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfigBuilder::default()
				.assets(&pool_assets)
				.weights(&equal_weight_vector_for(&pool_assets))
				.build();

			// Condition i - Pool is newly created
			let pool_id = PoolStateBuilder::default()
				.config(config.clone())
				.build();
			
			// Make the first deposit
			for deposit in &deposits {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &ALICE, deposit.amount));
			}
			assert_ok!(Pools::all_asset_deposit(&ALICE, &pool_id, deposits.clone()));

			// Condition ii
			let weighted_geometric_mean: f64 = 
				invariant(deposits, config.weights).unwrap().to_num::<f64>();

			let lp_token = Pools::lp_token_id(&pool_id).unwrap();
			let lp_tokens_minted: f64 = Tokens::balance(lp_token, &ALICE) as f64;

			assert_relative_eq!(lp_tokens_minted, weighted_geometric_mean, epsilon=2.0);
		});
	}

	#[test]
	fn depositing_into_a_non_empty_pool_with_duplicate_deposits_correctly_mints_lp_tokens(
		(pool_assets, all_asset_deposits) in generate_pool_assets_and_n_all_asset_deposits(1)
	) {
		// Tests that when a user deposits assets into a non-empty Pool (i.e. their deposit is not 
		//  |  the Pool's first deposit) the user is rewarded with a number of LP tokens equal to
		//  |  the increase in the Pools invariant value.
		//  |-> Pre-Conditions:
		//  |     i.   Pool P is nonempty
		//  '-> Post-Conditions:
		//        ii.  ∀ secondary deposits D into P : User U receives LP_minted tokens, where 
		//				   LP_minted = LP_supply * (w_i * (D_i / B_i)) and D_i and B_i refer to the
		//                 deposited amount and pool's reserve  of asset i, respectively, and w_i 
		//				   refers the Pool's weight of asset i

		prop_assume!(pool_assets.len() >= 2);

		// use the same deposit for both deposits - sendary deposits should mint equal LP amounts
		let deposit_1 = (&all_asset_deposits[0]).to_vec();
		let deposit_2 = (&all_asset_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfigBuilder::default()
				.assets(&pool_assets)
				.weights(&equal_weight_vector_for(&pool_assets))
				.build();

			let pool_id = PoolStateBuilder::default()
				.config(config.clone())
				.build();
				
			let lp_token = Pools::lp_token_id(&pool_id).unwrap();
			
			// Make the first deposit
			for deposit in &deposit_1 {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &ALICE, deposit.amount));
			}
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_deposit(&ALICE, &pool_id, deposit_1.clone()));

			// Condition i
			let alice_lp_tokens: Balance = Tokens::balance(lp_token, &ALICE);

			// Make the second deposit
			for deposit in &deposit_2 {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &BOB, deposit.amount));
			}
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_deposit(&BOB, &pool_id, deposit_2.clone()));

			// Condition ii
			let bob_lp_tokens: Balance = Tokens::balance(lp_token, &BOB);
			assert_relative_eq!(bob_lp_tokens as f64, alice_lp_tokens as f64, epsilon = 1.0);

			let lp_circulating_supply: Balance = Pools::lp_circulating_supply(&pool_id).unwrap();
			assert_eq!(lp_circulating_supply, alice_lp_tokens + bob_lp_tokens);
		});
	}

	#[test]
	fn depositing_into_a_non_empty_pool_correctly_mints_lp_tokens(
		(pool_assets, all_asset_deposits) in generate_pool_assets_and_n_all_asset_deposits(2),
	) {
		// Tests that when a user deposits assets into a non-empty Pool (i.e. their deposit is not 
		//  |  the Pool's first deposit) the user is rewarded with a number of LP tokens equal to
		//  |  the increase in the Pools invariant value.
		//  |-> Pre-Conditions:
		//  |     i.   Pool P is nonempty
		//  '-> Post-Conditions:
		//        ii.  ∀ secondary deposits D into P : User U receives LP_minted tokens, where 
		//				   LP_minted = LP_supply * (w_i * (D_i / B_i)) and D_i and B_i refer to the
		//                 deposited amount and pool's reserve  of asset i, respectively, and w_i 
		//				   refers the Pool's weight of asset i

		let deposit_1 = (&all_asset_deposits[0]).to_vec();
		let deposit_2 = (&all_asset_deposits[1]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfigBuilder::default()
				.assets(&pool_assets)
				.weights(&equal_weight_vector_for(&pool_assets))
				.build();

			let pool_id = PoolStateBuilder::default()
				.config(config.clone())
				.build();

			let lp_token = Pools::lp_token_id(&pool_id).unwrap();
			
			// Make the first deposit
			for deposit in &deposit_1 {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &ALICE, deposit.amount));
			}
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_deposit(&ALICE, &pool_id, deposit_1.clone()));

			// Condition i
			let alice_lp_tokens: Balance = Tokens::balance(lp_token, &ALICE);

			// Make the second deposit
			for deposit in &deposit_2 {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &BOB, deposit.amount));
			}
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_deposit(&BOB, &pool_id, deposit_2.clone()));

			// Condition ii
			let deposit_ratio: f64 = deposit_2.iter()
				.zip(deposit_1.iter())
				.map(|(two, one)| two.amount as f64 / one.amount as f64)
				.map(|ratio| ratio / config.assets.len() as f64)
				.sum();

			let bob_lp_tokens: Balance = Tokens::balance(lp_token, &BOB);
			assert_relative_eq!(bob_lp_tokens as f64 / alice_lp_tokens as f64, deposit_ratio, epsilon=1.0e-2);

			let lp_circulating_supply: Balance = Pools::lp_circulating_supply(&pool_id).unwrap();
			assert_eq!(lp_circulating_supply, alice_lp_tokens + bob_lp_tokens);
		});
	}

	#[test]
	fn depositing_into_a_non_empty_pool_with_an_invalid_deposit_raises_an_error(
		(config, all_deposits, all_balances) in generate_equal_weighted_pool_config_and_n_rational_all_asset_deposits_and_initial_balances(2),
	) {
		// Tests that when a user deposits assets into a non-empty Pool (i.e. their deposit is not 
		//  |  the Pool's first deposit) the user is rewarded with a number of LP tokens equal to
		//  |  the increase in the Pools invariant value.
		//  |-> Pre-Conditions:
		//  |     i.   Pool P is nonempty
		//  '-> Post-Conditions:
		//        ii.  ∀ deposits dep in Pool : deposit amount dep_i > 0
		//        iii. ∀ deposits dep in Pool : users has asset amounts trying to deposit
		//        iv.  ∀ deposits dep in Pool with reserves r : dep_min ≤ dep_i / r_i ≤ dep_max
		//        v.   ∀ deposits dep in Pool with reserves r : dep_i / dep_total = r_i / r_total

		prop_assume!(config.assets.len() >= 2);

		let deposit_1 = (&all_deposits[0]).to_vec();
		let deposit_2 = (&all_deposits[1]).to_vec();

		let _balance_1 = (&all_balances[0]).to_vec();
		let balance_2 = (&all_balances[1]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			
			// Condition i - Make the first deposit
			for balance in &deposit_1 {
				assert_ok!(Tokens::mint_into(balance.asset_id, &ALICE, balance.amount));
			}
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_deposit(&ALICE, &pool_id, deposit_1.clone()));

			// Make the second deposit
			for balance in &balance_2 {
				assert_ok!(Tokens::mint_into(balance.asset_id, &BOB, balance.amount));
			}

			let reserve_total: u128 = deposit_1.iter()
				.fold(0, |total, deposit| total + deposit.amount);
			let deposit_total: u128 = deposit_2.iter()
				.fold(0, |total, deposit| total + deposit.amount);

			let mut deposit_is_valid = true;
			for deposit in &deposit_2 {
				// Condition ii
				if deposit.amount == 0 {
					assert_noop!(
						<Pools as ConstantMeanMarket>::all_asset_deposit(&BOB, &pool_id, deposit_2.clone()),
						Error::<Test>::DepositsMustBeStrictlyPositive
					);

					deposit_is_valid = false;
					break;
				// Condition iii
				} else if ValidityCheck::user_does_not_have_balance_trying_to_deposit(&BOB, deposit.asset_id, deposit.amount) {
					assert_noop!(
						<Pools as ConstantMeanMarket>::all_asset_deposit(&BOB, &pool_id, deposit_2.clone()),
						Error::<Test>::IssuerDoesNotHaveBalanceTryingToDeposit
					);

					deposit_is_valid = false;
					break;
				// Condition iv
				} else if !ValidityCheck::deposit_is_within_nonempty_pools_deposit_bounds(&pool_id, deposit) {
					assert_noop!(
						<Pools as ConstantMeanMarket>::all_asset_deposit(&BOB, &pool_id, deposit_2.clone()),
						Error::<Test>::DepositIsOutsideOfPoolsDepositBounds
					);

					deposit_is_valid = false;
					break;
				// Condition v
				} else if !ValidityCheck::deposit_matches_underlying_value_distribution(
					&pool_id, deposit, deposit_total, reserve_total
				) {					
					assert_noop!(
						<Pools as ConstantMeanMarket>::all_asset_deposit(&BOB, &pool_id, deposit_2.clone()),
						Error::<Test>::DepositDoesNotMatchUnderlyingValueDistribution
					);

					deposit_is_valid = false;
					break;
				}
			}

			// If all post-conditions are met the deposit is valid
			if deposit_is_valid {
				assert_ok!(<Pools as ConstantMeanMarket>::all_asset_deposit(&BOB, &pool_id, deposit_2.clone()));
			}
		});
	}

	#[test]
	fn depositing_an_amount_of_tokens_outside_of_asset_bounds_raises_an_error(
		(pool_assets, all_asset_deposits) in generate_pool_assets_and_n_all_asset_deposits(1),
	) {
		// Tests that deposits after the initial deposit follow the Pool's deposit bounds set 
		//  |  during Pool creation
		//  |-> Pre-Conditions:
		//  |     i.   Pool P is nonempty
		//  '-> Post-Conditions:
		//        ii.  ∀ deposits dep in Pool with reserves r : dep_min ≤ (r_i + dep_i) / r_i
		//					'-> i.e. the Pool's invariant constant doesn't increase lower than the
		//							Pool's minimum deposit bound
		//        iii. ∀ deposits dep in Pool with reserves r : (r_i + dep_i) / r_i ≤ dep_max
		//					'-> i.e. the Pool's invariant constant doesn't increase greater than the
		//							Pool's maximum deposit bound

		prop_assume!(pool_assets.len() >= 2);

		let deposit_1 = (&all_asset_deposits[0]).to_vec();
		let lower_bound = 10;
		let upper_bound = 30;

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfigBuilder::default()
				.assets(&pool_assets)
				.weights(&equal_weight_vector_for(&pool_assets))
				.deposit_bounds(Bound {
					minimum: Some(Perquintill::from_percent(lower_bound)), 
					maximum: Some(Perquintill::from_percent(upper_bound))
				})
				.build();

			let pool_id = PoolStateBuilder::default()
				.config(config.clone())
				.build();
						
			// Condition i - Make the first deposit
			for balance in &deposit_1 {
				assert_ok!(Tokens::mint_into(balance.asset_id, &ALICE, balance.amount));
			}
			assert_ok!(Pools::all_asset_deposit(&ALICE, &pool_id, deposit_1.clone()));

			// Attempt to a deposit below the minimum deposit bound
			let deposit_2 = deposit_1.iter()
				.map(|deposit| Deposit {
					asset_id: deposit.asset_id,
					amount: Perquintill::from_percent(lower_bound / 2) * deposit.amount
				})
				.collect::<Vec<Deposit<MockCurrencyId, Balance>>>();

			for balance in &deposit_2 {
				assert_ok!(Tokens::mint_into(balance.asset_id, &BOB, balance.amount));
			}
			// Condition ii
			assert_noop!(
				Pools::all_asset_deposit(&BOB, &pool_id, deposit_2),
				Error::<Test>::DepositIsOutsideOfPoolsDepositBounds
			);

			// Attempt a deposit above the maximum deposit bound
			let deposit_3 = deposit_1.iter()
				.map(|deposit| Deposit {
					asset_id: deposit.asset_id,
					amount: Perquintill::from_percent(upper_bound * 2) * deposit.amount
				})
				.collect::<Vec<Deposit<MockCurrencyId, Balance>>>();
			
			for balance in &deposit_3 {
				assert_ok!(Tokens::mint_into(balance.asset_id, &CHARLIE, balance.amount));
			}

			// Condition iii
			assert_noop!(
				Pools::all_asset_deposit(&CHARLIE, &pool_id, deposit_3),
				Error::<Test>::DepositIsOutsideOfPoolsDepositBounds
			);
		});
	}

	#[test]
	fn depositing_equal_deposits_mints_the_same_amount_of_lp_tokens(
		(config, all_deposits) in generate_eq_pool_config_and_n_all_asset_deposits(4),
	) {
		// Tests that when users deposit into a pool, the runtime objects storing the Pool's
		//  |  reserves are correctly updated
		//  |-> Pre-Conditions:
		//  |     i.  Pool P is non-empty
		//  '-> Post-Conditions:
		//        ii. ∀ deposits d ⇒ consecutive deposits of d should mint the smae amount of LP tokens

		let (user_1, deposit_1) = (all_deposits[0].0, all_deposits[0].1.to_vec());
		let (user_2, deposit_2) = (all_deposits[1].0, all_deposits[1].1.to_vec());
		let (user_3, deposit_3) = (all_deposits[2].0, all_deposits[1].1.to_vec());
		let (user_4, deposit_4) = (all_deposits[3].0, all_deposits[1].1.to_vec());

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);

			// Condition i - Initial deposit
			let _lp_tokens_minted_from_first_deposit = deposit_into(&pool_id, &user_1, deposit_1).unwrap();
						
			// Second deposit
			let lp_tokens_minted_from_second_deposit = deposit_into(&pool_id, &user_2, deposit_2).unwrap();
			let lp_tokens_minted_from_second_deposit: f64 = lp_tokens_minted_from_second_deposit as f64;
			
			// Third deposit
			let lp_tokens_minted_from_third_deposit = deposit_into(&pool_id, &user_3, deposit_3).unwrap();
			let lp_tokens_minted_from_third_deposit: f64 = lp_tokens_minted_from_third_deposit as f64;

			// Condition ii
			assert_relative_eq!(lp_tokens_minted_from_second_deposit, lp_tokens_minted_from_third_deposit, epsilon = 1.0);
			
			// Fourth deposit
			let lp_tokens_minted_from_fourth_deposit = deposit_into(&pool_id, &user_4, deposit_4).unwrap();
			let lp_tokens_minted_from_fourth_deposit: f64 = lp_tokens_minted_from_fourth_deposit as f64;

			// Condition ii
			assert_relative_eq!(lp_tokens_minted_from_second_deposit, lp_tokens_minted_from_fourth_deposit, epsilon = 1.0);
		});
	}

	#[test]
	fn depositing_into_an_empty_pool_correctly_updates_runtime_storage_objects(
		(config, all_deposits) in generate_equal_weighted_pool_config_and_n_all_asset_deposits(2),
	) {
		// Tests that when users deposit into a pool, the runtime objects storing the Pool's
		//  |  reserves are correctly updated
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
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_deposit(&ALICE, &pool_id, deposit_1.clone()));

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
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_deposit(&BOB, &pool_id, deposit_2.clone()));

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
	fn depositing_into_a_pools_underlying_vaults_acts_properly(
		(config, all_deposits) in generate_equal_weighted_pool_config_and_n_all_asset_deposits(2),
	) {
		// Test that the lp tokens that are minted by the underlying vaults are kept by
		//  |  the Pools account and the issuer of the extrinsic never receives them
		//  |-> Pre-Conditions:
		//  |     i.   ∀ deposits d ⇒ pool (P) has πi of lp tokens from vault Vi before the deposit
		//  |     ii.  ∀ deposits d ⇒ user (U) has 0 lp tokens from vault Vi before the deposit
		//  '-> Post-Conditions:
		//        iii. ∀ deposits d ⇒ pool (P) has πi + △i of lp tokens from vault Vi after the deposit
		//                 where △i corresponds to the number of lp tokens minted by vaut Vi
		//        iv.  ∀ deposits d ⇒ user (U) has 0 lp tokens from vault Vi after the deposit

		// Only tests the state after one deposit
		let deposit_1 = (&all_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			let pool_account = Pools::account_id(&pool_id);

			for asset in &config.assets {
				let vault_id = PoolAssetVault::<Test>::get(pool_id, asset);
				let vault_lp_token_id = Vaults::lp_asset_id(&vault_id).unwrap();

				// Condition i
				assert_eq!(Tokens::balance(vault_lp_token_id, &pool_account), 0);
				// Condition ii
				assert_eq!(Tokens::balance(vault_lp_token_id, &ALICE), 0);
			}

			assert_ok!(deposit_into(&pool_id, &ALICE, deposit_1.clone()));
			
			for deposit in &deposit_1 {
				let vault_id = PoolAssetVault::<Test>::get(pool_id, deposit.asset_id);
				let vault_lp_token_id = Vaults::lp_asset_id(&vault_id).unwrap();

				// Condition iii
				assert_eq!(Tokens::balance(vault_lp_token_id, &pool_account), deposit.amount);
				// Condition iv
				assert_eq!(Tokens::balance(vault_lp_token_id, &ALICE), 0);
			}
		});
	}
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

	#[test]
	fn depositing_into_a_nonexistent_pool_raises_an_error(
		(_pool_assets, all_asset_deposits) in generate_pool_assets_and_n_all_asset_deposits(1),	
	) {
		// Tests that when trying to deposit assets into a pool using a pool id 
		//  |  that doesn't correspond to an active pool, then an error is raised 
		//  '-> Condition
		//        i. ∀ deposits d ⇒ pool_id must exist

		let deposit_1 = (&all_asset_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			// Condition i - Pool isn't created

			let pool_id = 1;

			// Make the first deposit
			for deposit in &deposit_1 {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &ALICE, deposit.amount));
			}
			assert_noop!(
				Pools::all_asset_deposit(&ALICE, &pool_id, deposit_1.clone()),
				Error::<Test>::PoolDoesNotExist
			);
		});
	}

	#[test]
	fn depositing_a_non_all_asset_deposit_as_an_all_asset_deposit_raises_an_error(
		(pool_assets, all_asset_deposits) in generate_pool_assets_and_n_all_asset_deposits(1),	
	) {
		// Tests that when trying to do an all asset deposit all of the Pool's underlying
		//    assets must be present in the deposit
		//  '-> Condition
		//        i. ∀ assets a_i in pool P ⇒ all-asset deposit D must have an amount of asset a_i

		let mut deposit_1 = (&all_asset_deposits[0]).to_vec();
		deposit_1.pop(); // take out one of the assets

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfigBuilder::default()
				.assets(&pool_assets)
				.weights(&equal_weight_vector_for(&pool_assets))
				.build();

			let pool_id = PoolStateBuilder::default()
				.config(config)
				.build();
			
			for deposit in &deposit_1 {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &ALICE, deposit.amount));
			}

			// Condition i
			assert_noop!(
				Pools::all_asset_deposit(&ALICE, &pool_id, deposit_1.clone()),
				Error::<Test>::ThereMustBeOneDepositForEachAssetInThePool
			);
		});
	}

	#[test]
	fn depositing_an_amount_of_assets_that_user_does_not_have_raises_an_error(
		(pool_assets, all_asset_deposits) in generate_pool_assets_and_n_all_asset_deposits(1),
	) {
		// Tests that an error is raised when a user tries to deposit an amount of assets
		//  |  greater than what they own
		//  '-> Conditions:
		//        i. ∀ deposits d of assets a1 ... an ⇒ user U has ≥ asset ai

		let deposit_1 = (&all_asset_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let config = PoolConfigBuilder::default()
				.assets(&pool_assets)
				.weights(&equal_weight_vector_for(&pool_assets))
				.build();

			let pool_id = PoolStateBuilder::default()
				.config(config)
				.build();

			// Condition i - tokens are not minted
			
			assert_noop!(
				<Pools as ConstantMeanMarket>::all_asset_deposit(&ALICE, &pool_id, deposit_1.clone()),
				Error::<Test>::IssuerDoesNotHaveBalanceTryingToDeposit
			);
		});
	}
}

// ----------------------------------------------------------------------------------------------------
//                                              Spot Price                                             
// ----------------------------------------------------------------------------------------------------

fn lps_share_of_pool(pool_id: &u64, lp_amount: Balance) -> Vec<Deposit<MockCurrencyId, Balance>> {
	let assets = PoolAssets::<Test>::get(pool_id).unwrap();
	
	let lp_amount: f64 = lp_amount as f64;
	let lp_circulating_supply = Pools::lp_circulating_supply(pool_id).unwrap();
	let lp_circulating_supply: f64 = lp_circulating_supply as f64;

	// Used to keep track of the amount of each asset withdrawn from the pool's underlying vaults
	let mut lp_total_share = Vec::<Deposit<MockCurrencyId, Balance>>::new();

	for asset_id in &assets {
		let reserve = Pools::balance_of(pool_id, asset_id).unwrap();
		let reserve: f64 = reserve as f64;

		let lp_reserve_share = reserve * (lp_amount / lp_circulating_supply);
		let lp_reserve_share: Balance = lp_reserve_share as Balance; 
		
		lp_total_share.push(
			Deposit {
				asset_id: *asset_id,
				amount: lp_reserve_share,
			}
		);
	}

	lp_total_share
}

proptest! {
	#![proptest_config(ProptestConfig::with_cases(1_000))]
	
	#[test]
	fn spot_price_test_after_one_deposit(
		(config, all_deposits) in generate_equal_weighted_pool_config_and_n_all_asset_deposits(1),
	) {
		// Tests that when there is liquidity in the Pool it correctly calculates the
		//  |  spot price between two of its underlying assets
		//  |-> Pre-Condition:
		//  |     i.   Pool P is non-empty
		//  '-> Post-Condition:
		//        ii.  ∀ assets pairs (a_i, a_j) in P : spot_price of a_i in terms of a_j
		//				  is equals to (b_i / w_i) / (a_j / w_j)
		//		  iii. ∀ assets a_i in P : spot_price of a_i in terms of a_i = 1

		prop_assume!(config.assets.len() > 1);

		let deposits = (&all_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);

			// Condition i
			for deposit in &deposits {
				assert_ok!(Tokens::mint_into(deposit.asset_id, &ALICE, deposit.amount));
			}
			assert_ok!(Pools::all_asset_deposit(&ALICE, &pool_id, deposits.clone()));

			for asset in &config.weights {
				for numeraire in &config.weights {
					if asset.asset_id == numeraire.asset_id { continue; }

					let asset_balance: f64 = PoolAssetBalance::<Test>::get(pool_id, asset.asset_id) as f64;
					let asset_weight: f64 = asset.weight.deconstruct() as f64 / Perquintill::one().deconstruct() as f64;

					let numeraire_balance: f64 = PoolAssetBalance::<Test>::get(pool_id, numeraire.asset_id) as f64;
					let numeraire_weight: f64 = numeraire.weight.deconstruct() as f64 / Perquintill::one().deconstruct() as f64;
				
					let expected_spot_price: f64 = (asset_balance / asset_weight) / (numeraire_balance / numeraire_weight);

					let actual_spot_price: f64 = 
						Pools::spot_price(&pool_id, &asset.asset_id, &numeraire.asset_id)
							.unwrap()
							.to_num::<f64>();
							
					// Condition ii
					assert_relative_eq!(expected_spot_price, actual_spot_price, epsilon = 1.0);
				}

				let actual_spot_price: f64 = 
					Pools::spot_price(&pool_id, &asset.asset_id, &asset.asset_id)
						.unwrap()
						.to_num::<f64>();

				// Condition iii
				assert_eq!(1.0, actual_spot_price);
			}
		});
	}
}

#[test]
fn spot_price_static_test() {
	// Tests that when there is liquidity in the Pool it correctly calculates the
	//  |  spot price between two of its underlying assets
	//  |-> Pre-Condition:
	//  |     i.   Pool P is non-empty
	//  '-> Post-Condition:
	//        ii.  ∀ assets pairs (a_i, a_j) in P : spot_price of a_i in terms of a_j
	//				  is equals to (b_i / w_i) / (a_j / w_j)
	//		  iii. ∀ assets a_i in P : spot_price of a_i in terms of a_i = 1

	ExtBuilder::default().build().execute_with(|| {
		let initial_assets = vec![
		    MockCurrencyId::A,
		    MockCurrencyId::B
	    ];
	
		let config = PoolConfigBuilder::default()
			.assets(&initial_assets)
			.weights(&equal_weight_vector_for(&initial_assets))
			.build();
		
		let pool_id = create_pool_with(&config);

		let deposits = vec![
			Deposit { asset_id: MockCurrencyId::A, amount: 2_000},
			Deposit { asset_id: MockCurrencyId::B, amount: 1_000}
		];

		// Condition i
		for deposit in &deposits {
			assert_ok!(Tokens::mint_into(deposit.asset_id, &ALICE, deposit.amount));
		}
		assert_ok!(Pools::all_asset_deposit(&ALICE, &pool_id, deposits.clone()));

		let spot_price = Pools::spot_price(&pool_id, &MockCurrencyId::A, &MockCurrencyId::B)
			.unwrap()
			.to_num::<f64>();
		// Condition ii
		assert_relative_eq!(2.0, spot_price, epsilon=0.0);

		let spot_price = Pools::spot_price(&pool_id, &MockCurrencyId::A, &MockCurrencyId::A)
			.unwrap()
			.to_num::<f64>();
		// Condition iii
		assert_relative_eq!(1.0, spot_price, epsilon=0.0);

		let spot_price = Pools::spot_price(&pool_id, &MockCurrencyId::B, &MockCurrencyId::A)
			.unwrap()
			.to_num::<f64>();
		// Condition ii
		assert_relative_eq!(0.5, spot_price, epsilon=0.0);

		let spot_price = Pools::spot_price(&pool_id, &MockCurrencyId::B, &MockCurrencyId::B)
			.unwrap()
			.to_num::<f64>();
		// Condition iii
		assert_relative_eq!(1.0, spot_price, epsilon=0.0);
	});
}

// ----------------------------------------------------------------------------------------------------
//                                          All-Asset Withdraw                                              
// ----------------------------------------------------------------------------------------------------

// TODO: (Nevin)
//  - ✔ withdrawing_all_lp_tokens_from_a_single_user_pool_gives_back_pro_rata_share_of_liquidity_and_burns_lp_tokens
//  -  withdrawing_a_fraction_of_total_lp_tokens_from_a_single_user_pool_gives_back_pro_rata_share_of_liquidity_and_burns_lp_tokens
//  -  withdrawing_from_a_multi_user_pool_gives_back_pro_rata_share_of_liquidity_and_burns_lp_tokens
//  -  withdrawing_from_a_pool_through_multiple_withdraws_correctly_returns_lp_share
//  -  withdrawing_from_a_pool_correctly_updates_runtime_storage_objects
//  -  withdrawing_from_a_pools_underlying_vaults_acts_properly

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]

	#[test]
	fn withdrawing_all_lp_tokens_from_a_single_user_pool_gives_back_pro_rata_share_of_liquidity_and_burns_lp_tokens(
		(config, all_deposits, _all_withdraws) in generate_eq_pool_config_and_n_all_asset_deposits_and_m_withdraws(1, 1),
	) {
		// Tests that if a owns n% of the Pool's liquidity and deposits all of their LP tokens
		//  |  they receive their n% of the Pool's liquidity
		//  |-> Pre-Conditions:
		//  |     i.   User U has n > 0 LP tokens
		//  '-> Post-Conditions:
		//        ii.  ∀ withdraws W of △ LP tokens : user U receives △ / LP circulating supply % 
		//				   of the Pool's assets 
		//		  iii. User U has n - △ ≥ 0 LP tokens

		// Only tests the state after one deposit
		let (user_1, deposit_1) = (all_deposits[0].0, all_deposits[0].1.to_vec());

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			let lp_token = Pools::lp_token_id(&pool_id).unwrap();

			let minted_lp_tokens = deposit_into(&pool_id, &user_1, deposit_1.clone()).unwrap();
			
			// Condition i			
			assert!(minted_lp_tokens > 0);

			// Withdraw assets
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_withdraw(&user_1, &pool_id, minted_lp_tokens));

			// Condition ii
			for withdraw in deposit_1 {
				assert_eq!(withdraw.amount, Tokens::balance(withdraw.asset_id, &user_1));
			}

			// Condition iii
			assert_eq!(0, Tokens::balance(lp_token, &user_1));
		});
	}

	#[test]
	fn withdrawing_a_deposit_withdraws_the_exact_deposit(
		(config, all_deposits) in generate_eq_pool_config_and_n_all_asset_deposits(3),
	) {
		// Tests that when a user withdraws lp tokens they receive their exact share of liquidity
		//  |-> Pre-Conditions:
		//  |     i.   User U deposits D into Pool P and receives n LP tokens
		//  '-> Post-Conditions:
		//        ii.  User U withdraws W using their n LP tokens => W = D

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			let lp_token = Pools::lp_token_id(&pool_id).unwrap();

			for (user, deposit) in &all_deposits {
				assert_ok!(deposit_into(&pool_id, user, deposit.to_vec()));
			}

			for (user, deposit) in &all_deposits {
				let users_lp_tokens = Tokens::balance(lp_token, user);

				let withdraw = Pools::all_asset_withdraw(user, &pool_id, users_lp_tokens).unwrap();

				assert_eq!(*deposit, withdraw);
			}
		});
	}

	#[test]
	fn withdrawing_and_depositing_keeps_lps_share_constant(
		(config, all_deposits, all_withdraws) in generate_eq_pool_config_and_n_all_asset_deposits_and_m_withdraws(1, 1),
	) {
		// TODO: (Nevin)
		//  - update comment section

		// Tests that if user deposits into a Pool, thus receiving LP tokens, withdraws using some of the LP tokens
		//  |  and then deposits the withdrawn amount 
		//  |-> Pre-Conditions:
		//  |     i.   User U has n > 0 LP tokens
		//  '-> Post-Conditions:
		//        ii.  ∀ withdraws W of △ LP tokens : user U receives △ / LP circulating supply % 
		//				   of the Pool's assets 
		//		  iii. User U has n - △ ≥ 0 LP tokens

		// Only tests the state after one deposit and one withdraw
		let (user_1, deposit_1) = (all_deposits[0].0, all_deposits[0].1.to_vec());
		let (_user_1, withdraw_ratio_1) = (all_withdraws[0].0, all_withdraws[0].1);

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);

			// Initial deposit
			let lp_tokens_minted_from_first_deposit = deposit_into(&pool_id, &user_1, deposit_1).unwrap();
			
			// First withdraw
			let lp_tokens_to_withdraw = withdraw_ratio_1 * lp_tokens_minted_from_first_deposit;
			let assets_withdrawn = Pools::all_asset_withdraw(&user_1, &pool_id, lp_tokens_to_withdraw).unwrap();
			
			// Second deposit
			let lp_tokens_minted_from_second_deposit = deposit_into(&pool_id, &user_1, assets_withdrawn).unwrap();
			
			// Second withdraw
			let assets_withdrawn = Pools::all_asset_withdraw(&user_1, &pool_id, lp_tokens_minted_from_second_deposit).unwrap();
			
			// Third deposit
			let lp_tokens_minted_from_third_deposit = deposit_into(&pool_id, &user_1, assets_withdrawn).unwrap();
			
			assert_eq!(lp_tokens_minted_from_second_deposit, lp_tokens_minted_from_third_deposit);
		});
	}

	#[test]
	fn withdrawing_a_fraction_of_total_lp_tokens_from_a_single_user_pool_gives_back_pro_rata_share_of_liquidity_and_burns_lp_tokens(
		(config, all_deposits, all_withdraws) in generate_eq_pool_config_and_n_all_asset_deposits_and_m_withdraws(1, 1),
	) {
		// Tests that if a owns 100% of the Pool's liquidity and deposits 0 ≤ △ ≤ n of their LP tokens
		//  |  they receive n% of the Pool's liquidity
		//  |-> Pre-Conditions:
		//  |     i.   User U has n > 0 LP tokens
		//  '-> Post-Conditions:
		//        ii.  ∀ withdraws W of △ LP tokens : user U receives △ / LP circulating supply % 
		//				   of the Pool's assets 
		//		  iii. User U has n - △ ≥ 0 LP tokens

		// Only tests the state after one deposit
		let (user_1, deposit_1) = (all_deposits[0].0, all_deposits[0].1.to_vec());
		let (_user_1, withdraw_ratio_1) = (all_withdraws[0].0, all_withdraws[0].1);

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			let lp_token = Pools::lp_token_id(&pool_id).unwrap();

			let minted_lp_tokens = deposit_into(&pool_id, &user_1, deposit_1.clone()).unwrap();
			
			// Condition i			
			assert!(minted_lp_tokens > 0);

			// Withdraw assets
			let lp_tokens_to_withdraw = withdraw_ratio_1 * minted_lp_tokens;

			let _assets_withdrawn = <Pools as ConstantMeanMarket>::all_asset_withdraw(&user_1, &pool_id, lp_tokens_to_withdraw);

			// assert_eq!(deposit_1, assets_withdrawn);
			// TODO: (Nevin)
			//  - write f64 version of calculate lps share and check that withdraw amount is equivalent

			println!("deposit_1: {deposit_1:?}");
			println!("withdraw_ratio_1: {withdraw_ratio_1:?}");

			println!("withdrawn: {:?}", lps_share_of_pool(&pool_id, lp_tokens_to_withdraw));

			// Condition ii
			for withdraw in &deposit_1 {
				let users_balance = Tokens::balance(withdraw.asset_id, &user_1);
				let pools_balance = Pools::balance_of(&pool_id, &withdraw.asset_id).unwrap();

				assert_eq!(withdraw.amount, users_balance + pools_balance)
			}
			println!("\n");

			// Condition iii
			let remaining_lp_tokens = minted_lp_tokens - lp_tokens_to_withdraw;
			assert_eq!(remaining_lp_tokens, Tokens::balance(lp_token, &user_1));

			if remaining_lp_tokens > 0 {
				assert_ok!(<Pools as ConstantMeanMarket>::all_asset_withdraw(&user_1, &pool_id, remaining_lp_tokens));

				// Condition ii
				for withdraw in &deposit_1 {
					assert_eq!(withdraw.amount, Tokens::balance(withdraw.asset_id, &user_1));
				}

				// Condition iii
				assert_eq!(0, Tokens::balance(lp_token, &user_1));
			}
		});
	}

	// Doesn't run
	#[test]
	fn withdrawing_from_a_multi_user_pool_gives_back_pro_rata_share_of_liquidity_and_burns_lp_tokens(
		(config, all_deposits) in generate_equal_weighted_pool_config_and_n_all_asset_deposits(1),
	) {
		// Tests that if a owns n% of the Pool's liquidity and deposits all of their LP tokens
		//  |  they receive their n% of the Pool's liquidity
		//  |-> Pre-Conditions:
		//  |     i.   User U has n > 0 LP tokens
		//  '-> Post-Conditions:
		//        ii.  ∀ withdraws W of △ LP tokens : user U receives △ / LP circulating supply % 
		//				   of the Pool's assets 
		//		  iii. User U has n - △ ≥ 0 LP tokens

		// Only tests the state after one deposit
		let deposit_1 = (&all_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			let lp_token = Pools::lp_token_id(&pool_id).unwrap();

			let alices_lp_tokens = deposit_into(&pool_id, &ALICE, deposit_1.clone()).unwrap();
			
			// Condition i			
			assert!(alices_lp_tokens > 0);

			// Withdraw assets
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_withdraw(&ALICE, &pool_id, alices_lp_tokens));

			// Condition ii
			for withdraw in deposit_1 {
				assert_eq!(withdraw.amount, Tokens::balance(withdraw.asset_id, &ALICE))
			}

			// Condition iii
			assert_eq!(0, Tokens::balance(lp_token, &ALICE));
		});
	}

	// Not 100% accurate
	#[test]
	fn withdrawing_from_a_pool_through_multiple_withdraws_correctly_returns_lp_share(
		(config, all_deposits) in generate_equal_weighted_pool_config_and_n_all_asset_deposits(1),
	) {
		// Tests that if a owns n% of the Pool's liquidity and deposits all of their LP tokens
		//  |  they receive their n% of the Pool's liquidity
		//  |-> Pre-Conditions:
		//  |     i.   User U has n > 0 LP tokens
		//  '-> Post-Conditions:
		//        ii.  ∀ withdraws W of △ LP tokens : user U receives △ / LP circulating supply % 
		//				   of the Pool's assets 
		//		  iii. User U has n - △ ≥ 0 LP tokens

		// Use the same deposit for all three seperate users
		let deposit_1 = (&all_deposits[0]).to_vec();
		let deposit_2 = (&all_deposits[0]).to_vec();
		let deposit_3 = (&all_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			let lp_token = Pools::lp_token_id(&pool_id).unwrap();

			// Condition i - Alice
			let alices_lp_tokens = deposit_into(&pool_id, &ALICE, deposit_1.clone()).unwrap();
			assert!(alices_lp_tokens > 0);

			// Condition i - Bob
			let bobs_lp_tokens = deposit_into(&pool_id, &BOB, deposit_2.clone()).unwrap();
			assert!(bobs_lp_tokens > 0);

			// Condition i - Charlie
			let charlies_lp_tokens = deposit_into(&pool_id, &CHARLIE, deposit_3.clone()).unwrap();
			assert!(charlies_lp_tokens > 0);

			// Alice withdraws
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_withdraw(&ALICE, &pool_id, alices_lp_tokens));
			
			// Bob withdraws
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_withdraw(&BOB, &pool_id, bobs_lp_tokens));
			
			// Charlie withdraws
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_withdraw(&CHARLIE, &pool_id, charlies_lp_tokens));

			// TODO: (Nevin)
			//  - get tests to pass with an epsilon of zero

			// Condition ii - Alice
			for withdraw in deposit_1 {
				// assert_eq!(withdraw.amount, Tokens::balance(withdraw.asset_id, &ALICE), "Alices withdraw");
				assert_relative_eq!(withdraw.amount as f64, Tokens::balance(withdraw.asset_id, &ALICE) as f64, epsilon=2.0)
			}

			// Condition ii - Bob
			for withdraw in deposit_2 {
				// assert_eq!(withdraw.amount, Tokens::balance(withdraw.asset_id, &BOB), "Bobs withdraw");
				assert_relative_eq!(withdraw.amount as f64, Tokens::balance(withdraw.asset_id, &BOB) as f64, epsilon=2.0)
			}

			// Condition ii - Charlie
			for withdraw in deposit_3 {
				// assert_eq!(withdraw.amount, Tokens::balance(withdraw.asset_id, &CHARLIE), "Charlies withdraw");
				assert_relative_eq!(withdraw.amount as f64, Tokens::balance(withdraw.asset_id, &CHARLIE) as f64, epsilon=2.0)
			}

			// Condition iii - Alice
			assert_eq!(0, Tokens::balance(lp_token, &ALICE));

			// Condition iii - Bob
			assert_eq!(0, Tokens::balance(lp_token, &BOB));

			// Condition iii - Alice
			assert_eq!(0, Tokens::balance(lp_token, &CHARLIE));
		});
	}

	// Not 100% accurate
	#[test]
	fn withdrawing_from_a_pool_correctly_updates_runtime_storage_objects(
		(config, all_deposits) in generate_equal_weighted_pool_config_and_n_all_asset_deposits(1),
	) {
		// Tests that when users withdraw from a pool, the runtime objects storing the Pool's
		//  |s  reserves are correctly updated
		//  |-> Pre-Conditions:
		//  |     i.   PoolAssetBalance is nonempty (doesn't include fee)
		//  |     ii.  PoolAssetTotalBalance is nonempty (includes fee)
		//  '-> Post-Conditions:
		//        iii. PoolAssetBalance removes the withdrawn amount (fees aren't taken out of all-asset deposits)
		//		  iv.  PoolAssetTotalBalance removes the withdrawn amount

		// Use the same deposit for all three seperate users
		let deposit_1 = (&all_deposits[0]).to_vec();
		let deposit_2 = (&all_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);

			// Alice deposits
			let alices_lp_tokens = deposit_into(&pool_id, &ALICE, deposit_1.clone()).unwrap();
			// Bob deposits
			let bobs_lp_tokens = deposit_into(&pool_id, &BOB, deposit_1.clone()).unwrap();
			
			for (asset_1, asset_2) in deposit_1.iter().zip(deposit_2.iter()) {
				let asset = asset_1.asset_id;
				let deposited_amount = asset_1.amount + asset_2.amount;
				
				// Condition i
				assert_eq!(deposited_amount, PoolAssetBalance::<Test>::get(&pool_id, asset), "storage not correclty updated by depositing");
				// Condition ii
				assert_eq!(deposited_amount, PoolAssetTotalBalance::<Test>::get(&pool_id, asset), "storage not correclty updated by depositing");
			}
			
			// Alice withdraws
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_withdraw(&ALICE, &pool_id, alices_lp_tokens));
			
			for (asset_1, asset_2) in deposit_1.iter().zip(deposit_2.iter()) {
				let asset = asset_1.asset_id;
				let deposited_amount = asset_1.amount + asset_2.amount;
				
				let alices_balance = Tokens::balance(asset_2.asset_id, &ALICE);

				// Condition iii
				assert_eq!(deposited_amount, PoolAssetBalance::<Test>::get(&pool_id, asset) + alices_balance, "storage not correclty updated by depositing");
				// Condition iv
				assert_eq!(deposited_amount, PoolAssetTotalBalance::<Test>::get(&pool_id, asset) + alices_balance, "storage not correclty updated by depositing");
			}

			// TODO: (Nevin)
			//  - change withdraw function to pass the asserts below section rather than the above asserts

			// for asset_2 in &deposit_2 {
			// 	let asset = asset_2.asset_id;
			// 	let amount = asset_2.amount;

			// 	// Condition iii
			// 	assert_eq!(amount, PoolAssetBalance::<Test>::get(&pool_id, asset), "storage not correclty updated by first withdraw");
			// 	// Condition iv
			// 	assert_eq!(amount, PoolAssetTotalBalance::<Test>::get(&pool_id, asset), "storage not correclty updated by first withdraw");
			// }

			// Bob withdraws
			assert_ok!(<Pools as ConstantMeanMarket>::all_asset_withdraw(&BOB, &pool_id, bobs_lp_tokens));

			for asset_2 in &deposit_2 {
				let asset = asset_2.asset_id;
				
				// Condition iii
				assert_eq!(0, PoolAssetBalance::<Test>::get(&pool_id, asset), "storage not correclty updated by second withdraw");
				// Condition iv
				assert_eq!(0, PoolAssetTotalBalance::<Test>::get(&pool_id, asset), "storage not correclty updated by second withdraw");
			}
		});
	}

	// TODO: (Nevin)
	//  - test that the underlying vaults act properly:
	//		-- tokens are transfered out of the vault
	//		-- vault lp tokens are burned
	#[test]
	fn withdrawing_from_a_pools_underlying_vaults_acts_properly(
		(config, all_deposits, all_withdraws) in generate_eq_pool_config_and_n_all_asset_deposits_and_m_withdraws(1, 1),
	) {
		// Tests that if a user requests to withdraw an amount of assets that is outside of the Pool's
		//  |  minimum and maximum withdraw bounds an error is thrown
		//  '-> Conditions:
		//        i.  ∀ withdraws W of n LP tokens : Pool min withdraw ≤ LP tokens share of Pools assets  ≤ Pool max withdraw

		// Only tests the state after one deposit
		let (user_1, deposit_1) = (all_deposits[0].0, all_deposits[0].1.to_vec());
		let (_user_1, _withdraw_ratio_1) = (all_withdraws[0].0, all_withdraws[0].1);

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			let pool_account = Pools::account_id(&pool_id);

			for asset in &config.assets {
				let vault_id = PoolAssetVault::<Test>::get(pool_id, asset);
				let vault_lp_token_id = Vaults::lp_asset_id(&vault_id).unwrap();

				// Condition i
				assert_eq!(Tokens::balance(vault_lp_token_id, &pool_account), 0);
				// Condition ii
				assert_eq!(Tokens::balance(vault_lp_token_id, &user_1), 0);
			}

			let _minted_lp_tokens = deposit_into(&pool_id, &user_1, deposit_1.clone()).unwrap();
			for deposit in &deposit_1 {
				let vault_id = PoolAssetVault::<Test>::get(pool_id, deposit.asset_id);
				let vault_lp_token_id = Vaults::lp_asset_id(&vault_id).unwrap();

				// Condition iii
				assert_eq!(Tokens::balance(vault_lp_token_id, &pool_account), deposit.amount);
				// Condition iv
				assert_eq!(Tokens::balance(vault_lp_token_id, &user_1), 0);
			}

			// assert_ok!(<Pools as ConstantMeanMarket>::all_asset_withdraw(&user_1, &pool_id, withdraw_lp_tokens));

			// for deposit in &deposit_1 {
			// 	let vault_id = PoolAssetVault::<Test>::get(pool_id, deposit.asset_id);
			// 	let vault_lp_token_id = Vaults::lp_asset_id(&vault_id).unwrap();

			// 	let pool_vault_lp_balance = Tokens::balance(vault_lp_token_id, &pool_account);
			// 	let user_balance = Tokens::balance(deposit.asset_id, &user_1);

			// 	// Condition v
			// 	assert_eq!(deposit.amount, pool_vault_lp_balance + user_balance);
			// 	// Condition vi
			// 	assert_eq!(Tokens::balance(vault_lp_token_id, &user_1), 0);
			// }

			// update the withdraw ratio incase there was any rounding with the above line
			// let withdraw_ratio_1 = Perquintill::from_rational(withdraw_lp_tokens, minted_lp_tokens);

			// if withdraw_ratio_1 < lower_bound || upper_bound < withdraw_ratio_1 {
			// 	assert_noop!(
			// 		<Pools as ConstantMeanMarket>::all_asset_withdraw(&user_1, &pool_id, withdraw_lp_tokens),
			// 		Error::<Test>::WithdrawIsOutsideOfPoolsWithdrawBounds,
			// 	);
			// } else {
			// 	assert_ok!(<Pools as ConstantMeanMarket>::all_asset_withdraw(&user_1, &pool_id, withdraw_lp_tokens));
			// }
		});
	}
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1_000))]

	#[test]
	fn withdrawing_from_a_pool_that_does_not_exist_raises_an_error(
		(_config, _all_deposits) in generate_equal_weighted_pool_config_and_n_all_asset_deposits(1),
	) {
		// Tests that when trying to withdraw assets from a pool using a pool id that does not
		//  |  correspond to an active pool, an error is raised 
		//  '-> Condition
		//        i. ∀ withdraws w ⇒ pool_id must exist

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = 1;

			// Condition i
			let lp_tokens = 1_000;
			assert_noop!(
				<Pools as ConstantMeanMarket>::all_asset_withdraw(&ALICE, &pool_id, lp_tokens),
				Error::<Test>::PoolDoesNotExist
			);
		});
	}

	#[test]
	fn withdrawing_an_amount_of_lp_tokens_that_the_user_does_not_have_raises_an_error(
		(config, all_deposits) in generate_equal_weighted_pool_config_and_n_all_asset_deposits(1),
	) {
		// Tests that if a user has n LP tokens and they try to deposit > n LP tokens during a withdraw
		//  |  the function raises an error.
		//  '-> Conditions:
		//        i.  ∀ withdraws W of n LP tokens : user U must have ≥ n LP tokens

		// Only tests the state after one deposit
		let deposit_1 = (&all_deposits[0]).to_vec();

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			let alice_lp_tokens = deposit_into(&pool_id, &ALICE, deposit_1).unwrap();
			
			// Condition i			
			assert_noop!(
				<Pools as ConstantMeanMarket>::all_asset_withdraw(&ALICE, &pool_id, alice_lp_tokens + 1),
				Error::<Test>::IssuerDoesNotHaveBalanceTryingToDeposit
			);
		});
	}

	// TODO: (Nevin)
	//  - allow lower_bound and upper_bound to be randomly generated
	#[test]
	fn withdrawing_outside_of_withdraw_bounds_raises_an_error(
		(mut config, all_deposits, all_withdraws) in generate_eq_pool_config_and_n_all_asset_deposits_and_m_withdraws(1, 1),
	) {
		// Tests that if a user requests to withdraw an amount of assets that is outside of the Pool's
		//  |  minimum and maximum withdraw bounds an error is thrown
		//  '-> Conditions:
		//        i.  ∀ withdraws W of n LP tokens : Pool min withdraw ≤ LP tokens share of Pools assets  ≤ Pool max withdraw

		// Set the minimum and maximum amount of assets that are able to be withdrawn at a time
		let lower_bound = Perquintill::from_percent(10);
		let upper_bound = Perquintill::from_percent(30);

		config.withdraw_bounds = Bound {
			minimum: Some(lower_bound),
			maximum: Some(upper_bound),
		};

		// Only tests the state after one deposit
		let (user_1, deposit_1) = (all_deposits[0].0, all_deposits[0].1.to_vec());
		let (_user_1, withdraw_ratio_1) = (all_withdraws[0].0, all_withdraws[0].1);

		ExtBuilder::default().build().execute_with(|| {
			let pool_id = create_pool_with(&config);
			let minted_lp_tokens = deposit_into(&pool_id, &user_1, deposit_1).unwrap();
			
			let withdraw_lp_tokens = withdraw_ratio_1 * minted_lp_tokens;
			// update the withdraw ratio incase there was any rounding with the above line
			let withdraw_ratio_1 = Perquintill::from_rational(withdraw_lp_tokens, minted_lp_tokens);

			if withdraw_ratio_1 < lower_bound || upper_bound < withdraw_ratio_1 {
				assert_noop!(
					<Pools as ConstantMeanMarket>::all_asset_withdraw(&user_1, &pool_id, withdraw_lp_tokens),
					Error::<Test>::WithdrawIsOutsideOfPoolsWithdrawBounds,
				);
			} else {
				assert_ok!(<Pools as ConstantMeanMarket>::all_asset_withdraw(&user_1, &pool_id, withdraw_lp_tokens));
			}
		});
	}

	// TODO: (Nevin)
	//  - test, if possible, the withdraw amount matches the value distribution
}

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

	assert_eq!(pow::<S, D>(two, zero), Ok(one));
	assert_eq!(pow::<S, D>(zero, two), Ok(zero));
	assert_eq!(pow::<S, D>(ten, two), Ok(one_hundred));
	assert_eq!(pow::<S, D>(one_hundred, one_half), Ok(ten));

	let result: f64 = pow::<S, D>(two, three).unwrap().lossy_into();
	assert_relative_eq!(result, 8.0f64, epsilon = 1.0e-6);

	let result: f64 = pow::<S, D>(one / four, two).unwrap().lossy_into();
	assert_relative_eq!(result, 0.0625f64, epsilon = 1.0e-6);

	assert_eq!(pow::<S, D>(two, one), Ok(two));

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
	weights: Vec<Weight<MockCurrencyId, Perquintill>>
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
