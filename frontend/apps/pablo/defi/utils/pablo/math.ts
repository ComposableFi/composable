import BigNumber from "bignumber.js";

export function calculator(
  sideUpdated: "base" | "quote",
  tokenAmount: BigNumber,
  oneBaseInQuote: BigNumber,
  slippage: number,
  feeRate: number,
  formatDecimals: number = 4
): {
  tokenOutAmount: BigNumber;
  feeChargedAmount: BigNumber;
  slippageAmount: BigNumber;
  minReceive: BigNumber;
} {
  let tokenOutAmount = new BigNumber(0);
  const slippagePercentage = new BigNumber(slippage).div(100);
  const feePercentage = new BigNumber(feeRate).div(100);

  let oneQuoteInBase = new BigNumber(1).div(oneBaseInQuote);

  tokenOutAmount =
    sideUpdated === "base"
      ? tokenAmount.times(oneBaseInQuote)
      : tokenAmount.times(oneQuoteInBase);

  let slippageAmount = new BigNumber(0);
  slippageAmount =
    sideUpdated === "base"
      ? tokenOutAmount.times(slippagePercentage)
      : tokenAmount.times(slippagePercentage);

  let feeChargedAmount = new BigNumber(0);
  feeChargedAmount =
    sideUpdated === "base"
      ? tokenOutAmount.times(feePercentage)
      : tokenAmount.times(feePercentage);

  let minReceive = new BigNumber(0);
  if (sideUpdated === "base") {
    minReceive = tokenOutAmount
      .minus(slippageAmount.plus(feeChargedAmount))
      .times(oneQuoteInBase);
  } else {
    minReceive = tokenAmount
      .minus(slippageAmount.plus(feeChargedAmount))
      .times(oneQuoteInBase);
  }

  return {
    feeChargedAmount: feeChargedAmount.dp(formatDecimals),
    slippageAmount: slippageAmount.dp(formatDecimals),
    tokenOutAmount: tokenOutAmount.dp(formatDecimals),
    minReceive: minReceive.dp(formatDecimals),
  };
}

export function calculatePoolTotalValueLocked(
  baseAmount: BigNumber,
  quoteAmount: BigNumber,
  basePrice: BigNumber,
  quotePrice: BigNumber
): BigNumber {
  return baseAmount.times(basePrice).plus(quoteAmount.times(quotePrice));
}

export function calculateConstantProductSpotPrice(
  baseBalance: BigNumber,
  quoteBalance: BigNumber,
  baseWeight: BigNumber
): BigNumber {
  let quoteWeight = new BigNumber(100).minus(baseWeight).div(100);
  baseWeight = baseWeight.div(100);
  let num = quoteBalance.div(quoteWeight);
  let den = baseBalance.div(baseWeight);

  return num.div(den);
}

export function calculateChangePercent(
  new_price: BigNumber,
  old_price: BigNumber
): BigNumber {
  let difference = new_price.minus(old_price).div(old_price);
  return difference.times(100);
}
