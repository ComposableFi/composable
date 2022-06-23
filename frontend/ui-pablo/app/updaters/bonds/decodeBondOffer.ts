import { BondOffer } from "../../store/bonds/bonds.types";
import type { AccountId32 } from "@polkadot/types/interfaces/runtime";
import { getToken, getTokenId } from "../../defi/Tokens";
import { Token } from "../../defi/types";
import { stringToBigNumber } from "../../utils/stringToBigNumber";
import { stringToNumber } from "../../utils/stringToNumber";
import { fromChainUnits } from "@/defi/utils";

export function decodeBondOffer(
  offerId: number,
  beneficiary: AccountId32,
  bondOffer: any,
  principalAsset: { base: Token; quote: Token } | Token
): BondOffer {
  console.log(bondOffer)
  return {
    offerId,
    beneficiary,
    currencyId: stringToNumber(bondOffer.asset),
    asset: principalAsset,
    bondPrice: fromChainUnits(bondOffer.bondPrice.replaceAll(",", "")),
    nbOfBonds: bondOffer.nbOfBonds,
    maturity: bondOffer.maturity.Finite
      ? stringToNumber(bondOffer.maturity.Finite.returnIn)
      : "Infinite",
    reward: {
      currencyId: stringToNumber(bondOffer.asset),
      asset: getToken(getTokenId(bondOffer.reward.asset)),
      amount: fromChainUnits(bondOffer.reward.amount.replaceAll(",", "")),
      maturity: Number(bondOffer.reward.maturity),
    },
  };
}
