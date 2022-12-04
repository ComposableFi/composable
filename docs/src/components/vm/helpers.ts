//	TODO : update to make other values editable
import { Env } from 'cw_vm_ts_sdk_001';
import { VMHostShared } from '../../utils/cosmwasm-vm/vm/types';

export const getVmEnv = (accountId: string): Env => {
	return {
		block: {
			height: 0,
			time: '0',
			chain_id: 'cosmwasm-vm',
		},
		transaction: {
			index: 0,
		},
		contract: {
			address: accountId.toString(),
		},
	};
};

export const getDefaultVmMessageOptions = () => ({
	messageInfo: {
		sender: '0',
		funds: [],
	},
	contractMeta: {
		code_id: 0,
		admin: null,
		label: '',
	},
});
