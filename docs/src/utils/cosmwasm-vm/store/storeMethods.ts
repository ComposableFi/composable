import { getInitialState, StorageId, vmStore } from './state';
import { VMHostShared } from '../vm/types';
import { ContractEvent, RawContractEvent } from '../types';
import { codeStore } from '../code/methods';
const codeUpload = (storageId: StorageId, codeHash: string) => {
	vmStore.setState(state => {
		//	TODO : update to work with multiple contract uploads to single VM
		state.codeHashMap[storageId] = codeHash;
	});
};

const vmStateUpdate = (storageId: StorageId, vmShared: VMHostShared, contractEvents?: RawContractEvent[]) => {
	vmStore.setState(state => {
		state.vmStates[storageId] = { state: vmShared, events: contractEvents || [] };
	});
};

const resetStore = async () => {
	vmStore.setState(getInitialState(true));
	await codeStore.clearCodes();
	console.log('store reset');
};

export const vmStoreMethods = {
	codeUpload,
	vmStateUpdate,
	resetStore,
};
