import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { useStore } from "@/stores/root";
import { useCallback, useEffect } from "react";
import { useQuery } from "@apollo/client";
import {
  GET_STAKING_POSITIONS,
  StakingPositions,
} from "@/apollo/queries/stakingPositions";
import {
  callbackGate,
  fromChainIdUnit,
  fromPerbill,
  unwrapNumberOrHex,
} from "shared";
import { useExecutor, useParachainApi } from "substrate-react";
import { fetchRewardPools } from "@/defi/polkadot/pallets/StakingRewards";
import {
  PortfolioItem,
  StakingPortfolio,
} from "@/stores/defi/polkadot/stakingRewards/slice";

export const useStakingRewards = () => {
  // const assetId = meta.supportedNetwork.picasso || 1; // revert once test is done
  const assetId = 130; // Set this to fetch proper asset from config

  const account = useSelectedAccount();
  const { parachainApi } = useParachainApi("picasso");

  const rewardPools = useStore((state) => state.rewardPools);
  const setStakingPositions = useStore((state) => state.setStakingPositions);
  const setStakingPositionLoadingState = useStore(
    (state) => state.setStakingPositionLoadingState
  );
  const setStakingPortfolio = useStore((state) => state.setStakingPortfolio);
  const stakingPositions = useStore((state) => state.stakingPositions);
  const stakingPortfolio = useStore((state) => state.stakingPortfolio);
  const isStakingPositionsLoadingState = useStore(
    (state) => state.isStakingPositionsLoadingState
  );
  const { data, loading, error, refetch } = useQuery<StakingPositions>(
    GET_STAKING_POSITIONS,
    {
      variables: {
        accountId: account?.address,
      },
      pollInterval: 30000,
    }
  );
  const hasRewardPools =
    Object.values(rewardPools).length > 0 && rewardPools[assetId]; // PICA reward pool is necessary
  const { meta } = useStore(
    (state) => state.substrateBalances.picasso.assets.pica
  );
  const balance = useStore(
    (state) => state.substrateBalances.picasso.native.balance
  );
  const setRewardPool = useStore((state) => state.setRewardPool);

  const picaRewardPool = useStore((state) => state.rewardPools[assetId]);

  const executor = useExecutor();

  useEffect(() => {
    callbackGate(
      (api) =>
        fetchRewardPools(api, assetId).then((pool) =>
          callbackGate(
            (poolToStore) => setRewardPool(assetId, poolToStore),
            pool
          )
        ),
      parachainApi
    );
  }, [assetId, parachainApi, setRewardPool]);

  const fetchPortfolio = useCallback(() => {
    callbackGate(
      async (positions, api) => {
        if (loading) return;
        let map: StakingPortfolio = [];
        for (const position of positions) {
          try {
            const result: any = (
              await api.query.stakingRewards.stakes(
                api.createType("u128", position.fnftCollectionId),
                api.createType("u64", position.fnftInstanceId)
              )
            ).toJSON();
            if (result) {
              const item: PortfolioItem = {
                collectionId: position.fnftCollectionId,
                instanceId: position.fnftInstanceId,
                assetId: position.assetId,
                endTimestamp: position.endTimestamp,
                id: position.id,
                unlockPenalty: fromPerbill(
                  rewardPools[assetId].lock.unlockPenalty
                ),
                multiplier:
                  rewardPools[assetId].lock.durationPresets[
                    result.lock.duration
                  ],
                share: fromChainIdUnit(unwrapNumberOrHex(result.share)),
                stake: fromChainIdUnit(unwrapNumberOrHex(result.stake)),
              };
              map = [...map, item];
            }
          } catch (error) {
            console.log(error);
          }

          setStakingPortfolio(map);
        }
      },
      stakingPositions,
      parachainApi
    );
  }, [stakingPositions, parachainApi]);

  // fetch position meta from chain
  useEffect(() => {
    const stakingPositions = data?.stakingPositions;
    setStakingPositions(stakingPositions ?? []);
    setStakingPositionLoadingState(loading);
    fetchPortfolio();
  }, [data, loading, setStakingPortfolio]);

  const refresh = () => {
    fetchPortfolio();
  };

  return {
    picaRewardPool,
    balance,
    meta,
    executor,
    parachainApi,
    assetId,
    stakingPortfolio,
    stakingPositions,
    hasRewardPools,
    isPositionsLoading: isStakingPositionsLoadingState,
    refresh,
  };
};
