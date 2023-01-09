import { TokenId } from "tokens";
import config from "@/constants/config";
import { SubstrateNetworkId } from "shared";

export type AllowedTransferList = {
  [key in SubstrateNetworkId]: Record<SubstrateNetworkId, Array<TokenId>>;
};

export type NetworkId = typeof config.evm.networkIds[number];
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

export type AMM_ID = typeof config.evm.ammIds[number];
export type AMM = {
  id: AMM_ID;
  icon: string;
  label: string;
};
