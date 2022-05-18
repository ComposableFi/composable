use crate::{
	mock::{
		accounts::{AccountId, ALICE},
		assets::{PICA, USDC},
		runtime::{
			Assets as AssetsPallet, Balance, ExtBuilder, Origin, Runtime, System as SystemPallet,
			TestPallet, TestPalletId,
		},
	},
	pallet::{AccountsMargin, Error, Event},
	tests::run_to_block,
};
use frame_support::{assert_noop, assert_ok, traits::fungibles::Inspect};
use orml_tokens::Error as TokenError;
use sp_runtime::traits::AccountIdConversion;

// ----------------------------------------------------------------------------------------------------
//                                             Add Margin
// ----------------------------------------------------------------------------------------------------

#[test]
fn add_margin_returns_transfer_error() {
	ExtBuilder::default().build().execute_with(|| {
		let origin = Origin::signed(ALICE);
		assert_noop!(
			TestPallet::add_margin(origin, USDC, 1_000_u32.into()),
			TokenError::<Runtime>::BalanceTooLow
		);
	});
}

#[test]
fn deposit_unsupported_collateral_returns_error() {
	ExtBuilder { balances: vec![(ALICE, PICA, 1_000_000)], ..Default::default() }
		.build()
		.execute_with(|| {
			let origin = Origin::signed(ALICE);
			assert_noop!(
				TestPallet::add_margin(origin, PICA, 1_000_u32.into()),
				Error::<Runtime>::UnsupportedCollateralType
			);
		});
}

#[test]
fn deposit_supported_collateral_succeeds() {
	ExtBuilder { balances: vec![(ALICE, USDC, 1_000_000)], ..Default::default() }
		.build()
		.execute_with(|| {
			run_to_block(1);
			let account = ALICE;
			let asset = USDC;
			let amount: Balance = 1_000;

			let before = (
				AccountsMargin::<Runtime>::get(&account).unwrap_or_default(),
				<AssetsPallet as Inspect<AccountId>>::balance(USDC, &ALICE),
			);
			assert_ok!(TestPallet::add_margin(Origin::signed(account), asset, amount));

			let after = (
				AccountsMargin::<Runtime>::get(&account).unwrap_or_default(),
				<AssetsPallet as Inspect<AccountId>>::balance(USDC, &ALICE),
			);
			assert_eq!(after.0 - before.0, amount);
			assert_eq!(after.1, before.1 - amount);

			let pallet_acc = TestPalletId::get().into_sub_account("Collateral");
			assert_eq!(<AssetsPallet as Inspect<AccountId>>::balance(USDC, &pallet_acc), amount);

			SystemPallet::assert_last_event(Event::MarginAdded { account, asset, amount }.into());
		})
}
