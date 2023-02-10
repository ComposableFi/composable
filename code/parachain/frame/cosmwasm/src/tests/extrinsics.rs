#![allow(clippy::disallowed_methods)]

use super::host_functions::*;
use crate::{
	mock::*,
	types::{CodeInfoOf, ContractCodeOf, ContractLabelOf},
	CodeHashToId, CodeIdToInfo, CodeIdentifier, Config, CosmwasmAccount, CurrentCodeId, FundsOf,
	InstrumentedCode, Pallet as Cosmwasm, PristineCode, INSTRUMENTATION_VERSION,
};
use alloc::collections::BTreeMap;
use cosmwasm_vm::{cosmwasm_std::instantiate2_address, vm::VMBase};
use cosmwasm_vm_wasmi::code_gen;
use frame_support::traits::fungibles::Mutate;
use frame_system::RawOrigin;
use primitives::currency::CurrencyId;
use sha2::{Digest, Sha256};
use sp_runtime::{traits::Convert, AccountId32};

pub fn create_funds(accounts: Vec<&AccountId32>) -> FundsOf<Test> {
	let mut funds = BTreeMap::new();
	let assets = CurrencyId::list_assets();
	for asset in assets {
		let currency_id = asset.id;
		let balance = 10_000_000_000_000_000_000u128;
		// We need to fund all accounts first
		for account in &accounts {
			<pallet_assets::Pallet<Test> as Mutate<AccountId32>>::mint_into(
				currency_id.into(),
				account,
				balance,
			)
			.unwrap();
		}
		funds.insert(currency_id.into(), (balance, false));
	}
	funds.try_into().unwrap()
}

#[test]
fn upload() {
	new_test_ext().execute_with(|| {
		let origin = create_funded_account("origin");

		let wasm_module: code_gen::WasmModule =
			code_gen::ModuleDefinition::new(Default::default(), 10, None).unwrap().into();
		let code: ContractCodeOf<Test> = wasm_module.code.try_into().unwrap();

		Cosmwasm::<Test>::upload(RawOrigin::Signed(origin.clone()).into(), code.clone()).unwrap();

		// 1. Current code id is 1.
		assert_eq!(CurrentCodeId::<Test>::get(), 1);

		// 2. Code is uploaded with the id 1.
		let code_info = CodeIdToInfo::<Test>::get(1).unwrap();

		// 3. Code info is correct.
		assert_eq!(
			CodeInfoOf::<Test> {
				creator: origin.clone(),
				pristine_code_hash: Sha256::digest(&code).to_vec().try_into().unwrap(),
				instrumentation_version: INSTRUMENTATION_VERSION,
				ibc_capable: false,
				refcount: 0
			},
			code_info
		);

		// 4. Code is instrumented.
		let instrumented_code = InstrumentedCode::<Test>::get(1).unwrap();
		let module = Cosmwasm::<Test>::do_load_module(&code).unwrap();
		assert_eq!(Cosmwasm::<Test>::do_instrument_code(module).unwrap(), instrumented_code);

		// 5. Pristine code is inserted.
		assert_eq!(PristineCode::<Test>::get(1).unwrap(), code);

		// 6. Code hash is inserted.
		assert_eq!(CodeHashToId::<Test>::get(code_info.pristine_code_hash).unwrap(), 1);

		// 7. Correct amount is reserved.
		assert_eq!(
			<Test as Config>::NativeAsset::reserved_balance(&origin),
			code.len()
				.saturating_mul(<Test as Config>::CodeStorageByteDeposit::get() as usize) as u128
		);

		// 8. Fails when the same code is uploaded.
		assert!(Cosmwasm::<Test>::upload(RawOrigin::Signed(origin).into(), code.clone()).is_err());
	})
}

#[test]
fn instantiate() {
	new_test_ext().execute_with(|| {
		let origin = create_funded_account("origin");

		let wasm_module: code_gen::WasmModule =
			code_gen::ModuleDefinition::new(Default::default(), 10, None).unwrap().into();
		let code: ContractCodeOf<Test> = wasm_module.code.try_into().unwrap();
		Cosmwasm::<Test>::do_upload(&origin, code).unwrap();

		let max_gas = 1_000_000_000_000_u64;
		let mut funds = create_funds(vec![&origin]);
		funds.get_mut(&1.into()).unwrap().0 = 2328472_u128;
		funds.get_mut(&2.into()).unwrap().0 = 1237242_u128;

		Cosmwasm::<Test>::instantiate(
			RawOrigin::Signed(origin.clone()).into(),
			CodeIdentifier::CodeId(1),
			b"salt".to_vec().try_into().unwrap(),
			Some(origin.clone()),
			b"label".to_vec().try_into().unwrap(),
			funds,
			max_gas,
			b"{}".to_vec().try_into().unwrap(),
		)
		.unwrap();

		let code_info = CodeIdToInfo::<Test>::get(1).unwrap();

		let address = instantiate2_address(
			&code_info.pristine_code_hash,
			&(AsRef::<[u8]>::as_ref(&origin).into()),
			b"salt",
		)
		.unwrap();
		let contract = Cosmwasm::<Test>::canonical_addr_to_account(address.into()).unwrap();

		// 1. Contract address is same as `instantiate2_address`.
		let contract_info = Cosmwasm::<Test>::contract_info(&contract).unwrap();

		let mut shared_vm = create_vm();
		let mut vm = Cosmwasm::<Test>::cosmwasm_new_vm(
			&mut shared_vm,
			origin.clone(),
			contract.clone(),
			vec![],
		)
		.unwrap();

		// 2. Funds are transferred.
		assert_eq!(
			vm.balance(&CosmwasmAccount::new(contract.clone()), AssetToDenom::convert(1.into()))
				.unwrap()
				.amount
				.u128(),
			2328472_u128,
		);
		assert_eq!(
			vm.balance(&CosmwasmAccount::new(contract), AssetToDenom::convert(2.into()))
				.unwrap()
				.amount
				.u128(),
			1237242_u128,
		);

		// 3. Code's refcount is increased.
		assert_eq!(code_info.refcount, 1);

		// 4. Contract info is correct.
		assert_eq!(contract_info.admin, Some(origin.clone()));
		assert_eq!(contract_info.instantiator, origin);
		assert_eq!(contract_info.code_id, 1);
		assert_eq!(
			contract_info.label,
			TryInto::<ContractLabelOf<Test>>::try_into(b"label".to_vec()).unwrap()
		);

		// TODO(aeryz): Improve code_gen to embed cosmwasm code, so that we can assert
		// `instantiate` function is really called.
	})
}

#[test]
fn migrate() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract = create_instantiated_contract(&mut shared_vm, origin.clone());

		let wasm_module: code_gen::WasmModule =
			code_gen::ModuleDefinition::new(Default::default(), 20, None).unwrap().into();
		let code: ContractCodeOf<Test> = wasm_module.code.try_into().unwrap();
		Cosmwasm::<Test>::do_upload(&origin, code).unwrap();

		Cosmwasm::<Test>::migrate(
			RawOrigin::Signed(origin.clone()).into(),
			contract.clone(),
			CodeIdentifier::CodeId(2),
			u64::MAX,
			b"{}".to_vec().try_into().unwrap(),
		)
		.unwrap();

		// 1. Switches to the new code.
		assert_eq!(Cosmwasm::<Test>::contract_info(&contract).unwrap().code_id, 2);

		// 2. Fails if the caller is not the admin of the contract.
		assert!(Cosmwasm::<Test>::migrate(
			RawOrigin::Signed(create_funded_account("random-origin")).into(),
			contract.clone(),
			CodeIdentifier::CodeId(2),
			u64::MAX,
			b"{}".to_vec().try_into().unwrap(),
		)
		.is_err());

		// 3. Also fails if the caller is not the admin and the code id is different.
		// This case is added because of a strange bug which result in this failing when
		// the code id is same but success if the code id is different.
		assert!(Cosmwasm::<Test>::migrate(
			RawOrigin::Signed(create_funded_account("random-origin")).into(),
			contract.clone(),
			CodeIdentifier::CodeId(1),
			u64::MAX,
			b"{}".to_vec().try_into().unwrap(),
		)
		.is_err());

		// 4. Fails if the contract has no admin.
		Cosmwasm::<Test>::do_set_contract_meta(&contract, 2, None, "label".into()).unwrap();
		assert!(Cosmwasm::<Test>::migrate(
			RawOrigin::Signed(origin).into(),
			contract.clone(),
			CodeIdentifier::CodeId(2),
			u64::MAX,
			b"{}".to_vec().try_into().unwrap(),
		)
		.is_err());
	})
}

#[test]
fn update_admin() {
	new_test_ext().execute_with(|| {
		let mut shared_vm = create_vm();
		let origin = create_funded_account("origin");
		let contract = create_instantiated_contract(&mut shared_vm, origin.clone());
		let new_admin = create_funded_account("admin");

		Cosmwasm::<Test>::update_admin(
			RawOrigin::Signed(origin.clone()).into(),
			contract.clone(),
			Some(new_admin.clone()),
			u64::MAX,
		)
		.unwrap();

		// 1. Updates the admin.
		assert_eq!(
			Cosmwasm::<Test>::contract_info(&contract).unwrap().admin,
			Some(new_admin.clone())
		);

		// 2. Fails if the caller is not the admin.
		assert!(Cosmwasm::<Test>::update_admin(
			RawOrigin::Signed(origin).into(),
			contract.clone(),
			Some(new_admin.clone()),
			u64::MAX,
		)
		.is_err());

		// 3. Removes admin.
		Cosmwasm::<Test>::update_admin(
			RawOrigin::Signed(new_admin.clone()).into(),
			contract.clone(),
			None,
			u64::MAX,
		)
		.unwrap();
		assert_eq!(Cosmwasm::<Test>::contract_info(&contract).unwrap().admin, None,);

		// 4. Fails if the contract has no admin.
		assert!(Cosmwasm::<Test>::update_admin(
			RawOrigin::Signed(new_admin.clone()).into(),
			contract,
			Some(new_admin),
			u64::MAX,
		)
		.is_err());
	})
}
