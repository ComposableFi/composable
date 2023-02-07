use super::*;
use crate::{
	instrument::INSTRUCTIONS_MULTIPLIER,
	runtimes::{
		abstraction::{CanonicalCosmwasmAccount, CosmwasmAccount, Gas},
		vm::{CosmwasmVMCache, CosmwasmVMShared},
	},
	types::*,
	Pallet as Cosmwasm,
};
use alloc::{
	borrow::ToOwned, boxed::Box, collections::BTreeMap, format, string::String, vec, vec::Vec,
};
use core::cell::SyncUnsafeCell;
use cosmwasm_vm::{
	cosmwasm_std::{Coin, Reply, SubMsgResult},
	system::CosmwasmContractMeta,
};
use cosmwasm_vm_wasmi::code_gen::{
	self, Function, FunctionBuilder, Table, WasmModule, INDEX_OF_USER_DEFINED_FNS,
};
use entrypoint::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::traits::{fungible, fungibles, fungibles::Mutate, Get};
use frame_system::RawOrigin;
use lazy_static::lazy_static;
use primitives::currency::CurrencyId;
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use sha2::{Digest, Sha256};
use sha3::Keccak256;
use wasm_instrument::parity_wasm::elements::{
	BlockType, BrTableData, Instruction, Instructions, ValueType,
};
use wasmi::{AsContext, AsContextMut};

const SECP256K1_MESSAGE_HEX: &str = "5c868fedb8026979ebd26f1ba07c27eedf4ff6d10443505a96ecaf21ba8c4f0937b3cd23ffdc3dd429d4cd1905fb8dbcceeff1350020e18b58d2ba70887baa3a9b783ad30d3fbf210331cdd7df8d77defa398cdacdfc2e359c7ba4cae46bb74401deb417f8b912a1aa966aeeba9c39c7dd22479ae2b30719dca2f2206c5eb4b7";
const SECP256K1_SIGNATURE_HEX: &str = "207082eb2c3dfa0b454e0906051270ba4074ac93760ba9e7110cd9471475111151eb0dbbc9920e72146fb564f99d039802bf6ef2561446eb126ef364d21ee9c4";
const SECP256K1_PUBLIC_KEY_HEX: &str = "04051c1ee2190ecfb174bfe4f90763f2b4ff7517b70a2aec1876ebcfd644c4633fb03f3cfbd94b1f376e34592d9d41ccaf640bb751b00a1fadeb0c01157769eb73";

const ED25519_MESSAGE_HEX: &str = "af82";
const ED25519_SIGNATURE_HEX: &str = "6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a";
const ED25519_PUBLIC_KEY_HEX: &str =
	"fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025";

const ED25519_MESSAGE2_HEX: &str = "72";
const ED25519_SIGNATURE2_HEX: &str = "92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00";
const ED25519_PUBLIC_KEY2_HEX: &str =
	"3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c";

const BASE_ADDITIONAL_BINARY_SIZE: usize = 10;
const FN_NAME: &str = "raw_fn";
const INSTRUCTIONS_SAMPLE_COUNT: u32 = 50;
// This is the first index where the user defined functions start
const EXTRA_FN_INDEX: u32 = INDEX_OF_USER_DEFINED_FNS;

// Substrate's benchmarks are compiled as follows: The upper part and the lower part are separate
// functions. The upper part is the setup and the lower part is the actual benchmark which is put
// in a closure. Hence, the benchmark cannot reference to an object from the setup. Since the
// closure is defined as `move ||`, it moves when references are used. But in our case, `vm` stores
// a mutable reference to the `shared vm`. Which means it points to a local object. Following
// alternatives are not used: 1. We could implement a trait and it could work regardless of `shared
// vm` being reference or owned. But    this would require a lot of changes and it would be useful
// only to benchmarks. 2. We could use `Rc<RefCell<CosmwasmVMShared>>` but this is an unnecessary
// performance penalty.
//
// We also used `SyncUnsafeCell` to be able to borrow the `shared vm` as  mutable. `Mutex` could
// have been used but again, we don't want to pay for `Mutex` during benchmarking.
lazy_static! {
	static ref SHARED_VM: SyncUnsafeCell<CosmwasmVMShared> = {
		SyncUnsafeCell::new(CosmwasmVMShared {
			storage_readonly_depth: 0,
			depth: 0,
			gas: Gas::new(64, u64::MAX),
			cache: CosmwasmVMCache { code: Default::default() },
		})
	};
	// ed25519_batch_verify functions gets a parameter of type `&[&[u8]]`. Since the closure
	// cannot point to a non-static references data like explained previously, these are done
	// globally.
	static ref ED25519_MESSAGE: Vec<u8> = hex::decode(ED25519_MESSAGE_HEX).unwrap();
	static ref ED25519_MESSAGE2: Vec<u8> = hex::decode(ED25519_MESSAGE2_HEX).unwrap();
	static ref ED25519_SIGNATURE: Vec<u8> = hex::decode(ED25519_SIGNATURE_HEX).unwrap();
	static ref ED25519_SIGNATURE2: Vec<u8> = hex::decode(ED25519_SIGNATURE2_HEX).unwrap();
	static ref ED25519_PUBLIC_KEY: Vec<u8> = hex::decode(ED25519_PUBLIC_KEY_HEX).unwrap();
	static ref ED25519_PUBLIC_KEY2: Vec<u8> = hex::decode(ED25519_PUBLIC_KEY2_HEX).unwrap();
}

/// Get a mutable reference to the shared vm
fn get_shared_vm() -> &'static mut CosmwasmVMShared {
	let mut shared = unsafe { SHARED_VM.get().as_mut().unwrap() };
	shared.gas = Gas::new(64, u64::MAX);
	shared.depth = 0;
	shared.storage_readonly_depth = 0;
	shared.cache = CosmwasmVMCache { code: Default::default() };
	shared
}

/// Create a CosmWasm module with additional custom functions
fn create_wasm_module_with_fns(
	mut functions: Vec<(Function, Option<u32>)>,
	table: Option<Table>,
) -> (wasmi::Store<()>, wasmi::Instance) {
	for (func, repeat) in &mut functions {
		let mut instructions: Vec<Instruction> = match repeat {
			Some(repeat) => (0..*repeat * INSTRUCTIONS_MULTIPLIER)
				.flat_map(|_| func.instructions().elements().to_owned())
				.collect(),
			None => func.instructions().elements().to_owned(),
		};
		instructions.push(Instruction::End);
		*func.definition.code_mut() = Instructions::new(instructions);
	}

	let wasm_module: WasmModule = code_gen::ModuleDefinition::new(
		functions.into_iter().map(|(fns, _)| fns).collect(),
		BASE_ADDITIONAL_BINARY_SIZE,
		table,
	)
	.unwrap()
	.try_into()
	.unwrap();

	let engine = wasmi::Engine::default();
	let module = wasmi::Module::new(&engine, wasm_module.code.as_slice()).unwrap();

	let mut store = wasmi::Store::new(&engine, ());
	let linker = <wasmi::Linker<()>>::new();

	let instance = linker.instantiate(&mut store, &module).unwrap().start(&mut store).unwrap();

	(store, instance)
}

fn create_wasm_module(
	instructions: Vec<Instruction>,
	repeat: u32,
) -> (wasmi::Store<()>, wasmi::Instance) {
	create_wasm_module_fn(Instruction::Nop, |_| instructions.clone(), repeat)
}

/// Create a CosmWasm module with additional instructions
fn create_wasm_module_fn<F>(
	instr: Instruction,
	instr_fn: F,
	repeat: u32,
) -> (wasmi::Store<()>, wasmi::Instance)
where
	F: Fn(Instruction) -> Vec<Instruction>,
{
	let instructions = (0..repeat * INSTRUCTIONS_MULTIPLIER)
		.flat_map(|_| instr_fn(instr.clone()))
		.collect();
	let wasm_module: WasmModule = code_gen::ModuleDefinition::with_instructions(
		FN_NAME,
		instructions,
		BASE_ADDITIONAL_BINARY_SIZE,
		None,
	)
	.unwrap()
	.try_into()
	.unwrap();

	let engine = wasmi::Engine::default();
	let module = wasmi::Module::new(&engine, wasm_module.code.as_slice()).unwrap();

	let mut store = wasmi::Store::new(&engine, ());
	let linker = <wasmi::Linker<()>>::new();

	let instance = linker.instantiate(&mut store, &module).unwrap().start(&mut store).unwrap();

	(store, instance)
}

// Note that the overhead of `get_export` and `call` is not important here
// because we are substracting the time it takes to execute N instruction
// from N-1 instruction. So this will eliminate the function call overheads.
fn wasm_invoke(mut store: wasmi::Store<()>, instance: wasmi::Instance) {
	instance
		.get_export(store.as_context(), FN_NAME)
		.and_then(wasmi::Extern::into_func)
		.unwrap()
		.call(store.as_context_mut(), &[], &mut [])
		.unwrap();
}

trait NumericInstruction {
	fn generate() -> i32 {
		let mut small_rng = SmallRng::seed_from_u64(0xcafebabedeadbeef);
		small_rng.next_u32() as i32 + 1
	}
	fn get() -> Instruction;
}

impl NumericInstruction for f64 {
	fn get() -> Instruction {
		Instruction::F64Const(Self::generate() as u64)
	}
}
impl NumericInstruction for i64 {
	fn get() -> Instruction {
		Instruction::I64Const(Self::generate() as i64)
	}
}
impl NumericInstruction for i32 {
	fn get() -> Instruction {
		Instruction::I32Const(Self::generate())
	}
}

#[inline]
fn create_binary_instruction_set<I: NumericInstruction>(
	unary_instr: Instruction,
) -> Vec<Instruction> {
	vec![I::get(), I::get(), unary_instr, Instruction::Drop]
}

#[inline]
fn create_unary_instruction_set<I: NumericInstruction>(
	binary_instr: Instruction,
) -> Vec<Instruction> {
	vec![I::get(), binary_instr, Instruction::Drop]
}

fn create_funded_account<
	T: Config + pallet_balances::Config + pallet_assets_transactor_router::Config,
>(
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

fn create_instantiated_contract<T>(origin: T::AccountId) -> T::AccountId
where
	T: Config + pallet_balances::Config + pallet_assets_transactor_router::Config,
	<T as pallet_balances::Config>::Balance: From<u128>,
{
	// 1. Generate a wasm code
	let wasm_module: WasmModule =
		code_gen::ModuleDefinition::new(Default::default(), BASE_ADDITIONAL_BINARY_SIZE, None)
			.unwrap()
			.into();
	// 2. Properly upload the code (so that the necessary storage items are modified)
	Cosmwasm::<T>::do_upload(&origin, wasm_module.code.try_into().unwrap()).unwrap();

	// 3. Instantiate the contract and get the contract address
	let contract_addr = setup_instantiate_call::<T>(
		origin.clone(),
		1,
		"salt".as_bytes(),
		Some(origin),
		vec![0x41_u8].try_into().unwrap(),
		"message".as_bytes(),
	)
	.unwrap()
	.top_level_call(get_shared_vm(), Default::default(), b"message".to_vec().try_into().unwrap())
	.unwrap();

	contract_addr
}

fn create_coins<T>(accounts: Vec<&AccountIdOf<T>>, n: u32) -> Vec<Coin>
where
	T: Config + pallet_balances::Config + pallet_assets_transactor_router::Config,
	<T as Config>::Balance: From<u128>,
	<T as Config>::AssetId: From<u128>,
	<T as pallet_balances::Config>::Balance: From<u128>,
	<T as pallet_assets_transactor_router::Config>::Balance: From<u128>,
	<T as pallet_assets_transactor_router::Config>::AssetId: From<u128>,
	<T as pallet_assets_transactor_router::Config>::NativeTransactor: fungible::Mutate<<T as pallet::Config>::AccountIdExtended>
		+ fungible::Inspect<
			<T as pallet::Config>::AccountIdExtended,
			Balance = <T as pallet_assets_transactor_router::Config>::Balance,
		>,
	<T as pallet_assets_transactor_router::Config>::LocalTransactor: fungibles::Mutate<<T as pallet::Config>::AccountIdExtended>
		+ fungibles::Inspect<
			<T as pallet::Config>::AccountIdExtended,
			Balance = <T as pallet_assets_transactor_router::Config>::Balance,
			AssetId = <T as pallet_assets_transactor_router::Config>::AssetId,
		>,
{
	let mut funds: Vec<Coin> = Vec::new();
	let assets = CurrencyId::list_assets();
	for i in 0..n {
		let currency_id = assets[i as usize].id;
		// We need to fund all accounts first
		for account in &accounts {
			<pallet_assets_transactor_router::Pallet<T> as Mutate<T::AccountId>>::mint_into(
				currency_id.into(),
				account,
				10_000_000_000_000_000_000u128.into(),
			)
			.unwrap();
		}
		funds.push(Cosmwasm::<T>::native_asset_to_cosmwasm_asset(
			currency_id.into(),
			1_000_000_000_000_000_000u128.into(),
		));
	}
	funds
}

benchmarks! {
	where_clause {
		where
			T: pallet_balances::Config + pallet_assets_transactor_router::Config<AssetId = CurrencyId>,
			<T as Config>::Balance: From<u128>,
			<T as Config>::AssetId: From<u128>,
			<T as pallet_balances::Config>::Balance: From<u128>,
			<T as pallet_assets_transactor_router::Config>::Balance: From<u128>,
			<T as pallet_assets_transactor_router::Config>::AssetId: From<u128>,
			<T as pallet_assets_transactor_router::Config>::NativeTransactor: fungible::Mutate<<T as pallet::Config>::AccountIdExtended>
				+ fungible::Inspect<
					<T as pallet::Config>::AccountIdExtended,
					Balance = <T as pallet_assets_transactor_router::Config>::Balance,
				>,
			<T as pallet_assets_transactor_router::Config>::LocalTransactor: fungibles::Mutate<<T as pallet::Config>::AccountIdExtended>
				+ fungibles::Inspect<
					<T as pallet::Config>::AccountIdExtended,
					Balance = <T as pallet_assets_transactor_router::Config>::Balance,
					AssetId = <T as pallet_assets_transactor_router::Config>::AssetId,
				>,
	}

	upload {
		let n in 1..T::MaxCodeSize::get() - 10000;
		let origin = create_funded_account::<T>("signer");
		let wasm_module: WasmModule = code_gen::ModuleDefinition::new(Default::default(), n as usize, None).unwrap().into();
	}: _(RawOrigin::Signed(origin), wasm_module.code.try_into().unwrap())

	instantiate {
		let n in 0..CurrencyId::list_assets().len().try_into().unwrap();
		let origin = create_funded_account::<T>("origin");
		// BASE_ADDITIONAL_BINARY_SIZE + 1 to make a different code so that it doesn't already exist
		// in `PristineCode` and we don't get an error back.
		let wasm_module: WasmModule = code_gen::ModuleDefinition::new(Default::default(), BASE_ADDITIONAL_BINARY_SIZE + 1, None).unwrap().into();
		Cosmwasm::<T>::do_upload(&origin, wasm_module.code.try_into().unwrap()).unwrap();
		let salt: ContractSaltOf<T> = vec![1].try_into().unwrap();
		let label: ContractLabelOf<T> = "label".as_bytes().to_vec().try_into().unwrap();
		let message: ContractMessageOf<T> = "{}".as_bytes().to_vec().try_into().unwrap();
		let mut funds = BTreeMap::new();
		let assets = CurrencyId::list_assets();
		for i in 0..n {
			let currency_id = assets[i as usize].id;
			<pallet_assets_transactor_router::Pallet<T> as Mutate<T::AccountId>>::mint_into(
				currency_id.into(),
				&origin,
				10_000_000_000_000_000_000u128.into(),
			)
			.unwrap();
			funds.insert(currency_id.into(), (1_000_000_000_000_000_000u128.into(), false));
		}
	}: _(RawOrigin::Signed(origin.clone()), CodeIdentifier::CodeId(1), salt.clone(), None, label.clone(), funds.try_into().unwrap(), 1_000_000_000_000u64, message.clone())
	verify {
		// Make sure refcount is increased
		assert_eq!(CodeIdToInfo::<T>::get(1).unwrap().refcount, 1);
		// Make sure contract address is derived correctly
		let code_hash = CodeIdToInfo::<T>::get(1).unwrap().pristine_code_hash;
		let contract_addr =
			Pallet::<T>::derive_contract_address(&origin, &salt, &code_hash, &message).unwrap();
		// Make sure trie_id is derived correctly
		let nonce = CurrentNonce::<T>::get();
		let trie_id = Pallet::<T>::derive_contract_trie_id(&contract_addr, nonce);
		// Make sure contract info is inserted
		let info = Pallet::<T>::contract_info(&contract_addr).unwrap();

		assert_eq!(ContractInfoOf::<T> {
			code_id: 1,
			trie_id,
			instantiator: origin,
			admin: None,
			label
		}, info);
	}

	execute {
		let n in 0..CurrencyId::list_assets().len().try_into().unwrap();
		let origin = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(origin.clone());
		let message = b"{}".to_vec().try_into().unwrap();
		let mut funds = BTreeMap::new();
		let assets = CurrencyId::list_assets();
		for i in 0..n {
			let currency_id = assets[i as usize].id;
			<pallet_assets_transactor_router::Pallet<T> as Mutate<T::AccountId>>::mint_into(
				currency_id.into(),
				&origin,
				10_000_000_000_000_000_000u128.into(),
			)
			.unwrap();
			funds.insert(currency_id.into(), (1_000_000_000_000_000_000u128.into(), false));
		}
	}: _(RawOrigin::Signed(origin), contract, funds.try_into().unwrap(), 1_000_000_000_000u64, message)

	migrate {
		let origin = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(origin.clone());
		{
			// Upload the second contract but do not instantiate it, this will get `code_id = 2`
			let wasm_module: WasmModule = code_gen::ModuleDefinition::new(Default::default(), 12, None).unwrap().into();
			Cosmwasm::<T>::do_upload(&origin, wasm_module.code.try_into().unwrap()).unwrap();
		}
		let message = b"{}".to_vec().try_into().unwrap();
		let assets = CurrencyId::list_assets();
		let CodeInfoOf::<T> {
			pristine_code_hash,
			..
		} = CodeIdToInfo::<T>::get(1).unwrap();
	}: _(RawOrigin::Signed(origin), contract.clone(), CodeIdentifier::CodeId(2), 1_000_000_000_000u64, message)
	verify {
		// Make sure code id doesn't exist
		assert_eq!(CodeIdToInfo::<T>::contains_key(1), false);
		assert_eq!(PristineCode::<T>::contains_key(1), false);
		assert_eq!(InstrumentedCode::<T>::contains_key(1), false);
		assert_eq!(CodeHashToId::<T>::contains_key(pristine_code_hash), false);
		// Make sure contract points to the new code
		assert_eq!(ContractToInfo::<T>::get(&contract).unwrap().code_id, 2);
	}

	update_admin {
		let origin = create_funded_account::<T>("origin");
		let new_admin = account::<<T as Config>::AccountIdExtended>("new_admin", 0, 0xCAFEBABE);
		let contract = create_instantiated_contract::<T>(origin.clone());

	}: _(RawOrigin::Signed(origin), contract.clone(), Some(new_admin.clone()), 100_000_000u64)
	verify {
		// Make sure contract points to the new code
		assert_eq!(ContractToInfo::<T>::get(&contract).unwrap().admin, Some(new_admin));
	}

	db_read {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract, vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_db_read(vm.0.data_mut(), "hello world".as_bytes()).unwrap();
	}

	db_read_other_contract {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
	let info = ContractToInfo::<T>::get(&contract).unwrap();
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract, vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_db_read_other_contract(vm.0.data_mut(), &info.trie_id, "hello world".as_bytes()).unwrap();
	}

	db_write {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract, vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_db_write(vm.0.data_mut(), "hello".as_bytes(), "world".as_bytes()).unwrap();
	}

	db_scan {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract, vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_db_scan(vm.0.data_mut()).unwrap();
	}

	db_next {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract, vec![]).unwrap();
		let iterator = Cosmwasm::<T>::do_db_scan(vm.0.data_mut()).unwrap();
	}: {
		Cosmwasm::<T>::do_db_next(vm.0.data_mut(), iterator).unwrap();
	}

	db_remove {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract, vec![]).unwrap();
		Cosmwasm::<T>::do_db_write(vm.0.data_mut(), "hello".as_bytes(), "world".as_bytes()).unwrap();
	}: {
		Cosmwasm::<T>::do_db_remove(vm.0.data_mut(), "hello".as_bytes());
	}

	balance {
		let sender = create_funded_account::<T>("origin");
	}: {
		Cosmwasm::<T>::do_balance(&sender, String::from("100000")).unwrap();
	}

	transfer {
		let n in 0..CurrencyId::list_assets().len().try_into().unwrap();
		let sender = create_funded_account::<T>("from");
		let receiver = account::<<T as Config>::AccountIdExtended>("to", 0, 0xCAFEBABE);
		let funds: Vec<Coin> = create_coins::<T>(vec![&sender], n);
	}: {
		Cosmwasm::<T>::do_transfer(&sender, &receiver, &funds, false).unwrap();
	}

	set_contract_meta {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let _ = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract.clone(), vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_set_contract_meta(&contract, 1, None, "hello world".into()).unwrap()
	}

	running_contract_meta {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract, vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_running_contract_meta(vm.0.data_mut())
	}

	contract_meta {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let _ = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract.clone(), vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_contract_meta(contract).unwrap();
	}

	addr_validate {
		let account = account::<<T as Config>::AccountIdExtended>("account", 0, 0xCAFEBABE);
		let address = Cosmwasm::<T>::account_to_cosmwasm_addr(account);
	}: {
		Cosmwasm::<T>::do_addr_validate(address).unwrap();
	}

	addr_canonicalize {
		let account = account::<<T as Config>::AccountIdExtended>("account", 0, 0xCAFEBABE);
		let address = Cosmwasm::<T>::account_to_cosmwasm_addr(account);
	}: {
		Cosmwasm::<T>::do_addr_canonicalize(address).unwrap();
	}

	addr_humanize {
		let account = account::<<T as Config>::AccountIdExtended>("account", 0, 0xCAFEBABE);
		let account = CanonicalCosmwasmAccount(CosmwasmAccount::new(account));
	}: {
		Cosmwasm::<T>::do_addr_humanize(&account);
	}

	secp256k1_recover_pubkey {
		let message = "connect all the things";
		let signature_hex = "dada130255a447ecf434a2df9193e6fbba663e4546c35c075cd6eea21d8c7cb1714b9b65a4f7f604ff6aad55fba73f8c36514a512bbbba03709b37069194f8a4";
		let signature = hex::decode(signature_hex).unwrap();
		let mut hasher = Keccak256::new();
		hasher.update(format!("\x19Ethereum Signed Message:\n{}", message.len()));
		hasher.update(message);
		let message_hash = hasher.finalize();
	}: {
		Cosmwasm::<T>::do_secp256k1_recover_pubkey(&message_hash[..], &signature, 0).unwrap();
	}

	secp256k1_verify {
		let message = hex::decode(SECP256K1_MESSAGE_HEX).unwrap();
		let message_hash = Sha256::digest(message);
		let signature = hex::decode(SECP256K1_SIGNATURE_HEX).unwrap();
		let public_key = hex::decode(SECP256K1_PUBLIC_KEY_HEX).unwrap();
	}: {
		Cosmwasm::<T>::do_secp256k1_verify(&message_hash, &signature, &public_key);
	}

	ed25519_verify {
		let message = ED25519_MESSAGE.as_slice();
		let signature = ED25519_SIGNATURE.as_slice();
		let public_key = ED25519_PUBLIC_KEY.as_slice();
	}: {
		Cosmwasm::<T>::do_ed25519_verify(message, signature, public_key)
	}

	ed25519_batch_verify {
		let messages = vec![ED25519_MESSAGE.as_slice(), ED25519_MESSAGE2.as_slice()];
		let signatures = vec![ED25519_SIGNATURE.as_slice(), ED25519_SIGNATURE2.as_slice()];
		let public_keys = vec![ED25519_PUBLIC_KEY.as_slice(), ED25519_PUBLIC_KEY2.as_slice()];
	}: {
		Cosmwasm::<T>::do_ed25519_batch_verify(&messages, &signatures, &public_keys);
	}

	continue_instantiate {
		let n in 0..CurrencyId::list_assets().len().try_into().unwrap();
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
	let info = ContractToInfo::<T>::get(&contract).unwrap();
		let meta: CosmwasmContractMeta<CosmwasmAccount<T>> = CosmwasmContractMeta { code_id: info.code_id, admin: None, label: String::from("test")};
		let funds = create_coins::<T>(vec![&sender, &contract], n);
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract, vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_continue_instantiate(vm.0.data_mut(), meta, funds, b"salt", "{}".as_bytes(), &mut |_event| {}).unwrap();
	}

	continue_execute {
		let n in 0..CurrencyId::list_assets().len().try_into().unwrap();
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let funds = create_coins::<T>(vec![&sender, &contract], n);
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract.clone(), vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_continue_execute(vm.0.data_mut(), contract, funds, "{}".as_bytes(), &mut |_event| {}).unwrap();
	}

	continue_migrate {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract.clone(), vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_continue_migrate(vm.0.data_mut(), contract, "{}".as_bytes(), &mut |_event| {}).unwrap();
	}

	continue_query {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract.clone(), vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_continue_query(vm.0.data_mut(), contract, "{}".as_bytes()).unwrap();
	}

	continue_reply {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract, vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_continue_reply(vm.0.data_mut(), Reply { id: 0, result: SubMsgResult::Err(String::new())}, &mut |_| {}).unwrap();
	}

	query_contract_info {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract.clone(), vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_query_contract_info(vm.0.data_mut(), contract).unwrap();
	}

	query_code_info {
		let sender = create_funded_account::<T>("origin");
		let _ = create_instantiated_contract::<T>(sender);
	}: {
		Cosmwasm::<T>::do_query_code_info(1).unwrap();
	}

	query_raw {
		let sender = create_funded_account::<T>("origin");
		let contract = create_instantiated_contract::<T>(sender.clone());
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(get_shared_vm(), sender, contract.clone(), vec![]).unwrap();
		Cosmwasm::<T>::do_db_write(vm.0.data_mut(), "hello".as_bytes(), "world".as_bytes()).unwrap();
	}: {
		Cosmwasm::<T>::do_query_raw(vm.0.data_mut(), contract, "hello".as_bytes()).unwrap();
	}

	// For `I64Const` and `Drop`. This will be also used to calculate the cost of an empty function call and additional
	// instructions.
	instruction_I64Const {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance)= create_wasm_module(vec![Instruction::I64Const(99), Instruction::Drop], r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Const {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![Instruction::F64Const(99), Instruction::Drop], r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Load {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![Instruction::I32Const(0), Instruction::I64Load(0, 0), Instruction::Drop], r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Load {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![Instruction::I32Const(0), Instruction::F64Load(0, 0), Instruction::Drop], r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Store {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![Instruction::I32Const(0), Instruction::I64Const(99), Instruction::I64Store(0, 0)], r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Store {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![Instruction::I32Const(0), Instruction::F64Const(99), Instruction::F64Store(0, 0)], r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Eq {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Eq, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Eqz {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Eqz, create_unary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Ne {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Ne, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64LtS {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64LtS, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64GtS {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64GtS, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64LeS {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64LeS, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64GeS {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64GeS, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Clz {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Clz, create_unary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Ctz {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Ctz, create_unary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Popcnt {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Popcnt, create_unary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Add {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Add, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Sub {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Sub, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Mul {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Mul, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64DivS {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64DivS, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64DivU {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64DivU, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64RemS {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64RemS, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64And {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64And, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Or {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Or, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Xor {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Xor, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Shl {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Shl, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64ShrS {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64ShrS, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Rotl {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Rotl, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64Rotr {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64Rotr, create_binary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I64ExtendSI32 {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I64ExtendSI32, create_unary_instruction_set::<i32>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_I32WrapI64 {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::I32WrapI64, create_unary_instruction_set::<i64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Eq {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Eq, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Ne {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Ne, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Lt {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Lt, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Gt {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Gt, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Le {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Le, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Ge {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Ge, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Abs {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Abs, create_unary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Neg {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Neg, create_unary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Ceil {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Ceil, create_unary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Floor {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Floor, create_unary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Trunc {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Trunc, create_unary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Nearest {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Nearest, create_unary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Sqrt {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Sqrt, create_unary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Add {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Add, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Sub {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Sub, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Mul {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Mul, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Div {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Div, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Min {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Min, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Max {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Max, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	instruction_F64Copysign {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_fn(Instruction::F64Copysign, create_binary_instruction_set::<f64>, r);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 4
	instruction_Select {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![
			Instruction::I32Const(99),
			Instruction::I32Const(55),
			Instruction::I32Const(0),
			Instruction::Select,
			Instruction::Drop
		], r);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 2
	instruction_If {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![
			Instruction::I32Const(99),
			Instruction::If(BlockType::NoResult),
			Instruction::End,
		], r);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 0 (`if` instruction will be subtracted from this)
	instruction_Else {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![
			Instruction::I32Const(99),
			Instruction::If(BlockType::NoResult),
			Instruction::Else,
			Instruction::End,
		], r);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 1
	instruction_GetLocal {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_with_fns(vec![(
			FunctionBuilder::new(FN_NAME)
				.local(1, ValueType::I32)
				.instructions(vec![Instruction::GetLocal(0), Instruction::Drop])
				.build(),
			Some(r))], None);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 1
	instruction_SetLocal {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_with_fns(vec![(
			FunctionBuilder::new(FN_NAME)
				.local(1, ValueType::I32)
				.instructions(vec![Instruction::I32Const(99), Instruction::SetLocal(0)])
				.build(),
			Some(r))], None);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 2
	instruction_TeeLocal {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_with_fns(vec![(
			FunctionBuilder::new(FN_NAME)
				.local(1, ValueType::I32)
				.instructions(vec![Instruction::I32Const(99), Instruction::TeeLocal(0), Instruction::Drop])
				.build(),
			None)], None);
	}: {
		wasm_invoke(store, instance);
	}

	// TODO(aeryz): We depend on the existence of the global variable that is used internally by code generator.
	// We could have a field to specify additional globals.
	// n_extra_instrs = 1
	instruction_GetGlobal {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_with_fns(vec![(
			FunctionBuilder::new(FN_NAME)
				.local(1, ValueType::I32)
				.instructions(vec![Instruction::GetGlobal(0), Instruction::Drop])
				.build(),
			None)], None);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 2
	instruction_SetGlobal {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_with_fns(vec![(
			FunctionBuilder::new(FN_NAME)
				.local(1, ValueType::I32)
				.instructions(vec![Instruction::I32Const(99), Instruction::SetGlobal(0)])
				.build(),
			None)], None);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 1
	instruction_CurrentMemory {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![
			Instruction::CurrentMemory(0),
			Instruction::Drop,
		], r);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 2
	instruction_GrowMemory {
		let r in 0..3;
		let (store, instance) = create_wasm_module(vec![
			Instruction::I32Const(1),
			Instruction::GrowMemory(0),
			Instruction::Drop,
		], r);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 0
	instruction_Br {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![
			Instruction::Block(BlockType::NoResult),
			Instruction::Br(0),
			Instruction::End
		], r);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 1
	instruction_BrIf {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![
			Instruction::Block(BlockType::NoResult),
			Instruction::I32Const(1),
			Instruction::Br(0),
			Instruction::End
		], r);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 1
	instruction_BrTable {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![
			Instruction::Block(BlockType::NoResult),
			Instruction::I32Const(0),
			Instruction::BrTable(Box::new(BrTableData {
				table: Box::new([0]),
				default: 0,
			})),
			Instruction::End
		], r);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 1
	instruction_BrTable_per_elem {
		let s in 1..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module(vec![
			Instruction::Block(BlockType::NoResult),
			Instruction::I32Const(0),
			Instruction::BrTable(Box::new(BrTableData {
				table: vec![0; s as usize].into_boxed_slice(),
				default: 0,
			})),
			Instruction::End
		], 1);
	}: {
		wasm_invoke(store, instance);
	}

	// n_extra_instrs = 2
	instruction_Call {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_with_fns(vec![
			(FunctionBuilder::new("garbage")
				.instructions(vec![Instruction::I32Const(99), Instruction::Drop])
				.build(), None),
			(FunctionBuilder::new(FN_NAME)
				.instructions(vec![Instruction::Call(EXTRA_FN_INDEX)])
				.build(),
			Some(r))], None);
	}: {
		wasm_invoke(store, instance);
	}

	// CallIndirect(u8, u32) calls a function through a table. That's why it is indirect.
	//
	// The first parameter is the index of the function signature. Note that this indices
	// are unique so if you have 8 functions and there are only 3 unique function signatures,
	// there will be 3 signature in the list. And the order is first occurrence based. That's
	// why although the function index is 9, we are using 4 for the signature index.
	//
	// The second parameter is reserved and not used right now.
	//
	// CallIndirect pops a I32 value from the stack as the function table index. Then it gets
	// the function at `function_index`. Then it compares the signature at the given index with
	// the given function's signature. It fails if they don't match. Then it does the actual call.
	//
	// n_extra_instrs = 3
	instruction_CallIndirect {
		let r in 0..INSTRUCTIONS_SAMPLE_COUNT;
		let (store, instance) = create_wasm_module_with_fns(vec![
			(FunctionBuilder::new("garbage")
				.instructions(vec![Instruction::I32Const(99), Instruction::Drop])
				.build(), None),
			(FunctionBuilder::new(FN_NAME)
				.instructions(vec![Instruction::I32Const(0), Instruction::CallIndirect(4, 0)])
				.build(), Some(r))
			], Some(Table::fill(EXTRA_FN_INDEX, 1)));
	}: {
		wasm_invoke(store, instance);
	}
}

impl_benchmark_test_suite!(Cosmwasm, crate::mock::new_test_ext(), crate::mock::Test,);
