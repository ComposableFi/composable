use crate::*;
use composable_traits::assets::AssetInfo;
use frame_support::{assert_noop, assert_ok};
use mocks::{new_test_ext, Balance, GovernanceRegistry, RuntimeOrigin, Test};
use orml_traits::MultiCurrency;
use sp_runtime::DispatchError;

const FROM_ACCOUNT: u128 = 1;
const TO_ACCOUNT: u128 = 2;
const INIT_AMOUNT: Balance = 1000;
const TRANSFER_AMOUNT: Balance = 500;

fn create_asset_id(protocol_id: [u8; 8], nonce: u64) -> u128 {
	let bytes = protocol_id
		.into_iter()
		.chain(nonce.to_be_bytes())
		.collect::<Vec<u8>>()
		.try_into()
		.expect("[u8; 8] + bytes(u64) = [u8; 16]");
	u128::from_be_bytes(bytes)
}

mod ensure_admin_or_governance {
	use super::*;

	#[test]
	fn should_only_be_set_by_root() {
		new_test_ext().execute_with(|| {
			assert_ok!(GovernanceRegistry::set(RuntimeOrigin::root(), 1, 1));
			assert_ok!(Pallet::<Test>::ensure_admin_or_governance(RuntimeOrigin::root(), &2));
			assert_noop!(
				Pallet::<Test>::ensure_admin_or_governance(RuntimeOrigin::signed(1), &2),
				DispatchError::BadOrigin
			);
			assert_noop!(
				Pallet::<Test>::ensure_admin_or_governance(RuntimeOrigin::signed(2), &1),
				DispatchError::BadOrigin
			);
			assert_ok!(Pallet::<Test>::ensure_admin_or_governance(RuntimeOrigin::signed(1), &1));
			assert_noop!(
				Pallet::<Test>::ensure_admin_or_governance(RuntimeOrigin::none(), &1),
				DispatchError::BadOrigin
			);
			assert_noop!(
				Pallet::<Test>::ensure_admin_or_governance(RuntimeOrigin::none(), &2),
				DispatchError::BadOrigin
			);
		});
	}
}

mod transfer {
	use super::*;

	#[test]
	fn should_transfer_given_amount() {
		let asset_id = 1;
		new_test_ext().execute_with(|| {
			assert_ok!(Pallet::<Test>::deposit(asset_id, &FROM_ACCOUNT, INIT_AMOUNT));
			assert_ok!(Pallet::<Test>::transfer(
				RuntimeOrigin::signed(FROM_ACCOUNT),
				asset_id,
				TO_ACCOUNT,
				TRANSFER_AMOUNT,
				true,
			));
			assert_eq!(
				Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT),
				INIT_AMOUNT - TRANSFER_AMOUNT
			);
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), TRANSFER_AMOUNT);
		});
	}
}

mod transfer_native {
	use super::*;

	#[test]
	fn should_transfer_given_amount() {
		let asset_id = 1;
		new_test_ext().execute_with(|| {
			assert_ok!(Pallet::<Test>::deposit(asset_id, &FROM_ACCOUNT, INIT_AMOUNT));
			assert_ok!(Pallet::<Test>::transfer_native(
				RuntimeOrigin::signed(FROM_ACCOUNT),
				TO_ACCOUNT,
				TRANSFER_AMOUNT,
				true,
			));
			assert_eq!(
				Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT),
				INIT_AMOUNT - TRANSFER_AMOUNT
			);
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), TRANSFER_AMOUNT);
		});
	}
}

mod force_transfer {
	use super::*;

	#[test]
	fn should_transfer_given_amount() {
		let asset_id = 1;
		new_test_ext().execute_with(|| {
			assert_ok!(Pallet::<Test>::deposit(asset_id, &FROM_ACCOUNT, INIT_AMOUNT));
			assert_ok!(Pallet::<Test>::force_transfer(
				RuntimeOrigin::root(),
				asset_id,
				FROM_ACCOUNT,
				TO_ACCOUNT,
				TRANSFER_AMOUNT,
				true,
			));
			assert_eq!(
				Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT),
				INIT_AMOUNT - TRANSFER_AMOUNT
			);
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), TRANSFER_AMOUNT);
		});
	}
}

mod force_transfer_native {
	use super::*;

	#[test]
	fn should_transfer_given_amount() {
		let asset_id = 1;
		new_test_ext().execute_with(|| {
			assert_ok!(Pallet::<Test>::deposit(asset_id, &FROM_ACCOUNT, INIT_AMOUNT));
			assert_ok!(Pallet::<Test>::force_transfer_native(
				RuntimeOrigin::root(),
				FROM_ACCOUNT,
				TO_ACCOUNT,
				TRANSFER_AMOUNT,
				true,
			));
			assert_eq!(
				Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT),
				INIT_AMOUNT - TRANSFER_AMOUNT
			);
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), TRANSFER_AMOUNT);
		});
	}
}

mod transfer_all {
	use super::*;

	#[test]
	fn should_transfer_entire_balance() {
		let asset_id = 1;
		new_test_ext().execute_with(|| {
			assert_ok!(Pallet::<Test>::deposit(asset_id, &FROM_ACCOUNT, INIT_AMOUNT));
			assert_ok!(Pallet::<Test>::transfer_all(
				RuntimeOrigin::signed(FROM_ACCOUNT),
				asset_id,
				TO_ACCOUNT,
				true,
			));
			// NOTE: Balance of 1 maintained by `FROM_ACCOUNT` for ED
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT), 1);
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), INIT_AMOUNT - 1);
		});
	}
}

mod transfer_all_native {
	use super::*;

	#[test]
	fn should_transfer_entire_balance() {
		let asset_id = 1;
		new_test_ext().execute_with(|| {
			assert_ok!(Pallet::<Test>::deposit(asset_id, &FROM_ACCOUNT, INIT_AMOUNT));
			assert_ok!(Pallet::<Test>::transfer_all_native(
				RuntimeOrigin::signed(FROM_ACCOUNT),
				TO_ACCOUNT,
				true
			));
			// NOTE: Balance of 1 maintained by `FROM_ACCOUNT` for ED
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT), 1);
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), INIT_AMOUNT - 1);
		});
	}
}

mod mint_into {
	use super::*;
	use composable_traits::assets::CreateAsset;

	#[test]
	fn should_mint_into_single_account() {
		let asset_id = 1;
		new_test_ext().execute_with(|| {
			assert_ok!(GovernanceRegistry::set(RuntimeOrigin::root(), asset_id, FROM_ACCOUNT));
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT), 0);
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), 0);

			assert_ok!(Pallet::<Test>::mint_into(
				RuntimeOrigin::signed(FROM_ACCOUNT),
				asset_id,
				TO_ACCOUNT,
				TRANSFER_AMOUNT,
			));
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT), 0);
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), TRANSFER_AMOUNT);
		});
	}

	#[test]
	fn should_create_local_asset_and_mint() {
		let protocol_id = *b"unittest";
		let nonce = 1;
		let asset_id = create_asset_id(protocol_id, nonce);
		let asset_info = AssetInfo {
			name: None,
			symbol: None,
			decimals: Some(12),
			ratio: None,
			existential_deposit: 0,
		};
		new_test_ext().execute_with(|| {
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), 0);

			let asset_id_new =
				<Pallet<Test> as CreateAsset>::create_local_asset(protocol_id, nonce, asset_info)
					.expect("Failed to create local asset");
			assert_eq!(asset_id, asset_id_new);
			assert_ok!(Pallet::<Test>::mint_into(
				RuntimeOrigin::root(),
				asset_id,
				TO_ACCOUNT,
				TRANSFER_AMOUNT,
			));
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), TRANSFER_AMOUNT);
		});
	}
}

mod burn_from {
	use super::*;

	#[test]
	fn should_burn_from_single_account() {
		let asset_id = 1;
		new_test_ext().execute_with(|| {
			assert_ok!(Pallet::<Test>::deposit(asset_id, &FROM_ACCOUNT, INIT_AMOUNT));
			assert_ok!(Pallet::<Test>::deposit(asset_id, &TO_ACCOUNT, INIT_AMOUNT));
			assert_ok!(GovernanceRegistry::set(RuntimeOrigin::root(), asset_id, FROM_ACCOUNT));
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT), INIT_AMOUNT);
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT), INIT_AMOUNT);

			assert_ok!(Pallet::<Test>::burn_from(
				RuntimeOrigin::signed(FROM_ACCOUNT),
				asset_id,
				TO_ACCOUNT,
				TRANSFER_AMOUNT,
			));
			assert_eq!(Pallet::<Test>::total_balance(asset_id, &FROM_ACCOUNT), INIT_AMOUNT);
			assert_eq!(
				Pallet::<Test>::total_balance(asset_id, &TO_ACCOUNT),
				INIT_AMOUNT - TRANSFER_AMOUNT
			);
		});
	}
}
