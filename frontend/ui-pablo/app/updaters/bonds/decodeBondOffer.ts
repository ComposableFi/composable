import { BondOffer } from "../../store/bonds/bonds.types";
import type { AccountId32 } from "@polkadot/types/interfaces/runtime";
import { TOKENS } from "../../defi/Tokens";
import { Token, TokenId } from "../../defi/types";
import { stringToBigNumber } from "../../utils/stringToBigNumber";
import { stringToNumber } from "../../utils/stringToNumber";
import { CURRENCY_ID_TO_TOKEN_ID_MAP } from "../../utils/constants";

export function decodeBondOffer(
  offerId: number,
  beneficiary: AccountId32,
  bondOffer: any,
  principalAsset: { base: Token; quote: Token } | Token
): BondOffer {
  return {
    offerId,
    beneficiary,
    currencyId: stringToNumber(bondOffer.asset),
    asset: principalAsset,
    bondPrice: stringToBigNumber(bondOffer.bondPrice),
    nbOfBonds: bondOffer.nbOfBonds,
    maturity: bondOffer.maturity.Finite
      ? bondOffer.maturity.Finite.returnIn
      : "Infinite",
    reward: {
      currencyId: stringToNumber(bondOffer.asset),
      asset: TOKENS[CURRENCY_ID_TO_TOKEN_ID_MAP[bondOffer.reward.asset]],
      amount: stringToBigNumber(bondOffer.reward.amount),
      maturity: Number(bondOffer.reward.maturity),
    },
  };
}
