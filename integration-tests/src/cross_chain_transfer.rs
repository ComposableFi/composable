use crate::{env_logger_init, kusama_test_net::*};
use codec::Encode;
use common::AccountId;
use composable_traits::assets::{RemoteAssetRegistry, XcmAssetLocation};
use cumulus_primitives_core::ParaId;
use dali_runtime as picasso_runtime;
use orml_traits::currency::MultiCurrency;
use primitives::currency::*;
use sp_runtime::traits::AccountIdConversion;
use support::assert_ok;
use xcm::latest::prelude::*;
use xcm_emulator::TestExt;
use xcm_executor::XcmExecutor;

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
						Junction::Parachain(PICASSO_PARA_ID),
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

/// Acala's tests
#[test]
#[ignore]
fn transfer_from_relay_chain_deposit_to_treasury_if_below_ed() {
	KusamaRelay::execute_with(|| {
		assert_ok!(kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(PICASSO_PARA_ID).into().into()),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			Box::new((Here, 128_000_111).into()),
			0
		));
	});

	Picasso::execute_with(|| {
		assert_eq!(
			picasso_runtime::Tokens::free_balance(CurrencyId::KSM, &AccountId::from(BOB)),
			0
		);
		assert_eq!(
			picasso_runtime::Tokens::free_balance(
				CurrencyId::KSM,
				&picasso_runtime::TreasuryAccount::get()
			),
			1_000_128_000_111
		);
	});
}

#[test]
#[ignore]
fn xcm_transfer_execution_barrier_trader_works() {
	let expect_weight_limit = 600_000_000;
	let weight_limit_too_low = 500_000_000;
	let unit_instruction_weight = 200_000_000;

	// relay-chain use normal account to send xcm, destination para-chain can't pass Barrier check
	let message = Xcm(vec![
		ReserveAssetDeposited((Parent, 100).into()),
		BuyExecution { fees: (Parent, 100).into(), weight_limit: Unlimited },
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	KusamaRelay::execute_with(|| {
		let r = pallet_xcm::Pallet::<kusama_runtime::Runtime>::send(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(PICASSO_PARA_ID).into().into()),
			Box::new(xcm::VersionedXcm::from(message)),
		);
		assert_ok!(r);
	});
	Picasso::execute_with(|| {
		assert!(picasso_runtime::System::events().iter().any(|r| matches!(
			r.event,
			picasso_runtime::Event::DmpQueue(cumulus_pallet_dmp_queue::Event::ExecutedDownward(
				_,
				Outcome::Error(XcmError::Barrier)
			))
		)));
	});

	// AllowTopLevelPaidExecutionFrom barrier test case:
	// para-chain use XcmExecutor `execute_xcm()` method to execute xcm.
	// if `weight_limit` in BuyExecution is less than `xcm_weight(max_weight)`, then Barrier can't
	// pass. other situation when `weight_limit` is `Unlimited` or large than `xcm_weight`, then
	// it's ok.
	let message = Xcm::<picasso_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, 100).into()),
		BuyExecution { fees: (Parent, 100).into(), weight_limit: Limited(weight_limit_too_low) },
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	Picasso::execute_with(|| {
		let r = XcmExecutor::<picasso_runtime::XcmConfig>::execute_xcm(
			Parent,
			message,
			expect_weight_limit,
		);
		assert_eq!(r, Outcome::Error(XcmError::Barrier));
	});

	// trader inside BuyExecution have TooExpensive error if payment less than calculated weight
	// amount. the minimum of calculated weight amount(`FixedRateOfFungible<KsmPerSecond>`) is
	// 96_000_000
	let message = Xcm::<picasso_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, 95_999_999).into()),
		BuyExecution {
			fees: (Parent, 95_999_999).into(),
			weight_limit: Limited(expect_weight_limit),
		},
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	Picasso::execute_with(|| {
		let r = XcmExecutor::<picasso_runtime::XcmConfig>::execute_xcm(
			Parent,
			message,
			expect_weight_limit,
		);
		assert_eq!(
			r,
			Outcome::Incomplete(
				expect_weight_limit - unit_instruction_weight,
				XcmError::TooExpensive
			)
		);
	});

	// all situation fulfilled, execute success
	let message = Xcm::<picasso_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, 96_000_000).into()),
		BuyExecution {
			fees: (Parent, 96_000_000).into(),
			weight_limit: Limited(expect_weight_limit),
		},
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	Picasso::execute_with(|| {
		let r = XcmExecutor::<picasso_runtime::XcmConfig>::execute_xcm(
			Parent,
			message,
			expect_weight_limit,
		);
		assert_eq!(r, Outcome::Complete(expect_weight_limit));
	});
}

#[test]
#[ignore]
fn subscribe_version_notify_works() {
	// relay chain subscribe version notify of para chain
	KusamaRelay::execute_with(|| {
		let r = pallet_xcm::Pallet::<kusama_runtime::Runtime>::force_subscribe_version_notify(
			kusama_runtime::Origin::root(),
			Box::new(Parachain(PICASSO_PARA_ID).into().into()),
		);
		assert_ok!(r);
	});
	KusamaRelay::execute_with(|| {
		kusama_runtime::System::assert_has_event(kusama_runtime::Event::XcmPallet(
			pallet_xcm::Event::SupportedVersionChanged(
				MultiLocation { parents: 0, interior: X1(Parachain(PICASSO_PARA_ID)) },
				2,
			),
		));
	});

	// para chain subscribe version notify of relay chain
	Picasso::execute_with(|| {
		let r = pallet_xcm::Pallet::<picasso_runtime::Runtime>::force_subscribe_version_notify(
			picasso_runtime::Origin::root(),
			Box::new(Parent.into()),
		);
		assert_ok!(r);
	});
	Picasso::execute_with(|| {
		picasso_runtime::System::assert_has_event(picasso_runtime::Event::RelayerXcm(
			pallet_xcm::Event::SupportedVersionChanged(
				MultiLocation { parents: 1, interior: Here },
				2,
			),
		));
	});

	// para chain subscribe version notify of sibling chain
	Picasso::execute_with(|| {
		let r = pallet_xcm::Pallet::<picasso_runtime::Runtime>::force_subscribe_version_notify(
			picasso_runtime::Origin::root(),
			Box::new((Parent, Parachain(PICASSO_PARA_ID)).into()),
		);
		assert_ok!(r);
	});
	Picasso::execute_with(|| {
		assert!(picasso_runtime::System::events().iter().any(|r| matches!(
			r.event,
			picasso_runtime::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent(
				Some(_)
			))
		)));
	});
	Dali::execute_with(|| {
		assert!(dali_runtime::System::events().iter().any(|r| matches!(
			r.event,
			picasso_runtime::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent(
				Some(_)
			)) | picasso_runtime::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success(
				Some(_)
			))
		)));
	});
}
