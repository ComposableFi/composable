// //! Tests parachain to parachain xcm communication between Statemine and Karura.
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
pub const TEN: Balance = 10_000_000_000_000;
pub const FEE_WEIGHT: Balance = 4_000_000_000;
//pub const FEE_STATEMINE: Balance = 10_666_664;
pub const FEE_STATEMINE: Balance = 4_000_000_000;
pub const FEE_KUSAMA: Balance = 106_666_660;

// fn init_statemine_xcm_interface() {
// 	let xcm_operation =
// 		module_xcm_interface::XcmInterfaceOperation::ParachainFee(Box::new((1, Parachain(1000)).into()));
// 	assert_ok!(<module_xcm_interface::Pallet<Runtime>>::update_xcm_dest_weight_and_fee(
// 		Origin::root(),
// 		vec![(xcm_operation.clone(), Some(4_000_000_000), Some(4_000_000_000),)],
// 	));
// 	System::assert_has_event(Event::XcmInterface(module_xcm_interface::Event::XcmDestWeightUpdated {
// 		xcm_operation: xcm_operation.clone(),
// 		new_xcm_dest_weight: 4_000_000_000,
// 	}));
// 	System::assert_has_event(Event::XcmInterface(module_xcm_interface::Event::XcmFeeUpdated {
// 		xcm_operation,
// 		new_xcm_dest_weight: 4_000_000_000,
// 	}));
// }

// #[test]
// fn statemine_min_xcm_fee_matched() {
// 	Statemine::execute_with(|| {
// 		use frame_support::weights::{IdentityFee, WeightToFeePolynomial};

// 		init_statemine_xcm_interface();
// 		let weight = FEE_WEIGHT as u64;

// 		let fee: Balance = IdentityFee::calc(&weight);
// 		let statemine: MultiLocation = (1, Parachain(parachains::statemine::ID)).into();
// 		let bifrost: MultiLocation = (1, Parachain(parachains::bifrost::ID)).into();

// 		let statemine_fee: u128 = ParachainMinFee::get(&statemine);
// 		assert_eq!(fee, statemine_fee);

// 		let bifrost_fee: u128 = ParachainMinFee::get(&bifrost);
// 		assert_eq!(u128::MAX, bifrost_fee);
// 	});
// }

use common::Balance;
use composable_traits::{assets::{RemoteAssetRegistry, XcmAssetLocation}, currency::{CurrencyFactory, RangeId}};
use cumulus_primitives_core::ParaId;
use orml_traits::MultiCurrency;
use primitives::currency::{CurrencyId, WellKnownCurrency};
use sp_runtime::{MultiAddress, traits::AccountIdConversion};
use support::{assert_ok, traits::Currency};
use xcm_emulator::TestExt;
use xcm::{v1::{Junction, MultiLocation}, VersionedMultiLocation};
use parachains_common::{AssetId as CommonAssetId};
use crate::{prelude::*, kusama_test_net::{*,}, helpers::simtest};

#[test]
fn transfer_native_from_relay_chain_to_statemine() {
	simtest();
	KusamaRelay::execute_with(|| {
		assert_ok!(kusama_runtime::XcmPallet::reserve_transfer_assets(
			kusama_runtime::Origin::signed(ALICE.into()),
			Box::new(Parachain(STATEMINE_PARA_ID).into().into()),
			Box::new(
				Junction::AccountId32 {
					id: BOB,
					network: NetworkId::Any
				}
				.into()
				.into()
			),
			Box::new((Here, CurrencyId::unit::<Balance>()).into()),
			0
		));
	});

	Statemine::execute_with(|| {
		assert_eq!(
			CurrencyId::unit::<Balance>() - FEE_STATEMINE,
			statemine_runtime::Balances::free_balance(&AccountId::from(BOB))
		);
	});
}

#[test]
fn this_chain_statemine_transfer_works() {
	simtest();
	let para_2000: AccountId = polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account();
	let child_2000: AccountId = ParaId::from(THIS_PARA_ID).into_account();
	let child_1000: AccountId = ParaId::from(STATEMINE_PARA_ID).into_account();

	// minimum asset should be: FEE_WEIGHT+FEE_KUSAMA+max(KUSAMA_ED,STATEMINE_ED+FEE_STATEMINE).
	// but due to current half fee, sender asset should at lease: FEE_WEIGHT + 2 * FEE_KUSAMA
	let asset_amount = FEE_WEIGHT + 2 * FEE_KUSAMA;
	let remote_asset_id = 0;
	let this_asset_id = register_asset(remote_asset_id);
	statemine_side(UNIT, remote_asset_id);

	KusamaRelay::execute_with(|| {
		let _ = kusama_runtime::Balances::make_free_balance_be(&child_2000, TEN);
	});

	// karura_side(asset_amount, this_asset_id);

	// KusamaRelay::execute_with(|| {
	// 	assert_eq!(
	// 		TEN - (asset_amount - FEE_WEIGHT),
	// 		kusama_runtime::Balances::free_balance(&child_2000)
	// 	);
	// 	assert_eq!(
	// 		asset_amount - FEE_WEIGHT - FEE_KUSAMA,
	// 		kusama_runtime::Balances::free_balance(&child_1000)
	// 	);
	// });

	// Statemine::execute_with(|| {
	// 	use statemine_runtime::*;
	// 	// Karura send back custom asset to Statemine, ensure recipient got custom asset
	// 	assert_eq!(UNIT, Assets::balance(0, &AccountId::from(BOB)));
	// 	// and withdraw sibling parachain sovereign account
	// 	assert_eq!(9 * UNIT, Assets::balance(0, &para_2000));

	// 	assert_eq!(
	// 		UNIT + FEE_WEIGHT - FEE_STATEMINE,
	// 		Balances::free_balance(&AccountId::from(BOB))
	// 	);
	// 	assert_eq!(
	// 		UNIT + asset_amount - FEE_WEIGHT - FEE_KUSAMA - FEE_STATEMINE - FEE_WEIGHT,
	// 		Balances::free_balance(&para_2000)
	// 	);
	// });
}

// TODO: statemine USDT asset_id == 11 and decimals == 4
// {
// 	deposit: 6,691,999,680
// 	name: USDT
// 	symbol: USDT
// 	decimals: 4
// 	isFrozen: false
//   }

// transfer custom asset from Karura to Statemine
fn karura_side(fee_amount: u128, asset_id: CurrencyId) {
		This::execute_with(|| {
		use this_runtime::*;
		//init_statemine_xcm_interface();

		assert_eq!(
			9_999_936_000_000,
			Tokens::free_balance(asset_id, &AccountId::from(BOB))
		);
		// ensure sender has enough KSM balance to be charged as fee
		assert_ok!(Tokens::deposit(CurrencyId::RELAY_NATIVE, &AccountId::from(BOB), TEN));

		assert_ok!(XTokens::transfer_multicurrencies(
			Origin::signed(BOB.into()),
			// state mint sends and receives only its ids from u32 range, which is our foreign range, 
			vec![(asset_id, UNIT), (CurrencyId::RELAY_NATIVE, fee_amount)],
			1,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(STATEMINE_PARA_ID),
						Junction::AccountId32 {
							network: NetworkId::Any,
							id: BOB.into(),
						}
					)
				)
				.into()
			),
			FEE_WEIGHT as u64
		));

		assert_eq!(
			8_999_936_000_000,
			Tokens::free_balance(asset_id, &AccountId::from(BOB))
		);
		assert_eq!(TEN - fee_amount, Tokens::free_balance(CurrencyId::RELAY_NATIVE, &AccountId::from(BOB)));
	});
}

// transfer custom asset from Statemine to This
fn statemine_side(para_2000_init_amount: u128, statemine_asset_id : CommonAssetId) {	
	let para_acc: AccountId = polkadot_parachain::primitives::Sibling::from(THIS_PARA_ID).into_account();

	Statemine::execute_with(|| {
		use statemine_runtime::*;

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
		Balances::make_free_balance_be(&para_acc, para_2000_init_amount);
		let noop : Xcm<Call> = Xcm(vec![]);
		// let tests = <PolkadotXcm as xcm::WrapVersion>::wrap_version(&MultiLocation::new(1, X1(Parachain(THIS_PARA_ID))).into(), noop.clone());
		// assert_ok!(tests);
		// PolkadotXcm::execute(origin.clone(), Box::new(xcm::VersionedXcm::V2(noop.into())), 42);
		assert_ok!(PolkadotXcm::reserve_transfer_assets(
			origin.clone(),
			Box::new(VersionedMultiLocation::V1(MultiLocation::new(1, X1(Parachain(THIS_PARA_ID)))).into()),
			Box::new(
				Junction::AccountId32 {
					id: BOB,
					network: NetworkId::Any
				}
				.into()
				.into()
			),
			// statemine knows only its asset ids an sends them to others to decode
			Box::new((X2(PalletInstance(50), GeneralIndex(statemine_asset_id as u128)), TEN).into()),
			0
		));

		// assert_eq!(0, Assets::balance(0, &AccountId::from(BOB)));

		// assert_eq!(TEN, Assets::balance(statemine_asset_id, &para_acc));
		// // the KSM balance of sibling parachain sovereign account is not changed
		// assert_eq!(para_2000_init_amount, Balances::free_balance(&para_acc));
	});

	// Rerun the Statemine::execute to actually send the egress message via XCM
	Statemine::execute_with(|| {});
}

fn register_asset(remote_asset_id : CommonAssetId) -> CurrencyId{
	This::execute_with(|| {
		// register foreign asset
		// TODO: move into assets registry as oneliner
		let id = <this_runtime::CurrencyFactory as CurrencyFactory<_>>::create(RangeId::FOREIGN_ASSETS).unwrap();
		assert_ok!(<this_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			id,
			XcmAssetLocation::new(MultiLocation::new(1, X3(Parachain(STATEMINE_PARA_ID), PalletInstance(50), GeneralIndex(remote_asset_id as u128))).into()),
		));
		id
	})
}
