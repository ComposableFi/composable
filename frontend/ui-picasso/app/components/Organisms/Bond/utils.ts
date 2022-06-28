import { Token } from "@/defi/Tokens";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "@/defi/polkadot/pallets/BondedFinance";

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

export function getClaimable(
  block: BigNumber,
  window: { blockNumberBased: { start: BigNumber; period: BigNumber } },
  perPeriod: BigNumber,
  lastBlock: BigNumber,
  periodCount: BigNumber
) {
  if (block.gt(lastBlock)) {
    if (periodCount.eq(1)) {
      return fromChainIdUnit(perPeriod);
    }
    return lastBlock // 1200
      .minus(window.blockNumberBased.start) // 45
      .dividedBy(fromChainIdUnit(perPeriod)) // 1000
      .multipliedBy(fromChainIdUnit(perPeriod))
      .abs();
  }

  if (periodCount.eq(1)) {
    return new BigNumber(0);
  }

  return block.gt(window.blockNumberBased.start)
    ? block
        .minus(window.blockNumberBased.start)
        .dividedBy(periodCount)
        .multipliedBy(fromChainIdUnit(perPeriod))
        .abs()
    : new BigNumber(0);
}
