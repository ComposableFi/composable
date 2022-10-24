use common::{multi_existential_deposits, xcmp::BaseXcmWeight, AccountId, Balance};
use composable_traits::{currency::AssetExistentialDepositInspect, xcm::assets::AssetRatioInspect};
use cumulus_primitives_core::ParaId;

use frame_support::log;
use primitives::currency::CurrencyId;
use sp_runtime::traits::AccountIdConversion;

use crate::{env_logger_init, kusama_test_net::SIBLING_PARA_ID, prelude::*};

// TODO: make macro of it
pub fn simtest() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();
}

/// create account ids from test para id
pub fn para_account_id(id: u32) -> AccountId {
	ParaId::from(id).into_account_truncating()
}

/// under ED, but above Weight
pub fn under_existential_deposit<
	AssetsRegistry: AssetRatioInspect<AssetId = CurrencyId>
		+ AssetExistentialDepositInspect<AssetId = CurrencyId, Balance = Balance>,
>(
	asset_id: LocalAssetId,
	_instruction_count: usize,
) -> Balance {
	multi_existential_deposits::<AssetsRegistry>(&asset_id) / 2_u128
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
	polkadot_parachain::primitives::Sibling::from(SIBLING_PARA_ID).into_account_truncating()
}

/// assert amount is supported deposit amount and is above it
pub fn assert_above_deposit<
	AssetsRegistry: AssetRatioInspect<AssetId = CurrencyId>
		+ AssetExistentialDepositInspect<AssetId = CurrencyId, Balance = Balance>,
>(
	asset_id: CurrencyId,
	amount: Balance,
) -> Balance {
	assert!(multi_existential_deposits::<AssetsRegistry>(&asset_id) <= amount);
	amount
}

/// weigh enough to handle any XCMP message
pub fn enough_weight() -> u128 {
	BaseXcmWeight::get() as u128 +
		100 * UnitWeightCost::get() as Balance * MaxInstructions::get() as Balance
}
