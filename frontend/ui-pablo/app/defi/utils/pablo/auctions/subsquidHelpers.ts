import {
  LiquidityBootstrappingPool,
  LiquidityPoolTransactionType,
} from "@/defi/types";
import { LiquidityBootstrappingPoolTrade } from "@/defi/types/auctions";
import { PoolTradeHistory } from "@/store/auctions/auctions.types";
import { queryAuctionStats } from "@/subsquid/queries/auctions";
import { queryPoolTransactionsByType } from "@/subsquid/queries/pools";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fetchBalanceByAssetId } from "../../assets";
import { AVERAGE_BLOCK_TIME } from "../../constants";
import { fromChainUnits } from "../../units";
import { createPabloPoolAccountId } from "../misc";
import { lbpCalculatePriceAtBlock } from "./weightCalculator";

export function transformAuctionsTransaction(
  transaction: {
    transactionType: LiquidityPoolTransactionType;
    receivedTimestamp: string;
    quoteAssetAmount: string;
    baseAssetAmount: string;
    quoteAssetId: string;
    blockNumber: string;
    baseAssetId: string;
    spotPrice: string;
    who: string;
    id: string;
  },
  onChainPoolQuoteAssetId: number
): LiquidityBootstrappingPoolTrade {
  const baseAssetId = Number(transaction.baseAssetId);
  const quoteAssetId = Number(transaction.quoteAssetId);

  let spotPrice: string = new BigNumber(transaction.spotPrice).toString();
  let baseAssetAmount: BigNumber | string = new BigNumber(0);
  let quoteAssetAmount: BigNumber | string = new BigNumber(0);
  let receivedTimestamp = Number(transaction.receivedTimestamp);
  let blockNumber = new BigNumber(transaction.blockNumber);
  let id = transaction.id;
  let walletAddress = transaction.who;
  let side: any = "SELL";

  if (quoteAssetId === onChainPoolQuoteAssetId) {
    side = "BUY";
    baseAssetAmount = fromChainUnits(transaction.baseAssetAmount).toString();
    quoteAssetAmount = fromChainUnits(transaction.quoteAssetAmount).toString();
  } else {
    baseAssetAmount = fromChainUnits(transaction.baseAssetAmount).toString();
    quoteAssetAmount = fromChainUnits(transaction.quoteAssetAmount).toString();
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
        "[fetchInitialBalance] Unable to retreive data from query"
      );

    const { pabloTransactions } = queryResponse.data;
    if (!pabloTransactions)
      throw new Error(
        "[fetchInitialBalance] Unable to retreive data from query"
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

export async function fetchTrades(
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
        "[fetchInitialBalance] Unable to retreive data from query"
      );

    const { pabloTransactions } = queryResponse.data;
    if (!pabloTransactions)
      throw new Error(
        "[fetchInitialBalance] Unable to retreive data from query"
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
        "[fetchInitialBalance] Unable to retreive data from query"
      );

    const { pabloPools } = queryResponse.data;
    if (!pabloPools)
      throw new Error(
        "[fetchInitialBalance] Unable to retreive data from query"
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

export async function fetchAuctions(
  api: ApiPromise,
  pool: LiquidityBootstrappingPool
): Promise<{
  startBalances: {
    quote: string;
    base: string;
  };
  currentBalances: {
    quote: string;
    base: string;
  };
  liquidity: string;
  totalVolume: string;
}> {
  let startBalances = { base: "0", quote: "0" };
  let currentBalances = { base: "0", quote: "0" };
  let liquidity = "0";
  let totalVolume = "0";
  const { base, quote } = pool.pair;
  const poolAccountId = createPabloPoolAccountId(api, pool.poolId);
  try {
    /**
     * Query for volume, liquidity
     */
    const stats = await fetchLbpStats(pool);
    totalVolume = stats.totalVolume.toString();
    liquidity = stats.totalLiquidity.toString();
    /**
     * Query trade history
     * for transactions tab
     */
    /**
     * Query for initial balances
     */
    const initialBalances = await fetchInitialBalance(pool);
    startBalances.base = initialBalances.baseBalance.toString();
    startBalances.quote = initialBalances.quoteBalance.toString();
    /**
     * Query amount of base tokens in
     * the pool
     */
    const baseCurrBalance = await fetchBalanceByAssetId(
      api,
      poolAccountId,
      base.toString()
    );
    currentBalances.base = baseCurrBalance;
    /**
     * Query amount of quote tokens in
     * the pool
     */
    const quoteCurrBalance = await fetchBalanceByAssetId(
      api,
      poolAccountId,
      quote.toString()
    );
    currentBalances.quote = quoteCurrBalance;
  } catch (err) {
    console.error(err);
  }
  return {
    startBalances,
    currentBalances,
    liquidity,
    totalVolume,
  };
}

export async function fetchAuctionChartSeries(
  parachainApi: ApiPromise,
  auction: LiquidityBootstrappingPool
): Promise<{
  chartSeries: [number, number][];
  predictedSeries: [number, number][];
}> {
  let chartSeries: [number, number][] = [];
  let predictedSeries: [number, number][] = [];
  try {
    const subsquidResponse = await queryPoolTransactionsByType(
      auction.poolId,
      "SWAP",
      100,
      "ASC"
    );

    const { error, data } = subsquidResponse;

    if (error) throw new Error(error.message);
    if (!data || !data.pabloTransactions)
      throw new Error("Unable to retrieve chart data.");

    const { pabloTransactions } = data;

    chartSeries = pabloTransactions
      .map((t: any) => transformAuctionsTransaction(t, auction.pair.quote))
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
      ts += AVERAGE_BLOCK_TIME * 1000;
      blockIter = blockIter.plus(1000);
    }
  } catch (err) {
    console.error(err);
  }
  return {
    chartSeries,
    predictedSeries,
  };
}
