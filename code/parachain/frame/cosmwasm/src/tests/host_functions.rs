#![allow(clippy::disallowed_methods)]

use super::helpers::*;
use crate::{
	mock::*, runtimes::abstraction::CosmwasmAccount, weights::WeightInfo, CodeHashToId,
	CodeIdToInfo, CodeInfoOf, Config, InstrumentedCode, PristineCode,
};
use alloc::collections::BTreeSet;
use cosmwasm_vm::{
	cosmwasm_std::{CodeInfoResponse, Coin, ContractInfoResponse, Order},
	system::CosmwasmContractMeta,
	vm::VMBase,
};
use cosmwasm_vm_wasmi::code_gen;
use frame_benchmarking::account;
use sp_runtime::AccountId32;

#[test]
fn db_read() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract = create_instantiated_contract(&mut shared_vm, origin.clone());

		let key = b"Hello".to_vec();
		let value = b"Value!".to_vec();

		let mut vm = Cosmwasm::cosmwasm_new_vm(&mut shared_vm, origin, contract, vec![]).unwrap();
		vm.db_write(key.clone(), value.clone()).unwrap();
		let mut gas = current_gas(&mut vm);

		// 1. Existing value is read.
		assert_eq!(vm.db_read(key).unwrap(), Some(value.clone()));

		// 3. Charges `weight(db_read) + (len(bytes) * ContractStorageByteReadPrice)`.
		assert_eq!(
			charged_gas(&mut vm, gas),
			<Test as Config>::ContractStorageByteReadPrice::get() as u64 * value.len() as u64 +
				<Test as Config>::WeightInfo::db_read().ref_time()
		);

		// 3. Read for a non-existing value returns `None`.
		gas = current_gas(&mut vm);
		assert_eq!(vm.db_read(b"garbage".to_vec()).unwrap(), None);

		// 4. If `db_read` returns `None`, charges `weight(db_read)`.
		assert_eq!(charged_gas(&mut vm, gas), <Test as Config>::WeightInfo::db_read().ref_time())
	})
}

#[test]
fn db_write() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract = create_instantiated_contract(&mut shared_vm, origin.clone());

		let key = b"Hello".to_vec();
		let value = b"World!".to_vec();
		let moon = b"Cool Moon!".to_vec();
		let sun = b"Sun!".to_vec();

		let mut vm = Cosmwasm::cosmwasm_new_vm(&mut shared_vm, origin, contract, vec![]).unwrap();

		// 1. Charges `weight(db_write) + (len(value) * ContractStorageByteWritePrice)`.
		let mut gas = current_gas(&mut vm);
		vm.db_write(key.clone(), value.clone()).unwrap();
		assert_eq!(
			charged_gas(&mut vm, gas),
			<Test as Config>::ContractStorageByteWritePrice::get() as u64 * value.len() as u64 +
				<Test as Config>::WeightInfo::db_write().ref_time()
		);
		// 2. Writes a value to db.
		assert_eq!(vm.db_read(key.clone()).unwrap(), Some(value.clone()));

		gas = current_gas(&mut vm);
		vm.db_write(key.clone(), moon.clone()).unwrap();

		// 3. Charges `weight(db_write) + (len(old_value) - len(new_value)) *
		// ContractStorageByteWritePrice`, if `len(old_value) < len(new_value)`
		assert_eq!(
			charged_gas(&mut vm, gas),
			<Test as Config>::ContractStorageByteWritePrice::get() as u64 * moon.len() as u64 -
				<Test as Config>::ContractStorageByteWritePrice::get() as u64 *
					value.len() as u64 +
				<Test as Config>::WeightInfo::db_write().ref_time()
		);
		// 4. Writes on an existing key, overwrites the value.
		assert_eq!(vm.db_read(key.clone()).unwrap(), Some(moon));

		// 5. Charges only `weight(db_write)` if the `len(old_value) >= len(new_value)`
		gas = current_gas(&mut vm);
		vm.db_write(key.clone(), sun).unwrap();
		assert_eq!(charged_gas(&mut vm, gas), <Test as Config>::WeightInfo::db_write().ref_time());

		// 6. Fails to write if the VM is read-only.
		vm.0.data_mut().shared.push_readonly();
		assert!(vm.db_write(key, value).is_err());
	})
}

#[test]
fn db_remove() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract = create_instantiated_contract(&mut shared_vm, origin.clone());

		let key = b"Hello".to_vec();
		let value = b"World!".to_vec();

		let mut vm = Cosmwasm::cosmwasm_new_vm(&mut shared_vm, origin, contract, vec![]).unwrap();
		vm.db_write(key.clone(), value).unwrap();

		// 1. Charges gas properly.
		let gas = current_gas(&mut vm);
		vm.db_remove(key.clone()).unwrap();
		assert_eq!(charged_gas(&mut vm, gas), <Test as Config>::WeightInfo::db_remove().ref_time());

		// 2. Removes the key from the database.
		assert_eq!(vm.db_read(key.clone()).unwrap(), None);

		// 3. Non-existent value removal does not fail.
		assert!(vm.db_remove(key.clone()).is_ok());

		// 4. Fails to remove if the VM is read-only.
		vm.0.data_mut().shared.push_readonly();
		assert!(vm.db_remove(key).is_err());
	})
}

#[test]
fn db_scan_next() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract = create_instantiated_contract(&mut shared_vm, origin.clone());

		let mut kv_pairs = BTreeSet::from([
			(b"John".to_vec(), b"Doe".to_vec()),
			(b"Foo".to_vec(), b"Bar".to_vec()),
			(b"Ama".to_vec(), b"Zing".to_vec()),
			(b"Sup".to_vec(), b"Er".to_vec()),
			(b"Co".to_vec(), b"Ol".to_vec()),
		]);

		let mut vm = Cosmwasm::cosmwasm_new_vm(&mut shared_vm, origin, contract, vec![]).unwrap();

		// 1. `db_scan` charges gas properly.
		let gas = current_gas(&mut vm);
		let iterator_id = vm.db_scan(None, None, Order::Ascending).unwrap();
		assert_eq!(charged_gas(&mut vm, gas), <Test as Config>::WeightInfo::db_scan().ref_time());

		// 2. `db_next` charges gas properly.
		let gas = current_gas(&mut vm);
		let next_item = vm.db_next(iterator_id).unwrap();
		assert_eq!(charged_gas(&mut vm, gas), <Test as Config>::WeightInfo::db_next().ref_time());

		// 3. Next item is `(Vec::new(), Vec::new())` if there is no item.
		assert_eq!(next_item, (Vec::new(), Vec::new()));

		for (key, value) in &kv_pairs {
			vm.db_write(key.clone(), value.clone()).unwrap();
		}
		let iterator1 = vm.db_scan(None, None, Order::Ascending).unwrap();
		let iterator2 = vm.db_scan(None, None, Order::Ascending).unwrap();

		for _ in 0..5 {
			assert!(kv_pairs.remove(&vm.db_next(iterator1).unwrap()));
		}

		// 4. Iterator ends with (Vec::new(), Vec::new()).
		assert_eq!(vm.db_next(iterator1).unwrap(), (Vec::new(), Vec::new()));

		// 5. Iterator exhausts the list
		assert!(kv_pairs.is_empty());

		// 6. Iterating an iterator doesn't affect other iterators
		assert_ne!(vm.db_next(iterator2).unwrap(), (Vec::new(), Vec::new()));
	})
}

#[test]
fn addr_canonicalize_humanize_validate() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract = create_instantiated_contract(&mut shared_vm, origin.clone());
		let mut vm = Cosmwasm::cosmwasm_new_vm(&mut shared_vm, origin, contract, vec![]).unwrap();

		let valid_addr = (
			"5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL",
			hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d")
				.unwrap(),
		);
		let invalid_addr = "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL2";

		// 1. Charges gas properly.
		let gas = current_gas(&mut vm);
		let canonical_addr = vm.addr_canonicalize(valid_addr.0).unwrap().unwrap();
		assert_eq!(
			charged_gas(&mut vm, gas),
			<Test as Config>::WeightInfo::addr_canonicalize().ref_time()
		);

		// 2. Canonical address is correct.
		let canonical: &[u8; 32] = canonical_addr.0.as_ref().as_ref();
		assert_eq!(canonical.to_vec(), valid_addr.1);

		// 3. Fails if the address is invalid. (`Ok(Err)`)
		assert!(vm.addr_canonicalize(invalid_addr).unwrap().is_err());

		// 4. `addr_humanize` gives back the original address.
		let gas = current_gas(&mut vm);
		assert_eq!(
			Into::<String>::into(vm.addr_humanize(&canonical_addr).unwrap().unwrap()),
			valid_addr.0.to_string()
		);

		// 5. `addr_humanize` charges gas properly.
		assert_eq!(
			charged_gas(&mut vm, gas),
			<Test as Config>::WeightInfo::addr_humanize().ref_time()
		);

		// 6. `addr_validate` validates valid address.
		let gas = current_gas(&mut vm);
		assert!(vm.addr_validate(valid_addr.0).unwrap().is_ok());

		// 7. `addr_validate` charges gas properly.
		assert_eq!(
			charged_gas(&mut vm, gas),
			<Test as Config>::WeightInfo::addr_validate().ref_time()
		);

		// 8. `addr_validate` fails if the address is invalid. (`Ok(Err)`)
		assert!(vm.addr_validate(invalid_addr).unwrap().is_err());
	})
}

#[test]
fn set_contract_meta() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract_1 =
			CosmwasmAccount::new(create_instantiated_contract(&mut shared_vm, origin.clone()));

		let contract_2 = CosmwasmAccount::new(instantiate_contract(
			&mut shared_vm,
			1,
			origin.clone(),
			b"different-salt",
			None,
			b"label",
		));

		let wasm_module: code_gen::WasmModule =
			code_gen::ModuleDefinition::new(Default::default(), 20, None).unwrap().into();
		// 1. Properly upload the code (so that the necessary storage items are modified)
		Cosmwasm::do_upload(&origin, wasm_module.code.try_into().unwrap()).unwrap();

		let mut vm = Cosmwasm::cosmwasm_new_vm(
			&mut shared_vm,
			origin.clone(),
			contract_1.clone().into_inner(),
			vec![],
		)
		.unwrap();

		let mut contract_meta_1 = vm.contract_meta(contract_1.clone()).unwrap();

		contract_meta_1.code_id = 2;
		contract_meta_1.admin = Some(contract_2.clone());
		contract_meta_1.label = "new-label-1".into();

		// 2. Charges gas properly.
		let gas = current_gas(&mut vm);
		vm.set_contract_meta(contract_1.clone(), contract_meta_1.clone()).unwrap();
		assert_eq!(
			charged_gas(&mut vm, gas),
			<Test as Config>::WeightInfo::set_contract_meta().ref_time()
		);

		// 3. Only refcount is changed since the old code id has still reference,
		// old code should still be present.
		let code_info_1 = CodeIdToInfo::<Test>::get(1).unwrap();
		assert_eq!(code_info_1.refcount, 1);
		let code_info_2 = CodeIdToInfo::<Test>::get(2).unwrap();
		assert_eq!(code_info_2.refcount, 1);

		// 4. Sets the meta fields correctly.
		assert_eq!(vm.contract_meta(contract_1.clone()).unwrap(), contract_meta_1);

		// 5. When `code_id` is not changed, don't touch to refcount
		vm.set_contract_meta(contract_1, contract_meta_1.clone()).unwrap();
		let code_info_2 = CodeIdToInfo::<Test>::get(2).unwrap();
		assert_eq!(code_info_2.refcount, 1);

		let code_1 = PristineCode::<Test>::get(1).unwrap();
		let reserved_balance = <Test as Config>::NativeAsset::reserved_balance(&origin);

		// 6. `code_id 1` is not referenced anymore so it is removed.
		vm.set_contract_meta(contract_2, contract_meta_1).unwrap();
		assert!(!CodeIdToInfo::<Test>::contains_key(1));
		assert!(!PristineCode::<Test>::contains_key(1));
		assert!(!InstrumentedCode::<Test>::contains_key(1));
		assert!(!CodeHashToId::<Test>::contains_key(code_info_1.pristine_code_hash));

		// 7. Reserved balance is unreserved since code 1 is now removed.
		assert_eq!(
			reserved_balance - <Test as Config>::NativeAsset::reserved_balance(&origin),
			code_1.len() as u128 * <Test as Config>::CodeStorageByteDeposit::get() as u128
		);
	})
}

#[test]
fn running_contract_meta() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract_1 = create_instantiated_contract(&mut shared_vm, origin.clone());
		let contract_2 = instantiate_contract(
			&mut shared_vm,
			1,
			contract_1.clone(),
			b"different-salt",
			None,
			b"label-2",
		);

		let mut vm =
			Cosmwasm::cosmwasm_new_vm(&mut shared_vm, origin.clone(), contract_1.clone(), vec![])
				.unwrap();

		let contract_meta_1 = CosmwasmContractMeta {
			code_id: 1,
			admin: Some(CosmwasmAccount::new(origin.clone())),
			label: "label-1".into(),
		};
		let gas = current_gas(&mut vm);

		// 1. Returns the executed contract's data, not the latest one.
		assert_eq!(vm.running_contract_meta().unwrap(), contract_meta_1);

		// 2. Charges gas properly.
		assert_eq!(
			charged_gas(&mut vm, gas),
			<Test as Config>::WeightInfo::contract_meta().ref_time()
		);

		let gas = current_gas(&mut vm);

		// 3. `contract_meta` returns the correct data.
		assert_eq!(
			vm.contract_meta(CosmwasmAccount::new(contract_2)).unwrap(),
			CosmwasmContractMeta { code_id: 1, admin: None, label: "label-2".into() }
		);

		// 4. `contract_meta` charges gas properly.
		assert_eq!(
			charged_gas(&mut vm, gas),
			<Test as Config>::WeightInfo::contract_meta().ref_time()
		);

		// 5. `contract_meta` returns `Err` for non-existing contract.
		assert!(vm.contract_meta(CosmwasmAccount::new(origin)).is_err());
	})
}

#[test]
fn query_contract_info() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract = create_instantiated_contract(&mut shared_vm, origin.clone());
		let mut vm =
			Cosmwasm::cosmwasm_new_vm(&mut shared_vm, origin.clone(), contract.clone(), vec![])
				.unwrap();
		let contract_info = Cosmwasm::contract_info(&contract).unwrap();

		// 1. Charges gas properly.
		let gas = current_gas(&mut vm);
		let contract_info_response =
			vm.query_contract_info(CosmwasmAccount::new(contract)).unwrap();
		assert_eq!(
			charged_gas(&mut vm, gas),
			<Test as Config>::WeightInfo::query_contract_info().ref_time()
		);

		let mut expected_response = ContractInfoResponse::default();
		expected_response.code_id = contract_info.code_id;
		expected_response.creator = CosmwasmAccount::<Test>::new(origin.clone()).into();
		expected_response.admin = Some(CosmwasmAccount::<Test>::new(origin).into());
		// it should be pinned since it is already instantiated
		expected_response.pinned = true;
		expected_response.ibc_port = None;

		// 2. Response is correct.
		assert_eq!(contract_info_response, expected_response);

		// TODO(aeryz): Have a case with ibc capable = true
	})
}

#[test]
fn query_code_info() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract = create_instantiated_contract(&mut shared_vm, origin.clone());
		let mut vm =
			Cosmwasm::cosmwasm_new_vm(&mut shared_vm, origin.clone(), contract, vec![]).unwrap();

		// 1. Charges gas properly;
		let gas = current_gas(&mut vm);
		let code_info_response = vm.query_code_info(1).unwrap();
		assert_eq!(
			charged_gas(&mut vm, gas),
			<Test as Config>::WeightInfo::query_code_info().ref_time(),
		);

		let mut expected_response = CodeInfoResponse::default();
		expected_response.code_id = 1;
		expected_response.creator = CosmwasmAccount::<Test>::new(origin).into();
		let CodeInfoOf::<Test> { pristine_code_hash, .. } = CodeIdToInfo::<Test>::get(1).unwrap();
		expected_response.checksum = pristine_code_hash.as_ref().into();

		// 2. Response is correct.
		assert_eq!(code_info_response, expected_response);
	})
}

#[test]
fn transfer_balance_supply() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract = create_instantiated_contract(&mut shared_vm, origin.clone());
		let mut vm =
			Cosmwasm::cosmwasm_new_vm(&mut shared_vm, origin, contract.clone(), vec![]).unwrap();

		let account_for_supply = account::<AccountId32>("supply", 0, 0xAAAAAAAA);
		let mut coins = create_coins(vec![&contract, &account_for_supply]);

		let initial_balances = [coins[0].amount, coins[1].amount, coins[2].amount];

		coins[0].amount = 929392_u128.into();
		coins[1].amount = 243242_u128.into();
		coins[2].amount = 123242232327_u128.into();

		let destination_account =
			CosmwasmAccount::new(account::<AccountId32>("dest", 0, 0xDEADBEEF));
		let contract = CosmwasmAccount::new(contract);

		// 1. Charges gas properly.
		let gas = current_gas(&mut vm);
		vm.transfer(&destination_account, &coins[0..2]).unwrap();
		assert_eq!(charged_gas(&mut vm, gas), <Test as Config>::WeightInfo::transfer(2).ref_time(),);

		// 2. Charges gas properly
		let gas = current_gas(&mut vm);
		vm.transfer_from(&contract, &destination_account, &coins[2..3]).unwrap();
		assert_eq!(charged_gas(&mut vm, gas), <Test as Config>::WeightInfo::transfer(1).ref_time(),);

		// 3. Balances are deducted from the contract and balance works.
		let gas = current_gas(&mut vm);
		for (index, initial_balance) in initial_balances.iter().enumerate() {
			assert_eq!(
				vm.balance(&contract, coins[index].denom.clone()).unwrap(),
				Coin {
					denom: coins[index].denom.clone(),
					amount: *initial_balance - coins[index].amount
				},
			);
		}

		// 4. Charges gas properly.
		assert_eq!(
			charged_gas(&mut vm, gas),
			initial_balances.len() as u64 * <Test as Config>::WeightInfo::balance().ref_time(),
		);

		// 5. Balances are added to the destination account.
		for coin in &coins[0..initial_balances.len()] {
			assert_eq!(vm.balance(&destination_account, coin.denom.clone()).unwrap(), *coin);
		}

		// 6. Zero balance does not return error but returns zero.
		assert_eq!(
			vm.balance(&destination_account, coins[initial_balances.len() + 1].denom.clone())
				.unwrap(),
			Coin { denom: coins[initial_balances.len() + 1].denom.clone(), amount: 0_u128.into() }
		);

		// 7. Supply is correct.
		assert_eq!(
			vm.supply(coins[1].denom.clone()).unwrap(),
			Coin {
				denom: coins[1].denom.clone(),
				amount: initial_balances[1].checked_mul(2_u128.into()).unwrap()
			}
		);

		// 8. Transfer more than the balance fails.
		coins[0].amount = initial_balances[0].checked_add(10000_u128.into()).unwrap();
		assert!(vm.transfer(&destination_account, &coins[0..1]).is_err())
	})
}
