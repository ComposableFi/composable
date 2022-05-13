import {
  PoolTradeHistory,
  LiquidityBootstrappingPoolTransactionType,
} from "@/store/auctions/auctions.types";
import BigNumber from "bignumber.js";

export function transformAuctionsTransaction(
  tradeItem: {
    baseAssetAmount: string;
    baseAssetId: string;
    id: string;
    quoteAssetAmount: string;
    quoteAssetId: string;
    receivedTimestamp: string;
    spotPrice: string;
    transactionType: LiquidityBootstrappingPoolTransactionType;
    who: string;
  },
  assetMetadata: {
    baseDecimals: BigNumber;
    quoteDecimals: BigNumber;
    onChainPoolQuoteAssetId: number;
  }
): PoolTradeHistory {
  const baseAssetId = Number(tradeItem.baseAssetId);
  const quoteAssetId = Number(tradeItem.quoteAssetId);
  const { baseDecimals, quoteDecimals, onChainPoolQuoteAssetId } =
    assetMetadata;

  let spotPrice: string = new BigNumber(tradeItem.spotPrice).toString()
  let side: any = "SELL";
  if (quoteAssetId === onChainPoolQuoteAssetId) {
    side = "BUY";
    spotPrice = new BigNumber(1).div(new BigNumber(spotPrice)).toString()
  }

  return {
    baseAssetId,
    baseAssetAmount: new BigNumber(tradeItem.baseAssetAmount)
      .div(side === "SELL" ? quoteDecimals : baseDecimals)
      .toFixed(4),
    id: tradeItem.id,
    quoteAssetAmount: new BigNumber(tradeItem.quoteAssetAmount)
      .div(side === "SELL" ? baseDecimals : quoteDecimals)
      .toFixed(4),
    quoteAssetId,
    receivedTimestamp: Number(tradeItem.receivedTimestamp),
    spotPrice: spotPrice,
    side,
    walletAddress: tradeItem.who,
  };
}

const SECONDS = 60 * 1000;
const MINUTES = 60 * SECONDS;
const HOURS = 60 * MINUTES;
const DAYS = 24 * HOURS;
const YEARS = 365 * DAYS;

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
