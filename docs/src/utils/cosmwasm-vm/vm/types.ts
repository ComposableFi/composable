import { Addr, CodeId, ContractMeta, Env, MessageInfo } from 'cw_vm_ts_sdk_001';

export interface VMHostShared {
	storage: Map<Addr, Map<String, Object>>; //  generates a map for each instantiations I think
	codes: Map<CodeId, UUID>; //  codeId - contract hash
	contracts: Map<Addr, ContractMeta>; //  instantiated contracts
	nextAccountId: number;
}

export const getInitVmHostShared = (): VMHostShared => ({
	storage: new Map<Addr, Map<String, Object>>(),
	codes: new Map<CodeId, UUID>(),
	contracts: new Map<Addr, ContractMeta>(),
	nextAccountId: 10000,
});

export interface VmInitParams {
	codeId: number;
	initialMessageInfo: MessageInfo;
	getInitialEnv: (addr: number) => Env;
	meta: ContractMeta;
}

export interface VmRunEnv {
	contractAddress?: number; //  mandatory for execute, optional for instantiate
	senderAddress: number;
	codeId: number;
	label?: string;
}

export const getVmInitParams = (params: VmRunEnv): VmInitParams => {
	return {
		codeId: params.codeId,
		initialMessageInfo: {
			sender: params.senderAddress.toString(),
			funds: [],
		},
		getInitialEnv: (addr: number) => ({
			block: {
				height: 0,
				time: '0',
				chain_id: 'html-chain',
			},
			transaction: {
				index: 0,
			},
			contract: {
				address: addr.toString(),
			},
		}),
		meta: {
			code_id: params.codeId,
			admin: null,
			label: params?.label || '',
		},
	};
};
