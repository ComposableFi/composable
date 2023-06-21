import { OperationResult } from "urql";
import { subsquidClient } from "../client";
import { PabloTransactions } from "../pools/queries";

export function querySpotPriceBeforeTimestamp(
  poolId: number,
  quoteAssetId: number,
  timestamp = Date.now() - 24 * 60 * 60 * 1000,
  orderBy: "ASC" | "DESC" = "DESC"
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
        limit: 1,
        where: {
          pool: {
            poolId_eq: ${poolId},
            quoteAssetId_eq: "${quoteAssetId}",
            calculatedTimestamp_lt: ${timestamp}
          },
          event: {
            eventType_eq: SWAP
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
  `
    )
    .toPromise();
}

export type PabloSpotPriceGQLResponse = {
  pabloSpotPrice: {
    spotPrice: string;
  };
};

export function querySpotPriceFromPool(
  poolId: string,
  quoteAssetId: string,
  baseAssetId: string
) {
  return subsquidClient()
    .query<PabloSpotPriceGQLResponse>(
      `
      query PabloSpotPrice {
  pabloSpotPrice(params: {baseAssetId: "${baseAssetId}", quoteAssetId: "${quoteAssetId}", poolId: "${poolId}"}) {
    spotPrice
  }
  }
  `
    )
    .toPromise();
}

export type PabloPoolAssetsGQLResponse = {
  pabloPoolAssets: {
    assetId: string;
    totalLiquidity: bigint;
    totalVolume: bigint;
  }[];
};

export function queryPabloPoolAssets(poolId: string) {
  return subsquidClient()
    .query<PabloPoolAssetsGQLResponse>(
      `
query PabloPoolAssets {
  pabloPoolAssets(where: {pool: {id_eq: "${poolId}"}}) {
    assetId
    totalLiquidity
    totalVolume
  }
}  
  `
    )
    .toPromise();
}
