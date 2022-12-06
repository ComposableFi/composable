//! Testing custom XCMP transactions call

use crate::{
	helpers::{assert_above_deposit, enough_weight, sibling_account, simtest},
	kusama_test_net::*,
	prelude::*,
};
use composable_traits::defi::Sell;
use frame_system::EventRecord;
use orml_traits::currency::MultiCurrency;
use primitives::currency::CurrencyId;
use xcm_emulator::TestExt;

#[macro_export]
macro_rules! match_this_event {
	($event:expr, $pattern_type: ident, $pattern_value: pat) => {
		if let EventRecord { event: this_runtime::Event::$pattern_type(inner), .. } = $event {
			if let $pattern_value = inner {
				true
			} else {
				false
			}
		} else {
			false
		}
	};
}

#[test]
fn dex() {
	simtest();
	let any_asset = CurrencyId::kUSD;
	let some_native_amount = 1_000_000_000;
	let this_liveness_native_amount = enough_weight();
	let this_native_asset = CurrencyId::PICA;

	let user = AccountId::from(ALICE);
	let some_enough_liquidation_weight = UnitWeightCost::get() * 2;
	let order = Sell::new(
		LocalAssetId::PICA,
		LocalAssetId::kUSD,
		// prices are set just for easy debugging
		100,
		FixedU128::saturating_from_integer(42_u64),
	);

	// this can be generated from scale by third parties
	let sell =
		this_runtime::Call::Liquidations(liquidations::Call::<this_runtime::Runtime>::sell {
			order: order.clone(),
			configuration: Default::default(),
		});
	let binary_sell =
		composable_traits::liquidation::XcmLiquidation::new(63, 1, order.clone(), vec![]);

	assert_eq!(sell.encode(), binary_sell.encode());

	let _this_native_treasury_amount = This::execute_with(|| {
		let sibling_non_native_amount =
			assert_above_deposit::<this_runtime::AssetsRegistry>(any_asset, 100_000_000_000);
		assert_ok!(this_runtime::Assets::deposit(
			any_asset,
			&sibling_account(SIBLING_PARA_ID),
			sibling_non_native_amount
		));
		log::error!("{:?}", &sibling_account(SIBLING_PARA_ID));
		let _ = <balances::Pallet<this_runtime::Runtime> as frame_support::traits::Currency<
			AccountId,
		>>::deposit_creating(
			&sibling_account(SIBLING_PARA_ID),
			this_liveness_native_amount * 1_000_000_000_000,
		);
		let _ = <assets::Pallet<this_runtime::Runtime> as MultiCurrency<AccountId>>::deposit(
			CurrencyId::kUSD,
			&sibling_account(SIBLING_PARA_ID),
			this_liveness_native_amount,
		);
		let _ =
			<balances::Pallet<this_runtime::Runtime> as frame_support::traits::Currency<
				AccountId,
			>>::deposit_creating(&this_runtime::TreasuryAccount::get(), this_liveness_native_amount);
		<balances::Pallet<this_runtime::Runtime> as frame_support::traits::Currency<AccountId>>::free_balance(
			&this_runtime::TreasuryAccount::get(),
		)
	});

	This::execute_with(|| {
		let result = this_runtime::Liquidations::sell(
			this_runtime::Origin::signed(user.clone()),
			order.clone(),
			vec![],
		);
		assert!(this_runtime::System::events().iter().any(|x| {
			match_this_event!(
				x,
				Liquidations,
				liquidations::Event::<_>::PositionWasSentToLiquidation { .. }
			)
		}));
		assert_ok!(result);
	});

	Sibling::execute_with(|| {
		let assets: MultiAsset = (
			(Parent, X2(Parachain(THIS_PARA_ID), GeneralIndex(this_native_asset.into()))),
			some_native_amount,
		)
			.into();
		let xcm = vec![
			WithdrawAsset(assets.clone().into()), /* withdraw native on target chain from origin
			                                       * account */
			BuyExecution {
				// pay for origin account
				fees: assets,
				weight_limit: Unlimited,
			},
			Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: some_enough_liquidation_weight,
				call: sell.encode().into(),
			},
		];
		assert_ok!(pallet_xcm::Pallet::<sibling_runtime::Runtime>::send_xcm(
			Here,
			(Parent, Parachain(THIS_PARA_ID)),
			Xcm(xcm),
		));
	});

	This::execute_with(|| {
		let result = this_runtime::System::events().iter().any(|x| {
			match_this_event!(
				x,
				Liquidations,
				liquidations::Event::<_>::PositionWasSentToLiquidation { .. }
			)
		});
		assert!(result);
	});

	Sibling::execute_with(|| {
		let assets: MultiAsset = (
			(Parent, X2(Parachain(THIS_PARA_ID), GeneralIndex(this_native_asset.0))),
			some_native_amount,
		)
			.into();
		let xcm = vec![
			WithdrawAsset(assets.clone().into()), /* withdraw native on target chain from origin
			                                       * account */
			BuyExecution {
				// pay for origin account
				fees: assets,
				weight_limit: Unlimited,
			},
			Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: some_enough_liquidation_weight,
				call: binary_sell.encode().into(),
			},
		];
		assert_ok!(pallet_xcm::Pallet::<sibling_runtime::Runtime>::send_xcm(
			Here,
			(Parent, Parachain(THIS_PARA_ID)),
			Xcm(xcm),
		));
	});

	This::execute_with(|| {
		let result = this_runtime::System::events().iter().any(|x| {
			match_this_event!(
				x,
				Liquidations,
				liquidations::Event::<_>::PositionWasSentToLiquidation { .. }
			)
		});
		assert!(result);
	});
}
