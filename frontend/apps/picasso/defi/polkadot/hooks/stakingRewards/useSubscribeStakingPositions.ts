import { useSelectedAccount } from "substrate-react";
import { useQuery } from "@apollo/client";
import {
  GET_STAKING_POSITIONS,
  StakingPositions,
} from "@/apollo/queries/stakingPositions";
import config from "@/constants/config";
import { useCallback, useEffect } from "react";
import { useStore } from "@/stores/root";

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
      setStakingPositions(data.stakingPositions);
    }
  }, [data, setStakingPositions]);

  useEffect(() => {
    setStakingPositionsLoading(loading);
  }, [loading, setStakingPositionsLoading]);
}
