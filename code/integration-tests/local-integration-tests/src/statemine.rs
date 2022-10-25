// //! Tests parachain to parachain xcm communication between Statemine and This.

pub const UNIT: Balance = 1_000_000_000_000;
pub const TEN: Balance = 10 * UNIT;
// NOTE: alternative to having that found via test, it could be reading directly into storage/config
// of polkadot and statemine NOTE: or try some basic simulate tests to get only fees out of runs
pub const FEE_WEIGHT_THIS: Balance = 4_000_000_000;
pub const FEE_NATIVE_STATEMINE: Balance = 10_666_664;
pub const FEE_NATIVE_KUSAMA: Balance = 106_666_660;

use crate::{helpers::simtest, kusama_test_net::*, prelude::*};
use common::{xcmp::STATEMINE_PARA_ID, Balance};
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

// <= what we may think users are ok
const ORDER_OF_FEE_ESTIMATE_ERROR: Balance = 10;

#[test]
fn transfer_native_from_relay_chain_to_statemine() {
	simtest();
	let bob_on_statemine_original = Statemine::execute_with(|| {
		statemine_runtime::Balances::free_balance(&AccountId::from(BOB))
	});
	let amount = CurrencyId::unit::<Balance>();
	KusamaRelay::execute_with(|| {
		assert_ok!(kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(STATEMINE_PARA_ID).into().into()),
			Box::new(Junction::AccountId32 { id: BOB, network: NetworkId::Any }.into().into()),
			Box::new((Here, amount).into()),
			0
		));
	});

	Statemine::execute_with(|| {
		assert!(
			bob_on_statemine_original <=
				statemine_runtime::Balances::free_balance(&AccountId::from(BOB)),
			"balance increased: {}, {}",
			bob_on_statemine_original,
			statemine_runtime::Balances::free_balance(&AccountId::from(BOB))
		);
		assert!(
			amount > statemine_runtime::Balances::free_balance(&AccountId::from(BOB)),
			"fee taken"
		);
	});
}

/// Statemine issues custom token
#[test]
fn this_chain_statemine_transfers_back_and_forth_work() {
	simtest();
	let this_parachain_account: AccountId =
		polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account_truncating();
	let this_para_id: AccountId = ParaId::from(THIS_PARA_ID).into_account_truncating();
	let statemine_para_id: AccountId = ParaId::from(STATEMINE_PARA_ID).into_account_truncating();

	// minimum asset should be:
	// FEE_WEIGHT_THIS+FEE_NATIVE_KUSAMA+max(KUSAMA_ED,STATEMINE_ED+FEE_NATIVE_STATEMINE). but due
	// to current half fee, sender asset should at least: FEE_WEIGHT_THIS + 2 * FEE_NATIVE_KUSAMA
	let relay_native_asset_amount = 3 * FEE_WEIGHT_THIS + 3 * FEE_NATIVE_KUSAMA;
	let remote_asset_id = 3451561; // magic number to avoid zero defaults and easy to find
	let foreign_asset_id_on_this = register_statemine_asset(remote_asset_id);
	let this_parachain_account_id: AccountId =
		polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account_truncating();

	statemine_side(TEN + relay_native_asset_amount, remote_asset_id);
	let accounted_native_balance = Statemine::execute_with(|| {
		statemine_runtime::Balances::balance(&this_parachain_account_id)
	});
	let (this_reserve, statemine_reserve) = KusamaRelay::execute_with(|| {
		let _ = kusama_runtime::Balances::make_free_balance_be(&this_para_id, TEN);
		(
			kusama_runtime::Balances::balance(&this_para_id),
			kusama_runtime::Balances::balance(&statemine_para_id),
		)
	});

	this_chain_side(relay_native_asset_amount, foreign_asset_id_on_this);

	// during transfer relay rebalanced amounts
	KusamaRelay::execute_with(|| {
		assert!(kusama_runtime::Balances::free_balance(&this_para_id) < this_reserve);
		assert_eq!(statemine_reserve, kusama_runtime::Balances::free_balance(&statemine_para_id));
	});

	log::info!(target : "xcmp::test", "checking that assets for Bob are back");
	Statemine::execute_with(|| {
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
		let new_native_amount = Balances::balance(&this_parachain_account);
		assert!(
			accounted_native_balance < new_native_amount,
			"we test only case when fee is less than total transfer"
		);

		let hops = 2;
		assert_eq_error_rate!(
			accounted_native_balance + relay_native_asset_amount,
			new_native_amount,
			hops * ORDER_OF_FEE_ESTIMATE_ERROR *
				(FEE_NATIVE_KUSAMA + FEE_NATIVE_STATEMINE + FEE_WEIGHT_THIS),
		);
		assert!(new_native_amount < accounted_native_balance + relay_native_asset_amount);
	});
}

// transfer custom asset from this chain  to Statemine
fn this_chain_side(relay_native_asset_amount: u128, foreign_asset_id_on_this: CurrencyId) {
	This::execute_with(|| {
		use this_runtime::*;

		let bob_statemine_asset_amount =
			Tokens::free_balance(foreign_asset_id_on_this, &AccountId::from(BOB));
		// approx. TEN - fee
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
						Parachain(STATEMINE_PARA_ID),
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

fn register_statemine_asset(remote_asset_id: CommonAssetId) -> CurrencyId {
	This::execute_with(|| {
		use this_runtime::*;
		// ISSUE: XTokens do not support minimal fee per assets and it is mentioned in their code
		let location = XcmAssetLocation::new(
			MultiLocation::new(
				1,
				X3(
					Parachain(STATEMINE_PARA_ID),
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
			Ratio::checked_from_rational(10_u8, 100),
			Some(4),
		)
		.unwrap();
		let location =
			XcmAssetLocation::new(MultiLocation::new(1, X1(Parachain(STATEMINE_PARA_ID))).into());
		AssetsRegistry::set_min_fee(
			frame_system::RawOrigin::Root.into(),
			ParaId::from(STATEMINE_PARA_ID),
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
