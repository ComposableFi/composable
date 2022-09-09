///! tests that various assets integration scenarios work well
// TODO: make sure that compile assets ED/weights are used when configured
// TODO: make sure that non compile assets work same
// TODO: make sure that compile time assets of same service as non compile,
// TODO: i.e. part of apis for symbol/name/local id/remote id/ED/weight
use crate::{
	helpers::*,
	kusama_test_net::{Sibling, This, ALICE, BOB, PICA, SIBLING_PARA_ID, THIS_PARA_ID},
	prelude::*,
};
use codec::Encode;
use common::{AccountId, PriceConverter};
use composable_traits::{defi::Ratio, oracle::MinimalOracle, xcm::assets::XcmAssetLocation};

use frame_system::RawOrigin;
use this_runtime::{Balances, Origin, UnitWeightCost, XTokens};

use orml_traits::currency::MultiCurrency;

use frame_support::{assert_ok, WeakBoundedVec};
use primitives::currency::*;
use sp_runtime::assert_eq_error_rate;
use xcm_emulator::TestExt;

#[test]
fn assets_registry_works_well_for_ratios() {
	simtest();
	This::execute_with(|| {
		use this_runtime::*;
		AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId(42),
			XcmAssetLocation(MultiLocation::new(1, X1(Parachain(666)))),
			Ratio::checked_from_integer::<u128>(10),
			None,
		)
		.unwrap();
		AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId(123),
			XcmAssetLocation(MultiLocation::new(1, X1(Parachain(4321)))),
			Ratio::checked_from_rational(10u32, 100u32),
			None,
		)
		.unwrap();
		assert_eq!(
			1000,
			<PriceConverter<AssetsRegistry>>::get_price_inverse(CurrencyId(42), 100).unwrap()
		);
		assert_eq!(
			10,
			<PriceConverter<AssetsRegistry>>::get_price_inverse(CurrencyId(123), 100).unwrap()
		);
	});
}

/// so we can map asset on one chain to asset on other chain to be 1 to 1.
#[test]
fn assets_registry_works_for_identity() {
	simtest();

	let identity_asset = CurrencyId::PICA;

	fn picasso_reserve_account() -> AccountId {
		use sp_runtime::traits::AccountIdConversion;
		polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account_truncating()
	}

	This::execute_with(|| {
		use this_runtime::*;
		let remote = XcmAssetLocation(
			MultiLocation::new(
				1,
				X2(
					Parachain(SIBLING_PARA_ID),
					GeneralKey(WeakBoundedVec::force_from(CurrencyId::PICA.encode(), None)),
				),
			)
			.into(),
		);

		assert_ok!(AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			identity_asset,
			remote,
			Ratio::checked_from_integer::<u128>(1),
			None,
		));
	});

	Sibling::execute_with(|| {
		use sibling_runtime::*;
		let remote = XcmAssetLocation(
			MultiLocation::new(
				1,
				X2(
					Parachain(THIS_PARA_ID),
					GeneralKey(WeakBoundedVec::force_from(CurrencyId::PICA.encode(), None)),
				),
			)
			.into(),
		);

		assert_ok!(AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			identity_asset,
			remote,
			Ratio::checked_from_integer::<u128>(1),
			None,
		));
	});

	Sibling::execute_with(|| {
		assert_eq!(Balances::free_balance(&picasso_reserve_account()), 0);

		assert_ok!(XTokens::transfer(
			this_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			5 * PICA,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(THIS_PARA_ID),
						Junction::AccountId32 { network: NetworkId::Any, id: BOB.into() }
					)
				)
				.into()
			),
			1_000_000_000,
		));

		assert_eq!(Balances::free_balance(&picasso_reserve_account()), 5 * PICA);
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), 200 * PICA - 5 * PICA);
	});

	This::execute_with(|| {
		let balance = this_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(BOB));
		assert_eq_error_rate!(balance, 5 * PICA, (UnitWeightCost::get() * 10) as u128);

		assert_ok!(XTokens::transfer(
			Origin::signed(BOB.into()),
			identity_asset,
			PICA,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(SIBLING_PARA_ID),
						Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }
					)
				)
				.into()
			),
			1_000_000_000,
		));

		let balance = this_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(BOB));
		assert_eq_error_rate!(balance, 5 * PICA - PICA, (UnitWeightCost::get() * 10) as u128);
	});

	Sibling::execute_with(|| {
		assert_eq!(Balances::free_balance(&picasso_reserve_account()), 5 * PICA);
		let balance = this_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(ALICE));
		assert_eq_error_rate!(
			balance,
			200 * PICA - 5 * PICA + PICA,
			(UnitWeightCost::get() * 10) as u128
		);
	});

	Sibling::execute_with(|| {
		assert_eq!(Balances::free_balance(&picasso_reserve_account()), 5 * PICA);
		let balance = this_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(ALICE));
		assert_eq_error_rate!(
			balance,
			200 * PICA - 5 * PICA + PICA,
			(UnitWeightCost::get() * 10) as u128
		);

		assert_ok!(XTokens::transfer(
			Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			5 * PICA,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(THIS_PARA_ID),
						Junction::AccountId32 { network: NetworkId::Any, id: BOB.into() }
					)
				)
				.into()
			),
			1_000_000_000,
		));

		assert_eq!(Balances::free_balance(&picasso_reserve_account()), 10 * PICA);
		let balance = this_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(ALICE));
		assert_eq_error_rate!(
			balance,
			200 * PICA - 5 * PICA + PICA - 5 * PICA,
			(UnitWeightCost::get() * 10) as u128
		);
	});

	This::execute_with(|| {
		let balance = this_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(BOB));
		assert_eq_error_rate!(
			balance,
			5 * PICA - PICA + 5 * PICA,
			(UnitWeightCost::get() * 10) as u128
		);
	});
}
