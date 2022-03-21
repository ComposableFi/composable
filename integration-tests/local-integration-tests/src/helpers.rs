use common::{xcmp::BaseXcmWeight, AccountId, Balance, MultiExistentialDeposits};
use cumulus_primitives_core::ParaId;

use num_traits::One;
use orml_traits::GetByKey;
use primitives::currency::CurrencyId;
use sp_runtime::traits::AccountIdConversion;
use support::log;

use crate::{env_logger_init, kusama_test_net::SIBLING_PARA_ID, prelude::*};

// TODO: make marco of it
pub fn simtest() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();
}

/// create account ids from test paraid
pub fn para_account_id(id: u32) -> AccountId {
	ParaId::from(id).into_account()
}

/// under ED, but above Weight
pub fn under_existential_deposit(asset_id: LocalAssetId, _instruction_count: usize) -> Balance {
	MultiExistentialDeposits::get(&asset_id).saturating_sub(Balance::one())
}

/// dumps events for debugging
#[allow(dead_code)]
pub fn dump_events() {
	sibling_runtime::System::events().iter().for_each(|x| {
		log::info!("{:?}", x);
	});
}

/// dumps events for debugging
#[allow(dead_code)]
pub fn relay_dump_events() {
	kusama_runtime::System::events().iter().for_each(|x| {
		log::info!("{:?}", x);
	});
}

pub fn sibling_account() -> AccountId {
	polkadot_parachain::primitives::Sibling::from(SIBLING_PARA_ID).into_account()
}

/// assert amount is supported deposit amount and is above it
pub fn assert_above_deposit(asset_id: CurrencyId, amount: Balance) -> Balance {
	assert!(MultiExistentialDeposits::get(&asset_id) <= amount);
	amount
}

/// weigh enough to handle any XCMP message
pub fn enough_weigth() -> u128 {
	let this_liveness_native_amount = BaseXcmWeight::get() as u128 +
		100 * UnitWeightCost::get() as Balance * MaxInstructions::get() as Balance;
	this_liveness_native_amount
}
