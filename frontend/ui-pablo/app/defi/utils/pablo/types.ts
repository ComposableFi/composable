import { AssetId } from "@/defi/polkadot/types";
import BigNumber from "bignumber.js";

export interface PabloExchangeParams {
  quoteAmount: BigNumber;
  baseAssetId: AssetId;
  quoteAssetId: AssetId;
  side: "quote" | "base";
  slippage: number;
}
