// use std::assert_matches::assert_matches;

pub use crate::{
	mock::{
		accounts::{AccountId, ALICE},
		assets::{AssetId, DOT, PICA, USDC},
		runtime::{Balance, ClearingHouse, ExtBuilder, Origin, Runtime, System, Tokens},
		vamm::VammParams,
	},
	pallet::*,
};
use frame_support::{assert_noop, assert_ok};
use orml_tokens::Error as TokenError;

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			native_balances: vec![],
			balances: vec![],
			collateral_types: vec![USDC],
			oracle_supports_assets: true,
		}
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

#[test]
fn test_create_market_succeeds() {
	ExtBuilder::default().build().execute_with(|| {
		let market = 1u32.into();
		let asset = DOT;
		let vamm_params = VammParams {};
		assert_ok!(ClearingHouse::create_market(Origin::signed(ALICE), market, asset, vamm_params));
		// let event =
		// 	Event::from(System::events().last().expect("Expected at least one event").event);
		// assert_matches!(event, Event::MarketCreated { market: _, asset });
		System::assert_last_event(Event::MarketCreated { market, asset }.into());
	})
}

#[test]
fn test_create_existing_market_id_fails() {
	ExtBuilder::default().build().execute_with(|| {
		let market = 1u32.into();
		assert_ok!(ClearingHouse::create_market(Origin::signed(ALICE), market, DOT, VammParams {}));

		assert_noop!(
			ClearingHouse::create_market(Origin::signed(ALICE), market, PICA, VammParams {}),
			Error::<Runtime>::MarketAlreadyExists
		);
	})
}

#[test]
fn test_fails_to_create_market_for_unsupported_asset_by_oracle() {
	ExtBuilder { oracle_supports_assets: false, ..Default::default() }
		.build()
		.execute_with(|| {
			let market = 1u32.into();
			assert_noop!(
				ClearingHouse::create_market(Origin::signed(ALICE), market, DOT, VammParams {}),
				Error::<Runtime>::NoPriceFeedForAsset
			);
		})
}
