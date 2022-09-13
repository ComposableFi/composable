import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import {
  fetchStakingRewardPools,
} from "@/defi/utils/stakingRewards";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import {
  putStakingRewardPools,
} from "@/store/stakingRewards/stakingRewards.slice";
import { useOnChainAssetIds } from "@/store/hooks/useOnChainAssetsIds";

const Updater = () => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const onChainAssetIds = useOnChainAssetIds();

  useAsyncEffect(async () => {
    if (parachainApi && onChainAssetIds.size > 0) {
      fetchStakingRewardPools(parachainApi, onChainAssetIds).then(putStakingRewardPools);
    }
  }, [parachainApi, onChainAssetIds]);

  return null;
};

export default Updater;
