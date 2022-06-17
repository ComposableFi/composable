import { Token } from "@/defi/Tokens";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import BigNumber from "bignumber.js";

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
