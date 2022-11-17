import { StorageId } from '@site/src/utils/cosmwasm-vm/store/state';
import { VMHostShared } from '../../utils/cosmwasm-vm/vm/types';

type InputTypes = 'string' | 'number';
type VmExecuteParams = Record<string, InputTypes>;

interface VmExecuteProps<T extends VmExecuteParams> {
	vmShared: VMHostShared;
	readonly inputParams: T;
	readonly placeholders?: Partial<Record<keyof T, string>>;
	readonly storageId: StorageId;

	readonly execute: (input: T) => Object; //	returns the execute params passed to execute an instantiation
}

//	1. Execute the contract in the way specified with a limit of how many times it can be executed
export function VmExecute<T extends VmExecuteParams>({ storageId }: VmExecuteProps<T>) {
	return <div></div>;
}
