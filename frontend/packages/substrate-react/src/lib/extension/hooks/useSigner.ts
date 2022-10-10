import { Signer } from '@polkadot/api/types';
import { useSubstrateReact } from '../store/extension.slice';

export const useSigner = (): Signer | undefined => useSubstrateReact().signer