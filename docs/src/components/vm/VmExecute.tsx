import { StorageId } from '@site/src/utils/cosmwasm-vm/store/state';
import { VMHostShared } from '../../utils/cosmwasm-vm/vm/types';
import React, { useCallback, useState } from 'react';
import { vmMethods } from '../../utils/cosmwasm-vm/vm/vmMethods';
import { vmStoreMethods } from '../../utils/cosmwasm-vm/store/storeMethods';
import { RawContractEvent } from '../../utils/cosmwasm-vm/types';

const clone = require('rfdc')();

interface VmExecuteProps {
	vmShared: VMHostShared;

	readonly storageId: StorageId;

	//	params are keys, default values for input are values
	readonly inputParams: Record<string, string>;
	readonly placeholders?: Partial<Record<string, string>>;

	readonly createExecuteMessage: (input: Record<string, string>) => Object; //	returns the execute params passed to execute an instantiation
}

//	TODO : make this headless
//	1. Execute the contract in the way specified with a limit of how many times it can be executed
export function VmExecute({ storageId, vmShared, inputParams, placeholders, createExecuteMessage }: VmExecuteProps) {
	const [state, setState] = useState<Record<string, string>>(() => ({ ...inputParams }));
	// console.log('vmExecute >>> ', vmShared, inputParams, placeholders);

	//	TODO : update to work when multiple contracts are instantiated
	const executeMessage = useCallback(
		(input: Record<string, string>) => {
			const codeHash = vmShared.codes.get(0)!;
			const msg = createExecuteMessage(input);
			const clonedVmShared = clone(vmShared);
			const res = vmMethods.execute(codeHash, msg, clonedVmShared, {
				codeId: 0,
				senderAddress: 0,
				contractAddress: 10000,
			});
			if ('Ok' in res) {
				console.log(clonedVmShared);
				vmStoreMethods.vmStateUpdate(storageId, clonedVmShared, res.Ok.events as RawContractEvent[]);
			}
			console.log(res);
		},
		[vmShared, createExecuteMessage]
	);

	return (
		<div className={'rounded-xl w-full'}>
			<ul className={'flex items-start gap-4'}>
				{Object.keys(inputParams).map(key => {
					return (
						<li className={'flex flex-col items-start gap-2 !mt-0'} key={key}>
							<label htmlFor={key}>{key}</label>
							<input
								onInput={e => {
									e.preventDefault();
									setState(prevState => ({ ...prevState, [key]: (e.target as HTMLInputElement)?.value }));
								}}
								value={state[key]}
								type={'text'}
								id={key}
								placeholder={placeholders?.[key]}
							/>
						</li>
					);
				})}
			</ul>
			<button type="button" onClick={() => executeMessage(state)} className={'rounded-2xl px-4 py-2 bg-gray-500 mt-5'}>
				<p className={'!mb-0'}>Execute button</p>
			</button>
		</div>
	);
}
