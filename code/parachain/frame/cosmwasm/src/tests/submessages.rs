#![allow(clippy::disallowed_methods)]

use super::{
	extrinsics::{instantiate_test_cases, migrate_test_cases, update_admin_test_cases},
	helpers::*,
	*,
};
use crate::types::CodeIdentifier;
use cosmwasm_vm::cosmwasm_std::{BankMsg, Response, WasmMsg};
use frame_benchmarking::account;
use sp_runtime::AccountId32;

#[test]
fn submessage_instantiate() {
	new_test_ext().execute_with(|| {
		// initialize();
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let mut coins = create_coins(vec![]);
		create_instantiated_contract(&mut shared_vm, origin.clone());
		coins[0].amount = COMMON_AMOUNT_1.into();
		coins[1].amount = COMMON_AMOUNT_2.into();

		// 1. `Instantiate2` message is handled the same way as `Instantiate`.
		// We are generating a wasm contract that returns the below message
		// when instantiated. This way, we will be able to make sure that
		// `continue_instantiate` is called by the VM and it works.
		let instantiate_msg = WasmMsg::Instantiate2 {
			admin: None,
			code_id: 1,
			msg: b"{}".into(),
			funds: coins,
			label: COMMON_LABEL.into(),
			salt: COMMON_SALT.into(),
		};

		let contract = create_instantiated_contract_with_response(
			&mut shared_vm,
			origin,
			Response::default().add_message(instantiate_msg),
			|contract_addr| {
				create_coins(vec![&contract_addr]);
			},
		)
		.unwrap();

		instantiate_test_cases(contract, None, 1, 2);
	})
}

#[test]
fn submessage_migrate() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract_1 = create_instantiated_contract(&mut shared_vm, origin.clone());

		let migrate_msg = WasmMsg::Migrate {
			contract_addr: contract_1.to_string(),
			new_code_id: 2,
			msg: b"{}".into(),
		};

		// 1. VM does the migrate before calling `continue_migrate`.
		create_instantiated_contract_with_response(
			&mut shared_vm,
			origin.clone(),
			Response::default().add_message(migrate_msg),
			|contract_addr| {
				Cosmwasm::do_set_contract_meta(&contract_1, 1, Some(contract_addr), "label".into())
					.unwrap();
			},
		)
		.unwrap();

		migrate_test_cases(contract_1, 2);

		// 2. Migrate fails if the migrator is not the admin.
		assert!(Cosmwasm::do_instantiate(
			&mut shared_vm,
			origin,
			CodeIdentifier::CodeId(2),
			b"different-salt".to_vec().try_into().unwrap(),
			None,
			b"label".to_vec().try_into().unwrap(),
			Default::default(),
			b"{}".to_vec().try_into().unwrap()
		)
		.is_err());
	})
}

#[test]
fn submessage_update_admin() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract_1 = create_instantiated_contract(&mut shared_vm, origin.clone());

		let update_msg = WasmMsg::UpdateAdmin {
			contract_addr: contract_1.to_string(),
			admin: contract_1.to_string(),
		};

		// 1. VM updates the admin.
		create_instantiated_contract_with_response(
			&mut shared_vm,
			origin,
			Response::default().add_message(update_msg),
			|contract_addr| {
				Cosmwasm::do_set_contract_meta(&contract_1, 1, Some(contract_addr), "label".into())
					.unwrap();
			},
		)
		.unwrap();

		update_admin_test_cases(contract_1.clone(), Some(contract_1));
	})
}

#[test]
fn submessage_clear_admin() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract_1 = create_instantiated_contract(&mut shared_vm, origin.clone());

		let update_msg = WasmMsg::ClearAdmin { contract_addr: contract_1.to_string() };

		// 1. VM updates the admin.
		create_instantiated_contract_with_response(
			&mut shared_vm,
			origin,
			Response::default().add_message(update_msg),
			|contract_addr| {
				Cosmwasm::do_set_contract_meta(&contract_1, 1, Some(contract_addr), "label".into())
					.unwrap();
			},
		)
		.unwrap();

		update_admin_test_cases(contract_1, None);
	})
}

#[test]
fn submessage_bank_transfer() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let destination = account::<AccountId32>("destination", 0, 0xCAFEBABE);

		let mut coins = create_coins(vec![]);

		let coins0_amount = coins[0].amount.u128();
		let coins1_amount = coins[1].amount.u128();

		coins[0].amount = COMMON_AMOUNT_1.into();
		coins[1].amount = COMMON_AMOUNT_2.into();

		let transfer_msg =
			BankMsg::Send { to_address: destination.to_string(), amount: coins[0..2].to_vec() };

		// 1. VM does the transfer.
		let contract = create_instantiated_contract_with_response(
			&mut shared_vm,
			origin,
			Response::default().add_message(transfer_msg),
			|contract_addr| {
				create_coins(vec![&contract_addr]);
			},
		)
		.unwrap();

		// 2. Balances are added to the destination account.
		assert_eq!(
			Cosmwasm::do_balance(&destination, coins[0].denom.clone()).unwrap(),
			coins[0].amount.u128()
		);
		assert_eq!(
			Cosmwasm::do_balance(&destination, coins[1].denom.clone()).unwrap(),
			coins[1].amount.u128()
		);

		// 2. Balances are withdrawn from the contract.
		assert_eq!(
			Cosmwasm::do_balance(&contract, coins[0].denom.clone()).unwrap(),
			coins0_amount - COMMON_AMOUNT_1
		);
		assert_eq!(
			Cosmwasm::do_balance(&contract, coins[1].denom.clone()).unwrap(),
			coins1_amount - COMMON_AMOUNT_2
		);
	})
}
