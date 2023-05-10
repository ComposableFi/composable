#![allow(clippy::disallowed_methods)]

use crate::{
	mock::*, setup_instantiate_call, CosmwasmVMCache, CosmwasmVMShared, CurrentCodeId,
	DefaultCosmwasmVM, FundsOf, Gas, Pallet as Cosmwasm,
};
use alloc::collections::BTreeMap;
use cosmwasm_vm::cosmwasm_std::{Coin, ContractResult, Empty, Response};
use cosmwasm_vm_wasmi::{code_gen, OwnedWasmiVM};
use frame_benchmarking::account;
use frame_support::traits::{fungible, fungibles::Mutate};
use primitives::currency::CurrencyId;
use sp_runtime::AccountId32;

pub fn current_gas(vm: &mut OwnedWasmiVM<DefaultCosmwasmVM<Test>>) -> u64 {
	vm.0.data().shared.gas.remaining()
}

pub fn charged_gas(vm: &mut OwnedWasmiVM<DefaultCosmwasmVM<Test>>, previous_gas: u64) -> u64 {
	previous_gas - current_gas(vm)
}

pub fn create_vm() -> CosmwasmVMShared {
	CosmwasmVMShared {
		storage_readonly_depth: 0,
		depth: 0,
		gas: Gas::new(64, u64::MAX),
		cache: CosmwasmVMCache { code: Default::default() },
	}
}

pub fn create_coins(accounts: Vec<&AccountId32>) -> Vec<Coin> {
	let mut funds: Vec<Coin> = Vec::new();
	let assets = CurrencyId::list_assets();
	for asset in assets {
		let currency_id = asset.id;
		// We need to fund all accounts first
		for account in &accounts {
			<pallet_assets_transactor_router::Pallet<Test> as Mutate<AccountId32>>::mint_into(
				currency_id.into(),
				account,
				u64::MAX as u128,
			)
			.unwrap();
		}
		funds.push(Cosmwasm::<Test>::native_asset_to_cosmwasm_asset(
			currency_id.into(),
			u64::MAX as u128,
		));
	}
	funds
}

pub fn create_funded_account(key: &'static str) -> AccountId32 {
	let origin = account(key, 0, 0xCAFEBABE);

	<pallet_balances::Pallet<Test> as fungible::Mutate<AccountId32>>::mint_into(
		&origin,
		u64::MAX as u128,
	)
	.unwrap();
	origin
}

pub fn create_funds(accounts: Vec<&AccountId32>) -> FundsOf<Test> {
	let mut funds = BTreeMap::new();
	let assets = CurrencyId::list_assets();
	for asset in assets {
		let currency_id = asset.id;
		let balance = u64::MAX as u128;
		// We need to fund all accounts first
		for account in &accounts {
			<pallet_assets_transactor_router::Pallet<Test> as Mutate<AccountId32>>::mint_into(
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

pub fn create_instantiated_contract_with_response<C: Fn(AccountId32)>(
	vm: &mut CosmwasmVMShared,
	origin: AccountId32,
	response: Response<Empty>,
	callback: C,
) -> Result<AccountId32, ()> {
	// 1. Generate a wasm code
	let wasm_module: code_gen::WasmModule =
		code_gen::ModuleDefinition::with_instantiate_response(ContractResult::Ok(response))
			.unwrap()
			.try_into()
			.unwrap();
	// 2. Properly upload the code (so that the necessary storage items are modified)
	Cosmwasm::<Test>::do_upload(&origin, wasm_module.code.try_into().unwrap()).map_err(|_| ())?;

	// 3. Instantiate the contract and get the contract address
	let call = setup_instantiate_call::<Test>(
		origin.clone(),
		CurrentCodeId::<Test>::get(),
		"salt-1".as_bytes(),
		Some(origin),
		b"label-1".to_vec().try_into().unwrap(),
	)
	.map_err(|_| ())?;
	let contract = call.contract.clone();

	callback(contract.clone());

	call.top_level_call(vm, Default::default(), b"message".to_vec().try_into().unwrap())
		.map_err(|_| ())?;

	Ok(contract)
}

pub fn create_instantiated_contract(vm: &mut CosmwasmVMShared, origin: AccountId32) -> AccountId32 {
	create_instantiated_contract_with_response(vm, origin, Response::default(), |_| {}).unwrap()
}

pub fn instantiate_contract(
	vm: &mut CosmwasmVMShared,
	code_id: u64,
	origin: AccountId32,
	salt: &[u8],
	admin: Option<AccountId32>,
	label: &[u8],
) -> AccountId32 {
	setup_instantiate_call::<Test>(origin, code_id, salt, admin, label.to_vec().try_into().unwrap())
		.unwrap()
		.top_level_call(vm, Default::default(), b"message".to_vec().try_into().unwrap())
		.unwrap()
}
