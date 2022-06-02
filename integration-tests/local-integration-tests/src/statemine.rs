// //! Tests parachain to parachain xcm communication between Statemine and This.
// use crate::relaychain::kusama_test_net::*;
// use crate::setup::*;
// use cumulus_primitives_core::ParaId;

// use frame_support::assert_ok;
// use module_asset_registry::AssetMetadata;
// pub use orml_traits::GetByKey;
// use polkadot_parachain::primitives::Sibling;
// use xcm::v1::{Junction, MultiLocation};
// use xcm_emulator::TestExt;

pub const UNIT: Balance = 1_000_000_000_000;
pub const TEN: Balance = 10 * UNIT;
pub const FEE_WEIGHT: Balance = 4_000_000_000;
//pub const FEE_STATEMINE: Balance = 10_666_664;
pub const FEE_KUSAMA: Balance = 106_666_660;

use crate::{helpers::simtest, kusama_test_net::*, prelude::*};
use common::{xcmp::STATEMINE_PARA_ID, Balance};
use composable_traits::{defi::Ratio, xcm::assets::XcmAssetLocation};
use cumulus_primitives_core::ParaId;
use frame_support::{
	assert_ok, log,
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
			bob_on_statemine_original <
				statemine_runtime::Balances::free_balance(&AccountId::from(BOB)),
			"balance increased"
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
		polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account();
	let this_para_id: AccountId = ParaId::from(THIS_PARA_ID).into_account();
	let state_mine_para_id: AccountId = ParaId::from(STATEMINE_PARA_ID).into_account();

	// minimum asset should be: FEE_WEIGHT+FEE_KUSAMA+max(KUSAMA_ED,STATEMINE_ED+FEE_STATEMINE).
	// but due to current half fee, sender asset should at lease: FEE_WEIGHT + 2 * FEE_KUSAMA
	let relay_native_asset_amount = 3 * FEE_WEIGHT + 3 * FEE_KUSAMA;
	let remote_asset_id = 3451561; // magic number to avoid zero defaults and easy to find
	let foreign_asset_id_on_this = register_statemine_asset(remote_asset_id);
	let accounted_native_balance = statemine_side(TEN + relay_native_asset_amount, remote_asset_id);

	let (this_reserve, statemine_reserve) = KusamaRelay::execute_with(|| {
		let _ = kusama_runtime::Balances::make_free_balance_be(&this_para_id, TEN);
		(
			kusama_runtime::Balances::balance(&this_para_id),
			kusama_runtime::Balances::balance(&state_mine_para_id),
		)
	});

	this_chain_side(relay_native_asset_amount, foreign_asset_id_on_this);

	// during transfer relay rebalanced amounts
	KusamaRelay::execute_with(|| {
		assert!(kusama_runtime::Balances::free_balance(&this_para_id) < this_reserve);
		assert!(statemine_reserve < kusama_runtime::Balances::free_balance(&state_mine_para_id));
	});

	log::info!(target : "xcmp::test", "checking that assets for Bob are back");
	Statemine::execute_with(|| {
		use statemine_runtime::*;
		// This send back custom asset to Statemine, ensure recipient got custom asset
		assert_eq!(UNIT, Assets::balance(remote_asset_id, &AccountId::from(BOB)));
		// and withdraw sibling parachain sovereign account
		assert_eq!(9 * UNIT, Assets::balance(remote_asset_id, &this_parachain_account));

		assert_eq!(
			1003989333336, // approx. UNIT + FEE_WEIGHT - FEE_STATEMINE,
			Balances::free_balance(&AccountId::from(BOB))
		);
		let new_balance = Balances::free_balance(&this_parachain_account);
		assert!(accounted_native_balance <= new_balance);
		//old value 10016522666636
		assert_eq!(
			10016599690386, /* approximately this UNIT + asset_amount - FEE_WEIGHT - FEE_KUSAMA
			                 * - FEE_STATEMINE - FEE_WEIGHT, */
			new_balance,
		);
	});
}

// transfer custom asset from this chain  to Statemine
fn this_chain_side(fee_amount: u128, foreign_asset_id_on_this: CurrencyId) {
	This::execute_with(|| {
		use this_runtime::*;

		let bob_statemine_asset_amount =
			Tokens::free_balance(foreign_asset_id_on_this, &AccountId::from(BOB));
		// approx. TEN - fee
		// TODO: simulate tests to get only fees so can do little bit better than hardcode
		assert_eq!(9_999_936_000_000, bob_statemine_asset_amount,);
		// ensure sender has enough KSM balance to be charged as fee
		assert_ok!(Tokens::deposit(CurrencyId::RELAY_NATIVE, &AccountId::from(BOB), TEN));
		assert!(fee_amount != 0);
		log::info!(target: "xcmp::test", "sending assets back to statemine");
		assert_ok!(XTokens::transfer_multicurrencies(
			Origin::signed(BOB.into()),
			// statemine sends and receives only its ids from u32 range, which is our foreign
			// range,
			vec![(CurrencyId::RELAY_NATIVE, fee_amount), (foreign_asset_id_on_this, UNIT),],
			0, //1,
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
			4 * FEE_WEIGHT as u64
		));

		assert_eq!(
			bob_statemine_asset_amount - UNIT,
			Tokens::free_balance(foreign_asset_id_on_this, &AccountId::from(BOB))
		);
		assert_eq!(
			TEN - fee_amount,
			Tokens::free_balance(CurrencyId::RELAY_NATIVE, &AccountId::from(BOB))
		);
	});
}

// transfer custom asset from Statemine to This
fn statemine_side(
	this_parachain_account_init_amount: u128,
	statemine_asset_id: CommonAssetId,
) -> Balance {
	use statemine_runtime::*;
	let target_parachain: AccountId =
		polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account();

	Statemine::execute_with(|| {
		let origin = Origin::signed(ALICE.into());
		Balances::make_free_balance_be(&ALICE.into(), TEN);
		Balances::make_free_balance_be(&BOB.into(), UNIT);

		// create custom asset cost 1 KSM
		assert_ok!(Assets::create(
			origin.clone(),
			statemine_asset_id,
			MultiAddress::Id(ALICE.into()),
			UNIT / 100
		));
		assert_eq!(9 * UNIT, Balances::free_balance(&AccountId::from(ALICE)));

		assert_ok!(Assets::mint(
			origin.clone(),
			statemine_asset_id,
			MultiAddress::Id(ALICE.into()),
			1000 * UNIT
		));

		// need to have some KSM to be able to receive user assets
		Balances::make_free_balance_be(&target_parachain, this_parachain_account_init_amount);

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

	// Rerun the Statemine::execute to actually send the egress message via XCM
	Statemine::execute_with(|| Balances::balance(&target_parachain))
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

/// just have chain at hand
/// https://substrate.stackexchange.com/questions/1200/how-to-calculate-sovereignaccount-for-parachain/1210#1210
#[test]
fn parachain_account_id() {
	use base58::*;
	let account_id =
		hex::decode("7061726127080000000000000000000000000000000000000000000000000000").unwrap();
	let account_id = account_id.to_base58();
	assert_eq!(&account_id, "8ZgqzCPXjjjavhJpqifQEsfGAG69F7yPe3XH3D7XJUqD");
}
