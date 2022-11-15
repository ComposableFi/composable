//  deliberate eslint disable due to how functions work here
/* eslint-disable no-param-reassign */
import {
	Coin,
	ContractMeta,
	Env,
	Err,
	MessageInfo,
	None,
	Ok,
	Result,
	Some,
	StorageValue,
	VMHost,
	VMStep,
	decode,
	encode,
	toHex,
	unit,
	vmContinueInstantiate,
	vmExecute,
	vmInstantiate,
	vmSetup,
} from 'cw_vm_ts_sdk_001';

import { SetRequired } from 'type-fest';

import { codeStore } from '../code/methods';
import { VMHostShared, VmRunEnv, getVmInitParams } from './types';
import { SetupState } from '../types';

const createVM = (info: MessageInfo, env: Env, metadata: ContractMeta, shared: VMHostShared): Partial<VMHost> => {
	const getStore = (address: string): Map<String, Object> => {
		const value = shared.storage.get(address);
		if (value) return value;
		shared.storage.set(address, new Map<String, Object>());
		return shared.storage.get(address)!;
	};
	const newContractAddress = (): number => ++shared.nextAccountId;
	const setMetadata = (address: string, newMetadata: ContractMeta) => {
		shared.contracts.set(address, newMetadata);
	};
	return {
		info: () => info,
		env: () => env,
		db_write: (key: Array<number>, value: Array<number>) => {
			const store = getStore(env.contract.address);
			store.set(toHex(key), JSON.parse(decode(value)));
			shared.storage.set(env.contract.address, store);
			return Ok(unit);
		},
		db_read: (key: Array<number>) => {
			const store = getStore(env.contract.address);
			const entryKey = toHex(key);
			if (store.has(entryKey)) return Ok(Some<StorageValue>(encode(store.get(entryKey)!)));
			return Ok(None<StorageValue>());
		},
		continue_instantiate: (
			newContractMeta: ContractMeta,
			funds: Array<Coin>,
			message: Array<number>,
			event_handler: any
		) => {
			const newAccountId = newContractAddress();
			setMetadata(newAccountId.toString(), newContractMeta);
			const newInfo: MessageInfo = {
				sender: env.contract.address,
				funds,
			};
			const newEnv: Env = {
				block: env.block,
				transaction: env.transaction,
				contract: {
					address: newAccountId.toString(),
				},
			};
			const subVM = createVM(newInfo, newEnv, newContractMeta, shared);
			// TODO: check existence of code
			console.log('continue_instantiate', newContractMeta);
			const codeHash = shared.codes.get(newContractMeta.code_id)!;
			// TODO: normally we reload a new host with the new meta for the newly running contract, we update the env to reflect sender/funds
			const result = vmContinueInstantiate(
				subVM as VMHost,
				codeStore.getCode(codeHash),
				JSON.parse(decode(message)),
				event_handler
			);
			// new contract address and result of instantiate call
			if ('Ok' in result) {
				// eslint-disable-next-line prettier/prettier
				return Ok([newAccountId.toString(), result.Ok!]); //  no idea why prettier bitches about this one
			}
			return Err(result.Err);
		},

		//  TODO: implement the rest when necessary
		transaction_begin: () => Ok(unit),
		transaction_rollback: () => Ok(unit),
		transaction_commit: () => Ok(unit),
		charge: (value: object) => Ok(unit),
		gas_checkpoint_push: () => Ok(unit),
		gas_checkpoint_pop: () => Ok(unit),
		addr_validate: (input: string) => Ok(Ok(unit)),
		running_contract_meta: () => Ok(metadata),
	};
};

const setupState: SetupState = {
	loaded: false,
	promise: null,
};
const safeSingleRunVmSetup = async (): Promise<void> => {
	if (setupState.loaded) return Promise.resolve();
	if (setupState.promise) return setupState.promise;
	const promise = vmSetup();
	setupState.promise = new Promise(async (res, rej) => {
		await promise;
		setupState.loaded = true;
		console.log('vm setup success');
		res();
	});
	return setupState.promise;
};

export const vmMethods = {
	createVM: (info: MessageInfo, env: Env, metadata: ContractMeta, shared: VMHostShared): Partial<VMHost> => {
		const vm = createVM(info, env, metadata, shared);
		return vm;
	},
	uploadContract: function (
		info: MessageInfo,
		env: Env,
		metadata: ContractMeta,
		shared: VMHostShared
	): Partial<VMHost> {
		return this.createVM(info, env, metadata, shared);
	},
	safeSingleRunVmSetup,
	instantiate: <T extends Object>(
		codeHash: UUID,
		message: T,
		vmShared: VMHostShared,
		getInitParamsParams: VmRunEnv
	): Result<VMStep, Error> => {
		const params = getVmInitParams(getInitParamsParams);
		const env = params.getInitialEnv(vmShared.nextAccountId++);
		const vm = vmMethods.createVM(params.initialMessageInfo, env, params.meta, vmShared);
		let ret;
		try {
			ret = vmInstantiate(vm as VMHost, codeStore.getCode(codeHash), message);
		} catch (ex) {
			console.error('instantiate error', ex);
			return { Err: new Error('Instantiate VM critical Error') };
		}
		vmShared.contracts.set(env.contract.address, params.meta);
		if ('Err' in ret) {
			console.log(ret.Err);
			console.error('instantiate error', ret.Err);

			return { Err: ret.Err };
		}
		const codeId = Array.from(vmShared.codes.entries()).find(([key, hash]) => hash === codeHash)?.[0];
		if (codeId === undefined) {
			console.log(Array.from(vmShared.codes.entries()), codeHash);
			throw new Error(`codeId not found for codeHash ${codeHash}`);
		}

		// const refinedEvents = getRefinedEvents(ret.Ok.events as RawContractEvent[]);

		return ret;
	},
	execute: <T extends Object>(
		codeHash: UUID,
		message: T,
		vmShared: VMHostShared,
		getInitParamsParams: SetRequired<VmRunEnv, 'contractAddress'>
	): Result<VMStep, Error> => {
		const params = getVmInitParams(getInitParamsParams);
		const vm = vmMethods.createVM(
			params.initialMessageInfo,
			params.getInitialEnv(getInitParamsParams.contractAddress || vmShared.nextAccountId),
			params.meta,
			vmShared
		);
		let ret;
		try {
			ret = vmExecute(vm as VMHost, codeStore.getCode(codeHash), message);
		} catch (ex) {
			return { Err: new Error('Execution VM critical Error') };
		}
		if ('Err' in ret) {
			console.error('execute error', ret.Err);
			const errorMsg = JSON.stringify(ret.Err);
			return { Err: new Error('Execution VM critical Error') };
		}
		const codeId = Array.from(vmShared.codes.entries()).find(([key, hash]) => hash === codeHash)?.[0];
		if (codeId === undefined) throw new Error(`codeId not found for codeHash ${codeHash}`);

		return ret;
	},
};
