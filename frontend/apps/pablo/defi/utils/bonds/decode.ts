import { BondOffer } from "@/defi/types";
import { humanizedBnToBn } from "shared";
import BigNumber from "bignumber.js";
import { fromChainUnits } from "../units";

export function decodeBondOffer(offer: any, index: number): BondOffer {
  const [beneficiary, bondOffer] = offer;
  return {
    offerId: new BigNumber(index),
    asset: humanizedBnToBn(bondOffer.asset).toString(),
    beneficiary,
    bondPrice: fromChainUnits(humanizedBnToBn(bondOffer.bondPrice)),
    nbOfBonds: humanizedBnToBn(bondOffer.nbOfBonds),
    maturity: bondOffer.maturity.Finite
      ? {
          Finite: {
            returnIn: humanizedBnToBn(bondOffer.maturity.Finite.returnIn),
          },
        }
      : "Infinite",
    reward: {
      amount: fromChainUnits(humanizedBnToBn(bondOffer.reward.amount)),
      asset: humanizedBnToBn(bondOffer.reward.asset).toString(), // assetid
      maturity: new BigNumber(humanizedBnToBn(bondOffer.reward.maturity)),
    },
  };
}
