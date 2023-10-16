import BigNumber from "bignumber.js";

export interface IPreBalances {
  "pica": string,
  "dot": string,
  "usdt": string,
  "ksm": string
}

export interface IPreBalancesOnPicasso{
  "1": string,
  "4": string,
  "6": string,
  "130": string
}

export interface Asset {
  id: string,
  balance: Map<string, BigNumber>,
  decimals: number,
  isNative: boolean,
  chain: string,
  totalIssuance?: BigNumber
}