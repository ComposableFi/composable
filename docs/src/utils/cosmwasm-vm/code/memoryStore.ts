//  this exists so that you don't have to await fetching contract code from idb
//  has the exact same contents as the persistentStore
/* eslint-disable no-param-reassign */ // -> because this is state definitions

import { devtools } from 'zustand/middleware';
import createVanilla from 'zustand/vanilla';

export type CodeKeyValue = {
	[key: UUID]: Uint8Array;
};

interface CodeState {
	state: CodeKeyValue;
	getKeys: () => UUID[];
	getAllCodes: () => CodeKeyValue;
	getCode: (id: UUID) => Uint8Array | undefined;
	setCode: (id: UUID, code: Uint8Array) => void;
	delCode: (id: UUID) => void;
}

const store = createVanilla<CodeState>()(
	devtools((set, get) => ({
		state: {} as CodeKeyValue,
		getKeys: (): UUID[] => Object.keys(get().state) as UUID[],
		getAllCodes: (): CodeKeyValue => {
			return get().state;
		},
		delCode: (id: UUID): void => {
			set(state => {
				delete state.state[id];
				return state;
			});
		},
		getCode: (id: UUID): Uint8Array => get().state[id],
		setCode: (id: UUID, code: Uint8Array): void => {
			set(storeState => {
				storeState.state[id] = code;
				return storeState;
			});
		},
	}))
);

export const memoryStore = {
	getKeys: (): UUID[] => store.getState().getKeys(),
	delCode: (id: UUID): void => store.getState().delCode(id),
	getAllCodes: (): CodeKeyValue => store.getState().getAllCodes(),
	getCode: (id: UUID): Uint8Array | undefined => store.getState().getCode(id),
	setCode: (id: UUID, code: Uint8Array): void => store.getState().setCode(id, code),
	clearCodes: (): void => store.setState({ state: {} }),
};
