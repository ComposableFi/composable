import { BondOffer } from "@/defi/types";
import { fromChainUnits } from "../units";
import BigNumber from "bignumber.js";

export function decodeBondOffer(offer: any, index: number): BondOffer {
  const [beneficiary, bondOffer] = offer.toJSON();
  return {
    offerId: new BigNumber(index),
    asset: new BigNumber(bondOffer.asset).toString(),
    beneficiary,
    bondPrice: fromChainUnits(bondOffer.bondPrice),
    nbOfBonds: new BigNumber(bondOffer.nbOfBonds),
    maturity: bondOffer.maturity.finite
      ? {
          Finite: {
            returnIn: new BigNumber(bondOffer.maturity.finite.returnIn),
          },
        }
      : "Infinite",
    reward: {
      amount: fromChainUnits(bondOffer.reward.amount),
      asset: new BigNumber(bondOffer.reward.asset).toString(), // asset id
      maturity: new BigNumber(bondOffer.reward.maturity),
    },
  };
}