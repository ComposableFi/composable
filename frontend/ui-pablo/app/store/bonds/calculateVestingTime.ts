import moment from "moment";

type CalculateBlockBasedVestingTimeProps = {
  start: number;
  currentBlock: number;
  period: number;
  blockNumberOrMomentAtEnd: number;
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

  return `${((blockNumberOrMomentAtEnd - currentBlock) / period) * 100}%`;
}

type CalculateMomentBasedVestingTimeProps = {
  start: number;
  currentTime: number;
  blockNumberOrMomentAtEnd: number;
};

export function calculateMomentBasedVestingTime({
  start,
  currentTime,
  blockNumberOrMomentAtEnd,
}: CalculateMomentBasedVestingTimeProps) {
  const startMoment = moment.unix(start);
  const endMoment = moment.unix(blockNumberOrMomentAtEnd);
  const duration = moment.duration(endMoment.diff(startMoment));
  return currentTime > start
    ? `0D 0H 0M 0S`
    : `${duration.asDays()}D ${duration.asHours()}H ${duration.minutes()}M ${duration.seconds()}S`;
}
