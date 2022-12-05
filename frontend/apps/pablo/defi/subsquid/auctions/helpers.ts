import { PabloLiquidityBootstrappingPool } from "shared";
import { LiquidityBootstrappingPoolTrade } from "@/defi/types/auctions";
import { fromChainUnits } from "@/defi/utils";
import { transformPabloTransaction } from "@/defi/utils/pablo/auctions";
import { PoolTradeHistory } from "@/store/auctions/auctions.types";
import { PabloTransactions } from "../pools/queries";
import { fetchSubsquid } from "../stakingRewards/helpers";
import { queryAuctionStats } from "./queries";
import BigNumber from "bignumber.js";

export async function fetchInitialBalance(
  pool: PabloLiquidityBootstrappingPool
): Promise<{ baseBalance: BigNumber; quoteBalance: BigNumber }> {
  let baseBalance = new BigNumber(0);
  let quoteBalance = new BigNumber(0);

  try {
    const { pabloTransactions } = await fetchPabloTransactions(
      (pool.getPoolId(true) as BigNumber).toNumber(),
      "ADD_LIQUIDITY"
    );

    const addLiqTxs: PoolTradeHistory[] = pabloTransactions.map((t: any) =>
      transformPabloTransaction(t, pool.getPair().getQuoteAsset().toNumber())
    );

    quoteBalance = addLiqTxs.reduce((agg, i) => {
      return agg.plus(i.quoteAssetAmount);
    }, new BigNumber(0));
    baseBalance = addLiqTxs.reduce((agg, i) => {
      return agg.plus(i.baseAssetAmount);
    }, new BigNumber(0));
  } catch (err) {
    console.error(err);
  }

  return {
    baseBalance,
    quoteBalance,
  };
}

export async function fetchAuctionTrades(
  pool: PabloLiquidityBootstrappingPool
): Promise<LiquidityBootstrappingPoolTrade[]> {
  let trades: LiquidityBootstrappingPoolTrade[] = [];
  try {
    const { pabloTransactions } = await fetchPabloTransactions((pool.getPoolId(true) as BigNumber).toNumber(), "SWAP");
    let poolQuote = pool.getPair().getQuoteAsset().toNumber();
    trades = pabloTransactions.map((i: any) =>
      transformPabloTransaction(i, poolQuote)
    );
  } catch (err) {
    console.error(err);
  }

  return trades;
}

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
