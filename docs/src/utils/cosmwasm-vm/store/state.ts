import { VMHostShared } from '@site/src/utils/cosmwasm-vm/vm/types';
import { ContractEvent, RawContractEvent } from '@site/src/utils/cosmwasm-vm/types';
import createVanilla from 'zustand/vanilla';
import { devtools, persist, subscribeWithSelector } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import create from 'zustand';
import { persistentStore } from '../code/persistentStore';
import { memoryStore } from '../code/memoryStore';
import { jsonReplacer, jsonReviver } from '../parsers';

export type StorageId = string;
interface GlobalState {
	vmStates: Record<StorageId, VmState>;
	codeHashMap: Record<StorageId, string>;
	hydrated: boolean;
}

export interface VmState {
	state: VMHostShared;
	events: RawContractEvent[];
}

export const getInitialState = (hydrated: boolean = false): GlobalState => {
	return {
		vmStates: {},
		codeHashMap: {},
		hydrated,
	};
};

export const vmStore = createVanilla<GlobalState>()(
	devtools(
		persist(subscribeWithSelector(immer((set, get) => getInitialState())), {
			name: 'cosmwasm-vm', // unique name
			getStorage: () => localStorage, // (optional) by default, 'localStorage' is used
			serialize: state => {
				state.state.hydrated = false;
				return JSON.stringify(state, jsonReplacer);
			},
			deserialize: (str: string) => JSON.parse(str, jsonReviver),
			onRehydrateStorage: () => async (state, error) => {
				if (error) {
					console.log('Error rehydrating vm store state');
					console.error(error);
				}
				if (state) {
					//  load code into the store
					const promiseArr: Promise<void>[] = [];
					state.hydrated = true;
					const codes = await persistentStore.getAllCodes();
					codes.forEach(([hash, codeArr]) => {
						promiseArr.push(
							new Promise(resolve => {
								memoryStore.setCode(hash, codeArr);
								resolve();
							})
						);
					});
					await Promise.all(promiseArr);
					console.log('storage hydrated');
				}
			},
		})
	)
);

export const useVmStore = create(vmStore);
