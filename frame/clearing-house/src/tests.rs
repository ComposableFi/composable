use crate::mock::runtime::VammId;
pub use crate::{
	mock::{
		accounts::{AccountId, ALICE},
		assets::{AssetId, DOT, PICA, USDC},
		oracle as mock_oracle,
		runtime::{Balance, ClearingHouse, ExtBuilder, Origin, Runtime, System, Timestamp},
		vamm as mock_vamm,
	},
	pallet::*,
};
use composable_traits::{oracle::Oracle, time::ONE_HOUR, vamm::Vamm};
use frame_support::{assert_err, assert_noop, assert_ok, pallet_prelude::Hooks, traits::UnixTime};
use orml_tokens::Error as TokenError;
use proptest::prelude::*;
use sp_runtime::FixedI128;

// ----------------------------------------------------------------------------------------------------
//                                             Setup
// ----------------------------------------------------------------------------------------------------

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

fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 0 {
			Timestamp::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		// Time is set in milliseconds, so at each block we increment the timestamp by 1000ms = 1s
		let _ = Timestamp::set(Origin::none(), System::block_number() * 1000);
		System::on_initialize(System::block_number());
		Timestamp::on_initialize(System::block_number());
	}
}

// ----------------------------------------------------------------------------------------------------
//                                           Mocked Pallets Tests
// ----------------------------------------------------------------------------------------------------

proptest! {
	// Can we guarantee that any::<Option<Value>> will generate at least one of `Some` and `None`?
	#[test]
	fn mock_oracle_asset_support_reflects_genesis_config(asset_support in any::<Option<bool>>()) {
		ExtBuilder { oracle_asset_support: asset_support, ..Default::default() }
		.build()
		.execute_with(|| {
			let is_supported = <Runtime as Config>::Oracle::is_supported(DOT);
			match asset_support {
				Some(support) => assert_ok!(is_supported, support),
				None => {
					assert_err!(is_supported, mock_oracle::Error::<Runtime>::CantCheckAssetSupport)
				},
			}
		})
	}
}

proptest! {
	// Can we guarantee that any::<Option<Value>> will generate at least one of `Some` and `None`?
	#[test]
	fn mock_vamm_created_id_reflects_genesis_config(vamm_id in any::<Option<VammId>>()) {
		ExtBuilder { vamm_id , ..Default::default() }.build().execute_with(|| {
			let created = <Runtime as Config>::Vamm::create(VammParams {});
			match vamm_id {
				Some(id) => assert_ok!(created, id),
				None => assert_err!(created, mock_vamm::Error::<Runtime>::FailedToCreateVamm),
			}
		})
	}
}

// ----------------------------------------------------------------------------------------------------
//                                             Pallet Tests
// ----------------------------------------------------------------------------------------------------

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
			run_to_block(1);
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

#[allow(clippy::disallowed_methods)]
#[test]
fn create_first_market_succeeds() {
	ExtBuilder::default().build().execute_with(|| {
		run_to_block(10); // Timestamp unix time does not work properly at genesis
		let old_count = ClearingHouse::market_count();
		let block_time_now = <Timestamp as UnixTime>::now().as_secs();

		let asset = DOT;
		// 10x max leverage to open a position
		let margin_ratio_initial = FixedI128::from_float(0.1);
		// liquidate when above 50x leverage
		let margin_ratio_maintenance = FixedI128::from_float(0.02);
		let funding_frequency = ONE_HOUR;
		let funding_period = ONE_HOUR * 24;
		assert_ok!(ClearingHouse::create_market(
			Origin::signed(ALICE),
			asset,
			VammParams {},
			margin_ratio_initial,
			margin_ratio_maintenance,
			funding_frequency,
			funding_period
		));

		// Ensure first market id is 0 (we know its type since it's defined in the mock runtime)
		System::assert_last_event(Event::MarketCreated { market: 0u64, asset }.into());
		assert!(Markets::<Runtime>::contains_key(0u64));

		// Ensure market count is increased by 1
		assert_eq!(ClearingHouse::market_count(), old_count + 1);

		// Ensure new market matches creation parameters
		let market = ClearingHouse::get_market(0u64).unwrap();
		assert_eq!(market.asset_id, asset);
		assert_eq!(market.margin_ratio_initial, margin_ratio_initial);
		assert_eq!(market.margin_ratio_maintenance, margin_ratio_maintenance);
		assert_eq!(market.funding_frequency, funding_frequency);
		assert_eq!(market.funding_period, funding_period);

		// Ensure last funding rate timestamp is the same as this block's time
		assert_eq!(market.funding_rate_ts, block_time_now);
	})
}

#[test]
fn fails_to_create_market_for_unsupported_asset_by_oracle() {
	ExtBuilder { oracle_asset_support: Some(false), ..Default::default() }
		.build()
		.execute_with(|| {
			assert_noop!(
				ClearingHouse::create_market(
					Origin::signed(ALICE),
					DOT,
					VammParams {},
					FixedI128::from_float(0.1),
					FixedI128::from_float(0.02),
					ONE_HOUR,
					ONE_HOUR * 24,
				),
				Error::<Runtime>::NoPriceFeedForAsset
			);
		})
}

#[test]
fn fails_to_create_market_if_fails_to_create_vamm() {
	ExtBuilder { vamm_id: None, ..Default::default() }.build().execute_with(|| {
		assert_noop!(
			ClearingHouse::create_market(
				Origin::signed(ALICE),
				DOT,
				VammParams {},
				FixedI128::from_float(0.1),
				FixedI128::from_float(0.02),
				ONE_HOUR,
				ONE_HOUR * 24,
			),
			mock_vamm::Error::<Runtime>::FailedToCreateVamm
		);
	})
}
