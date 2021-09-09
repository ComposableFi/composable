use crate::{
	mock::{new_test_ext, AccountId, Lending, Oracle, Origin, Vault},
	MarketIndex,
};
use composable_traits::{
	lending::{MarketConfigInput, NormalizedCollateralFactor},
	rate_model::*,
	vault::{Deposit, VaultConfig},
};
use frame_support::assert_ok;
use hex_literal::hex;
use sp_runtime::{traits::Zero, FixedPointNumber, Percent, Perquintill};
use sp_std::collections::btree_map::BTreeMap;

#[test]
fn account_id_should_work() {
	new_test_ext().execute_with(|| {
		let market_id = MarketIndex::new(0);
		assert_eq!(
			Lending::account_id(&market_id),
			AccountId::from_raw(hex!(
				"6d6f646c4c656e64696e67210000000000000000000000000000000000000000"
			))
		);
	})
}

#[test]
fn test_calc_utilization_ratio() {
	// 50% borrow
	assert_eq!(
		Lending::calc_utilization_ratio(&1, &1, &0).unwrap(),
		Ratio::saturating_from_rational(50, 100)
	);
	assert_eq!(
		Lending::calc_utilization_ratio(&100, &100, &0).unwrap(),
		Ratio::saturating_from_rational(50, 100)
	);
	// no borrow
	assert_eq!(Lending::calc_utilization_ratio(&1, &0, &0).unwrap(), Ratio::zero());
	// full borrow
	assert_eq!(
		Lending::calc_utilization_ratio(&0, &1, &0).unwrap(),
		Ratio::saturating_from_rational(100, 100)
	);
}
#[test]
fn test_create_market() {
	new_test_ext().execute_with(|| {
		// create vaults
		let eth_asset_id = 0;
		let usdt_asset_id = 1;
		let strategy_account_id = AccountId::from_raw(hex!(
			"6d6f646c4c656e64696e67210000000000000000000000000000000000000000"
		));
		let mut strategy = BTreeMap::new();
		strategy.insert(strategy_account_id, Perquintill::from_percent(90));
		let manager_account_id = AccountId::from_raw(hex!(
			"6d6f646c4c656e64696e67210000000000000000000000000000000000000000"
		));
		let v_eth = Vault::do_create_vault(
			Deposit::Existential,
			VaultConfig {
				asset_id: eth_asset_id,
				reserved: Perquintill::from_percent(10),
				manager: manager_account_id,
				strategies: strategy.clone(),
			},
		);
		assert_ok!(v_eth);
		let v_usdt = Vault::do_create_vault(
			Deposit::Existential,
			VaultConfig {
				asset_id: usdt_asset_id,
				reserved: Perquintill::from_percent(10),
				manager: manager_account_id,
				strategies: strategy,
			},
		);
		assert_ok!(v_usdt);
		let market_config = MarketConfigInput {
			manager: manager_account_id,
			reserve_factor: Perquintill::from_percent(8),
			collateral_factor: NormalizedCollateralFactor::saturating_from_rational(150, 100),
		};
		// Note: this market uses ConstantOracle as defined in src/mock.rs
		// create market
		let market = Lending::create(v_eth.unwrap().0, v_usdt.unwrap().0, market_config);
		assert_ok!(market);
	})
}
