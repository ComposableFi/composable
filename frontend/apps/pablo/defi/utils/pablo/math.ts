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
      .times(oneQuoteInBase)
      .minus(feeChargedAmount);
  } else {
    minReceive = tokenAmount
      .minus(slippageAmount)
      .times(oneQuoteInBase)
      .minus(feeChargedAmount);
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

export function compute_d(
  baseAssetAmount: BigNumber,
  quoteAssetAmount: BigNumber,
  amplificationCoefficient: BigNumber
): BigNumber {
  let n = new BigNumber(2);
  let one = new BigNumber(1);

  let net_aum = baseAssetAmount.plus(quoteAssetAmount);
  if (net_aum.eq(0)) {
    return net_aum;
  }
  let ann = amplificationCoefficient.times(n).times(n);
  let d = new BigNumber(net_aum.toString());

  let base_n = baseAssetAmount.times(n);
  let quote_n = quoteAssetAmount.times(n);

  for (let i = 0; i < 255; i++) {
    let d_p = new BigNumber(d.toString());
    let ann_d = ann.times(d);

    let d_p_d = d_p.times(d);
    d_p = d_p_d.div(base_n);
    d_p_d = d_p.times(d);
    d_p = d_p_d.div(quote_n);

    let d_prev = new BigNumber(d.toString());
    let numerator = ann.times(net_aum).plus(d_p.times(n)).times(d);
    let denominator = ann_d.plus(n.plus(one).times(d_p)).minus(d);

    d = numerator.div(denominator);

    if (d.gt(d_prev)) {
      if (d.minus(d_prev).lte(one)) {
        return d;
      }
    } else if (d_prev.minus(d).lte(one)) {
      return d;
    }
  }

  throw new Error("Could not compute d");
}

export function compute_base(
  new_quote: BigNumber,
  amp_coeff: BigNumber,
  d: BigNumber
): BigNumber {
  let n = new BigNumber(2);
  let one = new BigNumber(1);
  let two = new BigNumber(2);

  let ann = amp_coeff.times(n).times(n);

  // s and p are same as input base amount as pool supports only 2 assets.
  let s = new BigNumber(new_quote.toString());
  let p = new BigNumber(new_quote.toString());

  // term1 = d^(n + 1) / n^n * p
  // term2 = 2*y + s - d

  let d_n = d.div(n);
  let c = d_n.times(d_n).times(d);
  let term1 = c.div(p);

  let y = new BigNumber(d.toString());

  // y = (y^2 * ann + term1) / (ann * term2 + d)
  for (let i = 0; i < 255; i++) {
    let y_prev = new BigNumber(y.toString());
    let term2 = two.times(y).plus(s).minus(d);
    let numerator = ann.times(y).times(y).plus(term1);
    let denominator = ann.times(term2).plus(d);

    y = numerator.div(denominator);
    if (y.gt(y_prev)) {
      if (y.minus(y_prev).lte(one)) {
        return y;
      }
    } else if (y_prev.minus(y).lte(one)) {
      return y;
    }
  }

  throw new Error("Could not compute base");
}

export function compute_spot_price_stable_swap(
  poolBaseAum: BigNumber,
  poolQuoteAum: BigNumber,
  ampCoeff: BigNumber,
  quotedAmount: BigNumber
): BigNumber {
  let d = compute_d(poolBaseAum, poolQuoteAum, ampCoeff);
  let new_quote_amount = poolQuoteAum.plus(quotedAmount);
  let new_base_amount = compute_base(new_quote_amount, ampCoeff, d);
  let exchange_value = poolBaseAum.minus(new_base_amount);
  return exchange_value;
}
