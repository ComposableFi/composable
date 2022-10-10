import { ApiPromise } from "@polkadot/api";

export enum SupportedWalletId {
  Talisman = "talisman",
  Polkadotjs = "polkadot-js"
};

export type SubstrateChainApiStatus = "initializing" | "failed" | "connected";
export type DotSamaExtensionStatus =
  | "initializing"
  | "connecting"
  | "connected"
  | "no_extension"
  | "error";

export const relayChainIds = ['kusama', 'polkadot'] as const;
export const parachainIds = ['picasso', 'karura'] as const;
export const tokenIds = ['pica', 'ksm', 'dot', 'kar'] as const;
export const ChainIds = ["picasso", "kusama", "karura", "polkadot"] as const;
export type TokenId = typeof tokenIds[number];
export type ChainId = typeof ChainIds[number];
export type RelayChainId = typeof relayChainIds[number];
export type AccountType = "secp256k1" | "*25519";

export interface ConnectedAccount {
  address: string;
  name: string;
}
export interface SubstrateChainApi extends SubstrateNetwork {
  api: ApiPromise;
  apiStatus: SubstrateChainApiStatus;
  connectedAccounts: ConnectedAccount[];
}
export interface SubstrateNetwork {
  name: string;
  wsUrl: string;
  tokenId: TokenId;
  prefix: number;
  accountType: AccountType;
  subscanUrl: string;
  decimals: number;
  color?: string;
  symbol: string;
  logo: string;
  chainId: ChainId;
}
export interface ParachainNetwork extends SubstrateNetwork {
  parachainId: number;
  relayChain: RelayChainId;
}

export type ExtrinsicStatus =
  | 'isReady'
  | 'isBroadcast'
  | 'isInBlock'
  | 'isFinalized'
  | 'Error';

export interface ExtrinsicMetadata {
  hash: string;
  method: string;
  section: string;
  sender: string;
  timestamp: number;
  args: { [paramId: string]: any };
  status: ExtrinsicStatus;
  dispatchError?: string;

  isSigned: boolean;
  blockHash?: string;
}