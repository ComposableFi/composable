import { Token } from "@/defi/Tokens";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import BigNumber from "bignumber.js";
import { VestingSchedule } from "@/defi/polkadot/hooks/useOpenPositions";

export function lpToSymbolPair(acc: string, token: Token) {
  return acc.length > 0 ? acc + "-" + token.symbol : token.symbol;
}

export function getMaxPurchasableBonds(
  bondOffer?: BondOffer,
  balance?: BigNumber
) {
  if (!bondOffer || !balance) return new BigNumber(0);
  const tokensInAllBonds = bondOffer.bondPrice.multipliedBy(
    bondOffer.nbOfBonds
  );

  if (balance.gte(tokensInAllBonds)) {
    return new BigNumber(bondOffer.nbOfBonds);
  }
  if (balance.lt(bondOffer.bondPrice)) {
    return new BigNumber(0);
  }
  return balance.modulo(bondOffer.bondPrice);
}

export function getTokenString(asset: Token | Token[]) {
  return Array.isArray(asset) ? asset.reduce(lpToSymbolPair, "") : asset.symbol;
}

type CalculateClaimableArgs = {
  currentBlockOrMoment: number;
  start: number;
  perPeriod: BigNumber;
  periodCount: number;
  blockNumberOrMomentAtEnd: number;
};
export function calculateClaimable({
  currentBlockOrMoment,
  start,
  perPeriod,
  periodCount,
  blockNumberOrMomentAtEnd,
}: CalculateClaimableArgs) {
  const getClaimable = (currentBlockNumberOrMoment: number) =>
    perPeriod.times(
      Math.floor((currentBlockNumberOrMoment - start) / periodCount)
    );
  if (currentBlockOrMoment > blockNumberOrMomentAtEnd) {
    return periodCount === 1
      ? perPeriod
      : getClaimable(blockNumberOrMomentAtEnd);
  }
  if (currentBlockOrMoment > start) {
    return getClaimable(currentBlockOrMoment);
  }
  return new BigNumber(0);
}
