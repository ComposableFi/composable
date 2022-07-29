import moment from "moment";

type GetBlockBasedVestingTimeProps = {
  start: number;
  currentBlock: number;
  lastBlock: number;
};

export function fetchBlockBasedVestingTime({
  start,
  currentBlock,
  lastBlock,
}: GetBlockBasedVestingTimeProps) {
  const totalBlockPerVesting = lastBlock - start;
  if (currentBlock < start) {
    return "100%";
  }

  if (currentBlock >= lastBlock) {
    return "0%";
  }

  return `${((lastBlock - currentBlock) / totalBlockPerVesting) * 100}%`;
}

type GetMomentBasedVestingTimeProps = {
  start: number;
  currentTime: number;
  lastMoment: number;
};

export function fetchMomentBasedVestingTime({
  start,
  currentTime,
  lastMoment,
}: GetMomentBasedVestingTimeProps) {
  const startMoment = moment.unix(start);
  const endMoment = moment.unix(lastMoment);
  const duration = moment.duration(endMoment.diff(startMoment));
  return currentTime > start
    ? `0D 0H 0M 0S`
    : `${duration.asDays()}D ${duration.asHours()}H ${duration.minutes()}M ${duration.seconds()}S`;
}
