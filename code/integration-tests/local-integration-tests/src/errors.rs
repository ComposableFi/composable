use crate::{
	helpers::*,
	kusama_test_net::{KusamaRelay, This, THIS_PARA_ID},
	prelude::*,
};

use common::Balance;
use composable_traits::currency::{AssetExistentialDepositInspect, AssetRatioInspect};
use orml_traits::MultiCurrency;

/// under ED, but above Weight
pub fn under_existential_deposit<
	AssetsRegistry: AssetRatioInspect<AssetId = CurrencyId>
		+ AssetExistentialDepositInspect<AssetId = CurrencyId, Balance = Balance>,
>(
	asset_id: LocalAssetId,
	_instruction_count: usize,
) -> Balance {
	let ed = multi_existential_deposits::<AssetsRegistry>(&asset_id);
	assert_gt!(ed, Balance::one());
	ed - Balance::one()
}

#[test]
fn transfer_native_from_relay_enough_for_fee_but_not_enough_for_ed_ends_up_in_treasury() {
	simtest();
	let receiver = charlie();
	let (picasso_treasury, under_ed) = This::execute_with(|| {
		use this_runtime::*;
		let under_ed = under_existential_deposit::<AssetsRegistry>(LocalAssetId::KSM, 3);
		assert_eq!(Tokens::free_balance(CurrencyId::KSM, &AccountId::from(receiver)), 0,);
		(Tokens::free_balance(CurrencyId::KSM, &this_runtime::TreasuryAccount::get()), under_ed)
	});

	KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		let _ = <Balances as frame_support::traits::fungible::Balanced<AccountId>>::deposit(
			&AccountId::from(alice()),
			under_ed * 10000,
		)
		.unwrap();
		assert_ok!(XcmPallet::reserve_transfer_assets(
			Origin::signed(alice().into()),
			Box::new(Parachain(THIS_PARA_ID).into().into()),
			Box::new(Junction::AccountId32 { id: receiver, network: NetworkId::Any }.into().into()),
			Box::new((Here, under_ed).into()),
			0
		));
	});

	This::execute_with(|| {
		use this_runtime::*;
		assert_eq!(
			Tokens::free_balance(CurrencyId::KSM, &AccountId::from(receiver)),
			0,
			"assets did not get to recipient as it is not enough to pay ED"
		);
		assert_eq!(
			Tokens::free_balance(CurrencyId::KSM, &TreasuryAccount::get()),
			under_ed - picasso_treasury
		);
	});
}
