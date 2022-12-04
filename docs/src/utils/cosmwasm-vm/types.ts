import { Option } from 'cw_vm_ts_sdk_001';

export interface SetupState {
	loaded: boolean;
	promise: Option<Promise<void>>;
}
export interface RawContractEvent {
	type: ContractEventType;
	attributes: KeyValue[];
}

export interface ContractEvent {
	type: ContractEventType;
	attributes: Record<string, string>;
}

export type ContractEventType = 'vm-initialize' | 'instantiate' | 'execute' | 'contract-upload';
interface KeyValue {
	key: string;
	value: string;
}
