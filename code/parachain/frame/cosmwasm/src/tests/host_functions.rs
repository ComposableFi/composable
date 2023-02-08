#![allow(clippy::disallowed_methods)]

use crate::{
	mock::*,
	runtimes::{
		abstraction::Gas,
		vm::{CosmwasmVMCache, CosmwasmVMShared},
	},
	setup_instantiate_call,
	types::DefaultCosmwasmVM,
	weights::WeightInfo,
	Config,
};
use alloc::collections::BTreeSet;
use cosmwasm_vm::{cosmwasm_std::Order, vm::VMBase};
use cosmwasm_vm_wasmi::{code_gen, OwnedWasmiVM};
use frame_benchmarking::account;
use frame_support::traits::fungible;
use sp_runtime::AccountId32;

fn initialize() {
	use std::sync::Once;
	static INIT: Once = Once::new();
	INIT.call_once(|| {
		env_logger::init();
	});
}

fn create_vm() -> CosmwasmVMShared {
	CosmwasmVMShared {
		storage_readonly_depth: 0,
		depth: 0,
		gas: Gas::new(64, u64::MAX),
		cache: CosmwasmVMCache { code: Default::default() },
	}
}

fn create_funded_account(key: &'static str) -> AccountId32 {
	let origin = account(key, 0, 0xCAFEBABE);

	<pallet_balances::Pallet<Test> as fungible::Mutate<AccountId32>>::mint_into(
		&origin,
		10_000_000_000_000_u128,
	)
	.unwrap();
	origin
}

fn create_instantiated_contract(vm: &mut CosmwasmVMShared, origin: AccountId32) -> AccountId32 {
	// 1. Generate a wasm code
	let wasm_module: code_gen::WasmModule =
		code_gen::ModuleDefinition::new(Default::default(), 10, None).unwrap().into();
	// 2. Properly upload the code (so that the necessary storage items are modified)
	Cosmwasm::do_upload(&origin, wasm_module.code.try_into().unwrap()).unwrap();

	// 3. Instantiate the contract and get the contract address
	let contract_addr = setup_instantiate_call::<Test>(
		origin.clone(),
		1,
		"salt".as_bytes(),
		Some(origin),
		vec![0x41_u8].try_into().unwrap(),
		"message".as_bytes(),
	)
	.unwrap()
	.top_level_call(vm, Default::default(), b"message".to_vec().try_into().unwrap())
	.unwrap();

	contract_addr
}

fn current_gas(vm: &mut OwnedWasmiVM<DefaultCosmwasmVM<Test>>) -> u64 {
	vm.0.data().shared.gas.remaining()
}

fn charged_gas(vm: &mut OwnedWasmiVM<DefaultCosmwasmVM<Test>>, previous_gas: u64) -> u64 {
	previous_gas - current_gas(vm)
}

/// 1. Existing value is read.
/// 2. Charges `weight(db_read) + (len(bytes) * ContractStorageByteReadPrice)`.
/// 3. Read for a non-existing value returns `None`.
/// 4. If `db_read` returns `None`, charges `weight(db_read)`.
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
		// 1
		assert_eq!(vm.db_read(key).unwrap(), Some(value.clone()));
		// 2
		assert_eq!(
			charged_gas(&mut vm, gas),
			<Test as Config>::ContractStorageByteReadPrice::get() as u64 * value.len() as u64 +
				<Test as Config>::WeightInfo::db_read().ref_time()
		);
		// 3
		gas = current_gas(&mut vm);
		assert_eq!(vm.db_read(b"garbage".to_vec()).unwrap(), None);
		// 4
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
		// assert!(vm.db_remove(key.clone()).is_ok());

		// 4. Fails to remove if the VM is read-only.
		vm.0.data_mut().shared.push_readonly();
		assert!(vm.db_remove(key).is_err());
	})
}

#[test]
fn db_scan_next() {
	new_test_ext().execute_with(|| {
		initialize();
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
		assert_eq!(canonical_addr.0.as_ref().as_ref(), valid_addr.1);

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
		let contract = create_instantiated_contract(&mut shared_vm, origin.clone());
		let mut vm = Cosmwasm::cosmwasm_new_vm(&mut shared_vm, origin, contract, vec![]).unwrap();
	})
}
