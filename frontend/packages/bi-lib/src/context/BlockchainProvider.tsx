import { providers } from '@0xsequence/multicall'
import { ethers } from "ethers";
import React, { useCallback, useEffect, useState } from "react";
import { useSelector } from "react-redux";

import { connectorToConnectorType, ConnectorType } from "../hooks/useConnector";
import { HooksState, PriorityConnector } from "../workaround-web3-react-issues-379/HooksStore";
import { BlockchainProviderContext, BlockchainProviderDescriptor, BlockchainProvidersDescriptor } from "./BlockchainProviderContext";

export interface BlockchainDescriptor {
  chainId: number;
  rpcUrl: string;
}

export interface BlockchainProviderProps {
  children: any;
  blockchainInfo: Array<BlockchainDescriptor>;
}

export const BlockchainProvider = (props: BlockchainProviderProps) => {
  const { blockchainInfo } = props;

  const [blockchainProviders, setBlockchainProviders] = useState<BlockchainProvidersDescriptor>({});

  const priorityConnector = useSelector<HooksState, PriorityConnector>((state) => state.priorityConnector as PriorityConnector);

  const {
    usePriorityConnector,
    useSelectedAccount,
    useSelectedChainId,
    useSelectedProvider,
  } = priorityConnector;

  const connector = usePriorityConnector();

  const selectedProvider = useSelectedProvider(connector);
  const selectedChainId = useSelectedChainId(connector);
  const selectedConnectorType = connectorToConnectorType(connector);
  const selectedAccount = useSelectedAccount(connector);

  const createSelectedProvider = useCallback(
    () : BlockchainProviderDescriptor => {
      return {
        account: selectedAccount,
        chainId: selectedChainId,
        connectorType: selectedConnectorType,
        signer: selectedProvider?.getSigner(),
        provider: selectedProvider && new providers.MulticallProvider(selectedProvider),
      };
    },
    [selectedAccount, selectedChainId, selectedConnectorType, selectedProvider]
  );

  const createStaticProvider = (descriptor: BlockchainDescriptor) : BlockchainProviderDescriptor => {
    const {
      chainId,
      rpcUrl,
    } = descriptor;

    const provider = new ethers.providers.StaticJsonRpcProvider(rpcUrl);

    return {
      chainId: chainId,
      connectorType: ConnectorType.Static,
      signer: provider.getSigner(),
      provider: new providers.MulticallProvider(provider),
    };
  }

  useEffect(
    () => {
      blockchainInfo.forEach((descriptor: BlockchainDescriptor) => {
        if (descriptor.chainId in blockchainProviders && selectedChainId === descriptor.chainId && blockchainProviders[descriptor.chainId]?.connectorType !== selectedConnectorType) {
          blockchainProviders[descriptor.chainId].provider?.removeAllListeners();
        }
      })
    },
    [blockchainInfo, blockchainProviders, selectedChainId, selectedConnectorType]
  );

  useEffect(
    () => {
      const providers = blockchainInfo.reduce(
        (
          providers: BlockchainProvidersDescriptor, descriptor: BlockchainDescriptor
        ) => {
          providers[descriptor.chainId] = selectedChainId === descriptor.chainId ? createSelectedProvider() : createStaticProvider(descriptor);

          return providers;
        },
        {}
      )

      setBlockchainProviders(providers);
    },
    [blockchainInfo, createSelectedProvider, selectedChainId]
  );

  return (
    <BlockchainProviderContext.Provider value={{ blockchainProviders }}>
      {props.children}
    </BlockchainProviderContext.Provider>
  );
};
