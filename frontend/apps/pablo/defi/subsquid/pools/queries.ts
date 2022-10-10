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
    eventType: "ADD_LIQUIDITY" | "REMOVE_LIQUIDITY";
    accountId: string;
    blockNumber: string;
  }
}

export function queryUserProvidedLiquidity(
  poolId: number,
  orderBy: "ASC" | "DESC" = "DESC",
  limit: number = 50,
  accountId: string
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
            eventType_in: [ADD_LIQUIDITY,REMOVE_LIQUIDITY],
            accountId_eq: "${accountId}"
          },
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
          eventType,
          accountId,
          blockNumber
        }
      }
    }
  `).toPromise();
}

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