import { useStore } from "@/stores/root";

export const useStakingRewards = () => {
  const pica = useStore(({ substrateTokens }) => substrateTokens.tokens.pica);
  const rewardPools = useStore((store) => store.rewardPools);
  const stakingPositions = useStore((store) => store.stakingPositions);
  const stakingPortfolio = useStore((store) => store.stakingPortfolio);
  const isStakingPositionsLoadingState = useStore(
    (store) => store.isStakingPositionsLoading
  );
  const assetId = pica.chainId.picasso?.toNumber() || 1;
  const hasRewardPools =
    Object.values(rewardPools).length > 0 && !!rewardPools[assetId]; // PICA reward pool is necessary
  const balance = useStore(
    (state) => state.substrateBalances.balances.picasso.pica.free
  );
  const picaRewardPool = useStore((store) => store.rewardPools[assetId]);

  return {
    picaRewardPool,
    balance,
    pica,
    assetId,
    stakingPortfolio,
    stakingPositions,
    hasRewardPools,
    isPositionsLoading: isStakingPositionsLoadingState,
  };
};
