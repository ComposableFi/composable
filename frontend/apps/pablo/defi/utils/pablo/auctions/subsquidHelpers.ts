import { fetchAuctionStats, fetchInitialBalance } from "@/defi/subsquid/auctions/helpers";
import { PabloTransactions, queryPabloTransactions } from "@/defi/subsquid/pools/queries";
import {
  LiquidityBootstrappingPool,
  LiquidityPoolTransactionType,
} from "@/defi/types";
import { LiquidityBootstrappingPoolTrade } from "@/defi/types/auctions";
import { LiquidityBootstrappingPoolStatistics } from "@/store/auctions/auctions.types";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fetchBalanceByAssetId } from "../../assets";
import { AVERAGE_BLOCK_TIME } from "../../constants";
import { fromChainUnits } from "../../units";
import { createPabloPoolAccountId } from "../misc";
import { lbpCalculatePriceAtBlock } from "./weightCalculator";

export function transformPabloTransaction(tx: PabloTransactions, poolQuoteAssetId: number): LiquidityBootstrappingPoolTrade {
  const baseAssetId = Number(tx.baseAssetId);
  const quoteAssetId = Number(tx.quoteAssetId);

  let spotPrice: string = new BigNumber(tx.spotPrice).toString();
  let baseAssetAmount: BigNumber | string = new BigNumber(0);
  let quoteAssetAmount: BigNumber | string = new BigNumber(0);
  let receivedTimestamp = Number(tx.pool.calculatedTimestamp);
  let blockNumber = new BigNumber(tx.event.blockNumber);
  let id = tx.id;
  let walletAddress = tx.event.accountId;
  let side: any = "SELL";

  if (quoteAssetId === poolQuoteAssetId) {
    side = "BUY";
    baseAssetAmount = fromChainUnits(tx.baseAssetAmount).toString();
    quoteAssetAmount = fromChainUnits(tx.quoteAssetAmount).toString();
  } else {
    baseAssetAmount = fromChainUnits(tx.baseAssetAmount).toString();
    quoteAssetAmount = fromChainUnits(tx.quoteAssetAmount).toString();
    spotPrice = new BigNumber(1).div(new BigNumber(spotPrice)).toString();
  }

  return {
    baseAssetId,
    baseAssetAmount,
    id,
    quoteAssetAmount,
    quoteAssetId,
    receivedTimestamp,
    spotPrice: spotPrice,
    side,
    walletAddress,
    blockNumber,
  };
}

export async function fetchAndExtractAuctionStats(
  api: ApiPromise,
  pool: LiquidityBootstrappingPool
): Promise<LiquidityBootstrappingPoolStatistics> {
  let startLiquidity = { baseAmount: new BigNumber(0), quoteAmount: new BigNumber(0) };
  let liquidity = { baseAmount: new BigNumber(0), quoteAmount: new BigNumber(0) };
  let totalVolume = new BigNumber(0);
  let totalLiquidity = new BigNumber(0);

  const { base, quote } = pool.pair;
  const poolAccountId = createPabloPoolAccountId(api, pool.poolId);
  try {
    /**
     * Query for volume, liquidity
     */
    const stats = await fetchAuctionStats(pool);
    totalVolume = stats.totalVolume;
    totalLiquidity = stats.totalLiquidity;
    /**
     * Query trade history
     * for transactions tab
     */
    /**
     * Query for initial balances
     */
    const initialBalances = await fetchInitialBalance(pool);
    startLiquidity.baseAmount = initialBalances.baseBalance;
    startLiquidity.quoteAmount = initialBalances.quoteBalance;
    /**
     * Query amount of base tokens in
     * the pool
     */
    const baseCurrBalance = await fetchBalanceByAssetId(
      api,
      poolAccountId,
      base.toString()
    );
    liquidity.baseAmount = new BigNumber(baseCurrBalance);
    /**
     * Query amount of quote tokens in
     * the pool
     */
    const quoteCurrBalance = await fetchBalanceByAssetId(
      api,
      poolAccountId,
      quote.toString()
    );
    liquidity.quoteAmount = new BigNumber(quoteCurrBalance);
  } catch (err) {
    console.error(err);
  }
  return {
    totalLiquidity,
    startLiquidity,
    liquidity,
    totalVolume,
  };
}

export async function fetchAuctionChartSeries(
  parachainApi?: ApiPromise,
  auction?: LiquidityBootstrappingPool
): Promise<{
  chartSeries: [number, number][];
  predictedSeries: [number, number][];
}> {
  let chartSeries: [number, number][] = [];
  let predictedSeries: [number, number][] = [];
  try {
    if (!parachainApi || !auction || auction.poolId == -1) throw new Error('Cannot fetch data.');

    const subsquidResponse = await queryPabloTransactions(
      auction.poolId,
      "SWAP",
      "ASC",
      100
    );

    const { error, data } = subsquidResponse;

    if (error) throw new Error(error.message);
    if (!data || !data.pabloTransactions)
      throw new Error("Unable to retrieve chart data.");

    const { pabloTransactions } = data;

    chartSeries = pabloTransactions
      .map((t: any) => transformPabloTransaction(t, auction.pair.quote))
      .map((i: LiquidityBootstrappingPoolTrade) => {
        return [i.receivedTimestamp, Number(i.spotPrice)];
      }) as [number, number][];

    const accountId = createPabloPoolAccountId(parachainApi, auction.poolId);
    const baseBalance = await fetchBalanceByAssetId(
      parachainApi,
      accountId,
      auction.pair.base.toString()
    );
    const quoteBalance = await fetchBalanceByAssetId(
      parachainApi,
      accountId,
      auction.pair.quote.toString()
    );

    if (chartSeries.length > 0) {
      let lastTimeStamp = chartSeries[chartSeries.length - 1][0];
      let lastPrice = chartSeries[chartSeries.length - 1][1];
      predictedSeries.push([lastTimeStamp, lastPrice])
    }
  
    const block = await parachainApi.query.system.number();
    let blockIter = new BigNumber(block.toString());
    let ts = Date.now();

    while (ts < auction.sale.end) {
      const price = lbpCalculatePriceAtBlock(
        auction,
        new BigNumber(baseBalance),
        new BigNumber(quoteBalance),
        blockIter
      );

      predictedSeries.push([ts, price.toNumber()]);
      ts += 5 * 1000;
      blockIter = blockIter.plus(1);
    }
  } catch (err) {
    console.error(err);
  } finally {
    return {
      chartSeries,
      predictedSeries,
    };
  }

}
