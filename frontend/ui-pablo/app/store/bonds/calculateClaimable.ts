import BigNumber from "bignumber.js";

type Props = {
  currentBlockOrMoment: BigNumber;
  start: BigNumber;
  perPeriod: BigNumber;
  periodCount: BigNumber;
  blockNumberOrMomentAtEnd: BigNumber;
};
export function calculateClaimable({
  currentBlockOrMoment,
  start,
  perPeriod,
  periodCount,
  blockNumberOrMomentAtEnd,
}: Props) {
  const getClaimable = (currentBlockNumberOrMoment: BigNumber) =>
    perPeriod.times(
      Math.floor(
        currentBlockNumberOrMoment.minus(start).div(periodCount).toNumber()
      )
    );
  if (currentBlockOrMoment > blockNumberOrMomentAtEnd) {
    return periodCount.isEqualTo(1)
      ? perPeriod
      : getClaimable(blockNumberOrMomentAtEnd);
  }
  if (currentBlockOrMoment > start) {
    return getClaimable(currentBlockOrMoment);
  }
  return new BigNumber(0);
}
