import { useParachainApi } from "substrate-react";
import {
  DATE_FORMAT,
  DEFAULT_NETWORK_ID,
  fetchXTokenBalances,
  fromChainUnits,
} from "@/defi/utils";
import { useEffect, useMemo, useState } from "react";
import {
  putXTokenBalances,
  useOwnedFinancialNfts,
  useXTokenBalances,
} from "../financialNfts/financialNfts.slice";
import {
  useStakedPositionHistory,
  useStakingRewardPool,
} from "../stakingRewards/stakingRewards.slice";
import { decodeStake } from "@/defi/utils/stakingRewards";
import {
  Stake,
  StakedFinancialNftPosition,
  StakingPositionHistory,
} from "@/defi/types";
import moment from "moment";
import BigNumber from "bignumber.js";

export interface StakingPositionsProps {
  stakedAssetId?: string;
}

export function useStakingPositions({
  stakedAssetId,
}: StakingPositionsProps): StakedFinancialNftPosition[] {
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
  const currentlyOwnedFinancialNFtsHistory = useMemo(() => {
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
    if (!parachainApi || currentlyOwnedFinancialNFtsHistory.length <= 0) return;
    let allPromises = currentlyOwnedFinancialNFtsHistory.map((stake) =>
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
  }, [parachainApi, currentlyOwnedFinancialNFtsHistory]);
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
      currentlyOwnedFinancialNFtsHistory.length > 0 &&
      financialNftCollectionId &&
      financialNftCollectionId in ownedFinancialNfts
    ) {
      fetchXTokenBalances(
        parachainApi,
        currentlyOwnedFinancialNFtsHistory,
        stakingRewardPool
      )
        .then(putXTokenBalances)
        .catch(console.error);
    }
  }, [
    parachainApi,
    financialNftCollectionId,
    stakingRewardPool,
    currentlyOwnedFinancialNFtsHistory,
    ownedFinancialNfts,
  ]);
  const xTokenBalances = useXTokenBalances(
    financialNftCollectionId ? financialNftCollectionId : "-"
  );
  /**
   * Create formatted list of
   * owned financial NFTs to display
   * as is via UI components
   */
  const xPositions = useMemo(() => {
    if (
      !(currentlyOwnedFinancialNFtsHistory.length > 0) ||
      !(stakes.length > 0) ||
      !stakingRewardPool
    )
      return [];

    return currentlyOwnedFinancialNFtsHistory.map(
      (financialNftPosition: StakingPositionHistory) => {
        const { amount, fnftInstanceId, endTimestamp } = financialNftPosition;
        const lockedPrincipalAsset = fromChainUnits(amount);
        const expiryDate = moment(
          new BigNumber(endTimestamp).toNumber()
        ).format(DATE_FORMAT);
        const stakeForMultiplier = stakes.find(
          (stake) => stake.fnftInstanceId === fnftInstanceId
        );
        const preset = stakeForMultiplier
          ? stakeForMultiplier.lock.duration.toString()
          : null;
        const multiplier =
          preset === null || !(preset in stakingRewardPool.lock.durationPresets)
            ? "-"
            : `${stakingRewardPool.lock.durationPresets[preset].toString()} %`;
        let xTokenBalance = new BigNumber(0);
        if (fnftInstanceId in xTokenBalances) {
          xTokenBalance = xTokenBalances[fnftInstanceId];
        }
        const isExpired =
          Date.now() >
          new BigNumber(financialNftPosition.endTimestamp).toNumber();

        return {
          lockedPrincipalAsset,
          nftId: fnftInstanceId,
          expiryDate,
          isExpired,
          multiplier,
          xTokenBalance,
        };
      }
    );
  }, [stakes, currentlyOwnedFinancialNFtsHistory, stakingRewardPool, xTokenBalances]);
  return xPositions;
}
