use crate::{env_logger_init, kusama_test_net::*};
use codec::Encode;
use common::AccountId;
use composable_traits::assets::{RemoteAssetRegistry, XcmAssetLocation};
use cumulus_primitives_core::ParaId;
use orml_traits::currency::MultiCurrency;
use picasso_runtime as dali_runtime;
use primitives::currency::*;
use sp_runtime::traits::AccountIdConversion;
use support::assert_ok;
use xcm::latest::prelude::*;
use xcm_emulator::TestExt;

/// assumes that our parachain has native relay token on relay account
/// and kusama can send xcm message to our network and transfer native token onto local network
#[test]
fn transfer_from_relay_chain() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();
	Picasso::execute_with(|| {
		assert_ok!(picasso_runtime::AssetsRegistry::set_location(
			CurrencyId::KSM, // KSM id as it is locally
			// if we get tokens from parent chain, these can be only native token
			composable_traits::assets::XcmAssetLocation(MultiLocation::parent())
		));
	});
	KusamaRelay::execute_with(|| {
		let transfered = kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(PICASSO_PARA_ID).into().into()),
			Box::new(
				Junction::AccountId32 { id: crate::kusama_test_net::BOB, network: NetworkId::Any }
					.into()
					.into(),
			),
			Box::new((Here, 3 * PICA).into()),
			0,
		);
		assert_ok!(transfered);
		assert_eq!(
			kusama_runtime::Balances::free_balance(&ParaId::from(PICASSO_PARA_ID).into_account()),
			13 * PICA
		);
	});

	Picasso::execute_with(|| {
		let native_token =
			picasso_runtime::Assets::free_balance(CurrencyId::KSM, &AccountId::from(BOB));
		assert_eq!(native_token, 3 * PICA);
	});
}

#[test]
fn transfer_to_relay_chain() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();
	Picasso::execute_with(|| {
		assert_ok!(<picasso_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::KSM,
			XcmAssetLocation::RELAY_NATIVE,
		));
		let transferred = picasso_runtime::XTokens::transfer(
			picasso_runtime::Origin::signed(ALICE.into()),
			CurrencyId::KSM,
			3 * PICA,
			Box::new(
				MultiLocation::new(
					1,
					X1(Junction::AccountId32 { id: BOB, network: NetworkId::Any }),
				)
				.into(),
			),
			4_600_000_000,
		);

		assert_ok!(transferred);

		let remaining =
			picasso_runtime::Assets::free_balance(CurrencyId::KSM, &AccountId::from(ALICE));

		assert_eq!(remaining, ALICE_PARACHAIN_KSM - 3 * PICA);
	});

	KusamaRelay::execute_with(|| {
		assert_eq!(
			kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
			2999893333340 // 3 * PICA - fee
		);
	});
}

#[test]
fn transfer_from_dali() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();

	Picasso::execute_with(|| {
		assert_ok!(<picasso_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::PICA,
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(DALI_PARA_ID), GeneralKey(CurrencyId::PICA.encode()))
			))
		));
	});

	let local_withdraw_amount = 3 * PICA;
	Dali::execute_with(|| {
		assert_ok!(dali_runtime::XTokens::transfer(
			dali_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			local_withdraw_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(PICASSO_PARA_ID),
						Junction::AccountId32 { id: BOB, network: NetworkId::Any }
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(
			dali_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(ALICE)),
			200 * PICA - local_withdraw_amount
		);
	});

	Picasso::execute_with(|| {
		let balance =
			picasso_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(BOB));
		assert_eq!(balance, local_withdraw_amount);
	});
}

#[test]
fn transfer_from_picasso_to_dali() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();

	Dali::execute_with(|| {
		assert_ok!(<dali_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			// local id
			CurrencyId::PICA,
			// remote id
			// first part is remote network,
			// second part is id of asset on remote
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(PICASSO_PARA_ID), GeneralKey(CurrencyId::PICA.encode()))
			))
		));
	});

	Picasso::execute_with(|| {
		assert_ok!(<picasso_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::PICA,
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(DALI_PARA_ID), GeneralKey(CurrencyId::PICA.encode()))
			))
		));

		assert_ok!(picasso_runtime::XTokens::transfer(
			picasso_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			3 * PICA,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(DALI_PARA_ID),
						Junction::AccountId32 { id: BOB, network: NetworkId::Any }
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(
			picasso_runtime::Balances::free_balance(&AccountId::from(ALICE)),
			200 * PICA - 3 * PICA
		);
	});

	Dali::execute_with(|| {
		let balance = dali_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(BOB));
		assert_eq!(balance, 3 * PICA);
	});
}

#[test]
fn transfer_insufficient_amount_should_fail() {
	Dali::execute_with(|| {
		assert_ok!(dali_runtime::XTokens::transfer(
			dali_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			1_000_000 - 1,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(2000),
						Junction::AccountId32 { id: BOB, network: NetworkId::Any }
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(dali_runtime::Balances::free_balance(&AccountId::from(ALICE)), 199999999000001);
	});

	Picasso::execute_with(|| {
		// Xcm should fail therefore nothing should be deposit into beneficiary account
		assert_eq!(
			picasso_runtime::Tokens::free_balance(CurrencyId::PICA, &AccountId::from(BOB)),
			0
		);
	});
}
