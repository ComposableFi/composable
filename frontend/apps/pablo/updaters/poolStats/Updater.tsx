import { useEffect } from "react";
import { fetchPoolStats, calculatePoolStats, PabloPoolQueryResponse } from "@/defi/utils/pablo/pools/stats";
import useStore from "@/store/useStore";
import { useAllLpTokenRewardingPools } from "@/defi/hooks";

/**
 * Updates zustand store with all pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const { putPoolStats } = useStore();
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

  return null;
};

export default Updater;