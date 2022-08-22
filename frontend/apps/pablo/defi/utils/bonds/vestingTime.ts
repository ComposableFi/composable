import { VestingSchedule } from "@/defi/types";
import BigNumber from "bignumber.js";
import moment from "moment";

export function calculateVestingTime(
  maturity: BigNumber,
  blockInterval: BigNumber
) {
  const duration = maturity.times(blockInterval);
  return moment.utc(duration.toNumber()).format("HH:mm:ss");
}

export function calculateClaimableAt(
  vestingSchedule: VestingSchedule | undefined,
  currentBlock: BigNumber
): {
  claimable: BigNumber;
  pendingRewards: BigNumber;
  alreadyClaimed: BigNumber;
  totalVested: BigNumber;
} {
  let claimable = new BigNumber(0),
    _alreadyClaimed = new BigNumber(0),
    pendingRewardsToBeClaimed = new BigNumber(0),
    totalVested = new BigNumber(0);

  if (vestingSchedule) {
    if (vestingSchedule.type === "block") {
      const { start, period } = vestingSchedule.window;
      const { perPeriod, alreadyClaimed, periodCount } = vestingSchedule;

      let totalClaimable = perPeriod.times(periodCount);
      pendingRewardsToBeClaimed = totalClaimable.minus(alreadyClaimed);
      _alreadyClaimed = new BigNumber(alreadyClaimed);

      if (currentBlock.gt(start.plus(period.times(periodCount)))) {
        claimable = pendingRewardsToBeClaimed.gt(perPeriod)
          ? perPeriod
          : pendingRewardsToBeClaimed;
        totalVested = claimable.plus(alreadyClaimed);
      } else {
        let startBlock = new BigNumber(start);
        let rewardedAmount = new BigNumber(0);
        while (startBlock.lt(currentBlock)) {
          startBlock = startBlock.plus(period);
          if (startBlock.lt(currentBlock)) {
            rewardedAmount = rewardedAmount.plus(perPeriod);
          }
        }

        claimable = rewardedAmount.minus(alreadyClaimed);
        totalVested = claimable.plus(alreadyClaimed);
      }
    }
  }

  return {
    claimable,
    alreadyClaimed: _alreadyClaimed,
    pendingRewards: pendingRewardsToBeClaimed,
    totalVested,
  };
}
