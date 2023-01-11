import { OperationResult } from "@urql/core";
import { subsquidClient } from "../client";
import { tryCatch } from "fp-ts/TaskEither";
import { fetchSubsquid } from "@/defi/subsquid/stakingRewards/helpers";
import { Range } from "@/defi/subsquid/overview";

export interface PabloTransactions {
  id: string;
  spotPrice: string;
  baseAssetId: string;
  baseAssetAmount: string;
  quoteAssetAmount: string;
  quoteAssetId: string;
  fee: string;
  pool: {
    calculatedTimestamp: string;
  };
  event: {
    eventType: "ADD_LIQUIDITY" | "REMOVE_LIQUIDITY";
    accountId: string;
    blockNumber: string;
  };
}

export function queryUserProvidedLiquidity(
  poolId: number,
  orderBy: "ASC" | "DESC" = "DESC",
  limit: number = 50,
  accountId: string
): Promise<
  OperationResult<
    {
      pabloTransactions: PabloTransactions[];
    },
    {}
  >
> {
  return subsquidClient()
    .query(
      `
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
  `
    )
    .toPromise();
}

export const queryPabloPoolById = (poolId: number) =>
  subsquidClient()
    .query(
      `query queryPabloPoolById {
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
`
    )
    .toPromise();

const queryPabloDaily = (poolId: string) => `
query pablo24hStatsForPool {
  pabloDaily(params: {poolId: "${poolId}"}) {
    assetId
    fees
    transactions
    volume
  }
}
`;
export type PabloDaily = {
  fees: string;
  assetId: string;
  transactions: string;
  volume: string;
};

export function fetchPabloDailyForPool(poolId: string) {
  return () =>
    tryCatch(
      async () =>
        await fetchSubsquid<{ pabloDaily: PabloDaily }>(
          queryPabloDaily(poolId),
          true
        ),
      () => ({
        pabloDaily: {
          fees: "0",
          transactions: "0",
          assetId: "1",
          volume: "0",
        } as PabloDaily,
      })
    );
}

export function fetchPabloTVLChartForPool(poolId: string, range: Range) {
  return () =>
    tryCatch(
      async () =>
        await fetchSubsquid<PabloPoolTVLChart>(
          queryPabloPoolTVLChart(poolId, range),
          false
        ),
      () => ({
        pabloTVL: [],
      })
    );
}

export type PabloPoolTVLChart = {
  pabloTVL: {
    assetId: string;
    date: string;
    totalValueLocked: string;
  }[];
};

const queryPabloPoolTVLChart = (poolId: string, range: Range) => ` 
query totalValueLockedChartForPool {
  pabloTVL(params: {range: "${range}", poolId: "${poolId}"}) {
    assetId
    totalValueLocked
    date
  }
}
`;
