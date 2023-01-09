import { ApiPromise } from "@polkadot/api";
import { DEFI_CONFIG } from "./config";
import type { Signer as InjectedSigner } from "@polkadot/api/types";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { ParachainId, RelaychainId, SubstrateNetworkId } from "shared";

export type TokenId = typeof DEFI_CONFIG.tokenIds[number];
export type AccountType = "secp256k1" | "*25519";

export type SubstrateChainApiStatus = "initializing" | "failed" | "connected";

export enum SupportedWalletId {
  Talisman = "talisman",
  Polkadotjs = "polkadot-js",
}

export type DotSamaExtensionStatus =
  | "initializing"
  | "connecting"
  | "connected"
  | "no_extension"
  | "error";

export interface SubstrateChainApi {
  parachainApi: ApiPromise | undefined;
  apiStatus: SubstrateChainApiStatus;
  prefix: number;
}

export interface ChainApi extends SubstrateChainApi {
  chainId: SubstrateNetworkId;
}

export type ConnectedAccounts = Record<
  SubstrateNetworkId,
  InjectedAccountWithMeta[]
>;

export interface DotSamaContext {
  signer: InjectedSigner | undefined;
  parachainProviders: { [chainId in ParachainId]: ChainApi };
  relaychainProviders: { [chainId in RelaychainId]: ChainApi };
  extensionStatus: DotSamaExtensionStatus;
  activate?: (
    walletId?: SupportedWalletId,
    selectedDefaultAccount?: boolean
  ) => Promise<any[] | undefined>;
  deactivate?: () => Promise<void>;
  selectedAccount: number;
  setSelectedAccount?: (account: number) => void;
  connectedAccounts: ConnectedAccounts;
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
}

export interface ParachainNetwork extends SubstrateNetwork {
  parachainId: number;
  relayChain: "kusama" | "polkadot";
}

export interface RelaychainNetwork extends SubstrateNetwork {
  networkId: "kusama" | "polkadot";
}
