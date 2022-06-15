import { LiquidityPoolTransactionType } from "@/defi/types";
import { PoolTradeHistory } from "@/store/auctions/auctions.types";
import { LiquidityBootstrappingPool } from "@/store/pools/pools.types";
import { fromChainUnits } from "../units";
import BigNumber from "bignumber.js";

export function getCurrentWeights(
  pool: LiquidityBootstrappingPool,
  current_block: BigNumber
): { baseWeight: BigNumber; quoteWeight: BigNumber } {
  let baseWeight = new BigNumber(0),
    quoteWeight = new BigNumber(0);

  let one = new BigNumber(1);
  let pointInSale = new BigNumber(current_block).div(
    new BigNumber(pool.sale.endBlock).minus(pool.sale.startBlock)
  );
  let weightRange = new BigNumber(pool.sale.initialWeight)
    .div(100)
    .minus(new BigNumber(pool.sale.finalWeight).div(100));

  baseWeight = new BigNumber(pool.sale.initialWeight)
    .div(100)
    .minus(pointInSale.times(weightRange));
  quoteWeight = one.minus(baseWeight);

  return {
    baseWeight,
    quoteWeight,
  };
}

export function transformAuctionsTransaction(
    transaction: {
      transactionType: LiquidityPoolTransactionType;
      receivedTimestamp: string;
      quoteAssetAmount: string;
      baseAssetAmount: string;
      quoteAssetId: string;
      baseAssetId: string;
      spotPrice: string;
      who: string;
      id: string;
    },
    onChainPoolQuoteAssetId: number
  ): PoolTradeHistory {

    const baseAssetId = Number(transaction.baseAssetId);
    const quoteAssetId = Number(transaction.quoteAssetId);
  
    let spotPrice: string = new BigNumber(transaction.spotPrice).toString();
    let baseAssetAmount: BigNumber | string = new BigNumber(0);
    let quoteAssetAmount: BigNumber | string = new BigNumber(0);
    let receivedTimestamp = Number(transaction.receivedTimestamp);
    let id = transaction.id;
    let walletAddress = transaction.who;
    let side: any = "SELL";
  
    if (quoteAssetId === onChainPoolQuoteAssetId) {
      side = "BUY";
      baseAssetAmount = fromChainUnits(transaction.baseAssetAmount).toString();
      quoteAssetAmount = fromChainUnits(transaction.quoteAssetAmount).toString();
    } else {
      baseAssetAmount = fromChainUnits(transaction.baseAssetAmount).toString();
      quoteAssetAmount = fromChainUnits(transaction.quoteAssetAmount).toString();
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
    };
  }