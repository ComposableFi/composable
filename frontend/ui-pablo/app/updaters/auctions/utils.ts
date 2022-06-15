import { PoolTradeHistory } from "@/store/auctions/auctions.types";

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
