
import { ChainId, SubstrateChainApi } from '@/lib/types';
import { ApiPromise } from '@polkadot/api';
import { useSubstrateReact } from '../store/extension.slice';

export const useSubstrateNetworkApi = (chainId: ChainId): ApiPromise => {
  const { chainApi } = useSubstrateReact();
  return chainApi[chainId].api;
};

export const useSubstrateNetwork = (chainId: ChainId): SubstrateChainApi => {
  const { chainApi } = useSubstrateReact();
  return chainApi[chainId];
};
