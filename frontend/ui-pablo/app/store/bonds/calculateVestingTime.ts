import BigNumber from "bignumber.js";
import moment from "moment";

type CalculateBlockBasedVestingTimeProps = {
  start: BigNumber;
  currentBlock: BigNumber;
  period: BigNumber;
  blockNumberOrMomentAtEnd: BigNumber;
};

export function calculateBlockBasedVestingTime({
  start,
  currentBlock,
  period,
  blockNumberOrMomentAtEnd,
}: CalculateBlockBasedVestingTimeProps) {
  if (currentBlock < start) {
    return "100%";
  }

  if (currentBlock >= start) {
    return "0%";
  }

  return `${blockNumberOrMomentAtEnd
    .minus(currentBlock)
    .div(period)
    .times(100)}%`;
}

type CalculateMomentBasedVestingTimeProps = {
  start: BigNumber;
  currentTime: BigNumber;
  blockNumberOrMomentAtEnd: BigNumber;
};

export function calculateMomentBasedVestingTime({
  start,
  currentTime,
  blockNumberOrMomentAtEnd,
}: CalculateMomentBasedVestingTimeProps) {
  const startMoment = moment.unix(start.toNumber());
  const endMoment = moment.unix(blockNumberOrMomentAtEnd.toNumber());
  const duration = moment.duration(endMoment.diff(startMoment));
  return currentTime > start
    ? `0D 0H 0M 0S`
    : `${duration.asDays()}D ${duration.asHours()}H ${duration.minutes()}M ${duration.seconds()}S`;
}
