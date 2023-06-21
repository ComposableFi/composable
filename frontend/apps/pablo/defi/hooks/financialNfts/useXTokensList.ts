import { useMemo } from "react";
import { useStakingPositions } from "../stakingRewards/useStakingPositions";
import { StakedFinancialNftPosition, StakingPositionHistory } from "@/defi/types";
import { fromChainUnits, DATE_FORMAT } from "@/defi/utils";
import moment from "moment";
import BigNumber from "bignumber.js";

export interface XTokensListProps {
  stakedAssetId?: string;
}

export function useXTokensList({
  stakedAssetId,
}: XTokensListProps): StakedFinancialNftPosition[] {
    const {
      stakes,
      ownedFinancialNftsHistory,
      stakingRewardPool,
      xTokenBalances
    } = useStakingPositions({ stakedAssetId });


  /**
   * Create formatted list of
   * owned financial NFTs to display
   * as is via UI components
   */
  const xPositions = useMemo(() => {
    if (
      !(ownedFinancialNftsHistory.length > 0) ||
      !(stakes.length > 0) ||
      !stakingRewardPool
    )
      return [];

    return ownedFinancialNftsHistory.map(
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
  }, [stakes, ownedFinancialNftsHistory, stakingRewardPool, xTokenBalances]);
  return xPositions;
}
