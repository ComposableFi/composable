use super::*;
use crate::{
	runtimes::{
		abstraction::{CanonicalCosmwasmAccount, CosmwasmAccount},
		wasmi::{CosmwasmVMShared, InitialStorageMutability},
	},
	ContractInfoOf, Pallet as Cosmwasm,
};
use alloc::{collections::BTreeMap, format, string::String, vec, vec::Vec};
use cosmwasm_minimal_std::Coin;
use cosmwasm_vm::system::CosmwasmContractMeta;
use cosmwasm_vm_wasmi::code_gen::{self, WasmModule};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::traits::{fungible, fungibles, fungibles::Mutate, Get};
use frame_system::RawOrigin;
use primitives::currency::CurrencyId;
use sha2::{Digest, Sha256};
use sha3::Keccak256;

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

fn create_funded_account<T: Config + pallet_balances::Config + pallet_assets::Config>(
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
	T: Config + pallet_balances::Config + pallet_assets::Config,
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

fn create_coins<T>(accounts: Vec<&AccountIdOf<T>>, n: u32) -> Vec<Coin>
where
	T: Config + pallet_balances::Config + pallet_assets::Config,
	<T as Config>::Balance: From<u128>,
	<T as Config>::AssetId: From<u128>,
	<T as pallet_balances::Config>::Balance: From<u128>,
	<T as pallet_assets::Config>::Balance: From<u128>,
	<T as pallet_assets::Config>::AssetId: From<u128>,
	<T as pallet_assets::Config>::NativeCurrency: fungible::Mutate<<T as pallet::Config>::AccountIdExtended>
		+ fungible::Inspect<
			<T as pallet::Config>::AccountIdExtended,
			Balance = <T as pallet_assets::Config>::Balance,
		>,
	<T as pallet_assets::Config>::MultiCurrency: fungibles::Mutate<<T as pallet::Config>::AccountIdExtended>
		+ fungibles::Inspect<
			<T as pallet::Config>::AccountIdExtended,
			Balance = <T as pallet_assets::Config>::Balance,
			AssetId = <T as pallet_assets::Config>::AssetId,
		>,
{
	let mut funds: Vec<Coin> = Vec::new();
	let assets = CurrencyId::list_assets();
	for i in 0..n {
		let currency_id = assets[i as usize].id as u128;
		// We need to fund all accounts first
		for account in &accounts {
			<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
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
			T: pallet_balances::Config + pallet_assets::Config<AssetId = CurrencyId>,
			<T as Config>::Balance: From<u128>,
			<T as Config>::AssetId: From<u128>,
			<T as pallet_balances::Config>::Balance: From<u128>,
			<T as pallet_assets::Config>::Balance: From<u128>,
			<T as pallet_assets::Config>::AssetId: From<u128>,
			<T as pallet_assets::Config>::NativeCurrency: fungible::Mutate<<T as pallet::Config>::AccountIdExtended>
				+ fungible::Inspect<
					<T as pallet::Config>::AccountIdExtended,
					Balance = <T as pallet_assets::Config>::Balance,
				>,
			<T as pallet_assets::Config>::MultiCurrency: fungibles::Mutate<<T as pallet::Config>::AccountIdExtended>
				+ fungibles::Inspect<
					<T as pallet::Config>::AccountIdExtended,
					Balance = <T as pallet_assets::Config>::Balance,
					AssetId = <T as pallet_assets::Config>::AssetId,
				>,
	}

	upload {
		let n in 1..T::MaxCodeSize::get() - 10000;
		let origin = create_funded_account::<T>("signer");
		let wasm_module: WasmModule = code_gen::ModuleDefinition::new(n as usize).unwrap().into();
	}: _(RawOrigin::Signed(origin), wasm_module.code.try_into().unwrap())

	instantiate {
		let n in 0..CurrencyId::list_assets().len().try_into().unwrap();
		let origin = create_funded_account::<T>("origin");
		let wasm_module: WasmModule = code_gen::ModuleDefinition::new(10).unwrap().into();
		Cosmwasm::<T>::do_upload(&origin, wasm_module.code.try_into().unwrap()).unwrap();
		let salt = vec![1].try_into().unwrap();
		let label = vec![65].try_into().unwrap();
		let message = vec![b'{', b'}'].try_into().unwrap();
		let mut funds = BTreeMap::new();
		let assets = CurrencyId::list_assets();
		for i in 0..n {
			let currency_id = assets[i as usize].id as u128;
			<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
				currency_id.into(),
				&origin,
				10_000_000_000_000_000_000u128.into(),
			)
			.unwrap();
			funds.insert(currency_id.into(), (1_000_000_000_000_000_000u128.into(), false));
		}
	}: _(RawOrigin::Signed(origin), 1, salt, None, label, funds.try_into().unwrap(), 100_000_000u64, message)

	execute {
		let n in 0..CurrencyId::list_assets().len().try_into().unwrap();
		let origin = create_funded_account::<T>("origin");
		let (_, contract, _info) = create_instantiated_contract::<T>(origin.clone());
		let message = b"{}".to_vec().try_into().unwrap();
		let mut funds = BTreeMap::new();
		let assets = CurrencyId::list_assets();
		for i in 0..n {
			let currency_id = assets[i as usize].id as u128;
			<pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
				currency_id.into(),
				&origin,
				10_000_000_000_000_000_000u128.into(),
			)
			.unwrap();
			funds.insert(currency_id.into(), (1_000_000_000_000_000_000u128.into(), false));
		}
	}: _(RawOrigin::Signed(origin), contract, funds.try_into().unwrap(), 100_000_000u64, message)

	db_read {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info, vec![]).unwrap();
		Cosmwasm::<T>::do_db_read(&mut vm.0, "hello world".as_bytes()).unwrap()
	}

	db_read_other_contract {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info.clone(), vec![]).unwrap();
		Cosmwasm::<T>::do_db_read_other_contract(&mut vm.0, &info.trie_id, "hello world".as_bytes()).unwrap()
	}

	db_write {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info, vec![]).unwrap();
		Cosmwasm::<T>::do_db_write(&mut vm.0, "hello".as_bytes(), "world".as_bytes()).unwrap()
	}

	db_scan {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info, vec![]).unwrap();
		Cosmwasm::<T>::do_db_scan(&mut vm.0).unwrap()
	}

	db_next {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info, vec![]).unwrap();
		let iterator = Cosmwasm::<T>::do_db_scan(&mut vm.0).unwrap();
		Cosmwasm::<T>::do_db_next(&mut vm.0, iterator).unwrap()
	}

	db_remove {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info, vec![]).unwrap();
		Cosmwasm::<T>::do_db_write(&mut vm.0, "hello".as_bytes(), "world".as_bytes()).unwrap();
		Cosmwasm::<T>::do_db_remove(&mut vm.0, "hello".as_bytes())
	}

	balance {
		let sender = create_funded_account::<T>("origin");
	}: {
		Cosmwasm::<T>::do_balance(&sender, String::from("100000")).unwrap()
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
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
		let _vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract.clone(), info, vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_set_contract_meta(&contract, 1, None, "hello world".into()).unwrap()
	}

	running_contract_meta {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info, vec![]).unwrap();
		Cosmwasm::<T>::do_running_contract_meta(&mut vm.0)
	}

	contract_meta {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
		let _vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract.clone(), info, vec![]).unwrap();
	}: {
		Cosmwasm::<T>::do_contract_meta(contract).unwrap()
	}

	addr_validate {
		let account = account::<<T as Config>::AccountIdExtended>("account", 0, 0xCAFEBABE);
		let address = Cosmwasm::<T>::account_to_cosmwasm_addr(account);
	}: {
		Cosmwasm::<T>::do_addr_validate(address).unwrap()
	}

	addr_canonicalize {
		let account = account::<<T as Config>::AccountIdExtended>("account", 0, 0xCAFEBABE);
		let address = Cosmwasm::<T>::account_to_cosmwasm_addr(account);
	}: {
		Cosmwasm::<T>::do_addr_canonicalize(address).unwrap()
	}

	addr_humanize {
		let account = account::<<T as Config>::AccountIdExtended>("account", 0, 0xCAFEBABE);
		let account = CanonicalCosmwasmAccount(CosmwasmAccount::new(account));
	}: {
		Cosmwasm::<T>::do_addr_humanize(&account)
	}

	secp256k1_recover_pubkey {
		let message = "connect all the things";
		let signature_hex = "dada130255a447ecf434a2df9193e6fbba663e4546c35c075cd6eea21d8c7cb1714b9b65a4f7f604ff6aad55fba73f8c36514a512bbbba03709b37069194f8a4";
		// let signer_address = "0x12890D2cce102216644c59daE5baed380d84830c";
		let signature = hex::decode(&signature_hex).unwrap();
		let mut hasher = Keccak256::new();
		hasher.update(format!("\x19Ethereum Signed Message:\n{}", message.len()));
		hasher.update(message);
		let message_hash = hasher.finalize();
	}: {
		Cosmwasm::<T>::do_secp256k1_recover_pubkey(&message_hash[..], &signature, 0).unwrap()
	}

	secp256k1_verify {
		let message = hex::decode(SECP256K1_MESSAGE_HEX).unwrap();
		let message_hash = Sha256::digest(message);
		let signature = hex::decode(SECP256K1_SIGNATURE_HEX).unwrap();
		let public_key = hex::decode(SECP256K1_PUBLIC_KEY_HEX).unwrap();
	}: {
		Cosmwasm::<T>::do_secp256k1_verify(&message_hash, &signature, &public_key)
	}

	ed25519_verify {
		let message = hex::decode(ED25519_MESSAGE_HEX).unwrap();
		let signature = hex::decode(ED25519_SIGNATURE_HEX).unwrap();
		let public_key = hex::decode(ED25519_PUBLIC_KEY_HEX).unwrap();
	}: {
		Cosmwasm::<T>::do_ed25519_verify(&message, &signature, &public_key)
	}

	ed25519_batch_verify {
		let messages: Vec<Vec<u8>> = [ED25519_MESSAGE_HEX, ED25519_MESSAGE2_HEX]
			.iter()
			.map(|m| hex::decode(m).unwrap())
			.collect();
		let signatures: Vec<Vec<u8>> = [ED25519_SIGNATURE_HEX, ED25519_SIGNATURE2_HEX]
			.iter()
			.map(|m| hex::decode(m).unwrap())
			.collect();
		let public_keys: Vec<Vec<u8>> = [ED25519_PUBLIC_KEY_HEX, ED25519_PUBLIC_KEY2_HEX]
			.iter()
			.map(|m| hex::decode(m).unwrap())
			.collect();

	}: {
		let messages: Vec<&[u8]> = messages.iter().map(Vec::as_slice).collect();
		let signatures: Vec<&[u8]> = signatures.iter().map(Vec::as_slice).collect();
		let public_keys: Vec<&[u8]> = public_keys.iter().map(Vec::as_slice).collect();
		Cosmwasm::<T>::do_ed25519_batch_verify(&messages, &signatures, &public_keys)
	}

	continue_instantiate {
		let n in 0..CurrencyId::list_assets().len().try_into().unwrap();
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
		let meta: CosmwasmContractMeta<CosmwasmAccount<T>> = CosmwasmContractMeta { code_id: info.code_id, admin: None, label: String::from("test")};
		let funds = create_coins::<T>(vec![&sender, &contract], n);
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract, info, vec![]).unwrap();
		Cosmwasm::<T>::do_continue_instantiate(&mut vm.0, meta, funds, "{}".as_bytes(), &mut |_event| {}).unwrap()
	}

	continue_execute {
		let n in 0..CurrencyId::list_assets().len().try_into().unwrap();
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
		let funds = create_coins::<T>(vec![&sender, &contract], n);
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract.clone(), info, vec![]).unwrap();
		Cosmwasm::<T>::do_continue_execute(&mut vm.0, contract, funds, "{}".as_bytes(), &mut |_event| {}).unwrap()
	}

	continue_migrate {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract.clone(), info, vec![]).unwrap();
		Cosmwasm::<T>::do_continue_migrate(&mut vm.0, contract, "{}".as_bytes(), &mut |_event| {}).unwrap()
	}

	query_info {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract.clone(), info, vec![]).unwrap();
		Cosmwasm::<T>::do_query_info(&mut vm.0, contract).unwrap()
	}

	query_continuation {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract.clone(), info, vec![]).unwrap();
		Cosmwasm::<T>::do_query_continuation(&mut vm.0, contract, "{}".as_bytes()).unwrap()
	}

	query_raw {
		let sender = create_funded_account::<T>("origin");
		let (mut shared, contract, info) = create_instantiated_contract::<T>(sender.clone());
	}: {
		let mut vm = Cosmwasm::<T>::cosmwasm_new_vm(&mut shared, sender, contract.clone(), info, vec![]).unwrap();
		Cosmwasm::<T>::do_db_write(&mut vm.0, "hello".as_bytes(), "world".as_bytes()).unwrap();
		Cosmwasm::<T>::do_query_raw(&mut vm.0, contract, "hello".as_bytes()).unwrap()
	}
}

impl_benchmark_test_suite!(Cosmwasm, crate::mock::new_test_ext(), crate::mock::Test,);
