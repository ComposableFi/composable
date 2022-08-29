import { makeClient } from "../makeClient";

export const query24hOldTransactionByPoolQuoteAsset = (
    poolId: number,
    quoteAsset: number,
    transactionType = "SWAP",
    limit: number = 1
  ) => makeClient().query(`query subsquidLiquidityPool24HourOldQuoteAssetPrice {
    pabloTransactions(limit: ${limit}, orderBy: receivedTimestamp_DESC, where: {
      receivedTimestamp_lt: ${Date.now() - 24 * 60 * 60 * 1000},
      transactionType_eq: ${transactionType},
      pool: {poolId_eq: ${poolId.toString()}, quoteAssetId_eq: ${quoteAsset}}
    }) {
      spotPrice
    }
  }`).toPromise();