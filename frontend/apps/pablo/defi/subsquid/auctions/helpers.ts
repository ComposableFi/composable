import { PabloLiquidityBootstrappingPool } from "shared";
import { fromChainUnits } from "@/defi/utils";
import { PabloTransactions } from "../pools/queries";
import { fetchSubsquid } from "../stakingRewards/helpers";
import { queryAuctionStats } from "./queries";
import BigNumber from "bignumber.js";

export async function fetchAuctionStats(
  pool: PabloLiquidityBootstrappingPool
): Promise<{
  totalLiquidity: BigNumber;
  totalVolume: BigNumber;
}> {
  let totalLiquidity = new BigNumber(0);
  let totalVolume = new BigNumber(0);

  try {
    const queryResponse = await queryAuctionStats((pool.getPoolId(true) as BigNumber).toNumber());
    if (queryResponse.error) throw new Error(queryResponse.error.message);
    if (!queryResponse.data)
      throw new Error(
        "[fetchInitialBalance] Unable to retrieve data from query"
      );

    const { pabloPools } = queryResponse.data;
    if (!pabloPools)
      throw new Error(
        "[fetchInitialBalance] Unable to retrieve data from query"
      );

    if (pabloPools.length) {
      totalLiquidity = fromChainUnits(pabloPools[0].totalLiquidity);
      totalVolume = fromChainUnits(pabloPools[0].totalVolume);
    }
  } catch (err) {
    console.error(err);
  }

  return {
    totalLiquidity,
    totalVolume,
  };
}

export function fetchPabloTransactions(
  poolId: number,
  eventType: "SWAP" | "ADD_LIQUIDITY" | "CREATE_POOL" | "REMOVE_LIQUIDITY",
  orderBy: "ASC" | "DESC" = "DESC",
  limit: number = 50
): Promise<{ pabloTransactions: PabloTransactions[] }> {
  return fetchSubsquid<{ pabloTransactions: PabloTransactions[] }>(`
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
`);
}
