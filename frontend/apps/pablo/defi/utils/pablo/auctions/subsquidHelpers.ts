import {
  fetchAuctionStats,
  fetchInitialBalance,
  fetchPabloTransactions,
} from "@/defi/subsquid/auctions/helpers";
import {
  PabloTransactions,
} from "@/defi/subsquid/pools/queries";
import { LiquidityBootstrappingPool } from "@/defi/types";
import { LiquidityBootstrappingPoolTrade } from "@/defi/types/auctions";
import { LiquidityBootstrappingPoolStatistics } from "@/store/auctions/auctions.types";
import { ApiPromise } from "@polkadot/api";
import { fetchBalanceByAssetId } from "../../assets";
import { fromChainUnits } from "../../units";
import { createPabloPoolAccountId } from "../misc";
import { lbpCalculatePriceAtBlock } from "./weightCalculator";
import BigNumber from "bignumber.js";
import moment from "moment";

export function transformPabloTransaction(
  tx: PabloTransactions,
  poolQuoteAssetId: number
): LiquidityBootstrappingPoolTrade {
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
  let startLiquidity = {
    baseAmount: new BigNumber(0),
    quoteAmount: new BigNumber(0),
  };
  let liquidity = {
    baseAmount: new BigNumber(0),
    quoteAmount: new BigNumber(0),
  };
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
    if (!parachainApi || !auction || auction.poolId == -1)
      throw new Error("Cannot fetch data.");

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

    const { pabloTransactions } = await fetchPabloTransactions(
      auction.poolId,
      "SWAP",
      "ASC",
      100
    );

    const transactions = pabloTransactions.map((tx) =>
      transformPabloTransaction(tx, auction.pair.quote)
    );

    if (transactions.length) {
      chartSeries = transactions.map(({ receivedTimestamp, spotPrice }) => [
        receivedTimestamp,
        Number(spotPrice),
      ]);
      let predictedSeriesStartTimeStamp =
        transactions[transactions.length - 1].receivedTimestamp;
      let predictedSeriesStartSpotPrice = new BigNumber(
        transactions[transactions.length - 1].spotPrice
      );
      let predictedSeriesStartBlock = new BigNumber(
        pabloTransactions[transactions.length - 1].event.blockNumber
      );

      while (predictedSeriesStartTimeStamp < auction.sale.end) {
        predictedSeriesStartSpotPrice = lbpCalculatePriceAtBlock(
          auction,
          new BigNumber(baseBalance),
          new BigNumber(quoteBalance),
          predictedSeriesStartBlock
        );
        predictedSeries.push([
          predictedSeriesStartTimeStamp,
          predictedSeriesStartSpotPrice.toNumber(),
        ]);
        predictedSeriesStartBlock = predictedSeriesStartBlock.plus(5);
        predictedSeriesStartTimeStamp += 60 * 1000;
      }
    } else {
      let blockBn = await parachainApi.query.system.number();
      let predictedSeriesStartBlock = new BigNumber(blockBn.toString());
      let predictedSeriesStartTimeStamp = Date.now();
      let predictedSeriesStartSpotPrice = lbpCalculatePriceAtBlock(
        auction,
        new BigNumber(baseBalance),
        new BigNumber(quoteBalance),
        predictedSeriesStartBlock
      );

      while (predictedSeriesStartTimeStamp < auction.sale.end) {
        predictedSeriesStartSpotPrice = lbpCalculatePriceAtBlock(
          auction,
          new BigNumber(baseBalance),
          new BigNumber(quoteBalance),
          predictedSeriesStartBlock
        );
        predictedSeries.push([
          predictedSeriesStartTimeStamp,
          predictedSeriesStartSpotPrice.toNumber(),
        ]);
        // add 5 blocks per second
        predictedSeriesStartBlock = predictedSeriesStartBlock.plus(5);
        predictedSeriesStartTimeStamp += 60 * 1000;
      }
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
