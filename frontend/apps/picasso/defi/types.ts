import { DEFI_CONFIG } from "./config";
import { TokenId } from "tokens";

export type NetworkId = typeof DEFI_CONFIG.networkIds[number];
export type Network = {
  name: string;
  rpcUrl: string;
  infoPageUrl: string;
  infoPage: string;
  backgroundColor: string;
  logo: string;
  defaultTokenSymbol: string;
  publicRpcUrl: string;
  nativeToken: TokenId;
};

export type AMM_ID = typeof DEFI_CONFIG.ammIds[number];
export type AMM = {
  id: AMM_ID;
  icon: string;
  label: string;
};
