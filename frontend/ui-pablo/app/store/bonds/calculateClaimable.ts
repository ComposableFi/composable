import BigNumber from "bignumber.js";

type Props = {
  currentBlockOrMoment: number;
  start: number;
  perPeriod: BigNumber;
  periodCount: number;
  blockNumberOrMomentAtEnd: number;
};
export function calculateClaimable({
  currentBlockOrMoment,
  start,
  perPeriod,
  periodCount,
  blockNumberOrMomentAtEnd,
}: Props) {
  const getClaimable = (currentBlockNumberOrMoment: number) =>
    perPeriod.times(
      Math.floor((currentBlockNumberOrMoment - start) / periodCount)
    );
  if (currentBlockOrMoment > blockNumberOrMomentAtEnd) {
    return periodCount === 1
      ? perPeriod
      : getClaimable(blockNumberOrMomentAtEnd);
  }
  if (currentBlockOrMoment > start) {
    return getClaimable(currentBlockOrMoment);
  }
  return new BigNumber(0);
}
