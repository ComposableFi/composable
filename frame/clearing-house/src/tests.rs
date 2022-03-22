pub use crate::{
	mock::{
		accounts::{AccountId, ALICE},
		assets::{AssetId, PICA, USDC},
		runtime::{Balance, ClearingHouse, ExtBuilder, Origin, Runtime, System, Tokens},
	},
	pallet::*,
};
use frame_support::{assert_noop, assert_ok};
use orml_tokens::Error as TokenError;

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { native_balances: vec![], balances: vec![], collateral_types: vec![USDC] }
	}
}

#[test]
fn test_add_margin_returns_transfer_error() {
	ExtBuilder::default().build().execute_with(|| {
		let origin = Origin::signed(ALICE);
		assert_noop!(
			ClearingHouse::add_margin(origin, USDC, 1_000u32.into()),
			TokenError::<Runtime>::BalanceTooLow
		);
	});
}

#[test]
fn test_deposit_unsupported_collateral_returns_error() {
	ExtBuilder { balances: vec![(ALICE, PICA, 1_000_000)], ..Default::default() }
		.build()
		.execute_with(|| {
			let origin = Origin::signed(ALICE);
			assert_noop!(
				ClearingHouse::add_margin(origin, PICA, 1_000u32.into()),
				Error::<Runtime>::UnsupportedCollateralType
			);
		});
}

#[test]
fn test_deposit_supported_collateral_succeeds() {
	ExtBuilder { balances: vec![(ALICE, USDC, 1_000_000)], ..Default::default() }
		.build()
		.execute_with(|| {
			let account = ALICE;
			let asset = USDC;
			let amount: Balance = 1_000u32.into();

			let before = AccountsMargin::<Runtime>::get(&account).unwrap_or_default();
			assert_ok!(ClearingHouse::add_margin(Origin::signed(account), asset, amount));

			System::assert_last_event(Event::MarginAdded { account, asset, amount }.into());

			let after = AccountsMargin::<Runtime>::get(&account).unwrap_or_default();
			assert_eq!(after - before, amount);
		})
}
