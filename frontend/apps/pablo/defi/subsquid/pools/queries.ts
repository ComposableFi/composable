import { OperationResult } from "@urql/core";
import { makeClient } from "../makeClient";

export interface PabloTransactions {
  id: string;
  spotPrice:string;
  baseAssetId: string;
  baseAssetAmount: string;
  quoteAssetAmount: string;
  quoteAssetId: string;
  fee: string;
  pool: {
    calculatedTimestamp: string;
  },
  event: {
    accountId: string,
    blockNumber: string
  }
}

export function queryPabloTransactions(
  poolId: number,
  eventType: "SWAP" | "ADD_LIQUIDITY" | "CREATE_POOL" | "REMOVE_LIQUIDITY",
  orderBy: "ASC" | "DESC" = "DESC",
  limit: number = 50
): Promise<OperationResult<{
  pabloTransactions: PabloTransactions[]
}, {}>> {
  return makeClient().query(`
    query pabloTransactions {
      pabloTransactions (
        limit: ${limit},
        where: {
          pool: {
            poolId_eq: ${poolId}
          },
          event: {
            eventType_eq: ${eventType}
          }
        },
        orderBy: pool_calculatedTimestamp_${orderBy}
      ) {
        id
        spotPrice
        baseAssetId
        baseAssetAmount
        quoteAssetAmount
        quoteAssetId
        fee
        pool {
          calculatedTimestamp
        }
        event {
          accountId,
          blockNumber
        }
      }
    }
  `).toPromise();
}


export const queryPoolTransactionsByType = (
  poolId: number,
  transactionType: "SWAP" | "ADD_LIQUIDITY" | "CREATE_POOL" | "REMOVE_LIQUIDITY",
  limit: number = 50,
  orderBy: "ASC" | "DESC" = "DESC"
) => makeClient().query(`query queryPoolTransactionsByType {
  pabloTransactions(limit: ${limit}, orderBy: receivedTimestamp_${orderBy}, where: {
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
    blockNumber
  }
}`).toPromise();

export const liquidityTransactionsByAddressAndPool = (
  who: string,
  poolId: number | string
) => makeClient().query(`query queryAddOrRemoveLiquidityTransactionsByUserAddress {
  pabloTransactions(
    orderBy: receivedTimestamp_ASC,where: {
        who_eq: "${who}",
				transactionType_in: [ADD_LIQUIDITY,REMOVE_LIQUIDITY],
        pool: {
          poolId_eq: ${poolId}
        }
  }) {
    baseAssetId
    baseAssetAmount
    quoteAssetAmount
    quoteAssetId
    receivedTimestamp
    transactionType
    who
    pool {
      poolId
    }
  }
}`).toPromise();


export const queryPabloPoolById = (poolId: number) => makeClient().query(`query queryPabloPoolById {
  pabloPools(orderBy: calculatedTimestamp_DESC, where: {poolId_eq: ${poolId}}) {
    totalLiquidity
    totalVolume
    transactionCount
    totalFees
    calculatedTimestamp
    quoteAssetId
    poolId
  }
}
`).toPromise()