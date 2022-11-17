import { StorageId } from '@site/src/utils/cosmwasm-vm/store/state';

type InputTypes = 'string' | 'number';
type VmExecuteParams = Record<string, InputTypes>;

interface VmInputProps<T extends VmExecuteParams> {
	readonly inputParams: T;
	readonly placeholders?: Partial<Record<keyof T, string>>;
	readonly storageId: StorageId;

	readonly execute: (input: T) => Object; //	returns the execute params passed to execute an instantiation
}

//	1. Execute the contract in the way specified with a limit of how many times it can be executed
export function VmInput<T extends VmExecuteParams>({ storageId }: VmInputProps<T>) {
	return <div></div>;
}
