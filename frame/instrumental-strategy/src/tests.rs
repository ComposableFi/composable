#[allow(unused_imports)]

use crate::pallet::Error;
use crate::mock::runtime::{
    ExtBuilder, InstrumentalStrategy, MockRuntime,
};
use crate::mock::strategies::*;

use frame_support::{assert_ok, assert_noop};

#[test]
fn test_whitelist_strategy() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(InstrumentalStrategy::whitelist_strategy(PABLO_STRATEGY.account_id()));
    });
}

#[test]
fn test_whitelisting_a_strategy_twice_results_in_an_error() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(InstrumentalStrategy::whitelist_strategy(PABLO_STRATEGY.account_id()));
        assert_noop!(
            InstrumentalStrategy::whitelist_strategy(PABLO_STRATEGY.account_id()),
            Error::<MockRuntime>::StrategyAlreadyWhitelisted
        );
    });
}