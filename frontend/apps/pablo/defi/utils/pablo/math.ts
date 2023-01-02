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
      ? tokenOutAmount.minus(slippageAmount).times(feePercentage)
      : tokenAmount.minus(slippageAmount).times(feePercentage);

  return {
    feeChargedAmount: feeChargedAmount.dp(formatDecimals),
    slippageAmount: slippageAmount.dp(formatDecimals),
    tokenOutAmount: tokenOutAmount.dp(formatDecimals),
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

/**
 * Ao = Bo * (1- (Bi / (Bi + Ai))^(Wi / Wo)))
 *
 * @param Bo Balance before the trade of the token swapped out of the pool.
 * @param Bi Balance before the trade of the token swapped into the pool.
 * @param Ai Amount user puts in.
 * @param Wi Weight input token.
 * @param Wo Weight output token.
 * @return Ao => Amount user gets
 *
 * @example
 *       expectedAmountOut = calculateOutGivenIn(
 *           BigNumber(baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString()),
 *           BigNumber(quoteAssetFundsCurrentlyInPoolsBeforeTx.free.toString()),
 *           BigNumber(amount.toString()),
 *           BigNumber(5),
 *           BigNumber(5)
 *       );
 *       expectedReducedByFee = expectedAmountOut.minus(BigNumber(resultFee.fee.toString()));
 */
export const calculateOutGivenIn = function (
  Bo: BigNumber,
  Bi: BigNumber,
  Ai: BigNumber,
  Wi: BigNumber,
  Wo: BigNumber
) {
  const mostInnerBrackets = Bi.div(Bi.plus(Ai));
  const exponent = Wi.div(Wo);
  const toPower = mostInnerBrackets.pow(exponent);
  const subOne = BigNumber(1).minus(toPower);
  const timesBo = Bo.multipliedBy(subOne);

  return new BigNumber(timesBo.decimalPlaces(12).toString());
  //return Bo.mul(new bigint(1).sub(((Bi.div(Bi.add(Ai))).pow(new bigint(Wi/Wo)))))
};

/**
 * Ai = Bi * ((Bo / (Bo - Ao))^(Wo/Wi) - 1)
 *
 * @param Bo Balance before the trade of the token swapped out of the pool.
 * @param Bi Balance before the trade of the token swapped into the pool.
 * @param Ao Amount user wants out.
 * @param Wi Weight input token.
 * @param Wo Weight output token.
 * @return Ai => Amount user has to put in.
 *
 *
 * @example expectedAmountIn = calculateInGivenOut(
 *           BigNumber(baseAssetFundsCurrentlyInPoolsBeforeTx.free.toString()),
 *           BigNumber(quoteAssetFundsCurrentlyInPoolsBeforeTx.free.toString()),
 *           BigNumber(amount.toString()),
 *           BigNumber(5),
 *           BigNumber(5)
 *         );
 *         const expectedReducedByFee = expectedAmountIn.plus(BigNumber(resultFee.fee.toString()));
 */
export const calculateInGivenOut = function (
  Bo: BigNumber,
  Bi: BigNumber,
  Ao: BigNumber,
  Wi: BigNumber,
  Wo: BigNumber
) {
  const mostInnerBrackets = Bo.div(Bo.minus(Ao));
  const exponent = Wo.div(Wi);
  const toPower = mostInnerBrackets.pow(exponent);
  const subOne = toPower.minus(1);
  const timesBi = Bi.multipliedBy(subOne);
  return new BigNumber(timesBi.decimalPlaces(12).toString());
};
