import { ActiveBond } from "@/stores/defi/polkadot/bonds/slice";
import { getClaimable } from "@/components/Organisms/Bond/utils";
import { fromChainIdUnit, humanDate, SHORT_HUMAN_DATE } from "shared";
import BigNumber from "bignumber.js";
import { findCurrentBond } from "@/stores/defi/polkadot/bonds/utils";
import { useActiveBonds } from "@/defi/polkadot/hooks/useActiveBonds";
import {
  useBlockInterval,
  useCurrentBlockAndTime,
  usePicassoProvider,
} from "substrate-react";

export const useClaim = (bondOfferId?: string) => {
  const { activeBonds } = useActiveBonds();
  const interval = useBlockInterval();
  const { parachainApi } = usePicassoProvider();
  const { block } = useCurrentBlockAndTime(parachainApi);
  const activeBond = activeBonds.find((b: ActiveBond) =>
    findCurrentBond(b, bondOfferId?.toString() ?? "")
  );

  if (!activeBond) {
    return {
      claimable: new BigNumber(0),
      remainingBlocks: new BigNumber(0),
      vestingTime: "~",
      vestedTime: "~",
      pending: new BigNumber(0),
      lastBlock: new BigNumber(0),
      total: new BigNumber(0),
    };
  }

  const { perPeriod, periodCount, window } = activeBond;
  const lastBlock = window.blockNumberBased.start
    .plus(window.blockNumberBased.period)
    .multipliedBy(periodCount);

  const claimable = getClaimable(
    block,
    window,
    perPeriod,
    lastBlock,
    periodCount
  );

  const total = periodCount.multipliedBy(fromChainIdUnit(perPeriod));

  const pending = total.minus(claimable);
  const remainingBlocks = lastBlock.minus(block).lte(0)
    ? new BigNumber(0)
    : lastBlock.minus(block);
  const vestingTime = humanDate(
    remainingBlocks.multipliedBy(interval.toNumber() / 1000).toNumber()
  );

  const sourceBlock = block.gt(lastBlock) ? lastBlock : block;
  const vestedTime = humanDate(
    sourceBlock
      .minus(window.blockNumberBased.start)
      .multipliedBy(interval.toNumber())
      .toNumber(),
    SHORT_HUMAN_DATE
  );

  return {
    claimable,
    remainingBlocks,
    vestingTime,
    vestedTime,
    pending,
    lastBlock,
    total,
  };
};
