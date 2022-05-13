import { Assets, getAssetById } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import {
  LiquidityPoolType,
  SwapsChartRange,
  SwapsSlice,
} from "@/store/swaps/swaps.types";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import moment from "moment";
import { retrieveAssetBalance } from "../balances/Updater";

function takeFeeOnInputToken(poolType: LiquidityPoolType | "none"): boolean {
  return poolType === "Balancer" || poolType === "Uniswap";
}
export interface SwapMetadata {
  quoteAmount: BigNumber;
  baseAssetId: AssetId;
  quoteAssetId: AssetId;
  side: "quote" | "base";
  slippage: number;
}

export async function retrieveSpotPrice(
  api: ApiPromise,
  pair: {
    base: number;
    quote: number;
  },
  poolId: number
): Promise<BigNumber> {
  try {
    const quote = getAssetById("picasso", pair.quote);
    const base = getAssetById("picasso", pair.base);
    const quoteDecimals = quote ? new BigNumber(10).pow(quote.decimals) : 12;
    const baseDecimals = base ? new BigNumber(10).pow(base.decimals) : 12;
    const rpcResult = await (api.rpc as any).pablo.pricesFor(
      api.createType("PalletPabloPoolId", poolId.toString()),
      api.createType("CustomRpcCurrencyId", pair.base.toString()),
      api.createType("CustomRpcCurrencyId", pair.quote.toString()),
      api.createType(
        "CustomRpcBalance",
        new BigNumber(1).times(quoteDecimals).toFixed(0)
      )
    );

    return new BigNumber(rpcResult.toJSON().spotPrice).div(baseDecimals);
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
    const baseAssetReserve = await retrieveAssetBalance(
      api,
      "picasso",
      poolAccountId,
      pair.base.toString()
    );
    const quoteAssetReserve = await retrieveAssetBalance(
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
    let oneBaseInQuote = await retrieveSpotPrice(
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

export function createChartSeries(
  txs: {
    baseAssetId: number;
    quoteAssetId: number;
    receivedTimestamp: number;
    spotPrice: string;
  }[],
  chartRange: SwapsChartRange
): [number, number][] {
  if (!txs.length) return [];

  switch (chartRange) {
    case "24h":
      const latestTxTs = txs[0].receivedTimestamp;
      let filter = txs.filter(
        (t) => t.receivedTimestamp > latestTxTs - 24 * 60 * 60 * 1000
      );

      const ranges = filter.map((v) => {
        let st = moment(v.receivedTimestamp).startOf("hour").valueOf();
        let en = moment(v.receivedTimestamp).endOf("hour").valueOf();
        return [st, en];
      });

      const startSet = Array.from(
        new Set(ranges.map((i) => i[0])).values()
      ).sort((a, b) => {
        return a - b;
      });
      const endSet = Array.from(new Set(ranges.map((i) => i[1]))).sort(
        (a, b) => {
          return a - b;
        }
      );

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

    case "1w":
      const oldestTxTimeStamp = txs[txs.length - 1].receivedTimestamp;
      const newestTxTimeStamp = txs[0].receivedTimestamp;
      let oldestTxStartTs = moment(oldestTxTimeStamp).startOf("week").valueOf();
      let newestTxStartTs = moment(newestTxTimeStamp).valueOf();

      let weekseries: [number, number][] = [];

      while (oldestTxStartTs < newestTxStartTs) {
        const nextWeekStart = oldestTxStartTs + 7 * 24 * 60 * 60 * 1000;
        let spotPrice = new BigNumber(0);

        txs.forEach((t) => {
          if (
            t.receivedTimestamp > oldestTxStartTs &&
            t.receivedTimestamp < nextWeekStart &&
            spotPrice.lt(t.spotPrice)
          ) {
            spotPrice = new BigNumber(t.spotPrice);
          }
        });

        weekseries.push([oldestTxStartTs, spotPrice.toNumber()]);
        oldestTxStartTs = oldestTxStartTs + newestTxStartTs;
      }

      return weekseries;
    case "1m":
      let _oldestTxTs = txs[txs.length - 1].receivedTimestamp;
      let _newestTxTs = txs[0].receivedTimestamp;
      _oldestTxTs = moment(_oldestTxTs).startOf("month").valueOf();
      _newestTxTs = moment(_newestTxTs).valueOf();

      let moSeries: [number, number][] = [];

      while (_oldestTxTs < _newestTxTs) {
        let nextMoTs = moment(_oldestTxTs).endOf("month").valueOf();
        let spotPrice = new BigNumber(0);

        txs.forEach((t) => {
          if (
            t.receivedTimestamp > oldestTxStartTs &&
            t.receivedTimestamp < nextMoTs &&
            spotPrice.lt(t.spotPrice)
          ) {
            spotPrice = new BigNumber(t.spotPrice);
          }
        });

        moSeries.push([_oldestTxTs, spotPrice.toNumber()]);

        _oldestTxTs = nextMoTs + 1000 * 60 * 60 * 1;
      }

      return moSeries;
    default:
      return [];
  }
}
