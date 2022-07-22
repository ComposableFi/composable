import { LiquidityBootstrappingPool } from "@/defi/types";
import { LiquidityBootstrappingPoolTrade } from "@/defi/types/auctions";
import { fromChainUnits } from "@/defi/utils";
import { transformAuctionsTransaction } from "@/defi/utils/pablo/auctions";
import { PoolTradeHistory } from "@/store/auctions/auctions.types";
import BigNumber from "bignumber.js";
import { queryPoolTransactionsByType } from "../pools/queries";
import { queryAuctionStats } from "./queries";

export async function fetchInitialBalance(
  pool: LiquidityBootstrappingPool
): Promise<{ baseBalance: BigNumber; quoteBalance: BigNumber }> {
  let baseBalance = new BigNumber(0);
  let quoteBalance = new BigNumber(0);

  try {
    const queryResponse = await queryPoolTransactionsByType(
      pool.poolId,
      "ADD_LIQUIDITY"
    );
    if (queryResponse.error) throw new Error(queryResponse.error.message);
    if (!queryResponse.data)
      throw new Error(
        "[fetchInitialBalance] Unable to retrieve data from query"
      );

    const { pabloTransactions } = queryResponse.data;
    if (!pabloTransactions)
      throw new Error(
        "[fetchInitialBalance] Unable to retrieve data from query"
      );

    const addLiqTxs: PoolTradeHistory[] = pabloTransactions.map((t: any) =>
      transformAuctionsTransaction(t, pool.pair.quote)
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
  pool: LiquidityBootstrappingPool
): Promise<LiquidityBootstrappingPoolTrade[]> {
  let trades: LiquidityBootstrappingPoolTrade[] = [];

  try {
    const queryResponse = await queryPoolTransactionsByType(
      pool.poolId,
      "SWAP"
    );
    // map to a function later
    if (queryResponse.error) throw new Error(queryResponse.error.message);
    if (!queryResponse.data)
      throw new Error(
        "[fetchInitialBalance] Unable to retrieve data from query"
      );

    const { pabloTransactions } = queryResponse.data;
    if (!pabloTransactions)
      throw new Error(
        "[fetchInitialBalance] Unable to retrieve data from query"
      );

    let poolQuote = pool.pair.quote;
    trades = pabloTransactions.map((i: any) =>
      transformAuctionsTransaction(i, poolQuote)
    );
  } catch (err) {
    console.error(err);
  }

  return trades;
}

export async function fetchLbpStats(pool: LiquidityBootstrappingPool): Promise<{
  totalLiquidity: BigNumber;
  totalVolume: BigNumber;
}> {
  let totalLiquidity = new BigNumber(0);
  let totalVolume = new BigNumber(0);

  try {
    const queryResponse = await queryAuctionStats(pool.poolId);
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
