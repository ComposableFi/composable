export const queryPoolTransactionsByType = (
    poolId: number,
    transactionType: "SWAP" | "ADD_LIQUIDITY" | "CREATE_POOL",
    limit: number = 50
  ) => `
      query subsquidLiquidityPoolLatestTransactionsQuery {
        pabloTransactions(limit: ${limit}, orderBy: receivedTimestamp_DESC, where: {
          transactionType_eq: ${transactionType},
          pool: {poolId_eq: ${poolId.toString()}}
        }) {
          id
          spotPrice
          baseAssetId
          baseAssetAmount
          quoteAssetAmount
          quoteAssetId
          receivedTimestamp
          who
        }
      }
    `;
  