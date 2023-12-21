import BigNumber from "bignumber.js";

export interface Asset {
  id: string,
  balance: Map<string, BigNumber>,
  decimals: number,
  isNative: boolean,
  chain: string,
  symbol: string,
  totalIssuance?: BigNumber
}

export interface Chain {
  chainType: string,
  addresses?: object
}

export interface Chains {
  [key: string]: Chain,
}