import {
  PabloTransactions,
} from "@/defi/subsquid/pools/queries";
import { LiquidityBootstrappingPoolTrade } from "@/defi/types/auctions";
import { fromChainUnits } from "../../units";
import BigNumber from "bignumber.js";

export function transformPabloTransaction(
  tx: PabloTransactions,
  poolQuoteAssetId: number
): LiquidityBootstrappingPoolTrade {
  const baseAssetId = Number(tx.baseAssetId);
  const quoteAssetId = Number(tx.quoteAssetId);

  let spotPrice: string = new BigNumber(tx.spotPrice).toString();
  let baseAssetAmount: BigNumber | string = new BigNumber(0);
  let quoteAssetAmount: BigNumber | string = new BigNumber(0);
  let receivedTimestamp = Number(tx.pool.calculatedTimestamp);
  let blockNumber = new BigNumber(tx.event.blockNumber);
  let id = tx.id;
  let walletAddress = tx.event.accountId;
  let side: any = "SELL";

  if (quoteAssetId === poolQuoteAssetId) {
    side = "BUY";
    baseAssetAmount = fromChainUnits(tx.baseAssetAmount).toString();
    quoteAssetAmount = fromChainUnits(tx.quoteAssetAmount).toString();
  } else {
    baseAssetAmount = fromChainUnits(tx.baseAssetAmount).toString();
    quoteAssetAmount = fromChainUnits(tx.quoteAssetAmount).toString();
    spotPrice = new BigNumber(1).div(new BigNumber(spotPrice)).toString();
  }

  return {
    baseAssetId,
    baseAssetAmount,
    id,
    quoteAssetAmount,
    quoteAssetId,
    receivedTimestamp,
    spotPrice: spotPrice,
    side,
    walletAddress,
    blockNumber,
  };
}
