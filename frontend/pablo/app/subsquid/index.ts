import { createClient } from 'urql';

export const subsquidLiquidityPoolStatsQuery = (
  poolId: number
) => `query subsquidLiquidityPoolStatsQuery {
  pabloPools(orderBy: calculatedTimestamp_DESC, where: {poolId_eq: ${poolId.toString()}}) {
    id
    totalVolume
    totalLiquidity
  }
}`;

export const subsquidLiquidityPoolLatestTransactionsQuery = (
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

export const subsquidLiquidityPoolChartPricesQuery = (
  poolId: number,
  transactionType: "SWAP" | "ADD_LIQUIDITY" | "CREATE_POOL",
  limit: number = 50
) => `
  query subsquidLiquidityPoolLatestTransactionsQuery {
    pabloTransactions(limit: ${limit}, orderBy: receivedTimestamp_DESC, where: {
      transactionType_eq: ${transactionType},
      pool: {poolId_eq: ${poolId.toString()}}
    }) {
      spotPrice
      baseAssetId
      quoteAssetId
      receivedTimestamp
    }
  }
`;

export const subsquidLiquidityPool24HourOldQuoteAssetPrice = (
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

export const subsquidClient = createClient({
  url: process.env.SUBSQUID_URL || "",
});