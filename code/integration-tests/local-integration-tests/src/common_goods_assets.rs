// //! Tests parachain to parachain xcm communication between Statemine and This.

pub const UNIT: Balance = 1_000_000_000_000;
pub const TEN: Balance = 10 * UNIT;
// NOTE: alternative to having that found via test, it could be reading directly into storage/config
// of polkadot and statemine NOTE: or try some basic simulate tests to get only fees out of runs
pub const FEE_WEIGHT_THIS: Balance = 4_000_000_000;
pub const FEE_NATIVE_STATEMINE: Balance = 10_666_664;
pub const FEE_NATIVE_KUSAMA: Balance = 106_666_660;

use crate::{assert_lt_by, helpers::simtest, kusama_test_net::*, prelude::*};
use common::Balance;
use composable_traits::{defi::Ratio, xcm::assets::XcmAssetLocation};
use cumulus_primitives_core::ParaId;
use frame_support::{
	assert_ok, log,
	sp_runtime::assert_eq_error_rate,
	traits::{fungible::Inspect, Currency},
};
use orml_traits::MultiCurrency;
use parachains_common::AssetId as CommonAssetId;
use primitives::currency::{CurrencyId, WellKnownCurrency};
use sp_runtime::{traits::AccountIdConversion, MultiAddress};
use xcm::{
	v1::{Junction, MultiLocation},
	VersionedMultiLocation,
};
use xcm_emulator::TestExt;

#[test]
fn transfer_native_from_relay_chain_to_statemine() {
	simtest();
	let bob_on_statemine_original =
		Statemine::execute_with(|| statemine_runtime::Balances::balance(&AccountId::from(BOB)));
	let amount = RELAY_NATIVE_UNIT;
	KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		assert_ok!(XcmPallet::teleport_assets(
			Origin::signed(ALICE.into()),
			Box::new(Parachain(topology::common_good_assets::ID).into().into()),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			Box::new((Here, amount).into()),
			0,
		));
	});

	let bob_balance = Statemine::execute_with(|| {
		use statemine_runtime::*;
		let bob_balance = Balances::free_balance(&AccountId::from(BOB));
		assert_gt!(bob_balance, bob_on_statemine_original,);
		assert_lt_by!(bob_balance, amount, FEE_NATIVE_KUSAMA,);
		bob_balance
	});
}

#[test]
fn transfer_native_from_statemine_to_this() {
	simtest();
	let bob_on_statemine_original =
		Statemine::execute_with(|| statemine_runtime::Balances::balance(&AccountId::from(BOB)));
	let amount = RELAY_NATIVE_UNIT;
	KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		assert_ok!(XcmPallet::teleport_assets(
			Origin::signed(ALICE.into()),
			Box::new(Parachain(topology::common_good_assets::ID).into().into()),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			Box::new((Here, amount).into()),
			0,
		));
	});

	let bob_balance = Statemine::execute_with(|| {
		use statemine_runtime::*;
		let bob_balance = Balances::balance(&AccountId::from(BOB));
		let origin = Origin::signed(BOB.into());

		assert_ok!(PolkadotXcm::reserve_transfer_assets(
			origin.clone(),
			Box::new(
				VersionedMultiLocation::V1(MultiLocation::new(1, X1(Parachain(THIS_PARA_ID))))
					.into()
			),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			// statemine knows only its asset ids an sends them to others to decode
			Box::new((MultiLocation::new(1, Here), bob_balance).into()),
			0,
		));
		bob_balance
	});

	This::execute_with(|| {
		use this_runtime::*;
		let bob_ksm = Tokens::free_balance(CurrencyId::KSM, &AccountId::from(BOB));
		assert_lt_by!(bob_ksm, bob_balance, RELAY_CHAIN_NATIVE_FEE,);
	});
}

#[test]
fn transfer_usdt_from_statemine_to_this() {
	simtest();
	let bob_on_statemine_original =
		Statemine::execute_with(|| statemine_runtime::Balances::balance(&AccountId::from(BOB)));
	let amount = RELAY_NATIVE_UNIT;

	let statemine_asset_id = 1984;
	let remote_statemine_asset_id = CurrencyId::USDT;
	let usdt_transfer_amount = 1_000_000_000_000;
	let total_issuance = 3_500_000_000_000;
	Statemine::execute_with(|| {
		log::info!(target: "bdd", "Given USDT on Statemine registered");
		use statemine_runtime::*;
		let root = frame_system::RawOrigin::Root;

		Assets::force_create(
			root.clone().into(),
			statemine_asset_id,
			MultiAddress::Id(ALICE.into()),
			true,
			1000,
		)
		.unwrap();
		log::info!(target: "bdd", "	and Bob has a lot USDT on Statemine");
		Assets::mint(
			Origin::signed(ALICE.into()),
			statemine_asset_id,
			MultiAddress::Id(BOB.into()),
			total_issuance,
		)
		.unwrap();
	});

	Statemine::execute_with(|| {
		log::info!(target: "bdd", "When Bob transfers some {:?} USDT from from Statemine to Dali", usdt_transfer_amount);
		use statemine_runtime::*;
		let origin = Origin::signed(BOB.into());
		assert_ok!(PolkadotXcm::limited_reserve_transfer_assets(
			origin.clone(),
			Box::new(
				VersionedMultiLocation::V1(MultiLocation::new(1, X1(Parachain(THIS_PARA_ID))))
					.into()
			),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			Box::new(
				(
					X2(PalletInstance(50), GeneralIndex(statemine_asset_id as u128)),
					usdt_transfer_amount
				)
					.into()
			),
			0,
			WeightLimit::Unlimited,
		));
		assert_eq!(
			Assets::balance(statemine_asset_id, &AccountId::from(BOB)),
			total_issuance - usdt_transfer_amount
		);
	});
	Statemine::execute_with(|| {});
	This::execute_with(|| {
		log::info!(target: "bdd", "Then Bob gets some USDT on Dali");
		use this_runtime::*;
		let balance = Tokens::free_balance(remote_statemine_asset_id, &AccountId::from(BOB));
		assert_lt_by!(
			balance,
			usdt_transfer_amount,
			this_runtime::xcmp::xcm_asset_fee_estimator(5, remote_statemine_asset_id)
		);
	});
}

#[test]
fn rockmine_shib_to_dali_transfer() {
	simtest();
	let this_parachain_account: AccountId =
		polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account_truncating();
	let statemine_parachain_account: AccountId =
		ParaId::from(topology::common_good_assets::ID).into_account_truncating();
	let statemine_asset_id = 100500;
	let total_issuance = 3_500_000_000_000;
	Statemine::execute_with(|| {
		log::info!(target: "bdd", "Given SHIB on Statemine registered");
		use statemine_runtime::*;
		let root = frame_system::RawOrigin::Root;

		Assets::force_create(
			root.clone().into(),
			statemine_asset_id,
			MultiAddress::Id(ALICE.into()),
			true,
			1000,
		)
		.unwrap();
		log::info!(target: "bdd", "	and Bob has a lot SHIB on Statemine");
		Assets::mint(
			Origin::signed(ALICE.into()),
			statemine_asset_id,
			MultiAddress::Id(BOB.into()),
			total_issuance,
		)
		.unwrap();
	});

	let remote_statemine_asset_id = This::execute_with(|| {
		log::info!(target: "bdd", "	and USD on Dali registered");
		use this_runtime::*;
		let root = frame_system::RawOrigin::Root;
		let location = XcmAssetLocation::new(
			MultiLocation::new(
				1,
				X3(
					Parachain(topology::common_good_assets::ID),
					PalletInstance(50),
					GeneralIndex(statemine_asset_id as u128),
				),
			)
			.into(),
		);
		AssetsRegistry::register_asset(
			root.into(),
			location.clone(),
			1000,
			Some(Ratio::from_rational(15, 1000)),
			Some(4),
		)
		.unwrap();
		System::events()
			.iter()
			.find_map(|x| match x.event {
				Event::AssetsRegistry(assets_registry::Event::<Runtime>::AssetRegistered {
					asset_id,
					location: _,
				}) => Some(asset_id),
				_ => None,
			})
			.unwrap()
	});
	log::info!(target: "bdd", "{:?}", remote_statemine_asset_id);
	let transfer_amount = 1_000_000_000_000;
	Statemine::execute_with(|| {
		log::info!(target: "bdd", "When Bob transfers some {:?} SHIB from from Statemine to Dali", transfer_amount);
		use statemine_runtime::*;
		let origin = Origin::signed(BOB.into());
		assert_ok!(PolkadotXcm::limited_reserve_transfer_assets(
			origin.clone(),
			Box::new(
				VersionedMultiLocation::V1(MultiLocation::new(1, X1(Parachain(THIS_PARA_ID))))
					.into()
			),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			Box::new(
				(X2(PalletInstance(50), GeneralIndex(statemine_asset_id as u128)), transfer_amount)
					.into()
			),
			0,
			WeightLimit::Unlimited,
		));
		assert_eq!(
			Assets::balance(statemine_asset_id, &AccountId::from(BOB)),
			total_issuance - transfer_amount
		);
	});

	This::execute_with(|| {
		log::info!(target: "bdd", "Then Bob gets some SHIB on Dali");
		use this_runtime::*;
		let balance = Tokens::free_balance(remote_statemine_asset_id, &AccountId::from(BOB));
		assert_eq!(balance, transfer_amount,);
	});
}

#[test]
fn this_chain_statemine_transfers_back_and_forth_work() {
	simtest();
	let this_parachain_account: AccountId =
		polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account_truncating();
	let this_para_id: AccountId = ParaId::from(THIS_PARA_ID).into_account_truncating();
	let statemine_para_id: AccountId =
		ParaId::from(topology::common_good_assets::ID).into_account_truncating();

	let relay_native_asset_amount = 3 * FEE_WEIGHT_THIS + 3 * FEE_NATIVE_KUSAMA;
	let remote_asset_id = 3451561; // magic number to avoid zero defaults and easy to find
	let foreign_asset_id_on_this =
		register_statemine_asset(remote_asset_id, Ratio::checked_from_rational(10_u8, 100));

	statemine_side(TEN + relay_native_asset_amount, remote_asset_id);
	let statemine_native_this_balance_1 =
		Statemine::execute_with(|| statemine_runtime::Balances::balance(&this_parachain_account));

	let (this_reserve, statemine_reserve) = KusamaRelay::execute_with(|| {
		log::info!(target : "xcmp::test", "============ RELAY");
		let _ = relay_runtime::Balances::make_free_balance_be(&this_para_id, TEN);
		(
			relay_runtime::Balances::balance(&this_para_id),
			relay_runtime::Balances::balance(&statemine_para_id),
		)
	});

	this_chain_side(relay_native_asset_amount, foreign_asset_id_on_this);

	// during transfer relay rebalanced amounts
	KusamaRelay::execute_with(|| {
		assert!(relay_runtime::Balances::balance(&this_para_id) < this_reserve);
		assert_eq!(statemine_reserve, relay_runtime::Balances::free_balance(&statemine_para_id));
	});

	log::info!(target : "xcmp::test", "checking that assets for Bob are back");
	Statemine::execute_with(|| {
		log::info!(target : "xcmp::test", "============ ASSETS");
		use statemine_runtime::*;
		// This send back custom asset to Statemine, ensure recipient got custom asset
		assert_eq!(UNIT, Assets::balance(remote_asset_id, &AccountId::from(BOB)));
		// and withdraw sibling parachain sovereign account
		assert_eq!(9 * UNIT, Assets::balance(remote_asset_id, &this_parachain_account));

		assert_eq_error_rate!(
			UNIT + FEE_WEIGHT_THIS - FEE_NATIVE_STATEMINE,
			Balances::free_balance(&AccountId::from(BOB)),
			ORDER_OF_FEE_ESTIMATE_ERROR * FEE_NATIVE_STATEMINE,
		);

		let statemine_native_this_balance_2 = Balances::balance(&this_parachain_account);

		//	#[cfg(not(feature = "rococo"))]
		assert_lt!(
			statemine_native_this_balance_1,
			statemine_native_this_balance_2,
			"we transferred "
		);

		let hops = 2;
		assert_eq_error_rate!(
			statemine_native_this_balance_1 + relay_native_asset_amount,
			statemine_native_this_balance_2,
			hops * ORDER_OF_FEE_ESTIMATE_ERROR *
				(FEE_NATIVE_KUSAMA + FEE_NATIVE_STATEMINE + FEE_WEIGHT_THIS),
		);
		assert!(
			statemine_native_this_balance_2 <
				statemine_native_this_balance_1 + relay_native_asset_amount
		);
	});
}

// transfer custom asset from this chain  to Statemine
fn this_chain_side(relay_native_asset_amount: u128, foreign_asset_id_on_this: CurrencyId) {
	This::execute_with(|| {
		log::info!(target: "xcmp::test", "============= THIS");
		use this_runtime::*;

		let bob_statemine_asset_amount =
			Tokens::free_balance(foreign_asset_id_on_this, &AccountId::from(BOB));
		assert!(
			bob_statemine_asset_amount < TEN &&
				bob_statemine_asset_amount > TEN - FEE_NATIVE_STATEMINE - FEE_WEIGHT_THIS,
			"Fee taken up to some limit {:?} < {:?} && {:?} > {:?}",
			bob_statemine_asset_amount,
			TEN,
			bob_statemine_asset_amount,
			TEN - FEE_NATIVE_STATEMINE - FEE_WEIGHT_THIS
		);
		// ensure sender has enough KSM balance to be charged as fee
		assert_ok!(Tokens::deposit(CurrencyId::RELAY_NATIVE, &AccountId::from(BOB), TEN));
		assert!(relay_native_asset_amount != 0);
		log::info!(target: "xcmp::test", "sending assets back to statemine");
		assert_ok!(XTokens::transfer_multicurrencies(
			Origin::signed(BOB.into()),
			// statemine sends and receives only its ids from u32 range, which is our foreign
			// range,
			vec![
				(CurrencyId::RELAY_NATIVE, relay_native_asset_amount),
				(foreign_asset_id_on_this, UNIT),
			],
			0,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(topology::common_good_assets::ID),
						Junction::AccountId32 { network: NetworkId::Any, id: BOB.into() }
					)
				)
				.into()
			),
			4 * FEE_WEIGHT_THIS as u64
		));

		assert_eq!(
			bob_statemine_asset_amount - UNIT,
			Tokens::free_balance(foreign_asset_id_on_this, &AccountId::from(BOB))
		);
		assert_eq!(
			TEN - relay_native_asset_amount,
			Tokens::free_balance(CurrencyId::RELAY_NATIVE, &AccountId::from(BOB))
		);
	});
}

fn statemine_setup_assets(
	native_for_alice: Balance,
	native_for_bob: Balance,
	statemine_asset_id: CommonAssetId,
	other_ed: Balance,
	other_total: Balance,
	foreign_chain_account: AccountId,
	this_parachain_account_init_amount: Balance,
) -> () {
	use statemine_runtime::*;
	Statemine::execute_with(|| {
		let origin = Origin::signed(ALICE.into());
		Balances::make_free_balance_be(&ALICE.into(), native_for_alice);
		Balances::make_free_balance_be(&BOB.into(), native_for_bob);

		// create custom asset cost 1 KSM
		assert_ok!(Assets::create(
			origin.clone(),
			statemine_asset_id,
			MultiAddress::Id(ALICE.into()),
			other_ed,
		));

		assert_eq!(native_for_alice, Balances::balance(&AccountId::from(ALICE)),);

		assert_ok!(Assets::mint(
			origin.clone(),
			statemine_asset_id,
			MultiAddress::Id(ALICE.into()),
			other_total
		));

		// need to have some KSM to be able to receive user assets
		Balances::make_free_balance_be(&foreign_chain_account, this_parachain_account_init_amount);
	});
}

// transfer custom asset from Statemine to This
fn statemine_side(this_parachain_account_init_amount: u128, statemine_asset_id: CommonAssetId) {
	use statemine_runtime::*;
	let target_parachain: AccountId =
		polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account_truncating();
	statemine_setup_assets(
		TEN,
		UNIT,
		statemine_asset_id,
		UNIT / 100,
		1000 * UNIT,
		target_parachain.clone(),
		this_parachain_account_init_amount,
	);

	Statemine::execute_with(|| {
		log::info!(target: "xcmp::test", "============= ASSETS");
		let origin = Origin::signed(ALICE.into());

		assert_ok!(PolkadotXcm::reserve_transfer_assets(
			origin.clone(),
			Box::new(
				VersionedMultiLocation::V1(MultiLocation::new(1, X1(Parachain(THIS_PARA_ID))))
					.into()
			),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			// statemine knows only its asset ids an sends them to others to decode
			Box::new(
				(X2(PalletInstance(50), GeneralIndex(statemine_asset_id as u128)), TEN).into()
			),
			0
		));

		// assets was not transferred to local account for sure
		assert_eq!(0, Assets::balance(statemine_asset_id, &AccountId::from(BOB)));

		// asset is considered reserve on statemine
		assert_eq!(TEN, Assets::balance(statemine_asset_id, &target_parachain));

		// the KSM balance of sibling parachain sovereign account is not changed because we
		// transferred NOT KSM
		assert_eq!(this_parachain_account_init_amount, Balances::free_balance(&target_parachain));
	});
}

fn register_statemine_asset(remote_asset_id: CommonAssetId, ratio: Option<Ratio>) -> CurrencyId {
	This::execute_with(|| {
		use this_runtime::*;
		let location = XcmAssetLocation::new(
			MultiLocation::new(
				1,
				X3(
					Parachain(topology::common_good_assets::ID),
					PalletInstance(50),
					GeneralIndex(remote_asset_id as u128),
				),
			)
			.into(),
		);
		AssetsRegistry::register_asset(
			frame_system::RawOrigin::Root.into(),
			location.clone(),
			42,
			ratio,
			None,
		)
		.unwrap();
		let location = XcmAssetLocation::new(
			MultiLocation::new(1, X1(Parachain(topology::common_good_assets::ID))).into(),
		);
		AssetsRegistry::set_min_fee(
			frame_system::RawOrigin::Root.into(),
			ParaId::from(topology::common_good_assets::ID),
			location,
			Some(4_000_000_000),
		)
		.unwrap();
		System::events()
			.iter()
			.find_map(|x| match x.event {
				Event::AssetsRegistry(assets_registry::Event::<Runtime>::AssetRegistered {
					asset_id,
					location: _,
				}) => Some(asset_id),
				_ => None,
			})
			.unwrap()
	})
}

#[test]
fn general_index_asset() {
	let asset_id: u128 = 11;
	let asset_id = hex::encode(asset_id.encode());
	assert_eq!(&asset_id, "0b000000000000000000000000000000");
}
