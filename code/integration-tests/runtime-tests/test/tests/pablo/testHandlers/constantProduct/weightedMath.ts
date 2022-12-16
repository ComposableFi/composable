import BN from "bn.js";

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
export const calculateOutGivenIn = function (Bo: BN, Bi: BN, Ai: BN, Wi: number, Wo: number) {
  const mostInnerBrackets = Bi.div(Bi.add(Ai));
  const toPower = mostInnerBrackets.pow(new BN(Wi/Wo));
  const subOne = new BN(1).sub(toPower);
  const timesBo = Bo.mul(subOne);
  return timesBo;
  //return Bo.mul(new BN(1).sub(((Bi.div(Bi.add(Ai))).pow(new BN(Wi/Wo)))))
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
export const calculateInGivenOut = function (Bo: BN, Bi: BN, Ao: BN, Wi: number, Wo: number) {
  const mostInnerBrackets = (Bo.div(Bo.sub(Ao)));
  const toPower = mostInnerBrackets.pow(new BN(Wo / Wi));
  const subOne = toPower.sub(new BN(1));
  const timesBi = Bi.mul(subOne);
  // return Bi.mul(((Bo.div(Bo.sub(Ao))).pow(new BN(Wo/Wi))).sub(new BN(1)))
  return timesBi
};
