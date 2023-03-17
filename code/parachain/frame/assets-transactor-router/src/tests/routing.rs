use frame_support::{assert_ok, traits::Currency};
use orml_traits::{MultiCurrency, MultiReservableCurrency};

use crate::{
	mocks::{new_test_ext, AccountId, AssetId, RuntimeOrigin, Test},
	Config, Pallet,
};
use composable_traits::{
	assets::{AssetInfo, BiBoundedAssetName, BiBoundedAssetSymbol, CreateAsset},
	xcm::assets::XcmAssetLocation,
};
use frame_support::traits::fungibles::{Inspect, InspectHold, MutateHold};

const NATIVE_ASSET_ID: AssetId = 1;
const ACCOUNT_NATIVE: u128 = 1;
const ACCOUNT_NATIVE_BALANCE: u128 = 3000;
const ACCOUNT_LOCAL: u128 = 2;
const ACCOUNT_LOCAL_BALANCE: u128 = 1000;
const ACCOUNT_FOREIGN: u128 = 3;
const ACCOUNT_FOREIGN_BALANCE: u128 = 2000;
const ACCOUNT_TO: u128 = 4;
type LocalTransactor = <Test as Config>::LocalTransactor;
type NativeTransactor = <Test as Config>::NativeTransactor;
type ForeignTransactor = <Test as Config>::ForeignTransactor;

// creates for routing 1 local asset and 1 foreign asset(native asset is specified in config)
fn create_assets() -> (AssetId, AssetId) {
	let protocol_id_local = *b"testloca";
	let nonce_local = 0;
	let protocol_id_foreign = *b"testfore";
	let nonce_foreign = 0;
	let asset_info_local = AssetInfo {
		name: Some(
			BiBoundedAssetName::from_vec(b"local asset".to_vec()).expect("string is within bound"),
		),
		symbol: None,
		decimals: Some(12),
		ratio: None,
		existential_deposit: 100,
	};
	let asset_id_local =
		Pallet::<Test>::create_local_asset(protocol_id_local, nonce_local, asset_info_local)
			.unwrap();

	let foreign_asset_id = XcmAssetLocation(xcm::v2::MultiLocation::parent());
	let foreign_asset_info = AssetInfo {
		name: Some(
			BiBoundedAssetName::from_vec(b"Kusama".to_vec()).expect("string is within bound"),
		),
		symbol: Some(
			BiBoundedAssetSymbol::from_vec(b"KSM".to_vec()).expect("string is withing bound"),
		),
		decimals: Some(12),
		ratio: None,
		existential_deposit: 1000,
	};

	let asset_id_foreign = Pallet::<Test>::create_foreign_asset(
		protocol_id_foreign,
		nonce_foreign,
		foreign_asset_info,
		foreign_asset_id,
	)
	.unwrap();
	(asset_id_local, asset_id_foreign)
}

// issue assets to different accounts and in different amount
fn mint_assets(asset_id_local: AssetId, asset_id_foreign: AssetId) {
	Pallet::<Test>::mint_into(
		RuntimeOrigin::root(),
		asset_id_local,
		ACCOUNT_LOCAL,
		ACCOUNT_LOCAL_BALANCE,
	)
	.unwrap();
	Pallet::<Test>::mint_into(
		RuntimeOrigin::root(),
		asset_id_foreign,
		ACCOUNT_FOREIGN,
		ACCOUNT_FOREIGN_BALANCE,
	)
	.unwrap();
	Pallet::<Test>::mint_into(
		RuntimeOrigin::root(),
		NATIVE_ASSET_ID,
		ACCOUNT_NATIVE,
		ACCOUNT_NATIVE_BALANCE,
	)
	.unwrap();
}

mod route {

	use super::*;

	#[test]
	fn total_issuance() {
		new_test_ext().execute_with(|| {
			let (asset_id_local, asset_id_foreign) = create_assets();
			mint_assets(asset_id_local, asset_id_foreign);
			assert_eq!(
				<Pallet<Test> as MultiCurrency<AccountId>>::total_issuance(NATIVE_ASSET_ID),
				ACCOUNT_NATIVE_BALANCE
			);
			assert_eq!(
				<Pallet<Test> as MultiCurrency<AccountId>>::total_issuance(asset_id_local),
				ACCOUNT_LOCAL_BALANCE
			);
			assert_eq!(
				<Pallet<Test> as MultiCurrency<AccountId>>::total_issuance(asset_id_foreign),
				ACCOUNT_FOREIGN_BALANCE
			);

			assert_eq!(NativeTransactor::total_issuance(), ACCOUNT_NATIVE_BALANCE);
			assert_eq!(LocalTransactor::total_issuance(asset_id_local), ACCOUNT_LOCAL_BALANCE);
			assert_eq!(
				ForeignTransactor::total_issuance(asset_id_foreign),
				ACCOUNT_FOREIGN_BALANCE
			);
		});
	}
	#[test]
	fn hold() {
		new_test_ext().execute_with(|| {
			let (asset_id_local, asset_id_foreign) = create_assets();
			mint_assets(asset_id_local, asset_id_foreign);
			const NATIVE_HOLD_BALANCE: u128 = 300;
			const LOCAL_HOLD_BALANCE: u128 = 100;
			const FOREIGN_HOLD_BALANCE: u128 = 200;

			assert_ok!(<Pallet<Test> as MutateHold<AccountId>>::hold(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				NATIVE_HOLD_BALANCE,
			));
			assert_ok!(<Pallet<Test> as MutateHold<AccountId>>::hold(
				asset_id_local,
				&ACCOUNT_LOCAL,
				LOCAL_HOLD_BALANCE,
			));
			assert_ok!(<Pallet<Test> as MutateHold<AccountId>>::hold(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				FOREIGN_HOLD_BALANCE,
			));

			assert_eq!(
				NativeTransactor::free_balance(&ACCOUNT_NATIVE),
				ACCOUNT_NATIVE_BALANCE - NATIVE_HOLD_BALANCE
			);
			assert_eq!(
				LocalTransactor::free_balance(asset_id_local, &ACCOUNT_LOCAL),
				ACCOUNT_LOCAL_BALANCE - LOCAL_HOLD_BALANCE
			);
			assert_eq!(
				ForeignTransactor::free_balance(asset_id_foreign, &ACCOUNT_FOREIGN),
				ACCOUNT_FOREIGN_BALANCE - FOREIGN_HOLD_BALANCE
			);
			assert_eq!(
				<Pallet<Test> as InspectHold<AccountId>>::balance_on_hold(
					NATIVE_ASSET_ID,
					&ACCOUNT_NATIVE,
				),
				NATIVE_HOLD_BALANCE
			);
			assert_eq!(
				<Pallet<Test> as InspectHold<AccountId>>::balance_on_hold(
					asset_id_local,
					&ACCOUNT_LOCAL,
				),
				LOCAL_HOLD_BALANCE
			);
			assert_eq!(
				<Pallet<Test> as InspectHold<AccountId>>::balance_on_hold(
					asset_id_foreign,
					&ACCOUNT_FOREIGN,
				),
				FOREIGN_HOLD_BALANCE
			);
		});
	}
}

mod route_asset_type {
	use frame_support::traits::WithdrawReasons;

	use super::*;

	#[test]
	fn asset_exists() {
		new_test_ext().execute_with(|| {
			let (asset_id_local, asset_id_foreign) = create_assets();
			const ASSET_NOT_EXIST: AssetId = 100;
			// non native should not exist
			assert!(!<Pallet<Test> as Inspect<AccountId>>::asset_exists(ASSET_NOT_EXIST));
			assert!(<Pallet<Test> as Inspect<AccountId>>::asset_exists(NATIVE_ASSET_ID));
			assert!(!<Pallet<Test> as Inspect<AccountId>>::asset_exists(asset_id_local));
			assert!(!<Pallet<Test> as Inspect<AccountId>>::asset_exists(asset_id_foreign));
			assert!(!LocalTransactor::asset_exists(asset_id_local));
			assert!(!ForeignTransactor::asset_exists(asset_id_foreign));

			mint_assets(asset_id_local, asset_id_foreign);
			// all should exist except ASSET_NOT_EXIST
			assert!(!<Pallet<Test> as Inspect<AccountId>>::asset_exists(ASSET_NOT_EXIST));
			assert!(<Pallet<Test> as Inspect<AccountId>>::asset_exists(NATIVE_ASSET_ID));
			assert!(<Pallet<Test> as Inspect<AccountId>>::asset_exists(asset_id_local));
			assert!(<Pallet<Test> as Inspect<AccountId>>::asset_exists(asset_id_foreign));
			assert!(LocalTransactor::asset_exists(asset_id_local));
			assert!(ForeignTransactor::asset_exists(asset_id_foreign));
		});
	}

	#[test]
	fn ensure_can_withdraw() {
		new_test_ext().execute_with(|| {
			let (asset_id_local, asset_id_foreign) = create_assets();
			mint_assets(asset_id_local, asset_id_foreign);

			assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				ACCOUNT_NATIVE_BALANCE
			));
			assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(
				asset_id_local,
				&ACCOUNT_LOCAL,
				ACCOUNT_LOCAL_BALANCE
			));
			assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::ensure_can_withdraw(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				ACCOUNT_FOREIGN_BALANCE
			));

			assert_ok!(NativeTransactor::ensure_can_withdraw(
				&ACCOUNT_NATIVE,
				ACCOUNT_NATIVE_BALANCE,
				WithdrawReasons::all(),
				0
			));
			assert_ok!(LocalTransactor::ensure_can_withdraw(
				asset_id_local,
				&ACCOUNT_LOCAL,
				ACCOUNT_LOCAL_BALANCE
			));
			LocalTransactor::ensure_can_withdraw(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				ACCOUNT_LOCAL_BALANCE,
			)
			.expect_err("wrong route");
			assert_ok!(ForeignTransactor::ensure_can_withdraw(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				ACCOUNT_FOREIGN_BALANCE
			));
		});
	}

	#[test]
	fn transfer() {
		new_test_ext().execute_with(|| {
			let (asset_id_local, asset_id_foreign) = create_assets();
			mint_assets(asset_id_local, asset_id_foreign);
			assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::transfer(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				&ACCOUNT_TO,
				ACCOUNT_NATIVE_BALANCE
			));
			assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::transfer(
				asset_id_local,
				&ACCOUNT_LOCAL,
				&ACCOUNT_TO,
				ACCOUNT_LOCAL_BALANCE
			));
			assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::transfer(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				&ACCOUNT_TO,
				ACCOUNT_FOREIGN_BALANCE
			));

			assert_eq!(NativeTransactor::total_balance(&ACCOUNT_TO), ACCOUNT_NATIVE_BALANCE);
			assert_eq!(
				LocalTransactor::total_balance(asset_id_local, &ACCOUNT_TO),
				ACCOUNT_LOCAL_BALANCE
			);
			assert_eq!(
				ForeignTransactor::total_balance(asset_id_foreign, &ACCOUNT_TO),
				ACCOUNT_FOREIGN_BALANCE
			);
		});
	}

	#[test]
	fn deposit() {
		new_test_ext().execute_with(|| {
			let (asset_id_local, asset_id_foreign) = create_assets();

			assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::deposit(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				ACCOUNT_NATIVE_BALANCE
			));
			assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::deposit(
				asset_id_local,
				&ACCOUNT_LOCAL,
				ACCOUNT_LOCAL_BALANCE
			));
			assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::deposit(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				ACCOUNT_FOREIGN_BALANCE
			));

			assert_eq!(NativeTransactor::total_balance(&ACCOUNT_NATIVE), ACCOUNT_NATIVE_BALANCE);
			assert_eq!(
				LocalTransactor::total_balance(asset_id_local, &ACCOUNT_LOCAL),
				ACCOUNT_LOCAL_BALANCE
			);
			assert_eq!(
				ForeignTransactor::total_balance(asset_id_foreign, &ACCOUNT_FOREIGN),
				ACCOUNT_FOREIGN_BALANCE
			);
		});
	}

	#[test]
	fn withdraw() {
		new_test_ext().execute_with(|| {
			let (asset_id_local, asset_id_foreign) = create_assets();
			mint_assets(asset_id_local, asset_id_foreign);

			assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::withdraw(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				ACCOUNT_NATIVE_BALANCE
			));
			assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::withdraw(
				asset_id_local,
				&ACCOUNT_LOCAL,
				ACCOUNT_LOCAL_BALANCE
			));
			assert_ok!(<Pallet<Test> as MultiCurrency<AccountId>>::withdraw(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				ACCOUNT_FOREIGN_BALANCE
			));

			assert_eq!(NativeTransactor::total_balance(&ACCOUNT_NATIVE), 0);
			assert_eq!(LocalTransactor::total_balance(asset_id_local, &ACCOUNT_LOCAL), 0);
			assert_eq!(ForeignTransactor::total_balance(asset_id_foreign, &ACCOUNT_FOREIGN), 0);
		});
	}

	#[test]
	fn slash() {
		new_test_ext().execute_with(|| {
			let (asset_id_local, asset_id_foreign) = create_assets();
			mint_assets(asset_id_local, asset_id_foreign);

			assert_eq!(
				<Pallet<Test> as MultiCurrency<AccountId>>::slash(
					NATIVE_ASSET_ID,
					&ACCOUNT_NATIVE,
					ACCOUNT_NATIVE_BALANCE
				),
				0
			);
			assert_eq!(
				<Pallet<Test> as MultiCurrency<AccountId>>::slash(
					asset_id_local,
					&ACCOUNT_LOCAL,
					ACCOUNT_LOCAL_BALANCE
				),
				0
			);
			assert_eq!(
				<Pallet<Test> as MultiCurrency<AccountId>>::slash(
					asset_id_foreign,
					&ACCOUNT_FOREIGN,
					ACCOUNT_FOREIGN_BALANCE
				),
				0
			);

			assert_eq!(NativeTransactor::total_balance(&ACCOUNT_NATIVE), 0);
			assert_eq!(LocalTransactor::total_balance(asset_id_local, &ACCOUNT_LOCAL), 0);
			assert_eq!(ForeignTransactor::total_balance(asset_id_foreign, &ACCOUNT_FOREIGN), 0);
		});
	}

	#[test]
	fn slash_reserved() {
		new_test_ext().execute_with(|| {
			let (asset_id_local, asset_id_foreign) = create_assets();
			mint_assets(asset_id_local, asset_id_foreign);
			const NATIVE_RESERVED: u128 = 2500;
			const LOCAL_RESERVED: u128 = 900;
			const FOREIGN_RESERVED: u128 = 2000;
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserve(
				NATIVE_ASSET_ID,
				&ACCOUNT_NATIVE,
				NATIVE_RESERVED,
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserve(
				asset_id_local,
				&ACCOUNT_LOCAL,
				LOCAL_RESERVED,
			));
			assert_ok!(<Pallet<Test> as MultiReservableCurrency<AccountId>>::reserve(
				asset_id_foreign,
				&ACCOUNT_FOREIGN,
				FOREIGN_RESERVED,
			));

			assert_eq!(
				<Pallet<Test> as MultiReservableCurrency<AccountId>>::slash_reserved(
					NATIVE_ASSET_ID,
					&ACCOUNT_NATIVE,
					NATIVE_RESERVED
				),
				0
			);
			assert_eq!(
				<Pallet<Test> as MultiReservableCurrency<AccountId>>::slash_reserved(
					asset_id_local,
					&ACCOUNT_LOCAL,
					LOCAL_RESERVED
				),
				0
			);
			assert_eq!(
				<Pallet<Test> as MultiReservableCurrency<AccountId>>::slash_reserved(
					asset_id_foreign,
					&ACCOUNT_FOREIGN,
					FOREIGN_RESERVED
				),
				0
			);

			assert_eq!(
				NativeTransactor::total_balance(&ACCOUNT_NATIVE),
				ACCOUNT_NATIVE_BALANCE - NATIVE_RESERVED
			);
			assert_eq!(
				LocalTransactor::total_balance(asset_id_local, &ACCOUNT_LOCAL),
				ACCOUNT_LOCAL_BALANCE - LOCAL_RESERVED
			);
			assert_eq!(
				ForeignTransactor::total_balance(asset_id_foreign, &ACCOUNT_FOREIGN),
				ACCOUNT_FOREIGN_BALANCE - FOREIGN_RESERVED
			);
			assert_eq!(NativeTransactor::reserved_balance(&ACCOUNT_NATIVE), 0);
			assert_eq!(LocalTransactor::reserved_balance(asset_id_local, &ACCOUNT_LOCAL), 0);
			assert_eq!(ForeignTransactor::reserved_balance(asset_id_foreign, &ACCOUNT_FOREIGN), 0);
		});
	}
}
