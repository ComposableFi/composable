import { providers } from '@0xsequence/multicall'
import { Web3ReactHooks } from '@web3-react/core'
import { ethers, Signer } from "ethers";
import { createContext } from "react";

import { ConnectorType } from '../hooks';

export type EthersProvider =
  | ethers.providers.StaticJsonRpcProvider
  | ethers.providers.Web3Provider

export interface BlockchainProviderDescriptor {
  account?: ReturnType<Web3ReactHooks['useAccount']>;
  chainId?: ReturnType<Web3ReactHooks['useChainId']>;
  connectorType?: ConnectorType;
  provider?: providers.MulticallProvider;
  signer?: Signer;
}

export interface BlockchainProvidersDescriptor {
  [chainId: number]: BlockchainProviderDescriptor;
}

export type BlockchainProviderValues = {
  blockchainProviders: BlockchainProvidersDescriptor;
}

export const BlockchainProviderContext = createContext<BlockchainProviderValues>({ blockchainProviders: {} });