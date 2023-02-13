import { useStakingRewardsStats } from "@/apollo/hooks/useStakingRewardsStats";
import { useStore } from "@/stores/root";
import { useEffect } from "react";
import { fromChainIdUnit } from "shared";
import { StakingRewardsStats } from "@/apollo/queries/stakingRewards/stakingRewardsStats";

function setStakingStats(data: StakingRewardsStats) {
  useStore.setState((state) => {
    state.maximumPicaStaked = fromChainIdUnit(
      data.stakingRewardsStats.totalValueLocked
    );
    state.averageStakingLockTime = `${Math.floor(
      Number(data.stakingRewardsStats.averageLockDuration) / 86400
    )} days`;
  });
}

export function useSubscribeStats() {
  const { data } = useStakingRewardsStats();

  useEffect(() => {
    if (data) setStakingStats(data);
  }, [data]);
}
