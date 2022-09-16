import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { useStore } from "@/stores/root";
import { useEffect } from "react";
import { useQuery } from "@apollo/client";
import { GET_STAKING_POSITIONS, StakingPositions } from "@/apollo/queries/stakingPositions";
import { callbackGate, unwrapNumberOrHex } from "shared";
import { useExecutor } from "substrate-react";
import { fetchRewardPools } from "@/defi/polkadot/pallets/StakingRewards";

export const useStakingRewards = () => {
  const account = useSelectedAccount();
  const { parachainApi } = usePicassoProvider();
  const rewardPools = useStore(state => state.rewardPools);
  const setStakingPositions = useStore(state => state.setStakingPositions);
  const setStakingPositionLoadingState = useStore(state => state.setStakingPositionLoadingState);
  const setStakingPortfolio = useStore(state => state.setStakingPortfolio);
  const stakingPositions = useStore(state => state.stakingPositions);
  const stakingPortfolio = useStore(state => state.stakingPortfolio);
  const isStakingPositionsLoadingState = useStore(state => state.isStakingPositionsLoadingState);
  const { data, loading, error } = useQuery<StakingPositions>(GET_STAKING_POSITIONS, {
    variables: {
      accountId: account?.address
    },
    pollInterval: 30000
  });

  const { meta } = useStore(
    (state) => state.substrateBalances.picasso.assets.pica
  );
  const balance = useStore(
    (state) => state.substrateBalances.picasso.native.balance
  );
  const setRewardPool = useStore((state) => state.setRewardPool);
  const assetId = meta.supportedNetwork.picasso || 1;
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

  // fetch position meta from chain
  useEffect(() => {
    const stakingPositions = data?.stakingPositions;
    setStakingPositions(stakingPositions ?? []);
    setStakingPositionLoadingState(loading);
    callbackGate(async (positions, api) => {
      if (loading) return;
      let map: any = {};
      for (const position of positions) {
        try {
          const result: any = (await api.query.stakingRewards.stakes(
            api.createType("u128", position.fnftCollectionId),
            api.createType("u64", position.fnftInstanceId)
          )).toJSON();
          map = {
            ...map,
            [position.fnftCollectionId]: {
              ...map[position.fnftCollectionId],
              [position.fnftInstanceId]: {
                multiplier: rewardPools[1].lock.durationPresets[result.lock.duration],
                share: unwrapNumberOrHex(result.share),
                stake: unwrapNumberOrHex(result.stake)
              }
            }
          };
        } catch (error) {
          console.log(error);
        } finally {
          setStakingPortfolio(map);
        }
      }


    }, stakingPositions, parachainApi);
  }, [data?.stakingPositions, loading, setStakingPortfolio]);


  return {
    picaRewardPool,
    balance,
    meta,
    executor,
    parachainApi,
    assetId,
    stakingPortfolio,
    stakingPositions,
    isPositionsLoading: isStakingPositionsLoadingState
  };
};
