import { useEffect } from "react";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "../constants";
import { useAllLpTokenRewardingPools } from "../../store/hooks/useAllLpTokenRewardingPools";
import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import _ from "lodash";
import {
  calculatePoolStats,
  fetchPoolStats,
  PabloPoolStatsSquidResponse,
} from "./utils";

/**
 * Updates zustand store with all pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const { putPoolStats, assets, poolStats, putPoolStatsValue } = useStore();
  const { parachainApi } = useParachainApi("picasso");

  const allLpRewardingPools = useAllLpTokenRewardingPools();

  useEffect(() => {
    console.log(`[PoolStatsUpdater] Update Stats Effect (1)`);
    if (parachainApi && allLpRewardingPools.length) {
      let promises: Promise<PabloPoolStatsSquidResponse[]>[] = [];

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
  }, [parachainApi, allLpRewardingPools]);

  useEffect(() => {
    console.log(`[PoolStatsUpdater] Update Value Effect (2)`);

    if (allLpRewardingPools.length) {
      allLpRewardingPools.forEach((i) => {
        const quoteAsset = getAssetByOnChainId(
          DEFAULT_NETWORK_ID,
          i.pair.quote
        );
        if (quoteAsset && poolStats[i.poolId]) {
          if (assets[quoteAsset.assetId]) {
            const totalVolumeValue = new BigNumber(
              poolStats[i.poolId].totalVolume
            )
              .times(assets[quoteAsset.assetId].price)
              .toFixed(2);
            const _24HrFeeValue = new BigNumber(poolStats[i.poolId]._24HrFee)
              .times(assets[quoteAsset.assetId].price)
              .toFixed(2);
            const _24HrVolumeValue = new BigNumber(
              poolStats[i.poolId]._24HrVolume
            )
              .times(assets[quoteAsset.assetId].price)
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
  }, [assets, allLpRewardingPools.length, poolStats]);

  return null;
};

export default Updater;
