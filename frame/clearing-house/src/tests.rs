pub use crate::{
	mock::{
		accounts::{AccountId, ALICE},
		assets::{AssetId, DOT, PICA, USDC},
		oracle as mock_oracle,
		runtime::{
			Balance, ClearingHouse, ExtBuilder, Oracle, Origin, Runtime, System, Tokens, Vamm,
		},
		vamm as mock_vamm,
	},
	pallet::*,
};
use composable_traits::{oracle::Oracle as OracleTrait, vamm::VirtualAMM};
use frame_support::{assert_err, assert_noop, assert_ok};
use orml_tokens::Error as TokenError;

type VammParams = mock_vamm::VammParams;

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			native_balances: vec![],
			balances: vec![],
			collateral_types: vec![USDC],
			vamm_id: Some(0u64),
			oracle_asset_support: Some(true),
		}
	}
}

#[test]
fn add_margin_returns_transfer_error() {
	ExtBuilder::default().build().execute_with(|| {
		let origin = Origin::signed(ALICE);
		assert_noop!(
			ClearingHouse::add_margin(origin, USDC, 1_000u32.into()),
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
				ClearingHouse::add_margin(origin, PICA, 1_000u32.into()),
				Error::<Runtime>::UnsupportedCollateralType
			);
		});
}

#[test]
fn deposit_supported_collateral_succeeds() {
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
fn create_first_market_succeeds() {
	ExtBuilder::default().build().execute_with(|| {
		let old_count = ClearingHouse::market_count();

		let asset = DOT;
		let vamm_params = VammParams {};
		assert_ok!(ClearingHouse::create_market(Origin::signed(ALICE), asset, vamm_params));

		// Ensure first market id is 0 (we know its type since it's defined in the mock runtime)
		System::assert_last_event(Event::MarketCreated { market: 0u64, asset }.into());
		assert!(Markets::<Runtime>::contains_key(0u64));

		// Ensure market count is increased by 1
		assert_eq!(ClearingHouse::market_count(), old_count + 1);
	})
}

#[test]
fn mock_oracle_asset_support_reflects_genesis_config() {
	let asset_support = Some(false);
	ExtBuilder { oracle_asset_support: asset_support, ..Default::default() }
		.build()
		.execute_with(|| {
			let is_supported = <Oracle as OracleTrait>::is_supported(DOT);
			match asset_support {
				Some(support) => assert_ok!(is_supported, support),
				None =>
					assert_err!(is_supported, mock_oracle::Error::<Runtime>::CantCheckAssetSupport),
			}
		})
}

#[test]
fn mock_vamm_created_id_reflects_genesis_config() {
	let vamm_id = None;
	ExtBuilder { vamm_id, ..Default::default() }.build().execute_with(|| {
		let created = <Vamm as VirtualAMM>::create(VammParams {});
		match vamm_id {
			Some(id) => assert_ok!(created, id),
			None => assert_err!(created, mock_vamm::Error::<Runtime>::FailedToCreateVamm),
		}
	})
}

#[test]
fn fails_to_create_market_for_unsupported_asset_by_oracle() {
	ExtBuilder { oracle_asset_support: Some(false), ..Default::default() }
		.build()
		.execute_with(|| {
			assert_noop!(
				ClearingHouse::create_market(Origin::signed(ALICE), DOT, VammParams {}),
				Error::<Runtime>::NoPriceFeedForAsset
			);
		})
}

#[test]
fn fails_to_create_market_if_fails_to_create_vamm() {
	ExtBuilder { vamm_id: None, ..Default::default() }.build().execute_with(|| {
		assert_noop!(
			ClearingHouse::create_market(Origin::signed(ALICE), DOT, VammParams {}),
			mock_vamm::Error::<Runtime>::FailedToCreateVamm
		);
	})
}
