import { BondOffer } from "../../store/bonds/bonds.types";
import type { AccountId32 } from "@polkadot/types/interfaces/runtime";
import { TOKENS } from "../../defi/Tokens";
import { TokenId } from "../../defi/types";
import { stringToBigNumber } from "../../utils/stringToBigNumber";

const currencyIdToAssetMap: Record<string, TokenId> = {
  "1": "pica",
  "4": "ksm",
};

export function decodeBondOffer(
  offerId: number,
  beneficiary: AccountId32,
  bondOffer: any
): BondOffer {
  return {
    offerId,
    beneficiary,
    asset: TOKENS[currencyIdToAssetMap[bondOffer.asset]],
    bondPrice: stringToBigNumber(bondOffer.bondPrice),
    nbOfBonds: bondOffer.nbOfBonds,
    maturity: bondOffer.maturity.Finite
      ? bondOffer.maturity.Finite.returnIn
      : "Infinite",
    reward: {
      asset: TOKENS[currencyIdToAssetMap[bondOffer.reward.asset]],
      amount: stringToBigNumber(bondOffer.reward.amount),
      maturity: Number(bondOffer.reward.maturity),
    },
  };
}
