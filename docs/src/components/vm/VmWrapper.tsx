import { useVmContext, VmContextInit } from './useVmContext';
import { VMHostShared } from '../../utils/cosmwasm-vm/vm/types';
import { vmMethods } from '../../utils/cosmwasm-vm/vm/vmMethods';

//	TODO : update the storageId to different value if contract code or contractUrl is changed
//	premise is that only a single contract is uploaded with a single instantiation(for the time being)
//	since the instantiation part and execution part is decoupled, should not be too difficult to extend to do other stuff ie) IBC-ready instantiation with multiple contracts and instances
//	no fallback is provided since fallback / loading state should be handled by the children components themselves
type VmWrapperProps = {
	//	add relevant props to pass to children
	readonly children: (vmShared: VMHostShared | null) => JSX.Element;
} & VmContextInit;

if (typeof window !== 'undefined') {
	vmMethods.safeSingleRunVmSetup();
}

//	This wrapper component will do the following
//	1. Initialize the VM with the storageId(if not already initialized)
//	2. Load the contract from the URL provided
//	3. instantiate the contract with the provided params & message
export function VmWrapper({ storageId, initOptions, children }: VmWrapperProps): JSX.Element {
	const vmShared = useVmContext({ storageId, initOptions });

	return children(vmShared);
}
