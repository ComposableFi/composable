import { useSelectedAccount } from "substrate-react";
import { useQuery } from "@apollo/client";
import {
  GET_STAKING_POSITIONS,
  StakingPositions,
} from "@/apollo/queries/stakingRewards/stakingPositions";
import config from "@/constants/config";
import { useCallback, useEffect } from "react";
import { useStore } from "@/stores/root";
import { getFnftKey } from "@/defi/polkadot/pallets/StakingRewards";

export function useSubscribeStakingPositions() {
  const account = useSelectedAccount(config.defaultNetworkId);
  const { data, loading } = useQuery<StakingPositions>(GET_STAKING_POSITIONS, {
    variables: {
      accountId: account?.address,
    },
    pollInterval: 10000,
  });

  const setStakingPositions = useStore(
    useCallback((state) => state.setStakingPositions, [])
  );
  const setStakingPositionsLoading = useStore(
    useCallback((state) => state.setStakingPositionLoading, [])
  );

  useEffect(() => {
    if (data) {
      setStakingPositions(
        new Map(
          data.stakingPositions.map((item) => [
            getFnftKey(item.fnftCollectionId, item.fnftInstanceId),
            item,
          ])
        )
      );
    }
  }, [data, setStakingPositions]);

  useEffect(() => {
    setStakingPositionsLoading(loading);
  }, [loading, setStakingPositionsLoading]);
}
