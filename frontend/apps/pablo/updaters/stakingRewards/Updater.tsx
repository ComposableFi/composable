import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { fetchStakingRewardPools } from "@/defi/utils/stakingRewards";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import {
  putStakingRewardPools,
  putStakingRewardPoolStakedPositions,
} from "@/store/stakingRewards/stakingRewards.slice";
import { useOnChainAssetIds } from "@/store/hooks/useOnChainAssetsIds";
import { useEffect } from "react";
import { fetchStakingPositions } from "@/defi/subsquid/stakingRewards/queries";

const Updater = () => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const onChainAssetIds = useOnChainAssetIds();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  useAsyncEffect(async () => {
    if (parachainApi && onChainAssetIds.size > 0) {
      fetchStakingRewardPools(parachainApi, onChainAssetIds)
        .then(putStakingRewardPools)
        .catch(console.error);
    }
  }, [parachainApi, onChainAssetIds]);

  useEffect(() => {
    if (selectedAccount) {
      fetchStakingPositions(selectedAccount.address)
        .then(putStakingRewardPoolStakedPositions)
        .catch(console.error);
    }
  }, [selectedAccount]);

  return null;
};

export default Updater;
