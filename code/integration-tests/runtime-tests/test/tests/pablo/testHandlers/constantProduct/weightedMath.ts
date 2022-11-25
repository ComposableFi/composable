import BigNumber from "bignumber.js";

/**
 * Ao = Bo * (1- (Bi / (Bi + Ai))^(Wi / Wo)))
 *
 * @param Bo Balance before the trade of the token swapped out of the pool.
 * @param Bi Balance before the trade of the token swapped into the pool.
 * @param Ai Amount user puts in.
 * @param Wi Weight input token.
 * @param Wo Weight output token.
 * @return Ao => Amount user gets
 */
export const calculateOutGivenIn = function (Bo: BigNumber, Bi: BigNumber, Ai: BigNumber, Wi: BigNumber, Wo: BigNumber) {
  BigNumber.DEBUG = true;
  const mostInnerBrackets = Bi.div(Bi.plus(Ai));
  const exponent = Wi.div(Wo);
  const toPower = mostInnerBrackets.pow(exponent);
  const subOne = BigNumber(1).minus( toPower);
  const timesBo = Bo.multipliedBy(subOne);
  return timesBo;
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
 */
export const calculateInGivenOut = function (Bo: BigNumber, Bi: BigNumber, Ao: BigNumber, Wi: BigNumber, Wo: BigNumber) {
  BigNumber.DEBUG = true;
  const mostInnerBrackets = Bo.div(Bo.minus(Ao));
  const exponent = Wo.div(Wi)
  const toPower = mostInnerBrackets.pow(exponent);
  const subOne = toPower.minus(1);
  const timesBi = Bi.multipliedBy(subOne);
  // return Bi.mul(((Bo.div(Bo.sub(Ao))).pow(new bigint(Wo/Wi))).sub(new bigint(1)))
  return timesBi;
};

export const calculateSpotPrice = function (Bi: BigNumber, Bo: BigNumber, Wi: BigNumber, Wo:BigNumber, fee:BigNumber) {
  const firstFraction = (Bi.dividedBy(Wi)).dividedBy((Bo.dividedBy(Wo)))
  const secondFraction = BigNumber(1).dividedBy(BigNumber(1).minus(fee));
  return firstFraction.multipliedBy(secondFraction);
}

export const calculateEffectivePriceGivenIn = function(Bi:BigNumber, Bo:BigNumber, Ai:BigNumber, Wi:BigNumber, Wo:BigNumber) {
  const mostInnerBrackets = Bi.dividedBy(Bi.plus(Ai))
  const toPower = mostInnerBrackets.pow(Wi.dividedBy(Wo));
  const minus1 = BigNumber(1).minus(toPower);
  const timesBo = Bo.multipliedBy(minus1);
  const mainFraction = Ai.dividedBy(timesBo);
  return mainFraction;
}

/**
 * Returns effective price for given amount out.
 */
export const calculateEffectivePriceGivenOut = function(Bo:BigNumber, Bi:BigNumber, Ao:BigNumber, Wo:BigNumber, Wi:BigNumber) {
  const mostInnerBrackets = Bo.dividedBy((Bo.minus(Ao)));
  const toPower = mostInnerBrackets.pow(Wo.dividedBy(Wi));
  const minus1 = toPower.minus(1);
  const timesBi = Bi.multipliedBy(minus1);
  const mainFraction = timesBi.dividedBy(Ao);
  return mainFraction;
}

/**
 * Returns slippage in percent
 *  S(Ai) = (EPoi / SPoi) - 1
 */
export const calculateSlippageGivenIn = function(EPoi: BigNumber, SPoi: BigNumber) {
  const fraction = EPoi.dividedBy(SPoi);
  return fraction.minus(1);
}

export const calculateSlippageGivenOut = function(EPoi:BigNumber, SPoi:BigNumber) {
  const fraction = EPoi.dividedBy(SPoi);
  const minus1 = fraction.minus(1);
  return minus1
}
