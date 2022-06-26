import { LiquidityPoolType } from "@/store/pools/pools.types";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fetchSpotPrice } from "./spotPrice";
import { PabloExchangeParams } from "./types";

function takeFeeOnInputToken(poolType: LiquidityPoolType | "none"): boolean {
  return (
    poolType === "ConstantProduct" || poolType === "LiquidityBootstrapping"
  );
}

export const calculateSwap = async (
  api: ApiPromise,
  exchange: PabloExchangeParams,
  poolConstants: any
): Promise<{
  priceImpact: string;
  minimumRecieved: string;
  tokenOutAmount: string;
}> => {

  let tradeSummary = {
    priceImpact: "0",
    minimumRecieved: "0",
    tokenOutAmount: "0",
  };

  try {
    let oneBaseInQuote = await fetchSpotPrice(
      api,
      {
        base: poolConstants.pair.base.toString(),
        quote: poolConstants.pair.quote.toString()
      },
      poolConstants.poolIndex
    );
    let oneQuoteInBase = new BigNumber(1).div(oneBaseInQuote);

    if (oneQuoteInBase.lte(0) || oneBaseInQuote.lte(0))
      throw new Error("Price RPC Fail");

    let tokenOut = new BigNumber(0);
    if (exchange.side === "quote") {
      if (exchange.quoteAssetId === poolConstants.pair.quote.toString()) {
        // same quote as on chain
        tokenOut = exchange.quoteAmount.times(oneQuoteInBase);
      } else {
        // base as quote
        tokenOut = exchange.quoteAmount.times(oneBaseInQuote);
      }
    } else {
      if (exchange.quoteAssetId === poolConstants.pair.quote.toString()) {
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
        if (exchange.quoteAssetId === poolConstants.pair.quote.toString()) {
          // same quote asset as on chain, deduct in quote
          feeDeductionValue = fee
            .times(oneQuoteInBase)
            .times(exchange.quoteAmount);
        } else {
          // deduct fee in on chain base
          feeDeductionValue = fee.times(oneBaseInQuote).times(tokenOut);
        }
      } else {
        if (exchange.quoteAssetId === poolConstants.pair.quote.toString()) {
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
