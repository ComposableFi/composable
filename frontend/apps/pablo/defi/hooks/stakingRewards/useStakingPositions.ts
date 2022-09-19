import { Stake, StakingPositionHistory, StakingRewardPool } from "@/defi/types";
import { DEFAULT_NETWORK_ID, fetchXTokenBalances } from "@/defi/utils";
import { useOwnedFinancialNfts, putXTokenBalances, useXTokenBalances } from "@/store/financialNfts/financialNfts.slice";
import { useStakingRewardPool, useStakedPositionHistory } from "@/store/stakingRewards/stakingRewards.slice";
import { useParachainApi } from "substrate-react";
import { useEffect, useMemo, useState } from "react";
import { decodeStake } from "@/defi/utils/stakingRewards";
import BigNumber from "bignumber.js";

export interface StakingPositionsProps {
  stakedAssetId?: string;
}

export function useStakingPositions({
  stakedAssetId,
}: StakingPositionsProps): {
  stakingRewardPool: StakingRewardPool | undefined,
  ownedFinancialNftsHistory: StakingPositionHistory[],
  stakes: Stake[],
  xTokenBalances: Record<string, BigNumber>
} {
  const ownedFinancialNfts = useOwnedFinancialNfts();
  const stakingRewardPool = useStakingRewardPool(
    stakedAssetId ? stakedAssetId : "-"
  );
  const stakingPositions = useStakedPositionHistory(
    stakedAssetId ? stakedAssetId : "-"
  );
  /**
   * For a given asset is,
   * extract the financial NFT
   * collection Id
   */
  const financialNftCollectionId = useMemo(() => {
    if (!stakingRewardPool) return null;

    return stakingRewardPool.financialNftAssetId;
  }, [stakingRewardPool]);
  /**
   * Extract user staking
   * events previously from
   * subsquid
   */
  const userStakingEvents = useMemo(() => {
    if (!financialNftCollectionId) return [];

    return stakingPositions.filter(
      (position) => position.fnftCollectionId === financialNftCollectionId
    );
  }, [financialNftCollectionId, stakingPositions]);
  /**
   * query chain for currently
   * owned financial NFTs by the
   * user
   */
  const ownedFinancialNftsHistory = useMemo(() => {
    if (userStakingEvents.length <= 0 || financialNftCollectionId === null)
      return [];
    if (ownedFinancialNfts[financialNftCollectionId] === undefined) return [];

    return userStakingEvents.filter((x) => {
      return ownedFinancialNfts[financialNftCollectionId].includes(
        x.fnftInstanceId
      );
    });
  }, [userStakingEvents, ownedFinancialNfts, financialNftCollectionId]);
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const [stakes, setStakes] = useState<Array<Stake>>([]);
  /**
   * Here we query the `stakes`
   * storage from chain w.r.t
   * NFT Collection Id and instance ID
   * this is only used to show
   * xToken multiplier on UI
   */
  useEffect(() => {
    if (!parachainApi || ownedFinancialNftsHistory.length <= 0) return;
    let allPromises = ownedFinancialNftsHistory.map((stake) =>
      parachainApi.query.stakingRewards.stakes(
        parachainApi.createType("u128", stake.fnftCollectionId),
        parachainApi.createType("u128", stake.fnftInstanceId)
      )
    );
    Promise.all(allPromises)
      .then((response) => {
        const result = response.map((stake) => decodeStake(stake));
        setStakes(result);
      })
      .catch(console.error);
  }, [parachainApi, ownedFinancialNftsHistory]);
  /**
   * This effect will be used to store
   * xTokens balances, currently not
   * represented globally on UI so we
   * the hook
   */
  useEffect(() => {
    if (!parachainApi || !stakingRewardPool) return;
    if (
      !!financialNftCollectionId &&
      !(financialNftCollectionId in ownedFinancialNfts)
    )
      return;

    if (
      ownedFinancialNftsHistory.length > 0 &&
      financialNftCollectionId &&
      financialNftCollectionId in ownedFinancialNfts
    ) {
      fetchXTokenBalances(
        parachainApi,
        ownedFinancialNftsHistory,
        stakingRewardPool
      )
        .then(putXTokenBalances)
        .catch(console.error);
    }
  }, [
    parachainApi,
    financialNftCollectionId,
    stakingRewardPool,
    ownedFinancialNftsHistory,
    ownedFinancialNfts,
  ]);
  const xTokenBalances = useXTokenBalances(
    financialNftCollectionId ? financialNftCollectionId : "-"
  );

  return {
    xTokenBalances,
    stakingRewardPool,
    ownedFinancialNftsHistory,
    stakes
  };
}
