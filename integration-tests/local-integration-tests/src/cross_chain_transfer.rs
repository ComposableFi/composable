use crate::{
	env_logger_init,
	kusama_test_net::{Dali as Sibling, *},
};
use codec::Encode;
use common::{xcmp::BaseXcmWeight, AccountId, Balance};
use composable_traits::assets::{RemoteAssetRegistry, XcmAssetLocation};
use dali_runtime as picasso_runtime;
use orml_traits::currency::MultiCurrency;
use picasso_runtime::{MaxInstructions, UnitWeightCost};
use primitives::currency::*;
use sp_runtime::assert_eq_error_rate;
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
	let bob_before = Picasso::execute_with(|| {
		assert_ok!(picasso_runtime::AssetsRegistry::set_location(
			CurrencyId::KSM, // KSM id as it is locally
			// if we get tokens from parent chain, these can be only native token
			XcmAssetLocation::RELAY_NATIVE,
		));
		picasso_runtime::Assets::free_balance(CurrencyId::KSM, &AccountId::from(BOB))
	});
	let transfer_amount = 3 * KSM;
	KusamaRelay::execute_with(|| {
		let alice_before = kusama_runtime::Balances::free_balance(&AccountId::from(ALICE));
		let transfered = kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(PICASSO_PARA_ID).into().into()),
			Box::new(
				Junction::AccountId32 { id: crate::kusama_test_net::BOB, network: NetworkId::Any }
					.into()
					.into(),
			),
			Box::new((Here, transfer_amount).into()),
			0,
		);
		assert_ok!(transfered);
		let alice_after = kusama_runtime::Balances::free_balance(&AccountId::from(ALICE));
		assert_eq!(alice_before, alice_after + transfer_amount);
	});

	Picasso::execute_with(|| {
		let bob_after =
			picasso_runtime::Assets::free_balance(CurrencyId::KSM, &AccountId::from(BOB));
		assert_eq_error_rate!(
			bob_after - bob_before,
			transfer_amount,
			(UnitWeightCost::get() * 10) as u128
		);
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
		assert_eq_error_rate!(balance, local_withdraw_amount, (UnitWeightCost::get() * 10) as u128);
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
		assert_eq_error_rate!(balance, 3 * PICA, (UnitWeightCost::get() * 10) as u128);
	});
}

// from: Hydra
#[test]
fn transfer_insufficient_amount_should_fail() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();
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

// #[test]
// fn transfer_to_sibling() {
// 	TestNet::reset();

// 	fn karura_reserve_account() -> AccountId {
// 		use sp_runtime::traits::AccountIdConversion;
// 		polkadot_parachain::primitives::Sibling::from(2000).into_account()
// 	}

// 	Karura::execute_with(|| {
// 		assert_ok!(Tokens::deposit(BNC, &AccountId::from(ALICE), 100_000_000_000_000));
// 	});

// 	Sibling::execute_with(|| {
// 		assert_ok!(Tokens::deposit(BNC, &karura_reserve_account(), 100_000_000_000_000));
// 	});

// 	Karura::execute_with(|| {
// 		assert_ok!(XTokens::transfer(
// 			Origin::signed(ALICE.into()),
// 			BNC,
// 			10_000_000_000_000,
// 			Box::new(
// 				MultiLocation::new(
// 					1,
// 					X2(
// 						Parachain(2001),
// 						Junction::AccountId32 {
// 							network: NetworkId::Any,
// 							id: BOB.into(),
// 						}
// 					)
// 				)
// 				.into()
// 			),
// 			1_000_000_000,
// 		));

// 		assert_eq!(Tokens::free_balance(BNC, &AccountId::from(ALICE)), 90_000_000_000_000);
// 	});

// 	Sibling::execute_with(|| {
// 		assert_eq!(Tokens::free_balance(BNC, &karura_reserve_account()), 90_000_000_000_000);
// 		assert_eq!(Tokens::free_balance(BNC, &AccountId::from(BOB)), 9_989_760_000_000);

// 		assert_ok!(XTokens::transfer(
// 			Origin::signed(BOB.into()),
// 			BNC,
// 			5_000_000_000_000,
// 			Box::new(
// 				MultiLocation::new(
// 					1,
// 					X2(
// 						Parachain(2000),
// 						Junction::AccountId32 {
// 							network: NetworkId::Any,
// 							id: ALICE.into(),
// 						}
// 					)
// 				)
// 				.into()
// 			),
// 			1_000_000_000,
// 		));

// 		assert_eq!(Tokens::free_balance(BNC, &karura_reserve_account()), 95_000_000_000_000);
// 		assert_eq!(Tokens::free_balance(BNC, &AccountId::from(BOB)), 4_989_760_000_000);
// 	});

// 	Karura::execute_with(|| {
// 		assert_eq!(Tokens::free_balance(BNC, &AccountId::from(ALICE)), 94_989_760_000_000);
// 	});
// }

/// Acala's tests
#[test]
#[ignore]
fn transfer_from_relay_chain_deposit_to_treasury_if_below_existential_deposit() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();

	KusamaRelay::execute_with(|| {
		assert_ok!(kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(PICASSO_PARA_ID).into().into()),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			Box::new((Here, 128_000_111 / 50).into()),
			0
		));
	});

	Picasso::execute_with(|| {
		assert_eq!(
			picasso_runtime::Tokens::free_balance(CurrencyId::KSM, &AccountId::from(BOB)),
			0
		);
		// TODO: add treasury like in Acala to get available payment even if it lower than needed to
		// treasury (add treasury call) assert_eq!(
		// 	picasso_runtime::Tokens::free_balance(
		// 		CurrencyId::KSM,
		// 		&picasso_runtime::TreasuryAccount::get()
		// 	),
		// 	1_000_128_000_111
		// );
	});
}

/// from: Acala
/// this test resonably iff we know ratio of KSM to PICA, if not, it should be rewritten to ensure
/// permissioned execution of some very specific action from other chains
#[test]
fn xcm_transfer_execution_barrier_trader_works() {
	crate::kusama_test_net::KusamaNetwork::reset();
	env_logger_init();

	let unit_instruction_weight = UnitWeightCost::get() / 50;
	assert!(unit_instruction_weight > 0, "barrier makes sence iff there is pay for messages");

	// relay-chain use normal account to send xcm, destination para-chain can't pass Barrier check
	let tiny = 100;
	let message = Xcm(vec![
		ReserveAssetDeposited((Parent, tiny).into()),
		BuyExecution { fees: (Parent, tiny).into(), weight_limit: Unlimited },
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
		assert!(picasso_runtime::System::events().iter().any(|r| {
			matches!(
				r.event,
				picasso_runtime::Event::DmpQueue(
					cumulus_pallet_dmp_queue::Event::ExecutedDownward(
						_,
						Outcome::Error(XcmError::Barrier)
					)
				)
			)
		}));
	});

	// AllowTopLevelPaidExecutionFrom barrier test case:
	// para-chain use XcmExecutor `execute_xcm()` method to execute xcm.
	// if `weight_limit` in BuyExecution is less than `xcm_weight(max_weight)`, then Barrier can't
	// pass. other situation when `weight_limit` is `Unlimited` or large than `xcm_weight`, then
	// it's ok.
	let expect_weight_limit = UnitWeightCost::get() * (MaxInstructions::get() as u64) * 100;
	let message = Xcm::<picasso_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, tiny).into()),
		BuyExecution { fees: (Parent, tiny).into(), weight_limit: Limited(100) },
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
	let ksm_per_second = UnitWeightCost::get() as u128 / 50 - 1_000; // TODO: define all calculation somehow in runtime as in Acala
	let message = Xcm::<picasso_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, ksm_per_second).into()),
		BuyExecution {
			fees: (Parent, ksm_per_second).into(),
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
				unit_instruction_weight * 2 * 50, /* so here we have report in PICA, while we
				                                   * allowed to pay in KSM */
				XcmError::TooExpensive
			)
		);
	});

	// all situation fulfilled, execute success
	let total = (unit_instruction_weight * MaxInstructions::get() as u64) as u128;
	let message = Xcm::<picasso_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, total).into()),
		BuyExecution { fees: (Parent, total).into(), weight_limit: Limited(expect_weight_limit) },
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	Picasso::execute_with(|| {
		let r = XcmExecutor::<picasso_runtime::XcmConfig>::execute_xcm(
			Parent,
			message,
			expect_weight_limit,
		);
		assert_eq!(r, Outcome::Complete(unit_instruction_weight * 3 * 50));
	});
}

/// source: Acala
#[test]
fn para_chain_subscribe_version_notify_of_sibling_chain() {
	KusamaNetwork::reset();
	env_logger_init();
	Picasso::execute_with(|| {
		let r = pallet_xcm::Pallet::<picasso_runtime::Runtime>::force_subscribe_version_notify(
			picasso_runtime::Origin::root(),
			Box::new((Parent, Parachain(DALI_PARA_ID)).into()),
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

/// source: Acala
#[test]
fn para_chain_subscribe_version_notify_of_relay_chain() {
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
}

/// source: Acala
#[test]
fn relay_chain_subscribe_version_notify_of_para_chain() {
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
}

// #[test]
// fn test_asset_registry_module() {
// 	TestNet::reset();

// 	fn karura_reserve_account() -> AccountId {
// 		use sp_runtime::traits::AccountIdConversion;
// 		polkadot_parachain::primitives::Sibling::from(2000).into_account()
// 	}

// 	Karura::execute_with(|| {
// 		// register foreign asset
// 		assert_ok!(AssetRegistry::register_foreign_asset(
// 			Origin::root(),
// 			Box::new(MultiLocation::new(1, X2(Parachain(2001), GeneralKey(KAR.encode()))).into()),
// 			Box::new(AssetMetadata {
// 				name: b"Sibling Token".to_vec(),
// 				symbol: b"ST".to_vec(),
// 				decimals: 12,
// 				minimal_balance: Balances::minimum_balance() / 10, // 10%
// 			})
// 		));

// 		assert_eq!(
// 			Tokens::free_balance(CurrencyId::ForeignAsset(0), &TreasuryAccount::get()),
// 			0
// 		);
// 	});

// 	Sibling::execute_with(|| {
// 		let _ = Balances::deposit_creating(&AccountId::from(BOB), 100_000_000_000_000);
// 		assert_eq!(Balances::free_balance(&karura_reserve_account()), 0);
// 		assert_eq!(Balances::free_balance(&AccountId::from(BOB)), 100_000_000_000_000);

// 		assert_ok!(XTokens::transfer(
// 			Origin::signed(BOB.into()),
// 			KAR,
// 			5_000_000_000_000,
// 			Box::new(
// 				MultiLocation::new(
// 					1,
// 					X2(
// 						Parachain(2000),
// 						Junction::AccountId32 {
// 							network: NetworkId::Any,
// 							id: ALICE.into(),
// 						}
// 					)
// 				)
// 				.into()
// 			),
// 			1_000_000_000,
// 		));

// 		assert_eq!(Balances::free_balance(&karura_reserve_account()), 5_000_000_000_000);
// 		assert_eq!(Balances::free_balance(&AccountId::from(BOB)), 95_000_000_000_000);
// 	});

// 	Karura::execute_with(|| {
// 		assert_eq!(
// 			Tokens::free_balance(CurrencyId::ForeignAsset(0), &AccountId::from(ALICE)),
// 			4_999_360_000_000
// 		);
// 		// ToTreasury
// 		assert_eq!(
// 			Tokens::free_balance(CurrencyId::ForeignAsset(0), &TreasuryAccount::get()),
// 			640_000_000
// 		);

// 		assert_ok!(XTokens::transfer(
// 			Origin::signed(ALICE.into()),
// 			CurrencyId::ForeignAsset(0),
// 			1_000_000_000_000,
// 			Box::new(
// 				MultiLocation::new(
// 					1,
// 					X2(
// 						Parachain(2001),
// 						Junction::AccountId32 {
// 							network: NetworkId::Any,
// 							id: BOB.into(),
// 						}
// 					)
// 				)
// 				.into()
// 			),
// 			1_000_000_000,
// 		));

// 		assert_eq!(
// 			Tokens::free_balance(CurrencyId::ForeignAsset(0), &AccountId::from(ALICE)),
// 			3_999_360_000_000
// 		);
// 	});

// 	Sibling::execute_with(|| {
// 		assert_eq!(Balances::free_balance(&karura_reserve_account()), 4_000_000_000_000);
// 		assert_eq!(Balances::free_balance(&AccountId::from(BOB)), 95_993_600_000_000);
// 	});

// 	// remove it
// 	Karura::execute_with(|| {
// 		// register foreign asset
// 		assert_ok!(AssetRegistry::update_foreign_asset(
// 			Origin::root(),
// 			0,
// 			Box::new(MultiLocation::new(1, X2(Parachain(2001), GeneralKey(KAR.encode()))).into()),
// 			Box::new(AssetMetadata {
// 				name: b"Sibling Token".to_vec(),
// 				symbol: b"ST".to_vec(),
// 				decimals: 12,
// 				minimal_balance: 0, // buy_weight 0
// 			})
// 		));
// 	});

// 	Sibling::execute_with(|| {
// 		assert_eq!(Balances::free_balance(&karura_reserve_account()), 4_000_000_000_000);
// 		assert_eq!(Balances::free_balance(&AccountId::from(BOB)), 95_993_600_000_000);

// 		assert_ok!(XTokens::transfer(
// 			Origin::signed(BOB.into()),
// 			KAR,
// 			5_000_000_000_000,
// 			Box::new(
// 				MultiLocation::new(
// 					1,
// 					X2(
// 						Parachain(2000),
// 						Junction::AccountId32 {
// 							network: NetworkId::Any,
// 							id: ALICE.into(),
// 						}
// 					)
// 				)
// 				.into()
// 			),
// 			1_000_000_000,
// 		));

// 		assert_eq!(Balances::free_balance(&karura_reserve_account()), 9_000_000_000_000);
// 		assert_eq!(Balances::free_balance(&AccountId::from(BOB)), 90_993_600_000_000);
// 	});

// 	Karura::execute_with(|| {
// 		assert_eq!(
// 			Tokens::free_balance(CurrencyId::ForeignAsset(0), &AccountId::from(ALICE)),
// 			8_999_360_000_000
// 		);

// 		// ToTreasury
// 		assert_eq!(
// 			Tokens::free_balance(CurrencyId::ForeignAsset(0), &TreasuryAccount::get()),
// 			640_000_000
// 		);
// 	});
// }

// #[test]
// fn unspent_xcm_fee_is_returned_correctly() {
// 	let mut parachain_account: AccountId = AccountId::default();
// 	let homa_lite_sub_account: AccountId =
// 		hex_literal::hex!["d7b8926b326dd349355a9a7cca6606c1e0eb6fd2b506066b518c7155ff0d8297"].into();
// 	Karura::execute_with(|| {
// 		parachain_account = ParachainAccount::get();
// 	});
// 	KusamaNet::execute_with(|| {
// 		assert_ok!(kusama_runtime::Balances::transfer(
// 			kusama_runtime::Origin::signed(ALICE.into()),
// 			MultiAddress::Id(homa_lite_sub_account.clone()),
// 			1_000 * dollar(RELAY_CHAIN_CURRENCY)
// 		));
// 		assert_ok!(kusama_runtime::Balances::transfer(
// 			kusama_runtime::Origin::signed(ALICE.into()),
// 			MultiAddress::Id(parachain_account.clone()),
// 			1_000 * dollar(RELAY_CHAIN_CURRENCY)
// 		));
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&AccountId::from(ALICE)),
// 			2 * dollar(RELAY_CHAIN_CURRENCY)
// 		);
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&homa_lite_sub_account),
// 			1_000 * dollar(RELAY_CHAIN_CURRENCY)
// 		);
// 		assert_eq!(kusama_runtime::Balances::free_balance(&AccountId::from(BOB)), 0);
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&parachain_account.clone()),
// 			1_002 * dollar(RELAY_CHAIN_CURRENCY)
// 		);
// 	});

// 	Karura::execute_with(|| {
// 		// Construct a transfer XCM call with returning the deposit
// 		let transfer_call = RelayChainCallBuilder::<Runtime,
// ParachainInfo>::balances_transfer_keep_alive( 			AccountId::from(BOB),
// 			dollar(NATIVE_CURRENCY),
// 		);
// 		let batch_call = RelayChainCallBuilder::<Runtime,
// ParachainInfo>::utility_as_derivative_call(transfer_call, 0); 		let weight = 10_000_000_000;
// 		// Fee to transfer into the hold register
// 		let asset = MultiAsset {
// 			id: Concrete(MultiLocation::here()),
// 			fun: Fungibility::Fungible(dollar(NATIVE_CURRENCY)),
// 		};
// 		let xcm_msg = Xcm(vec![
// 			WithdrawAsset(asset.clone().into()),
// 			BuyExecution {
// 				fees: asset,
// 				weight_limit: Unlimited,
// 			},
// 			Transact {
// 				origin_type: OriginKind::SovereignAccount,
// 				require_weight_at_most: weight,
// 				call: batch_call.encode().into(),
// 			},
// 		]);

// 		let res = PolkadotXcm::send_xcm(Here, Parent, xcm_msg);
// 		assert!(res.is_ok());
// 	});

// 	KusamaNet::execute_with(|| {
// 		// 1 dollar is transferred to BOB
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&homa_lite_sub_account),
// 			999 * dollar(RELAY_CHAIN_CURRENCY)
// 		);
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
// 			dollar(RELAY_CHAIN_CURRENCY)
// 		);
// 		// 1 dollar is given to Hold Register for XCM call and never returned.
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&parachain_account.clone()),
// 			1_001 * dollar(RELAY_CHAIN_CURRENCY)
// 		);
// 	});

// 	Karura::execute_with(|| {
// 		// Construct a transfer using the RelaychainCallBuilder
// 		let transfer_call = RelayChainCallBuilder::<Runtime,
// ParachainInfo>::balances_transfer_keep_alive( 			AccountId::from(BOB),
// 			dollar(NATIVE_CURRENCY),
// 		);
// 		let batch_call = RelayChainCallBuilder::<Runtime,
// ParachainInfo>::utility_as_derivative_call(transfer_call, 0); 		let finalized_call =
// RelayChainCallBuilder::<Runtime, ParachainInfo>::finalize_call_into_xcm_message( 			batch_call,
// 			dollar(NATIVE_CURRENCY),
// 			10_000_000_000,
// 		);

// 		let res = PolkadotXcm::send_xcm(Here, Parent, finalized_call);
// 		assert!(res.is_ok());
// 	});

// 	KusamaNet::execute_with(|| {
// 		// 1 dollar is transferred to BOB
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&homa_lite_sub_account),
// 			998 * dollar(RELAY_CHAIN_CURRENCY)
// 		);
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
// 			2 * dollar(RELAY_CHAIN_CURRENCY)
// 		);
// 		// Unspent fund from the 1 dollar XCM fee is returned to the sovereign account.
// 		assert_eq!(
// 			kusama_runtime::Balances::free_balance(&parachain_account.clone()),
// 			1_000 * dollar(RELAY_CHAIN_CURRENCY) + 999_626_666_690
// 		);
// 	});
// }

// #[test]
// fn trap_assets_larger_than_ed_works() {
// 	TestNet::reset();

// 	let mut kar_treasury_amount = 0;
// 	let (ksm_asset_amount, kar_asset_amount) = (dollar(KSM), dollar(KAR));
// 	let trader_weight_to_treasury: u128 = 96_000_000;

// 	Karura::execute_with(|| {
// 		assert_ok!(Tokens::deposit(KSM, &AccountId::from(DEFAULT), 100 * dollar(KSM)));
// 		let _ = pallet_balances::Pallet::<Runtime>::deposit_creating(&AccountId::from(DEFAULT), 100 *
// dollar(KAR));

// 		kar_treasury_amount = Currencies::free_balance(KAR, &KaruraTreasuryAccount::get());
// 	});

// 	let assets: MultiAsset = (Parent, ksm_asset_amount).into();
// 	KusamaNet::execute_with(|| {
// 		let xcm = vec![
// 			WithdrawAsset(assets.clone().into()),
// 			BuyExecution {
// 				fees: assets,
// 				weight_limit: Limited(dollar(KSM) as u64),
// 			},
// 			WithdrawAsset(
// 				(
// 					(Parent, X2(Parachain(2000), GeneralKey(KAR.encode()))),
// 					kar_asset_amount,
// 				)
// 					.into(),
// 			),
// 		];
// 		assert_ok!(pallet_xcm::Pallet::<kusama_runtime::Runtime>::send_xcm(
// 			Here,
// 			Parachain(2000).into(),
// 			Xcm(xcm),
// 		));
// 	});
// 	Karura::execute_with(|| {
// 		assert!(System::events()
// 			.iter()
// 			.any(|r| matches!(r.event, Event::PolkadotXcm(pallet_xcm::Event::AssetsTrapped(_, _, _)))));

// 		assert_eq!(
// 			trader_weight_to_treasury + dollar(KSM),
// 			Currencies::free_balance(KSM, &KaruraTreasuryAccount::get())
// 		);
// 		assert_eq!(
// 			kar_treasury_amount,
// 			Currencies::free_balance(KAR, &KaruraTreasuryAccount::get())
// 		);
// 	});
// }

// from Acala
#[test]
#[ignore]
fn trap_assets_lower_than_existential_deposit_works() {
	simtest();

	let other_non_native_amount = 1_000_000_000;
	let some_native_amount = 1_000_000_000;
	let this_liveness_native_amount = 1_000_000;
	let any_asset = CurrencyId::KSM;
	let this_native_asset = CurrencyId::PICA;

	let this_treasury_amount = Picasso::execute_with(|| {
		assert_ok!(Assets::deposit(any_asset, &AccountId::from(CHARLIE), other_non_native_amount));
		let _ =
			<picasso_runtime::Balances as support::traits::Currency<AccountId>>::deposit_creating(
				&AccountId::from(CHARLIE),
				some_native_amount,
			);
		<Assets as MultiCurrency<AccountId>>::free_balance(
			this_native_asset,
			&picasso_runtime::TreasuryAccount::get(),
		)
	});

	let assets: MultiAsset = (Parent, other_non_native_amount).into();
	KusamaRelay::execute_with(|| {
		let xcm = vec![
			WithdrawAsset(assets.clone().into()),
			BuyExecution { fees: assets, weight_limit: Limited(other_non_native_amount as u64) },
			WithdrawAsset(
				(
					(
						Parent,
						X2(Parachain(PICASSO_PARA_ID), GeneralKey(this_native_asset.encode())),
					),
					some_native_amount,
				)
					.into(),
			),
			// two asset left in holding register, they both lower than ED, so goes to treasury.
		];
		assert_ok!(pallet_xcm::Pallet::<kusama_runtime::Runtime>::send_xcm(
			Here,
			Parachain(PICASSO_PARA_ID).into(),
			Xcm(xcm),
		));
	});

	Picasso::execute_with(|| {
		assert_eq!(
			System::events().iter().find(|r| matches!(
				r.event,
				picasso_runtime::Event::RelayerXcm(pallet_xcm::Event::AssetsTrapped(_, _, _))
			)),
			None
		);

		assert_eq!(
			other_non_native_amount + other_non_native_amount,
			<Assets as MultiCurrency<AccountId>>::free_balance(
				any_asset,
				&picasso_runtime::TreasuryAccount::get()
			)
		);
		assert_eq!(
			some_native_amount,
			<Assets as MultiCurrency<AccountId>>::free_balance(
				this_native_asset,
				&picasso_runtime::TreasuryAccount::get()
			) - this_treasury_amount
		);
	});
}

// From Acala
#[test]
fn sibling_trap_assets_works() {
	simtest();

	let sibling_non_native_amount = 1_000_000_000;
	let some_native_amount = 1_000_000_000;
	let this_liveness_native_amount = BaseXcmWeight::get() as u128 *
		100 * UnitWeightCost::get() as Balance *
		MaxInstructions::get() as Balance;
	let any_asset = CurrencyId::LAYR;
	let this_native_asset = CurrencyId::PICA;

	fn sibling_account() -> AccountId {
		use sp_runtime::traits::AccountIdConversion;
		polkadot_parachain::primitives::Sibling::from(SIBLING_PARA_ID).into_account()
	}

	let this_native_treasury_amount = Picasso::execute_with(|| {
		assert_ok!(Assets::deposit(any_asset, &sibling_account(), sibling_non_native_amount));
		let _ =
			<balances::Pallet<Runtime> as support::traits::Currency<AccountId>>::deposit_creating(
				&sibling_account(),
				this_liveness_native_amount,
			);
		let _ =
			<balances::Pallet<Runtime> as support::traits::Currency<AccountId>>::deposit_creating(
				&picasso_runtime::TreasuryAccount::get(),
				this_liveness_native_amount,
			);
		<balances::Pallet<Runtime> as support::traits::Currency<AccountId>>::free_balance(
			&picasso_runtime::TreasuryAccount::get(),
		)
	});

	let remote = composable_traits::assets::XcmAssetLocation(MultiLocation::new(
		1,
		X2(Parachain(SIBLING_PARA_ID), GeneralKey(CurrencyId::LAYR.encode())),
	));

	Picasso::execute_with(|| {
		assert_ok!(picasso_runtime::AssetsRegistry::set_location(any_asset, remote,));
	});

	// buy execution via native token, and try withdraw on this some amount
	Sibling::execute_with(|| {
		let assets: MultiAsset = (
			(Parent, X2(Parachain(PICASSO_PARA_ID), GeneralKey(this_native_asset.encode()))),
			some_native_amount,
		)
			.into();
		let xcm = vec![
			WithdrawAsset(assets.clone().into()), /* withdrow native on target chain from origin
			                                       * account */
			BuyExecution {
				// pay for origin account
				fees: assets,
				weight_limit: Unlimited,
			},
			WithdrawAsset(
				(
					(Parent, X2(Parachain(SIBLING_PARA_ID), GeneralKey(any_asset.encode()))),
					sibling_non_native_amount,
				) // withdraw into VM holder asset, and do nothing...
					.into(),
			),
		];
		assert_ok!(pallet_xcm::Pallet::<Runtime>::send_xcm(
			Here,
			(Parent, Parachain(PICASSO_PARA_ID)),
			Xcm(xcm),
		));
	});

	Picasso::execute_with(|| {
		assert_eq!(
			System::events().iter().find(|r| matches!(
				r.event,
				picasso_runtime::Event::RelayerXcm(pallet_xcm::Event::AssetsTrapped(_, _, _))
			)),
			None // non of assets trapped by hash, because all are known
		);
		assert_eq!(
			picasso_runtime::Assets::free_balance(
				any_asset,
				&picasso_runtime::TreasuryAccount::get()
			),
			sibling_non_native_amount
		);
		
		assert_eq!(
			picasso_runtime::Balances::free_balance(&picasso_runtime::TreasuryAccount::get()),
			some_native_amount + this_native_treasury_amount, 
		);
	});
}

pub fn simtest() {
	KusamaNetwork::reset();
	env_logger_init();
}
