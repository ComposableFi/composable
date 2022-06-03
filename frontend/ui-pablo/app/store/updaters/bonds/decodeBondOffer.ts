import { BondOffer } from "../../bonds/types";
import type { AccountId32 } from "@polkadot/types/interfaces/runtime";
import BigNumber from "bignumber.js";
import { TOKENS } from "../../../defi/Tokens";
import { TokenId } from "../../../defi/types";

const currencyIdToAssetMap: Record<string, TokenId> = {
  "1": "pica",
  "4": "ksm",
};

const stringToBigNumber = (value: string): BigNumber =>
  new BigNumber(value.replaceAll(",", ""));

/*TBD unit test the function */
export function decodeBondOffer(
  beneficiary: AccountId32,
  bondOffer: any
): BondOffer {
  return {
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
      maturity: new BigNumber(bondOffer.reward.maturity),
    },
  };
}
