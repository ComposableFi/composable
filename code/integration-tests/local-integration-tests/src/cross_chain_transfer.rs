use crate::{
	assert_lt_by,
	helpers::*,
	kusama_test_net::{
		KusamaRelay, Sibling, This, ALICE_PARACHAIN_KSM, PICA, SIBLING_PARA_ID, THIS_PARA_ID,
	},
	prelude::*,
};
use codec::Encode;
use common::{AccountId, Balance};
use composable_traits::{currency::RangeId, defi::Ratio};

use frame_system::RawOrigin;
use this_runtime::{
	Assets, MaxInstructions, Origin, Runtime, System, Tokens, UnitWeightCost, XTokens,
};

use num_traits::Zero;
use orml_traits::currency::MultiCurrency;

use frame_support::{
	assert_ok, log,
	sp_runtime::{DispatchError, ModuleError},
};
use primitives::currency::*;
use sp_runtime::{assert_eq_error_rate, traits::AccountIdConversion, MultiAddress};
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
	reserve_transfer(from, to);
}

/// how it works:
/// top level ReserveTransfer instruction is interpreted first on sending chain
/// it transfers amount from sender account to target chain account on sending chain
/// send it cut of wrapper part of XCM message, and sends remaining with deposit
/// target chain sees deposit amount and mints appreciate amount
/// validate origin of reserve (must be relay)
fn reserve_transfer(from: [u8; 32], to: [u8; 32]) {
	let from_account = &AccountId::from(from);
	let to_account = &AccountId::from(to);
	let balance = enough_weight();
	let before =
		This::execute_with(|| this_runtime::Assets::free_balance(CurrencyId::KSM, to_account));
	KusamaRelay::execute_with(|| {
		let _ = <relay_runtime::Balances as frame_support::traits::Currency<_>>::deposit_creating(
			from_account,
			balance,
		);
		let result = relay_runtime::XcmPallet::reserve_transfer_assets(
			relay_runtime::Origin::signed(from.into()),
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
	let transfer_amount = 3 * RELAY_NATIVE::ONE;
	let limit = 4_600_000_000;
	assert_eq!(
		KusamaRelay::execute_with(|| relay_runtime::Balances::balance(&AccountId::from(BOB))),
		0
	);
	This::execute_with(|| {
		let transferred = this_runtime::XTokens::transfer(
			this_runtime::Origin::signed(ALICE.into()),
			CurrencyId::KSM,
			transfer_amount,
			Box::new(
				MultiLocation::new(
					1,
					X1(Junction::AccountId32 { id: BOB, network: NetworkId::Any }),
				)
				.into(),
			),
			limit,
		);

		assert_ok!(transferred);

		let remaining =
			this_runtime::Assets::free_balance(CurrencyId::KSM, &AccountId::from(ALICE));

		assert_eq!(remaining, ALICE_PARACHAIN_KSM - transfer_amount);
	});

	KusamaRelay::execute_with(|| {
		assert_lt_by!(
			relay_runtime::Balances::balance(&AccountId::from(BOB)),
			transfer_amount,
			ORDER_OF_FEE_ESTIMATE_ERROR * (THIS_CHAIN_NATIVE_FEE + RELAY_CHAIN_NATIVE_FEE) +
				ORDER_OF_FEE_ESTIMATE_ERROR * limit as u128
		)
	});
}

#[test]
fn transfer_native_of_this_to_sibling() {
	simtest();

	Sibling::execute_with(|| {
		assert_ok!(this_runtime::AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId::PICA,
			composable_traits::xcm::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X1(Parachain(THIS_PARA_ID),)
			)),
			Some(Ratio::saturating_from_rational(1, 1)),
			None,
		));
	});

	This::execute_with(|| {
		use this_runtime::*;
		let before = Balances::balance(&sibling_account(SIBLING_PARA_ID));
		assert_ok!(RelayerXcm::limited_reserve_transfer_assets(
			Origin::signed(ALICE.into()),
			Box::new(VersionedMultiLocation::V1(MultiLocation::new(
				1,
				X1(Parachain(SIBLING_PARA_ID))
			))),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			Box::new((Here, 3 * PICA).into()),
			0,
			WeightLimit::Limited(399_600_000_000),
		));

		let after = Balances::balance(&sibling_account(SIBLING_PARA_ID));
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), 200 * PICA - 3 * PICA);
		assert_gt!(after, before);
		assert_eq!(after, 3 * PICA);
	});

	Sibling::execute_with(|| {
		let balance =
			sibling_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(BOB));
		assert_eq_error_rate!(balance, 3 * PICA, (UnitWeightCost::get() * 10) as u128);
	});
}

#[test]
fn transfer_native_of_this_to_sibling_by_local_id() {
	simtest();

	Sibling::execute_with(|| {
		assert_ok!(this_runtime::AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId::PICA,
			composable_traits::xcm::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X1(Parachain(THIS_PARA_ID),)
			)),
			Some(Ratio::saturating_from_rational(1, 1)),
			None,
		));
	});

	This::execute_with(|| {
		use this_runtime::*;
		let before = Balances::balance(&sibling_account(SIBLING_PARA_ID));

		assert_ok!(XTokens::transfer(
			Origin::signed(ALICE.into()),
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

		let after = Balances::balance(&sibling_account(SIBLING_PARA_ID));
		assert_eq!(Balances::free_balance(&AccountId::from(ALICE)), 200 * PICA - 3 * PICA);
		assert_gt!(after, before);
		assert_eq!(after, 3 * PICA);
	});

	Sibling::execute_with(|| {
		let balance =
			sibling_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(BOB));
		assert_eq_error_rate!(balance, 3 * PICA, (UnitWeightCost::get() * 10) as u128);
	});
}

#[test]
fn transfer_non_native_reserver_asset_from_this_to_sibling() {
	simtest();

	Sibling::execute_with(|| {
		assert_ok!(this_runtime::AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId::PBLO,
			composable_traits::xcm::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(THIS_PARA_ID), GeneralIndex(CurrencyId::PBLO.into()),)
			)),
			Some(Ratio::saturating_from_rational(1, 1)),
			None,
		));
	});

	This::execute_with(|| {
		use this_runtime::*;

		assert_ok!(Assets::deposit(CurrencyId::PBLO, &AccountId::from(ALICE), 10 * PICA));
		assert_ok!(RelayerXcm::limited_reserve_transfer_assets(
			Origin::signed(ALICE.into()),
			Box::new(VersionedMultiLocation::V1(MultiLocation::new(
				1,
				X1(Parachain(SIBLING_PARA_ID))
			))),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			Box::new((X1(GeneralIndex(CurrencyId::PBLO.into()),), 3 * PICA).into()),
			0,
			WeightLimit::Limited(399_600_000_000),
		));

		let after = Assets::free_balance(CurrencyId::PBLO, &AccountId::from(ALICE));
		assert_eq!(after, 7 * PICA,);
	});

	Sibling::execute_with(|| {
		use sibling_runtime::*;
		let balance = Assets::free_balance(CurrencyId::PBLO, &AccountId::from(BOB));
		assert_eq_error_rate!(balance, 3 * PICA, (UnitWeightCost::get() * 10) as u128);
	});
}

#[test]
fn transfer_non_native_reserver_asset_from_this_to_sibling_by_local_id() {
	simtest();

	Sibling::execute_with(|| {
		assert_ok!(this_runtime::AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId::PBLO,
			composable_traits::xcm::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(THIS_PARA_ID), GeneralIndex(CurrencyId::PBLO.into()),)
			)),
			Some(Ratio::saturating_from_rational(1, 1)),
			None,
		));
	});

	This::execute_with(|| {
		use this_runtime::*;

		assert_ok!(Assets::deposit(CurrencyId::PBLO, &AccountId::from(ALICE), 10 * PICA));

		assert_ok!(XTokens::transfer(
			Origin::signed(ALICE.into()),
			CurrencyId::PBLO,
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

		let after = Assets::free_balance(CurrencyId::PBLO, &AccountId::from(ALICE));
		assert_eq!(after, 7 * PICA,);
	});

	Sibling::execute_with(|| {
		use sibling_runtime::*;
		let balance = Assets::free_balance(CurrencyId::PBLO, &AccountId::from(BOB));
		assert_eq_error_rate!(balance, 3 * PICA, (UnitWeightCost::get() * 10) as u128);
	});
}

#[test]
fn transfer_insufficient_amount_should_fail() {
	simtest();
	Sibling::execute_with(|| {
		assert!(matches!(
			sibling_runtime::XTokens::transfer(
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
			),
			Err(DispatchError::Module(ModuleError { .. }))
		));
		assert_eq!(
			sibling_runtime::Balances::free_balance(&AccountId::from(ALICE)),
			200000000000000
		);
	});

	This::execute_with(|| {
		assert_eq!(
			this_runtime::Tokens::free_balance(CurrencyId::PICA, &AccountId::from(bob())),
			0
		);
	});
}

#[test]
#[ignore = "until fixed sibling trust map"]
fn transfer_to_sibling() {
	simtest();

	fn this_native_reserve_account() -> AccountId {
		use sp_runtime::traits::AccountIdConversion;
		polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account_truncating()
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
						Junction::AccountId32 { network: NetworkId::Any, id: BOB }
					)
				)
				.into()
			),
			weight_to_pay,
		));

		assert_eq!(Tokens::free_balance(CurrencyId::KSM, &AccountId::from(ALICE)), alice_remaining);
	});

	// TODO: also XCM not fails, it really fails with not enough balance, not clear so what balance
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
						Junction::AccountId32 { network: NetworkId::Any, id: ALICE }
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

/// if Alice sends amount of their tokens and these are above weight but less than ED,
/// than our treasury takes that amount, sorry Alice
/// from: Acala
#[test]
fn transfer_from_relay_chain_deposit_to_treasury_if_below_ed() {
	simtest();
	let receiver = charlie();
	let (picasso_treasury, under_ed) = This::execute_with(|| {
		use this_runtime::*;
		let under_ed = under_existential_deposit::<AssetsRegistry>(LocalAssetId::KSM, 3);
		assert_eq!(Tokens::free_balance(CurrencyId::KSM, &AccountId::from(receiver)), 0,);
		(Tokens::free_balance(CurrencyId::KSM, &this_runtime::TreasuryAccount::get()), under_ed)
	});

	KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		assert_ok!(XcmPallet::reserve_transfer_assets(
			Origin::signed(ALICE.into()),
			Box::new(Parachain(THIS_PARA_ID).into().into()),
			Box::new(Junction::AccountId32 { id: receiver, network: NetworkId::Any }.into().into()),
			Box::new((Here, under_ed).into()),
			0
		));
	});

	This::execute_with(|| {
		use this_runtime::*;
		assert_eq!(
			Tokens::free_balance(CurrencyId::KSM, &AccountId::from(receiver)),
			0,
			"assets did not get to recipient as it is not enough to pay ED"
		);
		assert_eq!(
			Tokens::free_balance(CurrencyId::KSM, &TreasuryAccount::get()),
			under_ed - picasso_treasury
		);
	});
}

/// from: Acala
/// this test reasonably iff we know ratio of KSM to PICA, if not, it should be rewritten to ensure
/// permissioned execution of some very specific action from other chains
#[test]
fn xcm_transfer_execution_barrier_trader_works() {
	simtest();

	let unit_instruction_weight = UnitWeightCost::get() / 50;
	assert!(unit_instruction_weight > 0, "barrier makes sense iff there is pay for messages");

	// relay-chain use normal account to send xcm, destination para-chain can't pass Barrier check
	let tiny = 100;
	let message = Xcm(vec![
		ReserveAssetDeposited((Parent, tiny).into()),
		BuyExecution { fees: (Parent, tiny).into(), weight_limit: Unlimited },
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		let r = pallet_xcm::Pallet::<Runtime>::send_xcm(
			X1(Junction::AccountId32 { network: NetworkId::Any, id: ALICE }),
			Parachain(THIS_PARA_ID).into(),
			message,
		);
		assert_ok!(r);
	});
	This::execute_with(|| {
		assert!(this_runtime::System::events().iter().any(|r| {
			matches!(
				r.event,
				this_runtime::Event::DmpQueue(cumulus_pallet_dmp_queue::Event::ExecutedDownward {
					message_id: _,
					outcome: Outcome::Error(XcmError::Barrier)
				})
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
		assert_eq!(r, Outcome::Complete(this_runtime::xcmp::xcm_fee_estimator(3)));
	});
}

#[test]
fn payment_with_foreign_asset_under_fee_reports_required() {
	let ksm_minimal_amount = 1;
	let weight_limit = this_runtime::xcmp::xcm_fee_estimator(2 + 1);
	let message = Xcm::<this_runtime::Call>(vec![
		ReserveAssetDeposited((Parent, ksm_minimal_amount).into()),
		BuyExecution {
			fees: (Parent, ksm_minimal_amount).into(),
			weight_limit: Limited(weight_limit),
		},
		DepositAsset { assets: All.into(), max_assets: 1, beneficiary: Here.into() },
	]);
	This::execute_with(|| {
		let result =
			XcmExecutor::<this_runtime::XcmConfig>::execute_xcm(Parent, message, weight_limit);
		assert_eq!(
			result,
			Outcome::Incomplete(this_runtime::xcmp::xcm_fee_estimator(2), XcmError::TooExpensive)
		);
	});
}

#[test]
fn unspent_xcm_fee_is_returned_correctly() {
	simtest();
	let parachain_account: AccountId = This::execute_with(|| {
		this_runtime::ParachainInfo::parachain_id().into_account_truncating()
	});
	let some_account: AccountId = charlie().into();

	let charlie_on_kusama_amount = 1_000 * CurrencyId::unit::<Balance>();
	let (original_parachain, _original_bob) = KusamaRelay::execute_with(|| {
		(
			relay_runtime::Balances::balance(&parachain_account.clone()),
			relay_runtime::Balances::balance(&AccountId::from(BOB)),
		)
	});
	let parachain_on_kusama_amount = 1_000 * CurrencyId::unit::<Balance>();
	KusamaRelay::execute_with(|| {
		log::info!("============ RELAY");
		assert_ok!(relay_runtime::Balances::transfer(
			relay_runtime::Origin::signed(ALICE.into()),
			MultiAddress::Id(some_account.clone()),
			charlie_on_kusama_amount,
		));
		assert_ok!(relay_runtime::Balances::transfer(
			relay_runtime::Origin::signed(ALICE.into()),
			MultiAddress::Id(parachain_account.clone()),
			parachain_on_kusama_amount,
		));
		assert_eq!(
			relay_runtime::Balances::free_balance(&AccountId::from(ALICE)),
			2 * CurrencyId::unit::<Balance>()
		);
		assert_eq!(relay_runtime::Balances::free_balance(&some_account), charlie_on_kusama_amount);
		assert_eq!(relay_runtime::Balances::free_balance(&AccountId::from(BOB)), Balance::zero());
		assert_eq!(
			relay_runtime::Balances::free_balance(&parachain_account.clone()),
			original_parachain + parachain_on_kusama_amount,
		);
	});

	let transfer_in_transact_amount: u128 = CurrencyId::unit();
	let payment_into_holder = transfer_in_transact_amount;
	let weight_limit = 10_000_000_000;
	This::execute_with(|| {
		log::info!("============ THIS");
		// Construct a transfer XCM call with returning the deposit
		let transfer_call = relay_runtime::Call::Balances(relay_runtime::BalancesCall::transfer {
			dest: <relay_runtime::Runtime as frame_system::Config>::Lookup::unlookup(
				AccountId::from(BOB),
			),
			value: transfer_in_transact_amount,
		});

		let asset = MultiAsset {
			id: Concrete(MultiLocation::here()),
			fun: Fungibility::Fungible(payment_into_holder),
		};
		let xcm_msg = Xcm(vec![
			WithdrawAsset(asset.clone().into()),
			BuyExecution { fees: asset, weight_limit: Unlimited },
			Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: weight_limit,
				call: transfer_call.encode().into(),
			},
		]);

		assert_ok!(this_runtime::RelayerXcm::send_xcm(Here, Parent, xcm_msg));
	});

	let parachain_balance = KusamaRelay::execute_with(|| {
		log::info!("============ RELAY");
		assert_eq!(
			relay_runtime::Balances::balance(&AccountId::from(BOB)),
			transfer_in_transact_amount,
			"because of Transact called transfer"
		);

		let new_balance = relay_runtime::Balances::balance(&parachain_account.clone());
		assert_eq!(
			new_balance,
			original_parachain + parachain_on_kusama_amount - payment_into_holder -  transfer_in_transact_amount ,
			"Parachain payed fee (remainder not deposited back) and transferred to Bob from Parachain Treasury"
		);
		new_balance
	});

	This::execute_with(|| {
		log::info!("============ THIS");
		let transfer_call =
			relay_runtime::Call::Balances(relay_runtime::BalancesCall::transfer_keep_alive {
				dest: <relay_runtime::Runtime as frame_system::Config>::Lookup::unlookup(
					some_account.clone(),
				),
				value: transfer_in_transact_amount,
			});

		let finalized_call = crate::relaychain::finalize_call_into_xcm_message::<Runtime>(
			transfer_call,
			payment_into_holder,
			weight_limit,
			this_runtime::ParachainInfo::parachain_id(),
		);

		assert_ok!(this_runtime::RelayerXcm::send_xcm(Here, Parent, finalized_call));
	});

	KusamaRelay::execute_with(|| {
		assert_eq!(
			relay_runtime::Balances::balance(&some_account),
			charlie_on_kusama_amount + transfer_in_transact_amount
		);
		let possible_fee =
			ORDER_OF_FEE_ESTIMATE_ERROR * (RELAY_CHAIN_NATIVE_FEE + THIS_CHAIN_NATIVE_FEE);
		assert_lt!(possible_fee, payment_into_holder,);
		// still not clear why fee is so big
		assert_lt_by!(
			relay_runtime::Balances::balance(&parachain_account.clone()),
			parachain_balance - transfer_in_transact_amount,
			payment_into_holder - possible_fee
		);
	});
}

#[test]
fn trap_assets_larger_than_ed_works() {
	simtest();

	let mut native_treasury_amount = 0;
	let (ksm_asset_amount, native_asset_amount) =
		(3 * CurrencyId::unit::<Balance>(), 2 * CurrencyId::unit::<Balance>());
	let parent_account: AccountId =
		ParentIsPreset::<AccountId>::convert(Parent.into()).expect("Conversion into is safe; QED");
	This::execute_with(|| {
		assert_ok!(Tokens::deposit(
			CurrencyId::KSM,
			&parent_account,
			42 * CurrencyId::unit::<Balance>()
		));
		let _ =
			<balances::Pallet<Runtime> as frame_support::traits::Currency<AccountId>>::deposit_creating(
				&parent_account,
				123 * CurrencyId::unit::<Balance>(),
			);
		// TODO: if we do not top up account initially, than any deposit_creating do not create
		// anything may be something with zero block or like - fix it better way
		let _ =
			<balances::Pallet<Runtime> as frame_support::traits::Currency<AccountId>>::deposit_creating(
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
			WithdrawAsset(((0, GeneralIndex(CurrencyId::PICA.into())), native_asset_amount).into()),
		];
		assert_ok!(pallet_xcm::Pallet::<relay_runtime::Runtime>::send_xcm(
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

#[test]
fn trap_assets_lower_than_existential_deposit_works() {
	simtest();

	let other_non_native_amount = 1_000_000_000_000;
	let some_native_amount = 1_000_000_000_000_000;
	let any_asset = CurrencyId::KSM;
	let this_native_asset = CurrencyId::PICA;

	let parent_account: AccountId =
		ParentIsPreset::<AccountId>::convert(Parent.into()).expect("Conversion into is safe; QED");

	let (this_treasury_amount, other_treasury_amount) = This::execute_with(|| {
		assert_ok!(Assets::deposit(any_asset, &parent_account, other_non_native_amount));
		let _ = <this_runtime::Balances as frame_support::traits::Currency<AccountId>>::deposit_creating(
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
					(Parent, X2(Parachain(THIS_PARA_ID), GeneralIndex(this_native_asset.into()))),
					some_native_amount,
				)
					.into(),
			),
			//two asset left in holding register, they both lower than ED, so goes to treasury.
		];
		assert_ok!(pallet_xcm::Pallet::<relay_runtime::Runtime>::send_xcm(
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

#[test]
fn sibling_trap_assets_works() {
	simtest();
	let any_asset = CurrencyId::kUSD;
	let some_native_amount = 1_000_000_000;
	let this_liveness_native_amount = enough_weight();
	let this_native_asset = CurrencyId::PICA;

	let (this_native_treasury_amount, sibling_non_native_amount) = This::execute_with(|| {
		let sibling_non_native_amount =
			assert_above_deposit::<this_runtime::AssetsRegistry>(any_asset, 100_000_000_000);

		assert_ok!(Assets::deposit(
			any_asset,
			&sibling_account(SIBLING_PARA_ID),
			sibling_non_native_amount
		));
		let _ =
			<balances::Pallet<Runtime> as frame_support::traits::Currency<AccountId>>::deposit_creating(
				&sibling_account(SIBLING_PARA_ID),
				this_liveness_native_amount,
			);
		let _ =
			<balances::Pallet<Runtime> as frame_support::traits::Currency<AccountId>>::deposit_creating(
				&this_runtime::TreasuryAccount::get(),
				this_liveness_native_amount,
			);
		let balance =
			<balances::Pallet<Runtime> as frame_support::traits::Currency<AccountId>>::free_balance(
				&this_runtime::TreasuryAccount::get(),
			);
		let remote = composable_traits::xcm::assets::XcmAssetLocation(MultiLocation::new(
			1,
			X2(Parachain(SIBLING_PARA_ID), GeneralIndex(any_asset.into())),
		));
		assert_ok!(this_runtime::AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			any_asset,
			remote,
			Ratio::checked_from_integer::<u128>(1),
			None
		));
		(balance, sibling_non_native_amount)
	});

	// buy execution via native token, and try withdraw on this some amount
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
			WithdrawAsset(
				(
					(Parent, X2(Parachain(SIBLING_PARA_ID), GeneralIndex(any_asset.into()))),
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

#[test]
fn sibling_shib_to_transfer() {
	simtest();
	let total_issuance = 3_500_000_000_000_000;
	let transfer_amount = SHIB::ONE;
	let sibling_asset_id = Sibling::execute_with(|| {
		log::info!(target: "bdd", "Given SHIB on sibling registered");
		use sibling_runtime::*;
		let sibling_asset_id =
			CurrencyFactory::create(RangeId::TOKENS, 42).expect("Valid range and ED; QED");
		let root = frame_system::RawOrigin::Root;
		let location = XcmAssetLocation(MultiLocation::new(
			1,
			X2(Parachain(SIBLING_PARA_ID), GeneralIndex(sibling_asset_id.into())),
		));
		AssetsRegistry::update_asset(
			root.into(),
			sibling_asset_id,
			location,
			Ratio::checked_from_integer::<u128>(1),
			Some(SHIB::RESERVE_EXPONENT as u32),
		)
		.expect("Asset already in Currency Factory; QED");

		log::info!(target: "bdd", "	and Bob has a lot SHIB on sibling");
		let root = frame_system::RawOrigin::Root;
		Tokens::set_balance(
			root.into(),
			MultiAddress::Id(BOB.into()),
			sibling_asset_id,
			total_issuance,
			0,
		)
		.expect("Balance is valid; QED");
		sibling_asset_id
	});

	let remote_sibling_asset_id = This::execute_with(|| {
		log::info!(target: "bdd", "	and USD on Dali registered");
		use this_runtime::*;
		let root = frame_system::RawOrigin::Root;
		let location = XcmAssetLocation(MultiLocation::new(
			1,
			X2(Parachain(SIBLING_PARA_ID), GeneralIndex(sibling_asset_id.into())),
		));
		AssetsRegistry::register_asset(
			root.into(),
			location,
			1000,
			Ratio::checked_from_integer::<u128>(1),
			Some(SHIB::RESERVE_EXPONENT as u32),
		)
		.expect("Asset details are valid; QED");
		System::events()
			.iter()
			.find_map(|x| match x.event {
				Event::AssetsRegistry(assets_registry::Event::<Runtime>::AssetRegistered {
					asset_id,
					location: _,
				}) => Some(asset_id),
				_ => None,
			})
			.expect("Map exists; QED")
	});
	log::info!(target: "bdd", "{:?}", remote_sibling_asset_id);
	Sibling::execute_with(|| {
		log::info!(target: "bdd", "When Bob transfers some {:?} SHIB from from sibling to Dali", transfer_amount);
		use sibling_runtime::*;
		let origin = Origin::signed(BOB.into());
		assert_ok!(RelayerXcm::limited_reserve_transfer_assets(
			origin,
			Box::new(VersionedMultiLocation::V1(MultiLocation::new(
				1,
				X1(Parachain(THIS_PARA_ID))
			))),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			Box::new((X1(GeneralIndex(sibling_asset_id.into()),), transfer_amount).into()),
			0,
			WeightLimit::Unlimited,
		));
		assert_eq!(
			<Tokens as frame_support::traits::fungibles::Inspect<AccountId>>::balance(
				sibling_asset_id,
				&AccountId::from(BOB)
			),
			total_issuance - transfer_amount
		);
	});

	This::execute_with(|| {
		use this_runtime::*;
		log::info!(target: "bdd", "Then Bob gets some SHIB on Dali");
		let fee = this_runtime::xcmp::xcm_asset_fee_estimator(5, remote_sibling_asset_id);
		assert_gt!(transfer_amount, fee);
		let balance = Tokens::free_balance(remote_sibling_asset_id, &AccountId::from(BOB));
		assert_lt_by!(balance, transfer_amount, fee);
	});
}
