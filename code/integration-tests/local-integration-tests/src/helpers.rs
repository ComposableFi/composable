use common::{fees::NATIVE_EXISTENTIAL_DEPOSIT, xcmp::BaseXcmWeight, AccountId, Balance};
use cumulus_primitives_core::ParaId;

use primitives::currency::CurrencyId;
use sp_runtime::traits::AccountIdConversion;

use crate::{env_logger_init, kusama_test_net::KusamaRelay, prelude::*};

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
		PriceConverter::<AssetsRegistry>::to_asset_balance(NATIVE_EXISTENTIAL_DEPOSIT, asset_id,)
			.unwrap() <= amount
	);
	amount
}

/// weigh enough to handle any XCMP message
pub fn enough_weight() -> u128 {
	BaseXcmWeight::get() as u128 +
		100 * UnitWeightCost::get() as Balance * MaxInstructions::get() as Balance
}

pub fn mint_relay_native_on_parachain(amount: Balance, to: &AccountId, para_id: u32) {
	KusamaRelay::execute_with(|| {
		use kusama_runtime::*;
		let _ = <Balances as frame_support::traits::Currency<_>>::deposit_creating(to, amount);
		XcmPallet::reserve_transfer_assets(
			RuntimeOrigin::signed(to.to_owned()),
			Box::new(Parachain(para_id).into().into()),
			Box::new(
				Junction::AccountId32 { id: to.to_owned().into(), network: NetworkId::Any }
					.into()
					.into(),
			),
			Box::new((Here, amount).into()),
			0,
		)
		.unwrap();
	});
}

#[allow(dead_code)]
pub fn mint_relay_native_on_common_good(amount: Balance, to: &AccountId, para_id: u32) {
	KusamaRelay::execute_with(|| {
		use kusama_runtime::*;
		let _ = <Balances as frame_support::traits::Currency<_>>::deposit_creating(to, amount);
		XcmPallet::teleport_assets(
			RuntimeOrigin::signed(to.to_owned()),
			Box::new(Parachain(para_id).into().into()),
			Box::new(
				Junction::AccountId32 { id: to.to_owned().into(), network: NetworkId::Any }
					.into()
					.into(),
			),
			Box::new((Here, amount).into()),
			0,
		)
		.unwrap();
	});
}

#[cfg(test)]
mod test {
	use codec::Encode;

	#[test]
	fn general_index_asset() {
		let asset_id: u128 = 11;
		let asset_id = hex::encode(asset_id.encode());
		assert_eq!(&asset_id, "0b000000000000000000000000000000");
	}
}
