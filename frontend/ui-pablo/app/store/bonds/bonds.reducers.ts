import BigNumber from "bignumber.js";
import produce from "immer";
import { calculateClaimable } from "./calculateClaimable";
import {
  calculateBlockBasedVestingTime,
  calculateMomentBasedVestingTime,
} from "./calculateVestingTime";
import { BondOffer, BondSlice, VestingSchedule } from "./bonds.types";
import { DEFAULT_DECIMALS } from "../../updaters/constants";

export const addActiveBond = (
  activeBonds: BondSlice["activeBonds"],
  bondOffer: BondOffer,
  vestingSchedule: VestingSchedule,
  currentBlock: number,
  currentTime: number
) => {
  return produce(activeBonds, (draft) => {
    const window = vestingSchedule.window;
    const totalPeriod = window.period * vestingSchedule.periodCount;
    const blockNumberOrMomentAtEnd = window.start + totalPeriod;
    const vestingTime =
      vestingSchedule.type === "block"
        ? calculateBlockBasedVestingTime({
            start: window.start,
            period: window.period,
            blockNumberOrMomentAtEnd,
            currentBlock,
          })
        : calculateMomentBasedVestingTime({
            start: window.start,
            blockNumberOrMomentAtEnd,
            currentTime,
          });
    const claimableAmount = calculateClaimable({
      currentBlockOrMoment:
        vestingSchedule.type === "block" ? currentBlock : currentTime,
      start: window.start,
      periodCount: vestingSchedule.periodCount,
      perPeriod: vestingSchedule.perPeriod,
      blockNumberOrMomentAtEnd,
    });
    draft.push({
      offerId: bondOffer.offerId,
      asset: bondOffer.asset,
      pendingAmount: vestingSchedule.perPeriod
        .times(vestingSchedule.periodCount)
        .minus(claimableAmount),
      claimableAmount,
      vestingTime,
      bondOffer,
    });
  });
};

export const addBond = (
  allBonds: BondSlice["allBonds"],
  bondOffer: BondOffer,
  assetPrice: number,
  rewardPrice: number
) => {
  const price = new BigNumber(bondOffer.bondPrice)
    .times(assetPrice)
    .div(DEFAULT_DECIMALS);
  return produce(allBonds, (draft) => {
    draft.push({
      offerId: bondOffer.offerId,
      asset: bondOffer.asset,
      price,
      roi: new BigNumber(rewardPrice)
        .times(bondOffer.reward.amount)
        .times(100)
        .div(DEFAULT_DECIMALS)
        .div(price),
      totalPurchased: new BigNumber(0), // TBD
      bondOffer,
    });
  });
};
