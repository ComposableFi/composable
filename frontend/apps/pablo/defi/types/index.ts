import { TokenId } from "tokens";
import BigNumber from "bignumber.js";
import { DEFI_CONFIG } from "../config";
export * from "./vesting";
export * from "./stakingRewards";

export type AssetDropdownOptions = Array<{
  value: string;
  label: string;
  shortLabel: string;
  icon: string;
}>;

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

export type AmmId = typeof DEFI_CONFIG.ammIds[number]
export type AMM = {
  id: AmmId,
  icon: string,
  label: string
}


export type ChartInterval = "24h" | "1m" | "1w";// | "1y";
export enum LiquidityPoolTransactionType { "SWAP", "ADD_LIQUIDITY", "CREATE_POOL", "REMOVE_LIQUIDITY" };


export type TableHeader = {
  header: string;
  tooltip?: string;
};

export type BondDetails = {
  tokenId1: TokenId;
  tokenId2: TokenId;
  roi: number;
  vesting_term: number;
  tvl: BigNumber;
  volume: BigNumber;
  discount_price: BigNumber;
  market_price: BigNumber;
  balance: BigNumber;
  rewardable_amount: BigNumber;
  buyable_amount: BigNumber;
  pending_amount: BigNumber;
  claimable_amount: BigNumber;
  remaining_term: number;
  vested_term: number;
};

export type TransactionSettings = {
  tolerance: number,
  deadline: number,
};

export type PoolLiquidityChartData = {
  series: number[],
  labels: string[],
};
