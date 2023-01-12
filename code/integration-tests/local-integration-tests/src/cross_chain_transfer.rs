use crate::{
	assert_lt_by,
	helpers::*,
	kusama_test_net::{KusamaRelay, Sibling, This, PICA, SIBLING_PARA_ID, THIS_PARA_ID},
	prelude::*,
};
use codec::Encode;
use common::{AccountId, Balance};
use composable_traits::{currency::RangeId, rational};

use frame_system::RawOrigin;

use num_traits::Zero;
use orml_traits::currency::MultiCurrency;

use frame_support::{assert_ok, log, weights::constants::WEIGHT_PER_MILLIS};
use primitives::currency::*;
use sp_runtime::{assert_eq_error_rate, traits::AccountIdConversion, MultiAddress};
use xcm::latest::prelude::*;
use xcm_builder::ParentIsPreset;
use xcm_emulator::TestExt;
use xcm_executor::{traits::Convert, XcmExecutor};

#[test]
fn reserve_transfer_from_relay_alice_bob() {
	simtest();
	let from = alice();
	let to = bob();
	reserve_transfer(from, to);
}

#[test]
fn reserve_transfer_from_relay_alice_alice() {
	simtest();
	let from = alice();
	let to = alice();
	reserve_transfer(from, to);
}

#[test]
fn reserve_transfer_from_relay_map() {
	simtest();
	let from = alice();
	let to = bob();
	reserve_transfer(from, to);
}

fn reserve_transfer(from: [u8; 32], to: [u8; 32]) {
	let from_account = &AccountId::from(from);
	let to_account = &AccountId::from(to);
	let balance = enough_weight();
	let destination = THIS_PARA_ID;
	let before =
		This::execute_with(|| this_runtime::Assets::free_balance(CurrencyId::KSM, to_account));
	KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		let _ = <Balances as frame_support::traits::Currency<_>>::deposit_creating(
			from_account,
			balance,
		);
		let result = XcmPallet::reserve_transfer_assets(
			Origin::signed(from.into()),
			Box::new(Parachain(destination).into().into()),
			Box::new(Junction::AccountId32 { id: to, network: NetworkId::Any }.into().into()),
			Box::new((Here, balance).into()),
			0,
		);
		assert_ok!(result);
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
fn transfer_this_native_to_sibling_overridden() {
	simtest();

	Sibling::execute_with(|| {
		log::info!(target: "bdd", "Sibling overrides some well known asset to this");
		use sibling_runtime::*;
		assert_ok!(AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId::PICA,
			composable_traits::xcm::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X1(Parachain(THIS_PARA_ID),)
			)),
			rational!(1 / 1),
			None,
		));
	});

	This::execute_with(|| {
		use this_runtime::*;
		let _ = <balances::Pallet<Runtime> as frame_support::traits::Currency<AccountId>>::deposit_creating(
			&alice().into(),
			4 * PICA,
		);
		let before = Balances::balance(&sibling_account(SIBLING_PARA_ID));
		let alice_before = Balances::balance(&alice().into());
		assert_ok!(RelayerXcm::limited_reserve_transfer_assets(
			Origin::signed(alice().into()),
			Box::new(VersionedMultiLocation::V1(MultiLocation::new(
				1,
				X1(Parachain(SIBLING_PARA_ID))
			))),
			Box::new(Junction::AccountId32 { id: bob(), network: NetworkId::Any }.into().into()),
			Box::new((Here, 3 * PICA).into()),
			0,
			WeightLimit::Limited(399_600_000_000),
		));

		let after = Balances::balance(&sibling_account(SIBLING_PARA_ID));
		assert_eq!(alice_before - Balances::free_balance(&alice().into()), 3 * PICA);
		assert_gt!(after, before);
		assert_eq!(after, 3 * PICA, "bdd: Sibling reserve account gets amount locked");
	});

	Sibling::execute_with(|| {
		let balance =
			sibling_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(bob()));
		assert_eq_error_rate!(balance, 3 * PICA, (UnitWeightCost::get() * 10) as u128);
	});
}

#[test]
fn transfer_non_native_reserve_asset_from_this_to_sibling() {
	simtest();

	Sibling::execute_with(|| {
		assert_ok!(this_runtime::AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId::PBLO,
			composable_traits::xcm::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(THIS_PARA_ID), GeneralIndex(CurrencyId::PBLO.into()),)
			)),
			Rational64::one(),
			None,
		));
	});

	This::execute_with(|| {
		use this_runtime::*;

		assert_ok!(Assets::deposit(CurrencyId::PBLO, &alice().into(), 10 * PICA));
		let _before = Assets::free_balance(CurrencyId::PBLO, &alice().into());
		assert_ok!(RelayerXcm::limited_reserve_transfer_assets(
			Origin::signed(alice().into()),
			Box::new(VersionedMultiLocation::V1(MultiLocation::new(
				1,
				X1(Parachain(SIBLING_PARA_ID))
			))),
			Box::new(Junction::AccountId32 { id: bob(), network: NetworkId::Any }.into().into()),
			Box::new((X1(GeneralIndex(CurrencyId::PBLO.into()),), 3 * PICA).into()),
			0,
			WeightLimit::Limited(399_600_000_000),
		));

		let after = Assets::free_balance(CurrencyId::PBLO, &alice().into());
		assert_eq!(after, 7 * PICA,);
	});

	Sibling::execute_with(|| {
		use sibling_runtime::*;
		let balance = Assets::free_balance(CurrencyId::PBLO, &AccountId::from(bob()));
		assert_eq_error_rate!(balance, 3 * PICA, (UnitWeightCost::get() * 10) as u128);
	});
}

#[test]
fn transfer_non_native_reserve_asset_from_this_to_sibling_by_local_id_overridden() {
	simtest();

	Sibling::execute_with(|| {
		assert_ok!(this_runtime::AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId::PBLO,
			composable_traits::xcm::assets::XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(THIS_PARA_ID), GeneralIndex(CurrencyId::PBLO.into()),)
			)),
			Rational64::one(),
			None,
		));
	});

	This::execute_with(|| {
		use this_runtime::*;

		assert_ok!(Tokens::deposit(CurrencyId::PBLO, &alice().into(), 10 * PICA));
		let _before = Assets::free_balance(CurrencyId::PBLO, &alice().into());

		assert_ok!(XTokens::transfer(
			Origin::signed(alice().into()),
			CurrencyId::PBLO,
			3 * PICA,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(SIBLING_PARA_ID),
						Junction::AccountId32 { id: bob(), network: NetworkId::Any }
					)
				)
				.into()
			),
			399_600_000_000
		));

		let after = Assets::free_balance(CurrencyId::PBLO, &alice().into());
		assert_eq!(after, 7 * PICA,);
	});

	Sibling::execute_with(|| {
		use sibling_runtime::*;
		let balance = Assets::free_balance(CurrencyId::PBLO, &AccountId::from(bob()));
		assert_eq_error_rate!(balance, 3 * PICA, (UnitWeightCost::get() * 10) as u128);
	});
}

#[test]
fn this_native_transferred_from_sibling_to_native_is_not_enough() {
	simtest();

	let remote_this_asset_id = Sibling::execute_with(|| {
		log::info!(target: "bdd", "Remote PICA registered on sibling");
		use sibling_runtime::*;
		let root = frame_system::RawOrigin::Root;
		let location = XcmAssetLocation::new(MultiLocation::new(1, X1(Parachain(THIS_PARA_ID))));
		AssetsRegistry::register_asset(root.into(), location.clone(), Rational64::one(), None)
			.unwrap();
		System::events()
			.iter()
			.find_map(|x| match x.event {
				Event::AssetsRegistry(assets_registry::Event::<Runtime>::AssetRegistered {
					asset_id,
					..
				}) => Some(asset_id),
				_ => None,
			})
			.unwrap()
	});

	This::execute_with(|| {
		use this_runtime::*;
		let _ = <balances::Pallet<Runtime> as frame_support::traits::Currency<AccountId>>::deposit_creating(
			&alice().into(),
			200_000_000_000_000,
		);

		assert_ok!(XTokens::transfer(
			Origin::signed(alice().into()),
			CurrencyId::PICA,
			100_000_000_000_000,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(SIBLING_PARA_ID),
						Junction::AccountId32 { id: alice(), network: NetworkId::Any }
					)
				)
				.into()
			),
			399_600_000_000
		));
		log::info!(target: "bdd", "Alice transferred PICA from this to her on sibling");
	});

	Sibling::execute_with(|| {
		use sibling_runtime::*;
		assert_ok!(XTokens::transfer(
			Origin::signed(alice().into()),
			remote_this_asset_id,
			1_000,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Junction::Parachain(THIS_PARA_ID),
						Junction::AccountId32 { id: bob(), network: NetworkId::Any }
					)
				)
				.into()
			),
			399_600_000_000
		),);
		log::info!(target: "bdd", "Alice sent too few PICA from sibling to Bob on this");
	});

	This::execute_with(|| {
		assert_eq!(
			this_runtime::Tokens::free_balance(CurrencyId::PICA, &AccountId::from(bob())),
			0,
			"Bob has nothing in his pocket on this"
		);
	});
}

#[test]
fn transfer_relay_native_from_this_to_sibling_by_local_id() {
	simtest();

	let ksm = 100_000_000_000_000_000;

	let parachain_reserve_account_on_relay: AccountId =
		ParaId::from(THIS_PARA_ID).into_account_truncating();

	let this_reserve_amount_on_original = KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		Balances::balance(&parachain_reserve_account_on_relay)
	});

	mint_relay_native_on_parachain(ksm, &alice().into(), THIS_PARA_ID);

	let this_reserve_amount_after_transfer = KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		Balances::balance(&parachain_reserve_account_on_relay)
	});
	assert_eq!(this_reserve_amount_on_original + ksm, this_reserve_amount_after_transfer);

	let alice_this_original = This::execute_with(|| {
		use this_runtime::*;
		Tokens::free_balance(CurrencyId::KSM, &alice().into())
	});

	let alice_ksm_transfer_amount = 1_000_000_000_000;
	let alice_remaining = alice_this_original - alice_ksm_transfer_amount;
	let weight_to_pay = 4 * WEIGHT_PER_MILLIS;

	let this_on_sibling = Sibling::execute_with(|| {
		use sibling_runtime::*;
		Tokens::free_balance(CurrencyId::KSM, &sibling_account(THIS_PARA_ID))
	});
	let _original_bob_on_sibling = Sibling::execute_with(|| {
		use sibling_runtime::*;
		Tokens::free_balance(CurrencyId::KSM, &AccountId::from(bob()))
	});

	assert_eq!(
		this_on_sibling,
		Balance::zero(),
		"basic amount of this parachain on sibling is zero"
	);

	let sibling_parachain_reserve_account_on_relay: AccountId =
		ParaId::from(SIBLING_PARA_ID).into_account_truncating();
	let _sibling_reserve_amount_on_original = KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		Balances::balance(&sibling_parachain_reserve_account_on_relay)
	});

	This::execute_with(|| {
		log::info!(target: "bdd", "When Alice sends from this to Bob on sibling");
		use this_runtime::*;
		assert_ok!(XTokens::transfer(
			Origin::signed(alice().into()),
			CurrencyId::KSM,
			alice_ksm_transfer_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(SIBLING_PARA_ID),
						Junction::AccountId32 { network: NetworkId::Any, id: bob() }
					)
				)
				.into()
			),
			weight_to_pay,
		));

		assert_eq!(Tokens::free_balance(CurrencyId::KSM, &alice().into()), alice_remaining);
	});
	let sibling_reserve_amount = KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		Balances::balance(&sibling_parachain_reserve_account_on_relay)
	});

	Sibling::execute_with(|| {
		log::info!(target: "bdd", "Then Bob on sibling receives amounts");
		use sibling_runtime::*;
		assert_eq!(
			Tokens::free_balance(CurrencyId::KSM, &sibling_account(THIS_PARA_ID)),
			this_on_sibling
		);

		let treasury =
			<Tokens as FungiblesInspect<_>>::balance(CurrencyId::KSM, &TreasuryAccount::get());
		assert_lt!(
			treasury,
			ORDER_OF_FEE_ESTIMATE_ERROR * xcmp::xcm_asset_fee_estimator(4, CurrencyId::KSM)
		);
		let new_bob_on_sibling =
			<Tokens as FungiblesInspect<_>>::balance(CurrencyId::KSM, &AccountId::from(bob()));
		assert_lt_by!(
			new_bob_on_sibling,
			sibling_reserve_amount,
			ORDER_OF_FEE_ESTIMATE_ERROR * xcmp::xcm_asset_fee_estimator(5, CurrencyId::KSM)
		);

		assert_eq!(new_bob_on_sibling + treasury, sibling_reserve_amount,);

		log::info!(target: "bdd", "Then Bob on sibling sends relay native to Alice on this");
		assert_ok!(XTokens::transfer(
			Origin::signed(bob().into()),
			CurrencyId::KSM,
			new_bob_on_sibling,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(THIS_PARA_ID),
						Junction::AccountId32 { network: NetworkId::Any, id: alice() }
					)
				)
				.into()
			),
			1_000_000_000,
		));

		assert_eq!(Tokens::free_balance(CurrencyId::KSM, &sibling_account(THIS_PARA_ID)), 0);
		assert_eq!(Tokens::free_balance(CurrencyId::KSM, &AccountId::from(bob())), 0);
	});

	// ISSUE:
	// 1. KSM Relay -> This
	// 2. KSM This -> Sibling
	// 3. KSM Sibling -> This
	// Expected: Step 3 works fine
	// Actual: Step 3 fails if using xTokens with Barrier
	// Notes:
	// - possibly this may relate that xTokens 2 XCM message and not handler properly in order
	// - possible fix is to craft single message doing 2 jumps
	// This::execute_with(|| {
	// 	use this_runtime::*;
	// 	assert_lt_by!(
	// 		Tokens::balance(CurrencyId::KSM, &alice().into()),
	// 		ksm,
	// 		4 * ORDER_OF_FEE_ESTIMATE_ERROR * xcmp::xcm_asset_fee_estimator(10, CurrencyId::KSM)
	// 	);
	// });

	let sibling_issuance = Sibling::execute_with(|| {
		use sibling_runtime::*;
		Tokens::total_issuance(CurrencyId::KSM)
	});
	let this_issuance = This::execute_with(|| {
		use this_runtime::*;
		Tokens::total_issuance(CurrencyId::KSM)
	});
	let relay_issuance = KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		Balances::total_issuance()
	});
	let this_reserve_amount_final = KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		Balances::balance(&parachain_reserve_account_on_relay)
	});
	let sibling_reserve_amount_final = KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		Balances::balance(&sibling_parachain_reserve_account_on_relay)
	});
	log::info!(target: "bdd", "Issuance and reservers of relay natives on parachains is less then on Relay");
	assert_lt!(
		sibling_issuance + this_issuance,
		this_reserve_amount_final + sibling_reserve_amount_final
	);
	assert_lt!(sibling_issuance + this_issuance, relay_issuance,);
	assert_lt!(this_reserve_amount_final + sibling_reserve_amount_final, relay_issuance,);
}

// if Alice sends amount of their tokens and these are above weight but less than ED,
// than our treasury takes that amount, sorry Alice
#[test]
fn one_chain_cannot_print_relay_native_reserve_tokens_on_us() {
	simtest();

	let _original_bob_on_sibling = This::execute_with(|| {
		use this_runtime::*;
		assert_ok!(Tokens::deposit(CurrencyId::KSM, &alice().into(), 100_000_000_000_000));
		Tokens::free_balance(CurrencyId::KSM, &alice().into())
	});

	let alice_original = This::execute_with(|| {
		use this_runtime::*;
		assert_ok!(Tokens::deposit(CurrencyId::KSM, &alice().into(), 100_000_000_000_000));
		Tokens::free_balance(CurrencyId::KSM, &alice().into())
	});

	let alice_from_amount = alice_original / 10;
	let _alice_remaining = alice_original - alice_from_amount;
	let weight_to_pay = (alice_from_amount / 2) as u64;

	This::execute_with(|| {
		use this_runtime::*;
		assert_ok!(XTokens::transfer(
			Origin::signed(alice().into()),
			CurrencyId::KSM,
			alice_from_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(SIBLING_PARA_ID),
						Junction::AccountId32 { network: NetworkId::Any, id: bob() }
					)
				)
				.into()
			),
			weight_to_pay,
		));
	});

	Sibling::execute_with(|| {
		use sibling_runtime::*;
		assert_eq!(Tokens::free_balance(CurrencyId::KSM, &AccountId::from(bob())), 0);
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
			X1(Junction::AccountId32 { network: NetworkId::Any, id: alice() }),
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
		use relay_runtime::*;
		let _ =
		<balances::Pallet<Runtime> as frame_support::traits::Currency<AccountId>>::deposit_creating(
			&alice().into(),
			1_000_000_000_000_000_000_000,
		);
		(Balances::balance(&parachain_account.clone()), Balances::balance(&AccountId::from(bob())))
	});
	let parachain_on_kusama_amount = 1_000 * CurrencyId::unit::<Balance>();
	KusamaRelay::execute_with(|| {
		use relay_runtime::*;

		assert_ok!(Balances::transfer(
			Origin::signed(alice().into()),
			MultiAddress::Id(some_account.clone()),
			charlie_on_kusama_amount,
		));
		assert_ok!(Balances::transfer(
			Origin::signed(alice().into()),
			MultiAddress::Id(parachain_account.clone()),
			parachain_on_kusama_amount,
		));

		assert_eq!(Balances::free_balance(&some_account), charlie_on_kusama_amount);
		assert_eq!(Balances::free_balance(&AccountId::from(bob())), Balance::zero());
		assert_eq!(
			Balances::free_balance(&parachain_account.clone()),
			original_parachain + parachain_on_kusama_amount,
		);
	});

	let transfer_in_transact_amount: u128 = CurrencyId::unit();
	let payment_into_holder = transfer_in_transact_amount;
	let weight_limit = 10_000_000_000;
	This::execute_with(|| {
		let transfer_call = relay_runtime::Call::Balances(relay_runtime::BalancesCall::transfer {
			dest: <relay_runtime::Runtime as frame_system::Config>::Lookup::unlookup(
				AccountId::from(bob()),
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
		assert_eq!(
			relay_runtime::Balances::balance(&AccountId::from(bob())),
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
		use this_runtime::*;
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

		assert_ok!(RelayerXcm::send_xcm(Here, Parent, finalized_call));
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
		use this_runtime::*;
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
		use this_runtime::*;
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
		use this_runtime::*;
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
		use this_runtime::*;
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
		use this_runtime::*;
		let sibling_non_native_amount =
			assert_above_deposit::<AssetsRegistry>(any_asset, 100_000_000_000);

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
				&TreasuryAccount::get(),
				this_liveness_native_amount,
			);
		let balance =
			<balances::Pallet<Runtime> as frame_support::traits::Currency<AccountId>>::free_balance(
				&TreasuryAccount::get(),
			);
		let remote = composable_traits::xcm::assets::XcmAssetLocation(MultiLocation::new(
			1,
			X2(Parachain(SIBLING_PARA_ID), GeneralIndex(any_asset.into())),
		));
		assert_ok!(AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			any_asset,
			remote,
			Rational64::one(),
			None
		));
		(balance, sibling_non_native_amount)
	});

	log::info!(target: "bdd", "buy execution via native token, and try withdraw on this some amount");
	Sibling::execute_with(|| {
		let assets: MultiAsset = (
			(Parent, X2(Parachain(THIS_PARA_ID), GeneralIndex(this_native_asset.into()))),
			some_native_amount,
		)
			.into();
		let xcm = vec![
			WithdrawAsset(assets.clone().into()),
			BuyExecution { fees: assets, weight_limit: Unlimited },
			WithdrawAsset(
				(
					(Parent, X2(Parachain(SIBLING_PARA_ID), GeneralIndex(any_asset.into()))),
					sibling_non_native_amount,
				)
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
		use this_runtime::*;
		assert_eq!(
			System::events().iter().find(|r| matches!(
				r.event,
				this_runtime::Event::RelayerXcm(pallet_xcm::Event::AssetsTrapped(_, _, _))
			)),
			None // non of assets trapped by hash, because all are known
		);
		assert_eq!(
			Assets::free_balance(any_asset, &TreasuryAccount::get()),
			sibling_non_native_amount
		);

		assert_eq!(
			Balances::free_balance(&TreasuryAccount::get()),
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
			CurrencyFactory::create(RangeId::TOKENS).expect("Valid range and ED; QED");
		let root = frame_system::RawOrigin::Root;
		let location = XcmAssetLocation(MultiLocation::new(
			1,
			X2(Parachain(SIBLING_PARA_ID), GeneralIndex(sibling_asset_id.into())),
		));
		AssetsRegistry::update_asset(
			root.into(),
			sibling_asset_id,
			location,
			Rational64::one(),
			Some(SHIB::EXPONENT),
		)
		.expect("Asset already in Currency Factory; QED");

		log::info!(target: "bdd", "	and Bob has a lot SHIB on sibling");
		let root = frame_system::RawOrigin::Root;
		Tokens::set_balance(
			root.into(),
			MultiAddress::Id(bob().into()),
			sibling_asset_id,
			total_issuance,
			0,
		)
		.expect("Balance is valid; QED");
		sibling_asset_id
	});

	let remote_sibling_asset_id = This::execute_with(|| {
		log::info!(target: "bdd", "	and SHIB on Dali registered");
		use this_runtime::*;
		let root = frame_system::RawOrigin::Root;
		let location = XcmAssetLocation(MultiLocation::new(
			1,
			X2(Parachain(SIBLING_PARA_ID), GeneralIndex(sibling_asset_id.into())),
		));
		AssetsRegistry::register_asset(
			root.into(),
			location,
			Rational64::one(),
			Some(SHIB::EXPONENT),
		)
		.expect("Asset details are valid; QED");
		System::events()
			.iter()
			.find_map(|x| match x.event {
				Event::AssetsRegistry(assets_registry::Event::<Runtime>::AssetRegistered {
					asset_id,
					location: _,
					decimals: _,
				}) => Some(asset_id),
				_ => None,
			})
			.expect("Map exists; QED")
	});
	log::info!(target: "bdd", "{:?}", remote_sibling_asset_id);
	Sibling::execute_with(|| {
		log::info!(target: "bdd", "When Bob transfers some {:?} SHIB from sibling to Dali", transfer_amount);
		use sibling_runtime::*;
		let origin = Origin::signed(bob().into());
		assert_ok!(RelayerXcm::limited_reserve_transfer_assets(
			origin,
			Box::new(VersionedMultiLocation::V1(MultiLocation::new(
				1,
				X1(Parachain(THIS_PARA_ID))
			))),
			Box::new(Junction::AccountId32 { id: bob(), network: NetworkId::Any }.into().into()),
			Box::new((X1(GeneralIndex(sibling_asset_id.into()),), transfer_amount).into()),
			0,
			WeightLimit::Unlimited,
		));
		assert_eq!(
			<Tokens as FungiblesInspect<_>>::balance(sibling_asset_id, &AccountId::from(bob())),
			total_issuance - transfer_amount
		);
	});

	This::execute_with(|| {
		use this_runtime::*;
		log::info!(target: "bdd", "Then Bob gets some SHIB on Dali");
		let fee = xcmp::xcm_asset_fee_estimator(5, remote_sibling_asset_id);
		assert_gt!(transfer_amount, fee);
		let balance = Tokens::free_balance(remote_sibling_asset_id, &AccountId::from(bob()));
		assert_lt_by!(balance, transfer_amount, fee);
	});
}

#[test]
fn transfer_unknown_token_from_known_origin_ends_up_in_unknown_tokens() {
	simtest();
	let total_issuance = 3_500_000_000_000_000;
	let transfer_amount = SHIB::ONE;
	let sibling_asset_id = Sibling::execute_with(|| {
		use sibling_runtime::*;
		log::info!(target: "bdd", "Given one well-known/registered/sufficient/payable Sibling chain asset");
		let sibling_asset_id =
			CurrencyFactory::create(RangeId::TOKENS).expect("Valid range and ED; QED");
		let root = frame_system::RawOrigin::Root;
		let location = XcmAssetLocation(MultiLocation::new(
			1,
			X2(Parachain(SIBLING_PARA_ID), GeneralIndex(sibling_asset_id.into())),
		));
		AssetsRegistry::update_asset(
			root.into(),
			sibling_asset_id,
			location,
			Rational64::one(),
			Some(SHIB::EXPONENT),
		)
		.expect("Asset already in Currency Factory; QED");

		let root = frame_system::RawOrigin::Root;
		Tokens::set_balance(
			root.into(),
			MultiAddress::Id(bob().into()),
			sibling_asset_id,
			total_issuance,
			0,
		)
		.expect("Balance is valid; QED");
		sibling_asset_id
	});

	let unknown_asset_id = Sibling::execute_with(|| {
		use sibling_runtime::*;
		log::info!(target: "bdd", "Given one not registered/nonpayable Sibling chain asset");
		let sibling_asset_id =
			CurrencyFactory::create(RangeId::TOKENS).expect("Valid range and ED; QED");
		let root = frame_system::RawOrigin::Root;
		let location = XcmAssetLocation(MultiLocation::new(
			1,
			X2(Parachain(SIBLING_PARA_ID), GeneralIndex(sibling_asset_id.into())),
		));
		AssetsRegistry::update_asset(
			root.into(),
			sibling_asset_id,
			location,
			Rational64::one(),
			Some(SHIB::EXPONENT),
		)
		.expect("Asset already in Currency Factory; QED");

		let root = frame_system::RawOrigin::Root;
		Tokens::set_balance(
			root.into(),
			MultiAddress::Id(bob().into()),
			sibling_asset_id,
			total_issuance,
			0,
		)
		.expect("Balance is valid; QED");
		sibling_asset_id
	});

	let remote_sibling_asset_id = This::execute_with(|| {
		use this_runtime::*;
		let root = frame_system::RawOrigin::Root;
		let location = XcmAssetLocation(MultiLocation::new(
			1,
			X2(Parachain(SIBLING_PARA_ID), GeneralIndex(sibling_asset_id.into())),
		));
		AssetsRegistry::register_asset(
			root.into(),
			location,
			Rational64::one(),
			Some(SHIB::EXPONENT),
		)
		.expect("Asset details are valid; QED");
		System::events()
			.iter()
			.find_map(|x| match x.event {
				Event::AssetsRegistry(assets_registry::Event::<Runtime>::AssetRegistered {
					asset_id,
					location: _,
					decimals: _,
				}) => Some(asset_id),
				_ => None,
			})
			.expect("Map exists; QED")
	});

	let assets = VersionedMultiAssets::V1(MultiAssets::from(vec![
		(X1(GeneralIndex(unknown_asset_id.into())), transfer_amount).into(),
		(X1(GeneralIndex(sibling_asset_id.into())), transfer_amount).into(),
	]));

	Sibling::execute_with(|| {
		log::info!(target: "bdd", "When Bob transfers some known asset (as fee) and unknown asset");
		use sibling_runtime::*;
		let origin = Origin::signed(bob().into());
		assert_ok!(RelayerXcm::limited_reserve_transfer_assets(
			origin,
			Box::new(VersionedMultiLocation::V1(MultiLocation::new(
				1,
				X1(Parachain(THIS_PARA_ID))
			))),
			Box::new(Junction::AccountId32 { id: bob(), network: NetworkId::Any }.into().into()),
			Box::new(assets),
			0,
			WeightLimit::Unlimited,
		));
		assert_eq!(
			<Tokens as FungiblesInspect<_>>::balance(sibling_asset_id, &AccountId::from(bob())),
			total_issuance - transfer_amount
		);
		assert_eq!(
			<Tokens as FungiblesInspect<_>>::balance(unknown_asset_id, &AccountId::from(bob())),
			total_issuance - transfer_amount
		);
	});

	This::execute_with(|| {
		use this_runtime::*;
		log::info!(target: "bdd",  "Then destination chain took fee payment from first asset");
		let fee = xcmp::xcm_asset_fee_estimator(5, remote_sibling_asset_id);
		assert_gt!(transfer_amount, fee);
		let balance = Tokens::free_balance(remote_sibling_asset_id, &AccountId::from(bob()));
		assert_lt_by!(balance, transfer_amount, fee);

		log::info!(target: "bdd",  "  and destination chain have unknown tokens balance");
		let real_transfer_amount =
			orml_unknown_tokens::Pallet::<Runtime>::concrete_fungible_balances(
				MultiLocation {
					parents: 0,
					interior: X1(AccountId32 { id: bob(), network: NetworkId::Any }),
				},
				MultiLocation {
					parents: 1,
					interior: X2(Parachain(SIBLING_PARA_ID), GeneralIndex(unknown_asset_id.into())),
				},
			);
		assert_eq!(real_transfer_amount, transfer_amount);
	});
}
