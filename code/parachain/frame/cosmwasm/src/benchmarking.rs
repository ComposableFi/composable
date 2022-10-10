use super::*;
use crate::{
	runtimes::wasmi::{CosmwasmVMShared, InitialStorageMutability},
	ContractInfoOf, Pallet as Cosmwasm,
};
use alloc::{string::String, vec};
use cosmwasm_vm_wasmi::code_gen::{self, WasmModule};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::traits::{fungible, Get};
use frame_system::RawOrigin;
use sp_runtime::traits::Convert;

fn create_funded_account<T: Config + pallet_balances::Config>(
	key: &'static str,
) -> <T as Config>::AccountIdExtended
where
	<T as pallet_balances::Config>::Balance: From<u128>,
{
	let origin = account::<<T as Config>::AccountIdExtended>(key, 0, 0xCAFEBABE);
	<pallet_balances::Pallet<T> as fungible::Mutate<T::AccountId>>::mint_into(
		&origin,
		10_000_000_000_000_u128.into(),
	)
	.unwrap();
	origin
}

fn create_instantiated_contract<T>(
	origin: T::AccountId,
) -> (CosmwasmVMShared, T::AccountId, ContractInfoOf<T>)
where
	T: Config + pallet_balances::Config,
	<T as pallet_balances::Config>::Balance: From<u128>,
{
	// 1. Generate a wasm code
	let wasm_module: WasmModule = code_gen::ModuleDefinition::new(10).unwrap().into();
	// 2. Properly upload the code (so that the necessary storage items are modified)
	Cosmwasm::<T>::do_upload(&origin, wasm_module.code.try_into().unwrap()).unwrap();
	// 3. Create the shared vm (inner vm)
	let shared =
		Cosmwasm::<T>::do_create_vm_shared(100_000_000u64, InitialStorageMutability::ReadOnly);
	// 4. Instantiate the contract and get the contract address
	let (contract_addr, contract_info) = Cosmwasm::<T>::do_instantiate_phase1(
		origin.clone(),
		1,
		"salt".as_bytes(),
		None,
		vec![0x41_u8].try_into().unwrap(),
		"message".as_bytes(),
	)
	.unwrap();

	(shared, contract_addr, contract_info)
}

benchmarks! {
	where_clause {
		where
			<T as Config>::Balance: From<u128>,
			<T as pallet_balances::Config>::Balance: From<u128>,
			T::AssetId: From<u128>,
			T: pallet_balances::Config
	}

	upload {
		let n in 1..T::MaxCodeSize::get() - 3000;
		let asset = T::AssetToDenom::convert(alloc::string::String::from("1")).unwrap();
		let origin = create_funded_account::<T>("signer");
		let wasm_module: WasmModule = code_gen::ModuleDefinition::new(n as usize).unwrap().into();
	}: _(RawOrigin::Signed(origin), wasm_module.code.try_into().unwrap())

	do_db_read {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info, vec![]).unwrap();
		Cosmwasm::<T>::do_db_read(&mut vm.0, "hello world".as_bytes()).unwrap()
	}

	do_db_read_other_contract {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info.clone(), vec![]).unwrap();
		Cosmwasm::<T>::do_db_read_other_contract(&mut vm.0, &info.trie_id, "hello world".as_bytes()).unwrap()
	}

	do_db_write {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info, vec![]).unwrap();
		Cosmwasm::<T>::do_db_write(&mut vm.0, "hello".as_bytes(), "world".as_bytes()).unwrap()
	}

	do_db_scan {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info, vec![]).unwrap();
		Cosmwasm::<T>::do_db_scan(&mut vm.0).unwrap()
	}

	do_db_next {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info, vec![]).unwrap();
		let iterator = Cosmwasm::<T>::do_db_scan(&mut vm.0).unwrap();
		Cosmwasm::<T>::do_db_next(&mut vm.0, iterator).unwrap()
	}

	do_db_remove {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info, vec![]).unwrap();
		Cosmwasm::<T>::do_db_write(&mut vm.0, "hello".as_bytes(), "world".as_bytes()).unwrap();
		Cosmwasm::<T>::do_db_remove(&mut vm.0, "hello".as_bytes())
	}

	do_balance {
		let sender = create_funded_account::<T>("origin");
	}: {
		Cosmwasm::<T>::do_balance(&sender, String::from("100000")).unwrap()
	}

	do_transfer {
		let sender = create_funded_account::<T>("from");
		let receiver = account::<<T as Config>::AccountIdExtended>("to", 0, 0xCAFEBABE);
		// TODO: funds
	}: {
		Cosmwasm::<T>::do_transfer(&sender, &receiver, &[], false).unwrap();
	}

	// TODO: do_secp256k1_recover_pubkey
	// TODO: do_secp256k1_verify
	// TODO: do_ed25519_batch_verify
	// TODO: do_ed25519_verify

}

impl_benchmark_test_suite!(Cosmwasm, crate::mock::new_test_ext(), crate::mock::Test,);
