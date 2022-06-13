import { Assets, getAssetById } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import { LiquidityPoolType } from "@/store/pools/pools.types";
import { SwapsChartRange, SwapsSlice } from "@/store/swaps/swaps.types";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import moment from "moment";
import { fetchBalanceByAssetId } from "../balances/utils";
import { DAYS, DEFAULT_DECIMALS, MINUTES } from "../constants";

function takeFeeOnInputToken(poolType: LiquidityPoolType | "none"): boolean {
  return (
    poolType === "ConstantProduct" || poolType === "LiquidityBootstrapping"
  );
}

export interface SwapMetadata {
  quoteAmount: BigNumber;
  baseAssetId: AssetId;
  quoteAssetId: AssetId;
  side: "quote" | "base";
  slippage: number;
}

export async function fetchSpotPrice(
  api: ApiPromise,
  pair: {
    base: number;
    quote: number;
  },
  poolId: number
): Promise<BigNumber> {
  try {
    const quote = getAssetById("picasso", pair.quote);
    const quoteDecimals = quote ? new BigNumber(10).pow(quote.decimals) : DEFAULT_DECIMALS;

    const rpcResult = await (api.rpc as any).pablo.pricesFor(
      api.createType("PalletPabloPoolId", poolId.toString()),
      api.createType("CustomRpcCurrencyId", pair.base.toString()),
      api.createType("CustomRpcCurrencyId", pair.quote.toString()),
      api.createType(
        "CustomRpcBalance",
        new BigNumber(1).times(quoteDecimals).toFixed(0)
      )
    );

    return new BigNumber(rpcResult.toJSON().spotPrice).div(quoteDecimals);
  } catch (err: any) {
    console.error(err);
    return new BigNumber(0);
  }
}

async function calculatePriceImpactLBP(
  api: ApiPromise,
  poolConstants: SwapsSlice["swaps"]["poolConstants"],
  exchange: SwapMetadata,
  tokenOutAmount: BigNumber,
  baseToQuotePrice: BigNumber,
  quoteToBasePrice: BigNumber
): Promise<BigNumber> {
  try {
    if (!poolConstants.lbpConstants) throw new Error("Might not be an LBP");

    const { poolAccountId, pair } = poolConstants;
    const { end, start, initialWeight, finalWeight } =
      poolConstants.lbpConstants;
    const baseAssetReserve = await fetchBalanceByAssetId(
      api,
      "picasso",
      poolAccountId,
      pair.base.toString()
    );
    const quoteAssetReserve = await fetchBalanceByAssetId(
      api,
      "picasso",
      poolAccountId,
      pair.quote.toString()
    );
    let baseAssetReserveBn = new BigNumber(baseAssetReserve);
    let quoteAssetReserveBn = new BigNumber(quoteAssetReserve);

    const { quoteAmount } = exchange;
    const cb = await api.query.system.number();
    const current_block = await cb.toNumber();
    let one = new BigNumber(1);
    let pointInSale = new BigNumber(current_block).div(end - start);
    let weightRange = new BigNumber(initialWeight)
      .div(100)
      .minus(new BigNumber(finalWeight).div(100));
    let baseWeight = new BigNumber(initialWeight)
      .div(100)
      .minus(pointInSale.times(weightRange));
    let quoteWeight = one.minus(baseWeight);

    if (exchange.side === "quote") {
      let num = quoteAssetReserveBn.plus(quoteAmount).div(quoteWeight);
      let denom = baseAssetReserveBn.minus(tokenOutAmount).div(baseWeight);
      let price = num.div(denom);
      return new BigNumber(1).minus(
        new BigNumber(
          /** Need to confirm which price to send here */
          quoteToBasePrice
        ).div(price)
      );
    } else {
      let num = quoteAssetReserveBn.plus(tokenOutAmount).div(quoteWeight);
      let denom = baseAssetReserveBn.minus(quoteAmount).div(baseWeight);
      let price = num.div(denom);
      return new BigNumber(1).minus(
        new BigNumber(
          /** Need to confirm which price to send here */
          baseToQuotePrice
        ).div(price)
      );
    }
  } catch (err: any) {
    console.error(err);
    return new BigNumber(0);
  }
}

export const onSwapAmountChange = async (
  api: ApiPromise,
  exchange: SwapMetadata,
  poolConstants: SwapsSlice["swaps"]["poolConstants"]
): Promise<{
  priceImpact: string;
  minimumRecieved: string;
  tokenOutAmount: string;
}> => {
  let selectedQuoteAsset =
    Assets[exchange.quoteAssetId].supportedNetwork.picasso;
  let selectedBaseAsset = Assets[exchange.baseAssetId].supportedNetwork.picasso;

  let tradeSummary = {
    priceImpact: "0",
    minimumRecieved: "0",
    tokenOutAmount: "0",
  };

  try {
    let oneBaseInQuote = await fetchSpotPrice(
      api,
      poolConstants.pair,
      poolConstants.poolIndex
    );
    let oneQuoteInBase = new BigNumber(1).div(oneBaseInQuote);

    if (!selectedQuoteAsset || !selectedBaseAsset)
      throw new Error("Unable to match pair with Asset");
    if (oneQuoteInBase.lte(0) || oneBaseInQuote.lte(0))
      throw new Error("Price RPC Fail");

    let tokenOut = new BigNumber(0);
    if (exchange.side === "quote") {
      if (selectedQuoteAsset === poolConstants.pair.quote) {
        // same quote as on chain
        tokenOut = exchange.quoteAmount.times(oneQuoteInBase);
      } else {
        // base as quote
        tokenOut = exchange.quoteAmount.times(oneBaseInQuote);
      }
    } else {
      if (selectedQuoteAsset === poolConstants.pair.quote) {
        tokenOut = exchange.quoteAmount.times(oneBaseInQuote);
      } else {
        tokenOut = exchange.quoteAmount.times(oneQuoteInBase);
      }
    }

    let fee = new BigNumber(poolConstants.fee).div(100);
    let minRecieved = new BigNumber(0);

    /**
     * For Balancer and Uniswap pools
     * Fee is deducted on the input tokens
     * e.g in USD -> PICA trade, USD will be
     * deducted as user will provide them for PICA
     */
    if (takeFeeOnInputToken(poolConstants.poolType)) {
      minRecieved = tokenOut.minus(tokenOut.times(exchange.slippage / 100));
      let feeDeductionValue = new BigNumber(0);
      // fee % of token in
      if (exchange.side === "quote") {
        if (selectedQuoteAsset === poolConstants.pair.quote) {
          // same quote asset as on chain, deduct in quote
          feeDeductionValue = fee
            .times(oneQuoteInBase)
            .times(exchange.quoteAmount);
        } else {
          // deduct fee in on chain base
          feeDeductionValue = fee.times(oneBaseInQuote).times(tokenOut);
        }
      } else {
        if (selectedQuoteAsset === poolConstants.pair.quote) {
          feeDeductionValue = fee.times(tokenOut).times(oneQuoteInBase);
        } else {
          feeDeductionValue = fee.times(tokenOut).times(oneBaseInQuote);
        }
      }

      minRecieved = minRecieved.minus(feeDeductionValue);
    }

    tradeSummary.minimumRecieved = minRecieved.toFixed(4);
    tradeSummary.tokenOutAmount = tokenOut.toFixed(4);

    // calculate price impact
    return tradeSummary;
  } catch (err) {
    console.error(err);
    return tradeSummary;
  }
};

// create newer version of this, similar to usePoolTvlChart
export function swapTransactionsToChartSeries(
  transactions: {
    baseAssetId: number;
    quoteAssetId: number;
    receivedTimestamp: number;
    spotPrice: string;
  }[],
  chartRange: SwapsChartRange
): [number, number][] {
  if (!transactions.length) return [];

  if (chartRange === "24h") {
    const latestTxTs = transactions[0].receivedTimestamp;
    let filter = transactions.filter(
      (t) => t.receivedTimestamp > latestTxTs - 1 * DAYS
    );

    const ranges = filter.map((v) => {
      let st = moment(v.receivedTimestamp).startOf("hour").valueOf();
      let en = moment(v.receivedTimestamp).endOf("hour").valueOf();
      return [st, en];
    });

    const startSet = Array.from(new Set(ranges.map((i) => i[0])).values()).sort(
      (a, b) => {
        return a - b;
      }
    );
    const endSet = Array.from(new Set(ranges.map((i) => i[1]))).sort((a, b) => {
      return a - b;
    });

    let series: [number, number][] = [];
    for (let i = 0; i < startSet.length; i++) {
      let spotPrice = new BigNumber(0);
      for (let j = 0; j < filter.length; j++) {
        if (
          filter[j].receivedTimestamp > startSet[i] &&
          filter[j].receivedTimestamp < endSet[i] &&
          spotPrice.lt(filter[j].spotPrice)
        ) {
          spotPrice = new BigNumber(filter[j].spotPrice);
        }
      }
      series.push([startSet[i], spotPrice.toNumber()]);
    }

    return series;
  } else if (chartRange === "1w" || chartRange === "1m") {
    const oldestTxTimeStamp =
      transactions[transactions.length - 1].receivedTimestamp;
    const newestTxTimeStamp = transactions[0].receivedTimestamp;

    let oldestTxStartTs = moment(oldestTxTimeStamp)
      .startOf(chartRange === "1w" ? "week" : "month")
      .valueOf();
    let newestTxStartTs = moment(newestTxTimeStamp).valueOf();

    let series: [number, number][] = [];

    while (oldestTxStartTs < newestTxStartTs) {
      const nextSeriesStart =
        oldestTxStartTs +
        (chartRange === "1w"
          ? 7 * DAYS
          : moment(oldestTxStartTs).endOf("month").valueOf() + 1 * MINUTES);

      let spotPrice = new BigNumber(0);
      transactions.forEach((t) => {
        if (
          t.receivedTimestamp > oldestTxStartTs &&
          t.receivedTimestamp < nextSeriesStart &&
          spotPrice.lt(t.spotPrice)
        ) {
          spotPrice = new BigNumber(t.spotPrice);
        }
      });

      series.push([oldestTxStartTs, spotPrice.toNumber()]);
      oldestTxStartTs = oldestTxStartTs + newestTxStartTs;
    }
    return series;
  } else {
    return [];
  }
}

export function transformSwapSubsquidTx(
  subsquidSwapTxs: {
    baseAssetId: string;
    quoteAssetId: string;
    receivedTimestamp: string;
    spotPrice: string;
  }[],
  selectedQuote: number
): {
  baseAssetId: number;
  quoteAssetId: number;
  receivedTimestamp: number;
  spotPrice: string;
}[] {
  return subsquidSwapTxs.map((tx) => {
    let spotPrice = new BigNumber(tx.spotPrice);
    if (Number(tx.quoteAssetId) !== selectedQuote) {
      spotPrice = new BigNumber(1).div(spotPrice);
    }

    return {
      baseAssetId: Number(tx.baseAssetId),
      quoteAssetId: Number(tx.quoteAssetId),
      receivedTimestamp: Number(tx.receivedTimestamp),
      spotPrice: spotPrice.toString(),
    };
  });
}
