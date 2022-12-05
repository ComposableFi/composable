import { useEffect } from "react";
import { fetchPoolStats, calculatePoolStats, PabloPoolQueryResponse } from "@/defi/utils/pablo/pools/stats";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { useAllLpTokenRewardingPools } from "@/defi/hooks";

/**
 * Updates zustand store with all pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const { putPoolStats, poolStats, putPoolStatsValue } = useStore();
  const allPermissionedConstantProductPools = useAllLpTokenRewardingPools();

  useEffect(() => {
    if (allPermissionedConstantProductPools.length) {
      let promises: Promise<PabloPoolQueryResponse[]>[] = [];

      allPermissionedConstantProductPools.forEach((pool) => {
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
  }, [allPermissionedConstantProductPools, putPoolStats]);

  // useEffect(() => {
  //   if (allPermissionedConstantProductPools.length) {
  //     allPermissionedConstantProductPools.forEach((i) => {
  //       const id = (i.getPoolId(true) as BigNumber).toNumber();

  //       if (poolStats[id]) {
  //         let quoteId = i.getPair().getQuoteAsset().toString();

  //         if (apollo[quoteId]) {
  //           const totalVolumeValue = new BigNumber(
  //             poolStats[id].totalVolume
  //           )
  //             .times(apollo[quoteId])
  //             .toFixed(2);
  //           const _24HrFeeValue = new BigNumber(poolStats[id]._24HrFee)
  //             .times(apollo[quoteId])
  //             .toFixed(2);
  //           const _24HrVolumeValue = new BigNumber(
  //             poolStats[id]._24HrVolume
  //           )
  //             .times(apollo[quoteId])
  //             .toFixed(2);

  //           putPoolStatsValue(id, {
  //             totalVolumeValue,
  //             _24HrFeeValue,
  //             _24HrVolumeValue,
  //           });
  //         }
  //       }
  //     });
  //   }
  // }, [apollo, allPermissionedConstantProductPools, poolStats, putPoolStatsValue]);

  return null;
};

export default Updater;