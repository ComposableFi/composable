import BigNumber from "bignumber.js";
import { DEFI_CONFIG } from "../config";
import { useFilteredAssetListDropdownOptions } from "../hooks/assets/useFilteredAssetListDropdownOptions";

export * from "./bonds";
export * from "./pablo";
export * from "./vesting";

export type AssetDropdownOptions = ReturnType <typeof useFilteredAssetListDropdownOptions>;

export type TokenId = typeof DEFI_CONFIG.tokenIds[number];
export type Token = {
  id: TokenId;
  icon: string;
  symbol: string;
  name: string;
};

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
  volumne: BigNumber;
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

export type Supply = {
  tokenId1: TokenId | 'none';
  tokenId2: TokenId | 'none';
  balance1: BigNumber;
  balance2: BigNumber;
  pooledAmount1: BigNumber;
  pooledAmount2: BigNumber;
  approvedToken1: boolean;
  approvedToken2: boolean;
  price1: BigNumber;
  price2: BigNumber;
  share: BigNumber;
  amount: BigNumber;
  confirmed: boolean;
};

export type Liquidity = {
  tokenId1: TokenId | 'none';
  tokenId2: TokenId | 'none';
  pooledAmount1: BigNumber;
  pooledAmount2: BigNumber;
  price1: BigNumber;
  price2: BigNumber;
  share: BigNumber;
  amount: BigNumber;
};

export type PoolInfo = {
  type: string,
  ammId: AmmId | 'none';
  tokenId1: TokenId | 'none';
  tokenId2: TokenId | 'none';
  tokenWeight1: BigNumber;
  tokenWeight2: BigNumber;
  initialSwapFee: BigNumber;
  createdAt?: number;
};

export type PoolDetails = {
  tokenId1: TokenId;
  tokenId2: TokenId;
  tokenWeight1: BigNumber;
  tokenWeight2: BigNumber;
  initialSwapFee: BigNumber;
  createdAt?: number;
  poolValue: BigNumber;
  poolAmount: BigNumber;
  rewardValue: BigNumber;
  rewardsLeft: {tokenId: TokenId, value: BigNumber}[],
  volume: BigNumber,
  fee24h: BigNumber,
  apr: number,
  transactions24h: number,
  tvlChartData: PoolTVLChartData,
};

export type TransactionSettings = {
  tolerance: number,
  deadline: number,
};

export type XPablo = {
  id: number,
  tokenId: TokenId,
  locked: BigNumber,
  expiry: number;
  multiplier: number;
  amount: BigNumber;
  withdrawableAmount: BigNumber;
};

export type PoolTVLChartData = {
  series: [number, number][],
  timeSlots: string[],
};

export type PoolLiquidityChartData = {
  series: number[],
  labels: string[],
};
