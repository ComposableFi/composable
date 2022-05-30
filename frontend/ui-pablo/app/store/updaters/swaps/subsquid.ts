export const query24hOldTransactionByPoolQuoteAsset = (
    poolId: number,
    quoteAsset: number,
    limit: number = 1
  ) => `
    query subsquidLiquidityPool24HourOldQuoteAssetPrice {
      pabloTransactions(limit: ${limit}, orderBy: receivedTimestamp_DESC, where: {
        receivedTimestamp_lt: ${Date.now() - 24 * 60 * 60 * 1000},
        transactionType_eq: SWAP,
        pool: {poolId_eq: ${poolId.toString()}, quoteAssetId_eq: ${quoteAsset}}
      }) {
        spotPrice
      }
    }
  `;