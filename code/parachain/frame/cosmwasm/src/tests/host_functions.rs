#![allow(clippy::disallowed_methods)]

use crate::{
	mock::*,
	runtimes::{
		abstraction::Gas,
		vm::{CosmwasmVMCache, CosmwasmVMShared},
	},
	setup_instantiate_call,
	weights::WeightInfo,
	Config,
};
use cosmwasm_vm::vm::VMBase;
use cosmwasm_vm_wasmi::code_gen;
use frame_benchmarking::account;
use frame_support::traits::fungible;
use sp_runtime::AccountId32;

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
		let gas = vm.0.data().shared.gas.remaining();
		// 1
		assert_eq!(vm.db_read(key).unwrap(), Some(value.clone()));
		let charged_gas = gas - vm.0.data().shared.gas.remaining();
		// 2
		assert_eq!(
			charged_gas,
			<Test as Config>::ContractStorageByteReadPrice::get() as u64 * value.len() as u64 +
				<Test as Config>::WeightInfo::db_read().ref_time()
		);
		// 3
		assert_eq!(vm.db_read(b"garbage".to_vec()).unwrap(), None);
		let charged_gas = gas - charged_gas - vm.0.data().shared.gas.remaining();
		// 4
		assert_eq!(charged_gas, <Test as Config>::WeightInfo::db_read().ref_time())
	})
}

/// 1. Writes a value to db.
/// 2. Charges `weight(db_write) + (len(value) * ContractStorageByteWritePrice)`.
/// 3. Write on an existing key overwrites the value.
/// 4. Charges `weight(db_write) + (len(old_value) - len(value)) * ContractStorageByteWritePrice`
/// 5. Write when the VM is read-only fails.
#[test]
fn db_write() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract = create_instantiated_contract(&mut shared_vm, origin.clone());

		let key = b"Hello".to_vec();
		let value = b"World!".to_vec();
		let moon = b"Moon!".to_vec();

		let mut vm = Cosmwasm::cosmwasm_new_vm(&mut shared_vm, origin, contract, vec![]).unwrap();
		let mut remaining_gas = vm.0.data().shared.gas.remaining();
		vm.db_write(key.clone(), value.clone()).unwrap();
		let charged_gas = remaining_gas - vm.0.data().shared.gas.remaining();
		// 1
		assert_eq!(vm.db_read(key.clone()).unwrap(), Some(value.clone()));
		remaining_gas = vm.0.data().shared.gas.remaining();
		// 2
		assert_eq!(
			charged_gas,
			<Test as Config>::ContractStorageByteWritePrice::get() as u64 * value.len() as u64 +
				<Test as Config>::WeightInfo::db_write().ref_time()
		);
		vm.db_write(key.clone(), moon.clone()).unwrap();
		// 3
		assert_eq!(
			remaining_gas - vm.0.data().shared.gas.remaining(),
			<Test as Config>::ContractStorageByteWritePrice::get() as u64 * value.len() as u64 -
				<Test as Config>::ContractStorageByteWritePrice::get() as u64 * moon.len() as u64 +
				<Test as Config>::WeightInfo::db_write().ref_time()
		);
		// 4
		assert_eq!(vm.db_read(key.clone()).unwrap(), Some(moon));
		vm.0.data_mut().shared.push_readonly();
		// 5
		assert!(vm.db_write(key, value).is_err());
	})
}
