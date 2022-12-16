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
  const exponent = Wi.div(Wo)
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
