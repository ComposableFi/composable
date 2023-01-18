use crate::*;
use mocks::{new_test_ext, GovernanceRegistry, Origin, Test};
use orml_traits::MultiCurrency;

const FROM_ACCOUNT: u64 = 1;
const TO_ACCOUNT: u64 = 2;
const INIT_AMOUNT: u64 = 1000;
const TRANSFER_AMOUNT: u64 = 500;

fn create_asset_id(protocol_id: [u8; 8], nonce: u64) -> u128 {
	let bytes = protocol_id
		.into_iter()
		.chain(nonce.to_le_bytes())
		.collect::<Vec<u8>>()
		.try_into()
		.expect("[u8; 8] + bytes(u64) = [u8; 16]");

	u128::from_le_bytes(bytes)
}

#[test]
#[ignore = "Not sure what this is testing"]
fn set_only_by_root() {
	new_test_ext().execute_with(|| {
		GovernanceRegistry::set(Origin::root(), 1, 1).unwrap();
		Pallet::<Test>::ensure_admin_or_governance(Origin::root(), &2).unwrap();
		Pallet::<Test>::ensure_admin_or_governance(Origin::signed(1), &2).unwrap_err();
		Pallet::<Test>::ensure_admin_or_governance(Origin::signed(2), &1).unwrap_err();
		Pallet::<Test>::ensure_admin_or_governance(Origin::signed(1), &1).unwrap();
		Pallet::<Test>::ensure_admin_or_governance(Origin::none(), &1).unwrap_err();
		Pallet::<Test>::ensure_admin_or_governance(Origin::none(), &2).unwrap_err();
	});
}

#[test]
fn test_transfer() {
	let asset_id = 1;
	new_test_ext().execute_with(|| {
		Pallet::<Test>::transfer(
			Origin::signed(FROM_ACCOUNT),
			asset_id,
			TO_ACCOUNT,
			TRANSFER_AMOUNT,
			true,
		)
		.expect("transfer should work");
		assert_eq!(
			Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT),
			INIT_AMOUNT - TRANSFER_AMOUNT
		);
		assert_eq!(
			Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
	});
}

#[test]
fn test_transfer_native() {
	let asset_id = 1;
	new_test_ext().execute_with(|| {
		Pallet::<Test>::transfer_native(
			Origin::signed(FROM_ACCOUNT),
			TO_ACCOUNT,
			TRANSFER_AMOUNT,
			true,
		)
		.expect("transfer_native should work");
		assert_eq!(
			Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT),
			INIT_AMOUNT - TRANSFER_AMOUNT
		);
		assert_eq!(
			Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
	});
}

#[test]
fn test_force_transfer() {
	let asset_id = 1;
	new_test_ext().execute_with(|| {
		Pallet::<Test>::force_transfer(
			Origin::root(),
			asset_id,
			FROM_ACCOUNT,
			TO_ACCOUNT,
			TRANSFER_AMOUNT,
			true,
		)
		.expect("force_transfer should work");
		assert_eq!(
			Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT),
			INIT_AMOUNT - TRANSFER_AMOUNT
		);
		assert_eq!(
			Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
	});
}

#[test]
fn test_force_transfer_native() {
	let asset_id = 1;
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
			Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT),
			INIT_AMOUNT - TRANSFER_AMOUNT
		);
		assert_eq!(
			Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
	});
}

#[test]
fn test_transfer_all() {
	let asset_id = 1;
	new_test_ext().execute_with(|| {
		Pallet::<Test>::transfer_all(Origin::signed(FROM_ACCOUNT), asset_id, TO_ACCOUNT, true)
			.expect("transfer_all should work");
		assert_eq!(Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT), 1);
		assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), INIT_AMOUNT * 2 - 1);
	});
}

#[test]
fn test_transfer_all_native() {
	let asset_id = 1;
	new_test_ext().execute_with(|| {
		Pallet::<Test>::transfer_all_native(Origin::signed(FROM_ACCOUNT), TO_ACCOUNT, true)
			.expect("transfer_all_native should work");
		assert_eq!(Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT), 1);
		assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), INIT_AMOUNT * 2 - 1);
	});
}

#[test]
fn test_mint_initialize() {
	let prototcol_id = *b"unittest";
	let nonce = 0;
	let asset_id = create_asset_id(prototcol_id, nonce);
	new_test_ext().execute_with(|| {
		assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), INIT_AMOUNT);
		Pallet::<Test>::mint_initialize(
			Origin::root(),
			prototcol_id,
			nonce,
			b"test_asset".to_vec(),
			b"TASS".to_vec(),
			12,
			None,
			TRANSFER_AMOUNT,
			TO_ACCOUNT,
		)
		.expect("mint_initialize should work");
		assert_eq!(
			Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
	});
}

#[test]
fn test_mint_initialize_with_governance() {
	let prototcol_id = *b"unittest";
	let nonce = 0;
	let asset_id = create_asset_id(prototcol_id, nonce);
	new_test_ext().execute_with(|| {
		assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), INIT_AMOUNT);
		Pallet::<Test>::mint_initialize_with_governance(
			Origin::root(),
			prototcol_id,
			nonce,
			b"test_asset".to_vec(),
			b"TASS".to_vec(),
			12,
			None,
			TRANSFER_AMOUNT,
			TO_ACCOUNT,
			TO_ACCOUNT,
		)
		.expect("mint_initialize_with_governance should work");
		assert_eq!(
			Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
		Pallet::<Test>::ensure_admin_or_governance(Origin::signed(TO_ACCOUNT), &asset_id).expect(
			"mint_initialize_with_governance should add governance_origin to GovernanceRegistry",
		);
	});
}

#[test]
fn test_mint_into() {
	let asset_id = 1;
	new_test_ext().execute_with(|| {
		GovernanceRegistry::set(Origin::root(), asset_id, FROM_ACCOUNT).unwrap();
		assert_eq!(Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT), INIT_AMOUNT);
		assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), INIT_AMOUNT);

		Pallet::<Test>::mint_into(
			Origin::signed(FROM_ACCOUNT),
			asset_id,
			TO_ACCOUNT,
			TRANSFER_AMOUNT,
		)
		.expect("mint_into should work");
		assert_eq!(Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT), INIT_AMOUNT);
		assert_eq!(
			Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT),
			INIT_AMOUNT + TRANSFER_AMOUNT
		);
	});
}

#[test]
fn test_burn_from() {
	let asset_id = 1;
	new_test_ext().execute_with(|| {
		GovernanceRegistry::set(Origin::root(), asset_id, FROM_ACCOUNT).unwrap();
		assert_eq!(Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT), INIT_AMOUNT);
		assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), INIT_AMOUNT);

		Pallet::<Test>::burn_from(
			Origin::signed(FROM_ACCOUNT),
			asset_id,
			TO_ACCOUNT,
			TRANSFER_AMOUNT,
		)
		.expect("burn_from should work");
		assert_eq!(Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT), INIT_AMOUNT);
		assert_eq!(
			Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT),
			INIT_AMOUNT - TRANSFER_AMOUNT
		);
	});
}
