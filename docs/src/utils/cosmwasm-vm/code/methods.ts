import { CodeKeyValue, memoryStore } from './memoryStore';
import { persistentStore } from './persistentStore';

// This was created purely so that code can be stored in IDB instead of local Storage
// While keeping the code in memory so that code can be retrieved synchronously without being stored inside the VM
export const codeStore = {
	setCode: async (id: UUID, code: Uint8Array): Promise<boolean> => {
		try {
			const res = await persistentStore.setCode(id, code);
			if (res) memoryStore.setCode(id, code);
			return res;
		} catch (e) {
			console.error(e);
			throw new Error('failed to set code');
		}
	},
	getCode: (id: UUID): Uint8Array => {
		const code = memoryStore.getCode(id);
		if (code) return code;
		console.log(memoryStore.getAllCodes());
		throw new Error(`code not found - ${id}`);
	},
	getAllCodes: (): CodeKeyValue => {
		return memoryStore.getAllCodes();
	},
	getKeys: async (): Promise<UUID[]> => {
		const keys = memoryStore.getKeys();
		if (keys.length) return keys;
		return persistentStore.getKeys();
	},
	deleteCode: async (id: UUID): Promise<void> => {
		try {
			await persistentStore.delCode(id);
			memoryStore.delCode(id);
		} catch (e) {
			console.error(e);
			throw new Error('failed to delete code');
		}
	},
	clearCodes: async (): Promise<void> => {
		memoryStore.clearCodes();
		persistentStore.clearCodes().then();
	},
	loadPersistentToMemory: async (): Promise<void> => {
		const codes = await persistentStore.getAllCodes();
		codes.forEach(([id, code]) => memoryStore.setCode(id, code));
	},
};
