use common::{xcmp::BaseXcmWeight, AccountId, Balance, NativeExistentialDeposit, PriceConverter};
use composable_traits::{oracle::MinimalOracle, xcm::assets::AssetRatioInspect};
use cumulus_primitives_core::ParaId;

use frame_support::log;
use primitives::currency::CurrencyId;
use sp_runtime::traits::AccountIdConversion;

use crate::{env_logger_init, prelude::*};

// TODO: make macro of it
pub fn simtest() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();
}

/// create account ids from test para id
pub fn para_account_id(id: u32) -> AccountId {
	ParaId::from(id).into_account_truncating()
}

pub fn sibling_account(para_id: u32) -> AccountId {
	polkadot_parachain::primitives::Sibling::from(para_id).into_account_truncating()
}

pub fn buy_execution_unlimited<Call>(fees: impl Into<MultiAsset>) -> Instruction<Call> {
	BuyExecution { fees: fees.into(), weight_limit: Unlimited }
}

pub fn deposit_all_one<Call>(beneficiary: impl Into<MultiLocation>) -> Instruction<Call> {
	DepositAsset { assets: All.into(), max_assets: 1, beneficiary: beneficiary.into() }
}

/// under ED, but above Weight
pub fn under_existential_deposit<AssetsRegistry: AssetRatioInspect<AssetId = CurrencyId>>(
	asset_id: LocalAssetId,
	_instruction_count: usize,
) -> Balance {
	PriceConverter::<AssetsRegistry>::get_price_inverse(asset_id, NativeExistentialDeposit::get())
		.unwrap() /
		Balance::from(2_u128)
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

/// assert amount is supported deposit amount and is above it
pub fn assert_above_deposit<AssetsRegistry: AssetRatioInspect<AssetId = CurrencyId>>(
	asset_id: CurrencyId,
	amount: Balance,
) -> Balance {
	assert!(
		PriceConverter::<AssetsRegistry>::get_price_inverse(
			asset_id,
			NativeExistentialDeposit::get()
		)
		.unwrap() <= amount
	);
	amount
}

/// weigh enough to handle any XCMP message
pub fn enough_weight() -> u128 {
	let this_liveness_native_amount = BaseXcmWeight::get() as u128 +
		100 * UnitWeightCost::get() as Balance * MaxInstructions::get() as Balance;
	this_liveness_native_amount
}
