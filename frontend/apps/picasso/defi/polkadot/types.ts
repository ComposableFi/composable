import { TokenId } from "tokens";
import { DEFI_CONFIG } from "./config";

export type SubstrateNetworkId = typeof DEFI_CONFIG.networkIds[number];
export type SubstrateNetwork = {
  relayChain: "polkadot" | "kusama";
  parachainId: number | 0;
  name: string;
  wsUrl: string;
  tokenId: TokenId;
  ss58Format: number;
  subscanUrl: string;
  decimals: number;
  color?: string;
  symbol: string;
  logo: string;
};
