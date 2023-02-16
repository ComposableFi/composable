import { useStore } from "@/stores/root";
import { useMemo } from "react";
import BigNumber from "bignumber.js";

export const useStakingRewards = () => {
  const pica = useStore(({ substrateTokens }) => substrateTokens.tokens.pica);
  const rewardPools = useStore((store) => store.rewardPools);
  const stakingPortfolio = useStore((store) => store.stakingPortfolio);
  const stakingRewardsFee = useStore((store) => store.stakingFee);
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
  const maxSpendable = useMemo(() => {
    if (stakingRewardsFee.assetId === pica.id) {
      return balance
        .minus(pica.existentialDeposit.picasso ?? 0)
        .minus(stakingRewardsFee.fee)
        .dp(pica.decimals.picasso ?? 12);
    }
    return balance.minus(pica.existentialDeposit.picasso ?? 0);
  }, [
    balance,
    pica.decimals.picasso,
    pica.existentialDeposit.picasso,
    pica.id,
    stakingRewardsFee.assetId,
    stakingRewardsFee.fee,
  ]);

  return {
    picaRewardPool,
    balance: maxSpendable.lt(0) ? new BigNumber(0) : maxSpendable,
    pica,
    assetId,
    stakingPortfolio,
    hasRewardPools,
    isPositionsLoading: isStakingPositionsLoadingState,
  };
};
