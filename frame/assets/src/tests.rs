use crate::*;
//use composable_traits::currency::MultiCurrency;
use mocks::{new_test_ext, GovernanceRegistry, Origin, Test};
use orml_traits::MultiCurrency;

const FROM_ACCOUNT: u64 = 1;
const TO_ACCOUNT: u64 = 2;
const ASSET_ID: u64 = 1;
const INIT_AMOUNT: u64 = 1000;
const TRANSFER_AMOUNT: u64 = 500;

#[test]
fn set_only_by_root() {
	new_test_ext().execute_with(|| {
		GovernanceRegistry::set(Origin::root(), 1, 1).unwrap();
		ensure_admin_or_governance::<Test>(Origin::root(), &2).unwrap();
		ensure_admin_or_governance::<Test>(Origin::signed(1), &2).unwrap_err();
		ensure_admin_or_governance::<Test>(Origin::signed(2), &1).unwrap_err();
		ensure_admin_or_governance::<Test>(Origin::signed(1), &1).unwrap();
		ensure_admin_or_governance::<Test>(Origin::none(), &1).unwrap_err();
		ensure_admin_or_governance::<Test>(Origin::none(), &2).unwrap_err();
	});
}

#[test]
fn test_transfer() {
	new_test_ext().execute_with(|| {
		Pallet::<Test>::transfer(
			Origin::signed(FROM_ACCOUNT),
			ASSET_ID,
			TO_ACCOUNT,
			TRANSFER_AMOUNT,
			true,
		)
		.expect("transfer should work");
		assert_eq!(
			Pallet::<Test>::total_balance(ASSET_ID, &FROM_ACCOUNT),
			INIT_AMOUNT - TRANSFER_AMOUNT
		);
		assert_eq!(
			Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
	});
}

#[test]
fn test_transfer_native() {
	new_test_ext().execute_with(|| {
		Pallet::<Test>::transfer_native(
			Origin::signed(FROM_ACCOUNT),
			TO_ACCOUNT,
			TRANSFER_AMOUNT,
			true,
		)
		.expect("transfer_native should work");
		assert_eq!(
			Pallet::<Test>::total_balance(ASSET_ID, &FROM_ACCOUNT),
			INIT_AMOUNT - TRANSFER_AMOUNT
		);
		assert_eq!(
			Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
	});
}

#[test]
fn test_force_transfer() {
	new_test_ext().execute_with(|| {
		Pallet::<Test>::force_transfer(
			Origin::root(),
			ASSET_ID,
			FROM_ACCOUNT,
			TO_ACCOUNT,
			TRANSFER_AMOUNT,
			true,
		)
		.expect("force_transfer should work");
		assert_eq!(
			Pallet::<Test>::total_balance(ASSET_ID, &FROM_ACCOUNT),
			INIT_AMOUNT - TRANSFER_AMOUNT
		);
		assert_eq!(
			Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
	});
}

#[test]
fn test_force_transfer_native() {
	new_test_ext().execute_with(|| {
		Pallet::<Test>::force_transfer_native(
			Origin::root(),
			FROM_ACCOUNT,
			TO_ACCOUNT,
			TRANSFER_AMOUNT,
			true,
		)
		.expect("force_transfer_native should work");
		assert_eq!(
			Pallet::<Test>::total_balance(ASSET_ID, &FROM_ACCOUNT),
			INIT_AMOUNT - TRANSFER_AMOUNT
		);
		assert_eq!(
			Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
	});
}

#[test]
fn test_transfer_all() {
	new_test_ext().execute_with(|| {
		Pallet::<Test>::transfer_all(Origin::signed(FROM_ACCOUNT), ASSET_ID, TO_ACCOUNT, true)
			.expect("transfer_all should work");
		assert_eq!(Pallet::<Test>::total_balance(ASSET_ID, &FROM_ACCOUNT), 1);
		assert_eq!(Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT), INIT_AMOUNT * 2 - 1);
	});
}

#[test]
fn test_transfer_all_native() {
	new_test_ext().execute_with(|| {
		Pallet::<Test>::transfer_all_native(Origin::signed(FROM_ACCOUNT), TO_ACCOUNT, true)
			.expect("transfer_all_native should work");
		assert_eq!(Pallet::<Test>::total_balance(ASSET_ID, &FROM_ACCOUNT), 1);
		assert_eq!(Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT), INIT_AMOUNT * 2 - 1);
	});
}

#[test]
fn test_mint_initialize() {
	new_test_ext().execute_with(|| {
		assert_eq!(Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT), INIT_AMOUNT);
		Pallet::<Test>::mint_initialize(Origin::root(), TRANSFER_AMOUNT, TO_ACCOUNT)
			.expect("mint_initialize should work");
		assert_eq!(
			Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
	});
}

#[test]
fn test_mint_initialize_with_governance() {
	new_test_ext().execute_with(|| {
		assert_eq!(Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT), INIT_AMOUNT);
		Pallet::<Test>::mint_initialize_with_governance(
			Origin::root(),
			TRANSFER_AMOUNT,
			TO_ACCOUNT,
			TO_ACCOUNT,
		)
		.expect("mint_initialize_with_governance should work");
		assert_eq!(
			Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
		ensure_admin_or_governance::<Test>(Origin::signed(TO_ACCOUNT), &ASSET_ID).expect(
			"mint_initialize_with_governance should add governance_origin to GovernanceRegistry",
		);
	});
}

#[test]
fn test_mint_into() {
	new_test_ext().execute_with(|| {
		GovernanceRegistry::set(Origin::root(), ASSET_ID, FROM_ACCOUNT).unwrap();
		assert_eq!(Pallet::<Test>::total_balance(ASSET_ID, &FROM_ACCOUNT), INIT_AMOUNT);
		assert_eq!(Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT), INIT_AMOUNT);

		Pallet::<Test>::mint_into(
			Origin::signed(FROM_ACCOUNT),
			ASSET_ID,
			TO_ACCOUNT,
			TRANSFER_AMOUNT,
		)
		.expect("mint_into should work");
		assert_eq!(Pallet::<Test>::total_balance(ASSET_ID, &FROM_ACCOUNT), INIT_AMOUNT);
		assert_eq!(
			Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
	});
}

#[test]
fn test_burn_from() {
	new_test_ext().execute_with(|| {
		GovernanceRegistry::set(Origin::root(), ASSET_ID, FROM_ACCOUNT).unwrap();
		assert_eq!(Pallet::<Test>::total_balance(ASSET_ID, &FROM_ACCOUNT), INIT_AMOUNT);
		assert_eq!(Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT), INIT_AMOUNT);

		Pallet::<Test>::burn_from(
			Origin::signed(FROM_ACCOUNT),
			ASSET_ID,
			TO_ACCOUNT,
			TRANSFER_AMOUNT,
		)
		.expect("burn_from should work");
		assert_eq!(Pallet::<Test>::total_balance(ASSET_ID, &FROM_ACCOUNT), INIT_AMOUNT);
		assert_eq!(
			Pallet::<Test>::total_balance(ASSET_ID, &TO_ACCOUNT),
			INIT_AMOUNT - TRANSFER_AMOUNT
		);
	});
}
