import { useStore } from "@/stores/root";
import { useCurrentBlockAndTime, usePicassoProvider } from "substrate-react";
import { useEffect, useRef } from "react";
import { getDiffInMinutes } from "shared";
import BigNumber from "bignumber.js";
import { useSnackbar } from "notistack";
import { usePicaPriceDiscovery } from "@/defi/polkadot/hooks/usePicaPriceDiscovery";

const MINUTES_IN_WEEK = 60 * 24 * 7;

export function useStakingRewardsNotifications() {
  const { parachainApi } = usePicassoProvider();
  const { block, time } = useCurrentBlockAndTime(parachainApi);
  const { enqueueSnackbar } = useSnackbar();
  const hasAlreadyPushedNotification = useRef(false);
  const picaPrice = usePicaPriceDiscovery();

  useEffect(() => {
    if (!hasAlreadyPushedNotification.current) {
      // Calculate the following every 5 block which is roughly 1 minute.
      if (block.mod(5).eq(0)) {
        const rewards = aggregateRewards(time);
        if (rewards.gt(0)) {
          enqueueSnackbar(`Stake period will end in one week`, {
            description: `You have $${rewards
              .multipliedBy(picaPrice)
              .toFormat()} worth of assets to claim`,
            variant: "info",
            preventDuplicate: true,
            isClosable: true,
            onClose: () => {
              hasAlreadyPushedNotification.current = true;
            },
          });
        }
      }
    }
  }, [block, enqueueSnackbar, picaPrice, time]);
}

function aggregateRewards(time: Date) {
  const portfolio = useStore.getState().stakingPortfolio;
  const claimableRewards = useStore.getState().claimableRewards;

  return Array.from(portfolio.entries()).reduce((sum, [key, item]) => {
    const when = new Date(Number(item.endTimestamp));
    const shouldShowNotification =
      getDiffInMinutes(time, when) > MINUTES_IN_WEEK;

    if (shouldShowNotification) {
      const picaRewards = claimableRewards[key]
        .filter((item) => item.assetId === "1")
        .reduce((acc, cur) => acc.plus(cur.balance), new BigNumber(0));
      return sum.plus(picaRewards);
    }
    return sum;
  }, new BigNumber(0));
}
