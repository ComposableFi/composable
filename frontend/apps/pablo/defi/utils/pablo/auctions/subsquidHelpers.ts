import {
  fetchAuctionStats,
  fetchInitialBalance,
  fetchPabloTransactions,
} from "@/defi/subsquid/auctions/helpers";
import {
  PabloTransactions,
} from "@/defi/subsquid/pools/queries";
import { LiquidityBootstrappingPoolTrade } from "@/defi/types/auctions";
import { LiquidityBootstrappingPoolStatistics } from "@/store/auctions/auctions.types";
import { fromChainUnits } from "../../units";
import { PabloLiquidityBootstrappingPool } from "@/../../packages/shared";
import { AVERAGE_BLOCK_TIME } from "../../constants";
import BigNumber from "bignumber.js";

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
  pool: PabloLiquidityBootstrappingPool
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

  const base = pool.getPair().getBaseAsset();
  const quote = pool.getPair().getQuoteAsset();
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
    const baseCurrBalance = await pool.getAssetLiquidity(base);
    liquidity.baseAmount = new BigNumber(baseCurrBalance);
    /**
     * Query amount of quote tokens in
     * the pool
     */
    const quoteCurrBalance = await pool.getAssetLiquidity(quote);
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
  auction: PabloLiquidityBootstrappingPool | null
): Promise<{
  chartSeries: [number, number][];
  predictedSeries: [number, number][];
}> {
  let chartSeries: [number, number][] = [];
  let predictedSeries: [number, number][] = [];
  try {
    if (!auction)
      throw new Error("Cannot fetch data.");

    const api = auction.getApi();
    const baseId = auction.getPair().getBaseAsset();
    const quoteId = auction.getPair().getQuoteAsset();
    const baseBalance = await auction.getAssetLiquidity(baseId);
    const quoteBalance = await auction.getAssetLiquidity(quoteId);
    let blockBn = await api.query.system.number();
    const { endTimestamp } = await auction.getSaleTiming(
      new BigNumber(blockBn.toString()),
      new BigNumber(AVERAGE_BLOCK_TIME)
    );

    const { pabloTransactions } = await fetchPabloTransactions(
      (auction.getPoolId(true) as BigNumber).toNumber(),
      "SWAP",
      "ASC",
      100
    );

    const transactions = pabloTransactions.map((tx) =>
      transformPabloTransaction(tx, quoteId.toNumber())
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

      while (predictedSeriesStartTimeStamp < endTimestamp) {
        predictedSeriesStartSpotPrice = auction.simulatePriceAt(
          predictedSeriesStartBlock,
          baseBalance,
          quoteBalance
        );
        predictedSeries.push([
          predictedSeriesStartTimeStamp,
          predictedSeriesStartSpotPrice.toNumber(),
        ]);
        predictedSeriesStartBlock = predictedSeriesStartBlock.plus(5);
        predictedSeriesStartTimeStamp += 60 * 1000;
      }
    } else {

      let predictedSeriesStartBlock = new BigNumber(blockBn.toString());
      let predictedSeriesStartTimeStamp = Date.now();
      let predictedSeriesStartSpotPrice = auction.simulatePriceAt(
        predictedSeriesStartBlock,
        baseBalance,
        quoteBalance
      );

      while (predictedSeriesStartTimeStamp < endTimestamp) {
        predictedSeriesStartSpotPrice = auction.simulatePriceAt(
          predictedSeriesStartBlock,
          baseBalance,
          quoteBalance,
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
