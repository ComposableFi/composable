import { useEffect } from "react";
import { useAllLpTokenRewardingPools } from "../../store/hooks/useAllLpTokenRewardingPools";
import { fetchPoolStats, calculatePoolStats, PabloPoolQueryResponse } from "@/defi/utils/pablo/pools/stats";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";

/**
 * Updates zustand store with all pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const { apollo, putPoolStats, poolStats, putPoolStatsValue } = useStore();
  const allLpRewardingPools = useAllLpTokenRewardingPools();

  useEffect(() => {
    if (allLpRewardingPools.length) {
      let promises: Promise<PabloPoolQueryResponse[]>[] = [];

      allLpRewardingPools.forEach((pool) => {
        promises.push(fetchPoolStats(pool as any));
      });

      Promise.all(promises).then((subsquidResponses) => {
        const pabloPools = subsquidResponses.filter((k) => k.length);

        pabloPools.forEach((pabloPoolStates) => {
          const poolStats = calculatePoolStats(pabloPoolStates);
          if (poolStats) {
            const {
              poolId,
              _24HrFee,
              _24HrTransactionCount,
              _24HrVolume,
              totalVolume,
            } = poolStats;
            putPoolStats(poolId, {
              _24HrFee,
              _24HrTransactionCount,
              _24HrVolume,
              totalVolume,
            });
          }
        });
      });
    }
  }, [allLpRewardingPools, putPoolStats]);

  useEffect(() => {
    if (allLpRewardingPools.length) {
      allLpRewardingPools.forEach((i) => {

        if (poolStats[i.poolId]) {
          let quoteId = i.pair.quote.toString();

          if (apollo[quoteId]) {
            const totalVolumeValue = new BigNumber(
              poolStats[i.poolId].totalVolume
            )
              .times(apollo[quoteId])
              .toFixed(2);
            const _24HrFeeValue = new BigNumber(poolStats[i.poolId]._24HrFee)
              .times(apollo[quoteId])
              .toFixed(2);
            const _24HrVolumeValue = new BigNumber(
              poolStats[i.poolId]._24HrVolume
            )
              .times(apollo[quoteId])
              .toFixed(2);

            putPoolStatsValue(i.poolId, {
              totalVolumeValue,
              _24HrFeeValue,
              _24HrVolumeValue,
            });
          }
        }
      });
    }
  }, [apollo, allLpRewardingPools, poolStats, putPoolStatsValue]);

  return null;
};

export default Updater;