// TODO:
// Withdraw assets and trap it via polka xcm trap- if will fail always
// let does_not_exists= u128::MAX-1;
// (0, GeneralKey(does_not_exists.encode())),
// Because Convert  will not find assets and execution will never reach to AssetsTrapped
// cannot handle it because of ORML design
//pallet_xcm::Event::AssetsTrapped

use crate::{
	helpers::*,
	kusama_test_net::{
		KusamaRelay, Sibling, This, ALICE, ALICE_PARACHAIN_KSM, BOB, CHARLIE, PICA,
		SIBLING_PARA_ID, THIS_PARA_ID,
	},
	prelude::*,
};
use codec::Encode;
use common::{AccountId, Balance};
use composable_traits::assets::{RemoteAssetRegistry, XcmAssetLocation};

use this_runtime::{
	Assets, AssetsRegistry, Balances, MaxInstructions, Origin, Runtime, System, Tokens,
	UnitWeightCost, XTokens,
};

use num_traits::Zero;
use orml_traits::currency::MultiCurrency;

use primitives::currency::*;
use sp_runtime::{assert_eq_error_rate, traits::AccountIdConversion, MultiAddress};
use support::{assert_ok, log};
use xcm::latest::prelude::*;
use xcm_builder::ParentIsPreset;
use xcm_emulator::TestExt;
use xcm_executor::{traits::Convert, XcmExecutor};

#[test]
fn reserve_transfer_from_relay_alice_bob() {
	simtest();
	let from = ALICE;
	let to = BOB;
	reserve_transfer(from, to);
}

#[test]
fn reserve_transfer_from_relay_alice_alice() {
	simtest();
	let from = ALICE;
	let to = ALICE;
	reserve_transfer(from, to);
}

#[test]
fn reserve_transfer_from_relay_map() {
	simtest();
	let from = ALICE;
	let to = BOB;
	This::execute_with(|| {
		assert_ok!(this_runtime::AssetsRegistry::set_location(
			CurrencyId::KSM, // KSM id as it is locally
			// if we get tokens from parent chain, these can be only native token
			XcmAssetLocation::RELAY_NATIVE,
		));
	});

	reserve_transfer(from, to);
}

/// how it works:
/// top level ReserveTransfer instruction is interprtered first on sending chain
/// it transfers amount from sender account to target chain account on sending chain
///  send it custs wrapper part of XCM message, and sends remaining with deposit
/// target chain sees deposit amount and mints approciate amount
/// validats origin of reserve (must be relay)
fn reserve_transfer(from: [u8; 32], to: [u8; 32]) {
	let from_account = &AccountId::from(from);
	let to_account = &AccountId::from(to);
	let balance = enough_weigth();
	let before =
		This::execute_with(|| this_runtime::Assets::free_balance(CurrencyId::KSM, to_account));
	KusamaRelay::execute_with(|| {
		<kusama_runtime::Balances as support::traits::Currency<_>>::deposit_creating(
			from_account,
			balance,
		);
		let result = kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::Origin::signed(from.into()),
			Box::new(Parachain(THIS_PARA_ID).into().into()),
			Box::new(Junction::AccountId32 { id: to, network: NetworkId::Any }.into().into()),
			Box::new((Here, balance).into()),
			0,
		);
		assert_ok!(result);
		relay_dump_events();
	});
	This::execute_with(|| {
		let new_balance = this_runtime::Assets::free_balance(CurrencyId::KSM, to_account);
		dump_events();
		assert_eq_error_rate!(new_balance, before + balance, (UnitWeightCost::get() * 10) as u128);
		assert!(!this_runtime::System::events()
			.iter()
			.any(|r| { matches!(r.event, this_runtime::Event::XTokens(_)) }));
	});
}

#[test]
fn transfer_to_relay_chain() {
	simtest();
	This::execute_with(|| {
		assert_ok!(<this_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::KSM,
			XcmAssetLocation::RELAY_NATIVE,
		));
		let transferred = this_runtime::XTokens::transfer(
			this_runtime::Origin::signed(ALICE.into()),
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
			this_runtime::Assets::free_balance(CurrencyId::KSM, &AccountId::from(ALICE));

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
fn transfer_from_this_to_sibling() {
	simtest();

	This::execute_with(|| {
		assert_ok!(<this_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::PICA,
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(SIBLING_PARA_ID), GeneralKey(CurrencyId::PICA.encode()))
			))
		));
	});

	let local_withdraw_amount = 3 * PICA;
	Sibling::execute_with(|| {
		assert_ok!(sibling_runtime::XTokens::transfer(
			sibling_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			local_withdraw_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(THIS_PARA_ID),
						Junction::AccountId32 { id: BOB, network: NetworkId::Any }
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(
			sibling_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(ALICE)),
			200 * PICA - local_withdraw_amount
		);
	});

	This::execute_with(|| {
		let balance = this_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(BOB));
		assert_eq_error_rate!(balance, local_withdraw_amount, (UnitWeightCost::get() * 10) as u128);
	});
}

#[test]
fn transfer_from_sibling_to_this() {
	simtest();

	Sibling::execute_with(|| {
		assert_ok!(<sibling_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			// local id
			CurrencyId::PICA,
			// remote id
			// first part is remote network,
			// second part is id of asset on remote
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(THIS_PARA_ID), GeneralKey(CurrencyId::PICA.encode()))
			))
		));
	});

	This::execute_with(|| {
		assert_ok!(<this_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::PICA,
			composable_traits::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(SIBLING_PARA_ID), GeneralKey(CurrencyId::PICA.encode()))
			))
		));

		assert_ok!(this_runtime::XTokens::transfer(
			this_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			3 * PICA,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(SIBLING_PARA_ID),
						Junction::AccountId32 { id: BOB, network: NetworkId::Any }
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(
			this_runtime::Balances::free_balance(&AccountId::from(ALICE)),
			200 * PICA - 3 * PICA
		);
	});

	Sibling::execute_with(|| {
		let balance =
			sibling_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(BOB));
		assert_eq_error_rate!(balance, 3 * PICA, (UnitWeightCost::get() * 10) as u128);
	});
}

// from: Hydra
#[test]
fn transfer_insufficient_amount_should_fail() {
	simtest();
	Sibling::execute_with(|| {
		assert_ok!(sibling_runtime::XTokens::transfer(
			sibling_runtime::Origin::signed(ALICE.into()),
			CurrencyId::PICA,
			1_000_000 - 1,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(THIS_PARA_ID),
						Junction::AccountId32 { id: BOB, network: NetworkId::Any }
					)
				)
				.into()
			),
			399_600_000_000
		));
		assert_eq!(
			sibling_runtime::Balances::free_balance(&AccountId::from(ALICE)),
			199999999000001
		);
	});

	This::execute_with(|| {
		// Xcm should fail therefore nothing should be deposit into beneficiary account
		assert_eq!(this_runtime::Tokens::free_balance(CurrencyId::PICA, &AccountId::from(BOB)), 0);
	});
}

#[test]
#[ignore = "until fixed sibling trust map"]
fn transfer_to_sibling() {
	simtest();
	let _other_currency = CurrencyId::KSM;
	fn this_native_reserve_account() -> AccountId {
		use sp_runtime::traits::AccountIdConversion;
		polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account()
	}

	let alice_original = This::execute_with(|| {
		assert_ok!(Tokens::deposit(CurrencyId::KSM, &AccountId::from(ALICE), 100_000_000_000_000));
		Tokens::free_balance(CurrencyId::KSM, &AccountId::from(ALICE))
	});
	let alice_from_amount = alice_original / 10;
	let alice_remaining = alice_original - alice_from_amount;
	let weight_to_pay = (alice_from_amount / 2) as u64;

	let picasso_on_sibling = Sibling::execute_with(|| {
		assert_ok!(Tokens::deposit(
			CurrencyId::KSM,
			&this_native_reserve_account(),
			100 * CurrencyId::unit::<Balance>(),
		));
		Tokens::free_balance(CurrencyId::KSM, &this_native_reserve_account())
	});

	assert_ne!(picasso_on_sibling, Balance::zero());

	This::execute_with(|| {
		assert_ok!(XTokens::transfer(
			Origin::signed(ALICE.into()),
			CurrencyId::KSM,
			alice_from_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(SIBLING_PARA_ID),
						Junction::AccountId32 { network: NetworkId::Any, id: BOB.into() }
					)
				)
				.into()
			),
			weight_to_pay,
		));

		assert_eq!(Tokens::free_balance(CurrencyId::KSM, &AccountId::from(ALICE)), alice_remaining);
	});

	// TODO: also XCM not failes, it really fails with not enough balance, not clear so what balance
	// is needed to transfer
	Sibling::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(CurrencyId::KSM, &this_native_reserve_account()),
			picasso_on_sibling
		);
		assert_eq!(Tokens::free_balance(CurrencyId::KSM, &AccountId::from(BOB)), 9_989_760_000_000);

		assert_ok!(XTokens::transfer(
			Origin::signed(BOB.into()),
			CurrencyId::KSM,
			5_000_000_000_000,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(THIS_PARA_ID),
						Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }
					)
				)
				.into()
			),
			1_000_000_000,
		));

		assert_eq!(
			Tokens::free_balance(CurrencyId::KSM, &this_native_reserve_account()),
			95_000_000_000_000
		);
		assert_eq!(Tokens::free_balance(CurrencyId::KSM, &AccountId::from(BOB)), 4_989_760_000_000);
	});

	This::execute_with(|| {
		assert_eq!(
			Tokens::free_balance(CurrencyId::KSM, &AccountId::from(ALICE)),
			94_989_760_000_000
		);
	});
}

/// if Bob sends amount of his tokens and these are above weigth but less than ED,
/// than our treasury takes that amount, sorry Bob
/// Acala's tests
#[test]
fn transfer_from_relay_chain_deposit_to_treasury_if_below_existential_deposit() {
	simtest();
	let amount = under_existential_deposit(LocalAssetId::KSM, 3);
	let receiver = CHARLIE;
	let picasso_treasury = This::execute_with(|| {
		assert_eq!(
			this_runtime::Tokens::free_balance(CurrencyId::KSM, &AccountId::from(receiver)),
			0,
		);
		this_runtime::Tokens::free_balance(CurrencyId::KSM, &this_runtime::TreasuryAccount::get())
	});

	KusamaRelay::execute_with(|| {
		assert_ok!(kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(THIS_PARA_ID).into().into()),
			Box::new(Junction::AccountId32 { id: receiver, network: NetworkId::Any }.into().into()),
			Box::new((Here, amount).into()),
			0
		));
	});

	This::execute_with(|| {
		assert_eq!(
			this_runtime::Tokens::free_balance(CurrencyId::KSM, &AccountId::from(receiver)),
			0,
			"assets did not get to recepient as it is not enough to pay ED"
		);
		assert_eq!(
			this_runtime::Tokens::free_balance(
				CurrencyId::KSM,
				&this_runtime::TreasuryAccount::get()
			),
			amount - picasso_treasury
		);
	});
}

/// from: Acala
/// this test resonably iff we know ratio of KSM to PICA, if not, it should be rewritten to ensure
/// permissioned execution of some very specific action from other chains
#[test]
fn xcm_transfer_execution_barrier_trader_works() {
	simtest();

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
			Box::new(Parachain(THIS_PARA_ID).into().into()),
			Box::new(xcm::VersionedXcm::from(message)),
		);
		assert_ok!(r);
	});
	This::execute_with(|| {
		assert!(this_runtime::System::events().iter().any(|r| {
			matches!(
				r.event,
				this_runtime::Event::DmpQueue(cumulus_pallet_dmp_queue::Event::ExecutedDownward(
					_,
					Outcome::Error(XcmError::Barrier)
				))
			)
		}));
	});

	// AllowTopLevelPaidExecutionFrom barrier test case:
	// para-chain use XcmExecutor `execute_xcm()` method to execute xcm.
	// if `weight_limit` in BuyExecution is less than `xcm_weight(max_weight)`, then Barrier can't
	// pass. other situation when `weight_limit` is `Unlimited` or large than `xcm_weight`, then
	// it's ok.
	let expect_weight_limit = UnitWeightCost::get() * (MaxInstructions::get() as u64) * 100;
	let message = Xcm::<this_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, tiny).into()),
		BuyExecution { fees: (Parent, tiny).into(), weight_limit: Limited(100) },
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	This::execute_with(|| {
		let r = XcmExecutor::<this_runtime::XcmConfig>::execute_xcm(
			Parent,
			message,
			expect_weight_limit,
		);
		assert_eq!(r, Outcome::Error(XcmError::Barrier));
	});

	// trader inside BuyExecution have TooExpensive error if payment less than calculated weight
	// amount. the minimum of calculated weight amount(`FixedRateOfFungible<KsmPerSecond>`) is
	let ksm_per_second = UnitWeightCost::get() as u128 / 50 - 1_000; // TODO: define all calculation somehow in runtime as in Acala
	let message = Xcm::<this_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, ksm_per_second).into()),
		BuyExecution {
			fees: (Parent, ksm_per_second).into(),
			weight_limit: Limited(expect_weight_limit),
		},
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	This::execute_with(|| {
		let r = XcmExecutor::<this_runtime::XcmConfig>::execute_xcm(
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
	let message = Xcm::<this_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, total).into()),
		BuyExecution { fees: (Parent, total).into(), weight_limit: Limited(expect_weight_limit) },
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	This::execute_with(|| {
		let r = XcmExecutor::<this_runtime::XcmConfig>::execute_xcm(
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
	simtest();
	This::execute_with(|| {
		let r = pallet_xcm::Pallet::<this_runtime::Runtime>::force_subscribe_version_notify(
			this_runtime::Origin::root(),
			Box::new((Parent, Parachain(SIBLING_PARA_ID)).into()),
		);
		assert_ok!(r);
	});
	This::execute_with(|| {
		assert!(this_runtime::System::events().iter().any(|r| matches!(
			r.event,
			this_runtime::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent(
				Some(_)
			))
		)));
	});
	Sibling::execute_with(|| {
		assert!(sibling_runtime::System::events().iter().any(|r| matches!(
			r.event,
			this_runtime::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::XcmpMessageSent(
				Some(_)
			)) | this_runtime::Event::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success(Some(_)))
		)));
	});
}

/// source: Acala
#[test]
fn para_chain_subscribe_version_notify_of_relay_chain() {
	This::execute_with(|| {
		let r = pallet_xcm::Pallet::<this_runtime::Runtime>::force_subscribe_version_notify(
			this_runtime::Origin::root(),
			Box::new(Parent.into()),
		);
		assert_ok!(r);
	});
	This::execute_with(|| {
		this_runtime::System::assert_has_event(this_runtime::Event::RelayerXcm(
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
			Box::new(Parachain(THIS_PARA_ID).into().into()),
		);
		assert_ok!(r);
	});
	KusamaRelay::execute_with(|| {
		kusama_runtime::System::assert_has_event(kusama_runtime::Event::XcmPallet(
			pallet_xcm::Event::SupportedVersionChanged(
				MultiLocation { parents: 0, interior: X1(Parachain(THIS_PARA_ID)) },
				2,
			),
		));
	});
}

#[test]
fn test_assets_registry_module() {
	simtest();

	let local_asset = CurrencyId::PICA;
	let foreign_asset = CurrencyId::PICA;

	fn picasso_reserve_account() -> AccountId {
		use sp_runtime::traits::AccountIdConversion;
		polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account()
	}

	This::execute_with(|| {
		let local_admin = AccountId::from(ALICE);
		let foreign_admin = AccountId::from(BOB);
		let decimals = 12;
		let location = XcmAssetLocation(
			MultiLocation::new(
				1,
				X2(Parachain(SIBLING_PARA_ID), GeneralKey(CurrencyId::PICA.encode())),
			)
			.into(),
		);

		assert_ok!(AssetsRegistry::set_local_admin(Origin::root(), local_admin.clone()));
		assert_ok!(AssetsRegistry::set_foreign_admin(Origin::root(), foreign_admin.clone()));

		assert_ok!(AssetsRegistry::approve_assets_mapping_candidate(
			Origin::signed(local_admin),
			local_asset,
			foreign_asset,
			location.clone(),
			decimals,
		));
		assert_ok!(AssetsRegistry::approve_assets_mapping_candidate(
			Origin::signed(foreign_admin),
			local_asset,
			foreign_asset,
			location,
			decimals,
		));
	});

	Sibling::execute_with(|| {
		let local_admin = AccountId::from(ALICE);
		let foreign_admin = AccountId::from(BOB);
		let decimals = 12;
		let location = XcmAssetLocation(
			MultiLocation::new(
				1,
				X2(Parachain(THIS_PARA_ID), GeneralKey(CurrencyId::PICA.encode())),
			)
			.into(),
		);

		assert_ok!(AssetsRegistry::set_local_admin(Origin::root(), local_admin.clone()));
		assert_ok!(AssetsRegistry::set_foreign_admin(Origin::root(), foreign_admin.clone()));

		assert_ok!(AssetsRegistry::approve_assets_mapping_candidate(
			Origin::signed(local_admin),
			foreign_asset,
			local_asset,
			location.clone(),
			decimals,
		));
		assert_ok!(AssetsRegistry::approve_assets_mapping_candidate(
			Origin::signed(foreign_admin),
			foreign_asset,
			local_asset,
			location,
			decimals,
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
			foreign_asset,
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

#[test]
fn unspent_xcm_fee_is_returned_correctly() {
	let parachain_account: AccountId =
		This::execute_with(|| this_runtime::ParachainInfo::parachain_id().into_account());
	let some_account: AccountId = AccountId::from(CHARLIE);

	KusamaRelay::execute_with(|| {
		assert_ok!(kusama_runtime::Balances::transfer(
			kusama_runtime::Origin::signed(ALICE.into()),
			MultiAddress::Id(some_account.clone()),
			1_000 * CurrencyId::unit::<Balance>()
		));
		assert_ok!(kusama_runtime::Balances::transfer(
			kusama_runtime::Origin::signed(ALICE.into()),
			MultiAddress::Id(parachain_account.clone()),
			1_000 * CurrencyId::unit::<Balance>()
		));
		assert_eq!(
			kusama_runtime::Balances::free_balance(&AccountId::from(ALICE)),
			2 * CurrencyId::unit::<Balance>()
		);
		assert_eq!(
			kusama_runtime::Balances::free_balance(&some_account),
			1_000 * CurrencyId::unit::<Balance>()
		);
		assert_eq!(kusama_runtime::Balances::free_balance(&AccountId::from(BOB)), 0);
		assert_eq!(
			kusama_runtime::Balances::free_balance(&parachain_account.clone()),
			1_010 * CurrencyId::unit::<Balance>()
		);
	});

	This::execute_with(|| {
		// Construct a transfer XCM call with returning the deposit
		let transfer_call = crate::relaychain::balances_transfer_keep_alive::<Runtime>(
			AccountId::from(BOB),
			CurrencyId::unit(),
		);
		let batch_call = crate::relaychain::utility_as_derivative_call::<Runtime>(transfer_call, 0);
		let weight = 10_000_000_000; // Fee to transfer into the hold register
		let asset = MultiAsset {
			id: Concrete(MultiLocation::here()),
			fun: Fungibility::Fungible(CurrencyId::unit()),
		};
		let xcm_msg = Xcm(vec![
			WithdrawAsset(asset.clone().into()),
			BuyExecution { fees: asset, weight_limit: Unlimited },
			Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: weight,
				call: batch_call.encode().into(),
			},
		]);

		let res = this_runtime::RelayerXcm::send_xcm(Here, Parent, xcm_msg);
		assert!(res.is_ok());
	});

	KusamaRelay::execute_with(|| {
		// 1 dollar is transferred to BOB
		assert_eq!(
			kusama_runtime::Balances::free_balance(&some_account),
			1000 * CurrencyId::unit::<Balance>()
		);
		// ISSUE: ported from Acala, not clear how BOB at all got s amount as we never transfer that
		//there is no transfer of KSM at all
		// assert_eq!(
		// 	kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
		// 	CurrencyId::unit::<Balance>()
		// );
		// 1 dollar is given to Hold Register for XCM call and never returned.
		assert_eq!(
			kusama_runtime::Balances::free_balance(&parachain_account.clone()),
			1_009 * CurrencyId::unit::<Balance>()
		);
	});

	This::execute_with(|| {
		// Construct a transfer using the RelaychainCallBuilder
		let transfer_call = crate::relaychain::balances_transfer_keep_alive::<Runtime>(
			AccountId::from(BOB),
			CurrencyId::unit(),
		);
		let batch_call = crate::relaychain::utility_as_derivative_call::<Runtime>(transfer_call, 0);
		let finalized_call = crate::relaychain::finalize_call_into_xcm_message::<Runtime>(
			batch_call,
			CurrencyId::unit(),
			10_000_000_000,
		);

		let res = this_runtime::RelayerXcm::send_xcm(Here, Parent, finalized_call);
		assert!(res.is_ok());
	});

	KusamaRelay::execute_with(|| {
		// 1 dollar is transferred to BOB
		assert_eq!(
			kusama_runtime::Balances::free_balance(&some_account),
			1_000 * CurrencyId::unit::<Balance>()
		);
		// ISSUE: ported from Acala, not clear how BOB at all got s amount as we never transfer that
		// there is no transfer of KSM at all
		// assert_eq!(
		// 	kusama_runtime::Balances::free_balance(&AccountId::from(BOB)),
		// 	2 * CurrencyId::unit::<Balance>()
		// );
		// Unspent fund from the 1 dollar XCM fee is returned to the sovereign account.
		assert_eq!(
			kusama_runtime::Balances::free_balance(&parachain_account.clone()),
			1_000 * CurrencyId::unit::<Balance>() + 8_999_626_666_690
		);
	});
}

// from Acala
#[test]
fn trap_assets_larger_than_ed_works() {
	simtest();

	let mut native_treasury_amount = 0;
	let (ksm_asset_amount, native_asset_amount) =
		(3 * CurrencyId::unit::<Balance>(), 2 * CurrencyId::unit::<Balance>());
	let parent_account: AccountId = ParentIsPreset::<AccountId>::convert(Parent.into()).unwrap();
	This::execute_with(|| {
		assert_ok!(Tokens::deposit(
			CurrencyId::KSM,
			&parent_account,
			42 * CurrencyId::unit::<Balance>()
		));
		let _ =
			<balances::Pallet<Runtime> as support::traits::Currency<AccountId>>::deposit_creating(
				&parent_account,
				123 * CurrencyId::unit::<Balance>(),
			);
		// TODO: if we do not top up account initially, than any depositn_creating do not create
		// anything may be somethign with zero block or like - fix it better way
		let _ =
			<balances::Pallet<Runtime> as support::traits::Currency<AccountId>>::deposit_creating(
				&this_runtime::TreasuryAccount::get(),
				7 * CurrencyId::unit::<Balance>(),
			);

		native_treasury_amount =
			Assets::free_balance(CurrencyId::PICA, &this_runtime::TreasuryAccount::get());
	});

	let assets: MultiAsset = (Parent, ksm_asset_amount).into();
	KusamaRelay::execute_with(|| {
		let xcm = vec![
			WithdrawAsset(assets.clone().into()),
			BuyExecution {
				fees: assets,
				weight_limit: Limited(CurrencyId::unit::<Balance>() as u64),
			},
			WithdrawAsset(((0, GeneralKey(CurrencyId::PICA.encode())), native_asset_amount).into()),
		];
		assert_ok!(pallet_xcm::Pallet::<kusama_runtime::Runtime>::send_xcm(
			Here,
			Parachain(THIS_PARA_ID).into(),
			Xcm(xcm),
		));
	});

	This::execute_with(|| {
		assert_eq!(
			3 * CurrencyId::unit::<Balance>(),
			Assets::free_balance(CurrencyId::KSM, &this_runtime::TreasuryAccount::get())
		);
		log::error!("{:?}", &this_runtime::TreasuryAccount::get());
		assert_eq!(
			native_asset_amount,
			this_runtime::Balances::free_balance(&this_runtime::TreasuryAccount::get()) -
				7 * CurrencyId::unit::<Balance>(),
		);
	});
}

// from Acala
#[test]
fn trap_assets_lower_than_existential_deposit_works() {
	simtest();

	let other_non_native_amount = 1_000_000_000_000;
	let some_native_amount = 1_000_000_000_000_000;
	let any_asset = CurrencyId::KSM;
	let this_native_asset = CurrencyId::PICA;

	let parent_account: AccountId = ParentIsPreset::<AccountId>::convert(Parent.into()).unwrap();

	let (this_treasury_amount, other_treasury_amount) = This::execute_with(|| {
		assert_ok!(Assets::deposit(any_asset, &parent_account, other_non_native_amount));
		let _ = <this_runtime::Balances as support::traits::Currency<AccountId>>::deposit_creating(
			&parent_account,
			some_native_amount,
		);
		(
			<Assets as MultiCurrency<AccountId>>::free_balance(
				this_native_asset,
				&this_runtime::TreasuryAccount::get(),
			),
			<Assets as MultiCurrency<AccountId>>::free_balance(
				any_asset,
				&this_runtime::TreasuryAccount::get(),
			),
		)
	});

	let assets: MultiAsset = (Parent, other_non_native_amount).into();
	KusamaRelay::execute_with(|| {
		let xcm = vec![
			WithdrawAsset(assets.clone().into()),
			BuyExecution { fees: assets, weight_limit: Limited(other_non_native_amount as u64) },
			WithdrawAsset(
				(
					(Parent, X2(Parachain(THIS_PARA_ID), GeneralKey(this_native_asset.encode()))),
					some_native_amount,
				)
					.into(),
			),
			//two asset left in holding register, they both lower than ED, so goes to treasury.
		];
		assert_ok!(pallet_xcm::Pallet::<kusama_runtime::Runtime>::send_xcm(
			Here,
			Parachain(THIS_PARA_ID).into(),
			Xcm(xcm),
		));
	});

	This::execute_with(|| {
		assert_eq!(
			System::events().iter().find(|r| matches!(
				r.event,
				this_runtime::Event::RelayerXcm(pallet_xcm::Event::AssetsTrapped(_, _, _))
			)),
			None
		);

		assert_eq!(
			some_native_amount,
			<Assets as MultiCurrency<AccountId>>::free_balance(
				this_native_asset,
				&this_runtime::TreasuryAccount::get()
			) - this_treasury_amount
		);

		assert_eq!(
			other_non_native_amount,
			<Assets as MultiCurrency<AccountId>>::free_balance(
				any_asset,
				&this_runtime::TreasuryAccount::get()
			) - other_treasury_amount
		);
	});
}

// From Acala
#[test]
fn sibling_trap_assets_works() {
	simtest();

	let any_asset = CurrencyId::kUSD;
	// TODO: create  foregn asset via factory
	// TODO: set key for it to allow transfer
	// TODO: parametriz test. ISSUE: how to solve DEX swap paying for transfer?
	let sibling_non_native_amount = assert_above_deposit(any_asset, 100_000_000_000);
	let some_native_amount = 1_000_000_000;
	let this_liveness_native_amount = enough_weigth();
	let this_native_asset = CurrencyId::PICA;

	let this_native_treasury_amount = This::execute_with(|| {
		assert_ok!(Assets::deposit(any_asset, &sibling_account(), sibling_non_native_amount));
		let _ =
			<balances::Pallet<Runtime> as support::traits::Currency<AccountId>>::deposit_creating(
				&sibling_account(),
				this_liveness_native_amount,
			);
		let _ =
			<balances::Pallet<Runtime> as support::traits::Currency<AccountId>>::deposit_creating(
				&this_runtime::TreasuryAccount::get(),
				this_liveness_native_amount,
			);
		<balances::Pallet<Runtime> as support::traits::Currency<AccountId>>::free_balance(
			&this_runtime::TreasuryAccount::get(),
		)
	});

	let remote = composable_traits::assets::XcmAssetLocation(MultiLocation::new(
		1,
		X2(Parachain(SIBLING_PARA_ID), GeneralKey(any_asset.encode())),
	));

	This::execute_with(|| {
		assert_ok!(this_runtime::AssetsRegistry::set_location(any_asset, remote,));
	});

	// buy execution via native token, and try withdraw on this some amount
	Sibling::execute_with(|| {
		let assets: MultiAsset = (
			(Parent, X2(Parachain(THIS_PARA_ID), GeneralKey(this_native_asset.encode()))),
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
			WithdrawAsset(
				(
					(Parent, X2(Parachain(SIBLING_PARA_ID), GeneralKey(any_asset.encode()))),
					sibling_non_native_amount,
				) // withdraw into VM holder asset, and do nothing...
					.into(),
			),
		];
		assert_ok!(pallet_xcm::Pallet::<sibling_runtime::Runtime>::send_xcm(
			Here,
			(Parent, Parachain(THIS_PARA_ID)),
			Xcm(xcm),
		));
	});

	This::execute_with(|| {
		assert_eq!(
			System::events().iter().find(|r| matches!(
				r.event,
				this_runtime::Event::RelayerXcm(pallet_xcm::Event::AssetsTrapped(_, _, _))
			)),
			None // non of assets trapped by hash, because all are known
		);
		assert_eq!(
			this_runtime::Assets::free_balance(any_asset, &this_runtime::TreasuryAccount::get()),
			sibling_non_native_amount
		);

		assert_eq!(
			this_runtime::Balances::free_balance(&this_runtime::TreasuryAccount::get()),
			some_native_amount + this_native_treasury_amount,
		);
	});
}
