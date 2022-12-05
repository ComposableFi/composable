import {
  calculateConstantProductSpotPrice,
  calculateChangePercent,
} from "@/defi/utils";
import { usePrevious } from "@/hooks/usePrevious";
import { useEffect, useMemo, useState } from "react";
import BigNumber from "bignumber.js";

type PriceImpactProps = {
  tokenInAmount: BigNumber;
  tokenOutAmount: BigNumber;
  baseWeight: BigNumber;
  baseBalance: BigNumber;
  quoteBalance: BigNumber;
  // amplificationCoefficient: BigNumber;
  isConstantProductPool: boolean;
};

export function usePriceImpact({
  tokenInAmount,
  tokenOutAmount,
  baseWeight,
  baseBalance,
  quoteBalance,
  // amplificationCoefficient,
  isConstantProductPool,
}: PriceImpactProps) {
  const [priceImpact, setPriceImpact] = useState(new BigNumber(0));
  const previousTokenIn = usePrevious(tokenInAmount);
  const previousTokenOut = usePrevious(tokenOutAmount);

  const amountIsChanged = useMemo(() => {
    if (!previousTokenIn || !previousTokenOut) return true;
    
    if (previousTokenIn.eq(tokenInAmount) && previousTokenOut.eq(tokenOutAmount)) {
      return false;
    }

    return true;
  }, [tokenInAmount, tokenOutAmount, previousTokenIn, previousTokenOut]);

  useEffect(() => {
    if (
      amountIsChanged
    ) {
      if (isConstantProductPool) {
        if (
          tokenInAmount.gt(0) &&
          tokenOutAmount.gt(0) &&
          baseWeight.gt(0) &&
          baseBalance.gt(0) &&
          quoteBalance.gt(0)
        ) {
          let currentSpotPrice = calculateConstantProductSpotPrice(
            baseBalance,
            quoteBalance,
            baseWeight
          );
          let spotPriceAfterTrade = calculateConstantProductSpotPrice(
            baseBalance.minus(tokenOutAmount),
            quoteBalance.plus(tokenInAmount),
            baseWeight
          );
    
          setPriceImpact(
            calculateChangePercent(spotPriceAfterTrade, currentSpotPrice)
          );
        } else {
          setPriceImpact(new BigNumber(0));
        }
      } else {
        // if (
        //   tokenInAmount.gt(0) &&
        //   tokenOutAmount.gt(0) &&
        //   baseBalance.gt(0) &&
        //   quoteBalance.gt(0)
        // ) {
        //   try {
        //     let currentSpotPrice = compute_spot_price_stable_swap(baseBalance, quoteBalance, amplificationCoefficient, new BigNumber(1));
        //     let changedSpotPrice = compute_spot_price_stable_swap(baseBalance.minus(tokenOutAmount), quoteBalance.plus(tokenInAmount), amplificationCoefficient, new BigNumber(1));
        //     setPriceImpact(calculateChangePercent(changedSpotPrice, currentSpotPrice));
        //   } catch (err) {
        //     console.error(err);
        //     setPriceImpact(new BigNumber(0));
        //   }
        // } else {
        //   setPriceImpact(new BigNumber(0));
        // }
      }
    }
  }, [
    amountIsChanged,
    tokenInAmount,
    tokenOutAmount,
    baseWeight,
    baseBalance,
    quoteBalance,
    // amplificationCoefficient,
    isConstantProductPool,
  ]);

  return priceImpact;
}
