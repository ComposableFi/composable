import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import {
  fetchStakingRewardPools,
} from "@/defi/utils/stakingRewards";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import {
  putStakingRewardPools,
} from "@/store/stakingRewards/stakingRewards.slice";

const Updater = () => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);

  useAsyncEffect(async () => {
    if (parachainApi) {
      fetchStakingRewardPools(parachainApi).then(putStakingRewardPools);
    }
  }, [parachainApi]);

  return null;
};

export default Updater;
