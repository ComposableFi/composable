import React from 'react';
import { useMemo, useState } from 'react';
import { unhexifyKeys } from '../../../utils/cosmwasm-vm/parsers';
import { VmWrapper } from './VmWrapper';

interface VmDisplayStorageWrapperProps {
	readonly children: (data: Record<string, Object> | null) => JSX.Element;
	readonly storageId: string;
}
export const VmDisplayStorageWrapper = ({ storageId, children }: VmDisplayStorageWrapperProps) => {
	return (
		<VmWrapper storageId={storageId}>
			{vmShared => {
				const ret = useMemo(() => {
					if (!vmShared) return null;
					const storage = vmShared.storage.get('10000');
					if (!storage) return null;
					return unhexifyKeys(Object.fromEntries(storage.entries()));
				}, [vmShared]);

				return children(ret);
			}}
		</VmWrapper>
	);
};
