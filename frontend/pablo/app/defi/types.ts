import BigNumber from "bignumber.js";
import { DEFI_CONFIG } from "./config";

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

export type Auction = {
  id: string;
  tokenId: TokenId;
  networkId: NetworkId;
  duration: number; //in days
  totalVolume: BigNumber;
  liquidity: BigNumber;
  price: BigNumber;
  tokenSold: BigNumber;
  fundsRaised: BigNumber;
  contract: {
    tokenAddress: string;
    ownerAddress: string;
    docLink: string;
  };
  statistics: {
    startBalances: {
      token: BigNumber;
      base: BigNumber;
    };
    currentBalances: {
      token: BigNumber;
      base: BigNumber;
    };
    totalSold: BigNumber;
    totalRaised: BigNumber;
  };
  start_at: number; //timestamp
  end_at: number; //timestamp
};

export type AuctionHistory = {
  type: "Buy" | "Sell";
  input: {
    tokenId: TokenId;
    amount: BigNumber;
  };
  output: {
    tokenId: TokenId;
    amount: BigNumber;
  };
  price: BigNumber;
  walletAddress: string;
  created_at: number; //timestamp
};

export type AuctionLaunchDescritpion = {
  paragraphs: string[];
};

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

export type TransactionSettings = {
  tolerance: number,
  deadline: number,
};

export type XPablo = {
  tokenId: TokenId,
  locked: BigNumber,
  expiry: number;
  muliplier: number;
  amount: BigNumber;
}
