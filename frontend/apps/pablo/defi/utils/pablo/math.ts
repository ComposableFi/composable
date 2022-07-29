import BigNumber from "bignumber.js";

export function stableSwapCalculator(
  sideUpdated: "base" | "quote",
  tokenAmount: BigNumber,
  oneBaseInQuote: BigNumber,
  slippage: number, // in %
  feeRate: number, // in %
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
      ? tokenAmount.times(feePercentage)
      : tokenOutAmount.times(feePercentage);

  let minReceive = new BigNumber(0);
  if (sideUpdated === "base") {
    minReceive = tokenOutAmount
      .minus(slippageAmount)
      .times(oneQuoteInBase).minus(feeChargedAmount);
  } else {
    minReceive = tokenAmount
      .minus(slippageAmount)
      .times(oneQuoteInBase).minus(feeChargedAmount);
  }

  return {
    feeChargedAmount: feeChargedAmount.dp(formatDecimals),
    slippageAmount: slippageAmount.dp(formatDecimals),
    tokenOutAmount: tokenOutAmount.dp(formatDecimals),
    minReceive: minReceive.dp(formatDecimals),
  };
}

export function calculator(
  sideUpdated: "base" | "quote",
  tokenAmount: BigNumber,
  oneBaseInQuote: BigNumber,
  slippage: number, // in %
  feeRate: number, // in %
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
    minReceive = tokenOutAmount.minus(
      slippageAmount.plus(feeChargedAmount)
    ).times(oneQuoteInBase);
  } else {
    minReceive = tokenAmount.minus(
      slippageAmount.plus(feeChargedAmount)
    ).times(oneQuoteInBase);
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