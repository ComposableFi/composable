import React from 'react';
import Layout from '@theme/Layout';
import { vmMethods } from '@site/src/utils/cosmwasm-vm/vm/vmMethods';

vmMethods.safeSingleRunVmSetup();
export default function TestVM() {
	return (
		<Layout title="Hello" description="Hallo">
			<div className={'flex justify-center items-center h-[50vh]'}>
				<p className={'text-[20px]'}>
					Edit <code>src/pages/test-vm/index.tsx</code> and save to reload.
				</p>
			</div>
		</Layout>
	);
}
