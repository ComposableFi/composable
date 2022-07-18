import BigNumber from "bignumber.js";

export interface PabloExchangeParams {
  quoteAmount: BigNumber;
  baseAssetId: string;
  quoteAssetId: string;
  side: "quote" | "base";
  slippage: number;
}
