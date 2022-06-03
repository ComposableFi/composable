import BigNumber from "bignumber.js";
import produce from "immer";
import { BondOffer, BondSlice } from "./types";

export const setActiveBonds = (
  activeBonds: BondSlice["activeBonds"],
  bondOffer: BondOffer
) => {
  return produce(activeBonds, (draft) => {
    draft.push({
      assetPair: {
        token1: bondOffer.asset,
        token2: bondOffer.reward.asset,
      },
      pending_amount: new BigNumber(0), // TBD
      claimable_amount: new BigNumber(0), // TBD
      vesting_time: 4, // TBD
    });
  });
};

export const setAllBonds = (
  allBonds: BondSlice["allBonds"],
  bondOffer: BondOffer
) => {
  return produce(allBonds, (draft) => {
    draft.push({
      assetPair: {
        token1: bondOffer.asset,
        token2: bondOffer.reward.asset,
      },
      price: bondOffer.bondPrice,
      roi: new BigNumber(0), // TBD
      totalPurchased: new BigNumber(0), // TBD
    });
  });
};
