import BigNumber from "bignumber.js";

type Props = {
  currentBlockOrMoment: number;
  start: number;
  perPeriod: BigNumber;
  periodCount: number;
  lastBlockOrMoment: number;
};

export function fetchClaimable({
  currentBlockOrMoment,
  start,
  perPeriod,
  periodCount,
  lastBlockOrMoment,
}: Props) {
  let blockOrMoment = currentBlockOrMoment;
  if (currentBlockOrMoment > lastBlockOrMoment) {
    if (periodCount === 1) return perPeriod;
    blockOrMoment = lastBlockOrMoment;
  }
  return calculateClaimable({
    blockOrMoment,
    start,
    perPeriod,
    periodCount,
  });
}

type CalculateClaimableProps = {
  blockOrMoment: number;
  start: number;
  perPeriod: BigNumber;
  periodCount: number;
};

function calculateClaimable({
  blockOrMoment,
  start,
  perPeriod,
  periodCount,
}: CalculateClaimableProps) {
  return perPeriod.times(Math.floor((blockOrMoment - start) / periodCount));
}
