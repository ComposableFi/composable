import {
  PoolTradeHistory,
  LiquidityBootstrappingPoolTransactionType,
} from "@/store/auctions/auctions.types";
import BigNumber from "bignumber.js";

export function transformAuctionsTransaction(
  transaction: {
    transactionType: LiquidityBootstrappingPoolTransactionType;
    receivedTimestamp: string;
    quoteAssetAmount: string;
    baseAssetAmount: string;
    quoteAssetId: string;
    baseAssetId: string;
    spotPrice: string;
    who: string;
    id: string;
  },
  selectedPool: {
    onChainPoolQuoteAssetId: number;
    baseDecimals: BigNumber;
    quoteDecimals: BigNumber;
  }
): PoolTradeHistory {
  const { baseDecimals, quoteDecimals, onChainPoolQuoteAssetId } = selectedPool;
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
    baseAssetAmount = new BigNumber(transaction.baseAssetAmount)
      .div(baseDecimals)
      .toFixed(4);
    quoteAssetAmount = new BigNumber(transaction.quoteAssetAmount)
      .div(quoteDecimals)
      .toFixed(4);
  } else {
    baseAssetAmount = new BigNumber(transaction.baseAssetAmount)
      .div(quoteDecimals)
      .toFixed(4);
    quoteAssetAmount = new BigNumber(transaction.quoteAssetAmount)
      .div(baseDecimals)
      .toFixed(4);
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

export function aggregateTrades(
  swapTxs: PoolTradeHistory[]
): [number, number][] {
  const series = swapTxs
    .map((tx) => {
      return [tx.receivedTimestamp, Number(tx.spotPrice)];
    })
    .sort((p1, p2) => {
      return p1[0] - p2[0];
    }) as [number, number][];

  return series;
}
