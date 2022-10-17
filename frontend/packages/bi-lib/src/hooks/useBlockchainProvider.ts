import { useContext, useMemo } from "react";

import { BlockchainProviderContext } from "../context/BlockchainProviderContext";

export const useBlockchainProvider = (chainId: number) => {
  const { blockchainProviders } = useContext(BlockchainProviderContext);

  const provider = useMemo(
    () => chainId in blockchainProviders && blockchainProviders[chainId] || {},
    [chainId, blockchainProviders]
  );

  return provider;
};