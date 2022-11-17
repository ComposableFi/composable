import {
	UseStore,
	createStore,
	clear as idbClear,
	del as idbDel,
	entries as idbEntries,
	get as idbGet,
	keys as idbKeys,
	set as idbSet,
} from 'idb-keyval';

import { UUID } from '../vm/types';

//  the shit you have to do because SSR
let hashStore: UseStore;
const getHashStore = (): UseStore | undefined => {
	if (typeof window === 'undefined') return undefined;
	if (!hashStore) hashStore = createStore('hashCode', 'hashCode');
	return hashStore;
};

const persistentStore = {
	getKeys: async (): Promise<string[]> => {
		try {
			const keys = await idbKeys(getHashStore());
			return keys as string[];
		} catch (e) {
			console.error(e);
			throw new Error('failed to fetch keys');
		}
	},
	setCode: async (id: UUID, code: Uint8Array): Promise<boolean> => {
		try {
			const search = await idbGet(id, getHashStore());
			if (search) return false;
			await idbSet(id, code, getHashStore());
			return true;
		} catch (e) {
			console.error(e);
			throw new Error('failed to set code in idb');
		}
	},
	delCode: async (id: UUID): Promise<void> => {
		try {
			await idbDel(id, getHashStore());
		} catch (e) {
			console.error(e);
			throw new Error('failed to remove code from idb');
		}
	},
	getCode: async (id: UUID): Promise<Uint8Array> => {
		const code = await idbGet(id, getHashStore());
		return code;
	},
	getAllCodes: async (): Promise<[string, Uint8Array][]> => {
		return idbEntries(getHashStore());
	},
	clearCodes: async (): Promise<void> => {
		await idbClear(getHashStore());
	},
};

export { persistentStore };
