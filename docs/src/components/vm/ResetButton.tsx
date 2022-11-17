import { vmStoreMethods } from '../../utils/cosmwasm-vm/store/storeMethods';
import React from 'react';

export const ResetButton = () => {
	return (
		<button
			className={'rounded-2xl px-5 py-2 bg-red-500 flex items-center justify-center'}
			onClick={() => vmStoreMethods.resetStore()}>
			<p>reset state</p>
		</button>
	);
};
