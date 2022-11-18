import { StorageId, useVmStore, vmStore } from '../../../utils/cosmwasm-vm/store/state';
import { Result, VMStep } from 'cw_vm_ts_sdk_001';
import { getInitVmHostShared, VMHostShared } from '../../../utils/cosmwasm-vm/vm/types';
import { useEffect, useLayoutEffect, useMemo, useState } from 'react';
import { loadRemoteContract } from '../../../utils/cosmwasm-vm/code/utils';
import { vmStoreMethods } from '../../../utils/cosmwasm-vm/store/storeMethods';
import { vmMethods } from '../../../utils/cosmwasm-vm/vm/vmMethods';
import { RawContractEvent } from '../../../utils/cosmwasm-vm/types';

export interface VmContextInit {
	readonly storageId: StorageId;

	//	if initOptions is not provided, it waits until the sharedVm is populated at the storageId
	readonly initOptions?: {
		readonly contractUrl: string;
		readonly instantiateObj: Object;
	};
}

export const useVmContext = ({ storageId, initOptions }: VmContextInit): VMHostShared | null => {
	const vmShared = useVmStore(state => state.vmStates[storageId]);

	useEffect(() => {
		if (!initOptions) return;
		fetchContract(storageId, initOptions.contractUrl).then(() => {
			getVmShared(storageId, initOptions.instantiateObj);
		});
	}, []);

	return vmShared?.state;
};

//	TODO : currently uploads only one code to address 0, update to process array of codeIds
//	 maybe extract code upload logic out to separate function
const getVmShared = (storageId: StorageId, instantiateObj?: Object) => {
	const codeHash = vmStore.getState().codeHashMap[storageId];
	if (!codeHash) {
		console.log('code hash should be loaded by here');
		return;
	}

	const storedVmShared = vmStore.getState().vmStates[storageId];
	if (storedVmShared) return;

	const vmShared = getInitVmHostShared();
	const codeId = 0;
	vmShared.codes.set(codeId, codeHash);
	let res: Result<VMStep, Error>;
	if (instantiateObj) {
		res = vmMethods.instantiate(codeHash, instantiateObj, vmShared, { senderAddress: 0, codeId });
		if ('Ok' in res) {
			vmStoreMethods.vmStateUpdate(storageId, vmShared, res.Ok.events as RawContractEvent[]);
		} else {
			//	TODO : handle error
			console.error('instantiate error', res.Err);
		}
	}
};

const fetchContract = async (storageId: StorageId, contractUrl: string): Promise<void> => {
	let codeHash: string;
	codeHash = vmStore.getState().codeHashMap[storageId];
	if (codeHash) return;

	console.log('fetch contract');
	codeHash = await loadRemoteContract(contractUrl);
	if (codeHash === '') return;
	vmStoreMethods.codeUpload(storageId, codeHash);
};
