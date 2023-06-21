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
      // Will be reworked by me
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
