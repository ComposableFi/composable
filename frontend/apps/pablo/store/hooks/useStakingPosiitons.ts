import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID, fromChainUnits } from "@/defi/utils";
import { useEffect, useMemo, useState } from "react";
import { useOwnedFinancialNfts } from "../financialNfts/financialNfts.slice";
import {
  useStakedPositionHistory,
  useStakingRewardPool,
} from "../stakingRewards/stakingRewards.slice";
import { decodeStake } from "@/defi/utils/stakingRewards";
import { Stake, StakedFinancialNftPosition, StakingPositionHistory } from "@/defi/types";
import moment from "moment";
import BigNumber from "bignumber.js";

export interface StakingPositionsProps {
  stakedAssetId?: string;
}

export function useStakingPositions({ stakedAssetId }: StakingPositionsProps): StakedFinancialNftPosition[] {
  const ownedFinancialNfts = useOwnedFinancialNfts();
  const stakingRewardPool = useStakingRewardPool(
    stakedAssetId ? stakedAssetId : "-"
  );
  const stakingPositions = useStakedPositionHistory(
    stakedAssetId ? stakedAssetId : "-"
  );

  const financialNftCollectionId = useMemo(() => {
    if (!stakingRewardPool) return null;

    return stakingRewardPool.financialNftAssetId;
  }, [stakingRewardPool]);

  const userStakingEvents = useMemo(() => {
    if (!financialNftCollectionId) return [];

    return stakingPositions.filter(
      (position) => position.fnftCollectionId === financialNftCollectionId
    );
  }, [financialNftCollectionId, stakingPositions]);

  const currentlyOwnedFinancialNFts = useMemo(() => {
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
  useEffect(() => {
    if (!parachainApi || currentlyOwnedFinancialNFts.length <= 0) return;

    let allPromises = currentlyOwnedFinancialNFts.map((stake) =>
      parachainApi.query.stakingRewards.stakes(
        parachainApi.createType("u128", stake.fnftCollectionId),
        parachainApi.createType("u128", stake.fnftInstanceId)
      )
    );

    Promise.all(allPromises).then((response) => {
      const result = response.map((stake) => decodeStake(stake));
      setStakes(result);
    });
  }, [parachainApi, currentlyOwnedFinancialNFts]);

  const xPositions = useMemo(() => {
    if (
      !(currentlyOwnedFinancialNFts.length > 0) ||
      !(stakes.length > 0) ||
      !stakingRewardPool
    )
      return [];

    return currentlyOwnedFinancialNFts.map(
      (financialNftPosition: StakingPositionHistory) => {
        const lockedPrincipalAsset = fromChainUnits(financialNftPosition.amount);
        const nftId = financialNftPosition.fnftInstanceId;
        const expiryDate = moment(
          new BigNumber(financialNftPosition.endTimestamp).toNumber()
        ).format("DD/MM/YYYY");
        const stakeForMultiplier = stakes.find(
          (stake) =>
            stake.fnftInstanceId === financialNftPosition.fnftInstanceId
        );
        const preset = stakeForMultiplier
          ? stakeForMultiplier.lock.duration.toString()
          : null;
        const multiplier =
          preset === null || !(preset in stakingRewardPool.lock.durationPresets)
            ? "-"
            : `${stakingRewardPool.lock.durationPresets[preset].toString()} %`;
        const xTokenBalance = new BigNumber(0);
        const isExpired = Date.now() > new BigNumber(financialNftPosition.endTimestamp).toNumber()

        return {
          lockedPrincipalAsset,
          nftId,
          expiryDate,
          isExpired,
          multiplier,
          xTokenBalance,
        };
      }
    );
  }, [stakes, currentlyOwnedFinancialNFts, stakingRewardPool]);

  return xPositions;
}
