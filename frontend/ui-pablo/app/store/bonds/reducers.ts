import BigNumber from "bignumber.js";
import produce from "immer";
import { calculateClaimable } from "./calculateClaimable";
import {
  calculateBlockBasedVestingTime,
  calculateMomentBasedVestingTime,
} from "./calculateVestingTime";
import { BondOffer, BondSlice, VestingSchedule } from "./types";

const DECIMALS = new BigNumber(10).pow(12);

export const addActiveBond = (
  activeBonds: BondSlice["activeBonds"],
  bondOffer: BondOffer,
  vestingSchedule: VestingSchedule,
  currentBlock: BigNumber,
  currentTime: BigNumber
) => {
  return produce(activeBonds, (draft) => {
    const window = vestingSchedule.window;
    const totalPeriod = window.period.times(vestingSchedule.periodCount);
    const blockNumberOrMomentAtEnd = window.start.plus(totalPeriod);
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
      asset: bondOffer.asset,
      pendingAmount: vestingSchedule.perPeriod
        .times(vestingSchedule.periodCount)
        .minus(claimableAmount),
      claimableAmount,
      vestingTime,
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
    .times(bondOffer.nbOfBonds)
    .div(DECIMALS);
  return produce(allBonds, (draft) => {
    draft.push({
      asset: bondOffer.asset,
      price,
      roi: new BigNumber(rewardPrice)
        .times(bondOffer.reward.amount)
        .times(100)
        .div(DECIMALS)
        .div(price),
      totalPurchased: new BigNumber(0), // TBD
    });
  });
};
