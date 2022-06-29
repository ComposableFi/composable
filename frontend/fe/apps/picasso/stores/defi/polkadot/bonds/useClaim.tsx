import { useAppSelector } from "@/hooks/store";
import { ActiveBond } from "@/stores/defi/polkadot/bonds/slice";
import { getClaimable } from "@/components/Organisms/Bond/utils";
import { fromChainIdUnit } from "@/defi/polkadot/pallets/BondedFinance";
import BigNumber from "bignumber.js";
import { humanDate, SHORT_HUMAN_DATE } from "@/utils/formatters";
import { useCurrentBlockAndTime } from "@/defi/polkadot/utils";
import { useBlockInterval, usePicassoProvider } from "@/defi/polkadot/hooks";
import { findCurrentBond } from "@/stores/defi/polkadot/bonds/utils";

export const useClaim = (bondOfferId?: string) => {
  const openBonds = useAppSelector<ActiveBond[]>(
    (state) => state.bonding.openPositions
  );
  const interval = useBlockInterval();
  const { parachainApi } = usePicassoProvider();
  const { block } = useCurrentBlockAndTime(parachainApi);
  const activeBond = openBonds.find((b: ActiveBond) =>
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
