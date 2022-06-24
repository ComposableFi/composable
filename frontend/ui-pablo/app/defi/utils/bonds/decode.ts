import { BondOffer } from "@/defi/types";
import BigNumber from "bignumber.js";
import { fromChainUnits } from "../units";

export function decodeBondOffer(offer: any, index: number): BondOffer {
  const [beneficiary, bondOffer] = offer;
  return {
    offerId: new BigNumber(index),
    asset: bondOffer.asset.replaceAll(",", ""),
    beneficiary,
    bondPrice: fromChainUnits(bondOffer.bondPrice.replaceAll(",", "")),
    nbOfBonds: new BigNumber(bondOffer.nbOfBonds.replaceAll(",", "")),
    maturity: bondOffer.maturity.Finite
      ? {
          Finite: {
            returnIn: new BigNumber(
              bondOffer.maturity.Finite.returnIn.replace(",", "")
            ),
          },
        }
      : "Infinite",
    reward: {
      amount: fromChainUnits(bondOffer.reward.amount.replaceAll(",", "")),
      asset: bondOffer.reward.asset.replaceAll(",", ""),
      maturity: new BigNumber(bondOffer.reward.maturity.replaceAll(",", "")),
    },
  };
}
