import { useEffect } from "react";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { useParachainApi } from "substrate-react";
import { OperationResult } from "urql";
import _ from "lodash";
import { DAYS, DEFAULT_NETWORK_ID } from "../constants";
import { useAllLpTokenRewardingPools } from "../../store/hooks/useAllLpTokenRewardingPools";
import { queryPabloPoolById } from "../pools/subsquid";
import { getAssetByOnChainId } from "@/defi/polkadot/Assets";

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
      let promises: Promise<OperationResult<any, {}>>[] = [];
      allLpRewardingPools.forEach((pool) => {
        promises.push(queryPabloPoolById(pool.poolId));
      });

      Promise.all(promises).then((subsquidResponses) => {
        const pabloPools = subsquidResponses
          .filter((i) => !!i.data && !!i.data.pabloPools)
          .map((i) => i.data.pabloPools);
  
        pabloPools.forEach((pool) => {
          const _pool = allLpRewardingPools.find(
            (p) => p.poolId === Number(pool[0].poolId)
          );
          let quoteDecimals = 12;
          if (_pool) {
            quoteDecimals = getAssetByOnChainId(
              DEFAULT_NETWORK_ID,
              _pool.pair.quote
            ).decimals;
          }
  
          let yesterday = Number(pool[0].calculatedTimestamp) - 1 * DAYS
  
          const yesterdayState = pool.find((i: any) => (Number(i.calculatedTimestamp) < yesterday))

          let _24HourVolume = new BigNumber(pool[0].totalVolume);
          let _24HourFee = new BigNumber(pool[0].totalFees);
          let _24HourTxCount = new BigNumber(pool[0].transactionCount);
  
          if (yesterdayState) {
            _24HourVolume = new BigNumber(pool[0].totalVolume).minus(
              yesterdayState.totalVolume
            );
            _24HourFee = new BigNumber(pool[0].totalFees).minus(
              yesterdayState.totalFees
            );
            _24HourTxCount = new BigNumber(pool[0].transactionCount).minus(
              yesterdayState.transactionCount
            );
          }
  
          _24HourFee = _24HourFee.div(new BigNumber(10).pow(quoteDecimals));
          _24HourVolume = _24HourVolume.div(new BigNumber(10).pow(quoteDecimals));
          let totalVolume = new BigNumber(pool[0].totalVolume).div(
            new BigNumber(10).pow(quoteDecimals)
          );
  
          putPoolStats(Number(pool[0].poolId), {
            _24HrFee: _24HourFee.toString(),
            _24HrVolume: _24HourVolume.toString(),
            _24HrTransactionCount: _24HourTxCount.toNumber(),
            totalVolume: totalVolume.toString(),
            dailyRewards: [
              {
                assetId: "kusd",
                icon: "/tokens/usd-coin-usdc.svg",
                symbol: "KUSD",
                name: "KUSD",
                rewardAmount: "1000",
                rewardAmountLeft: "1",
              },
            ],
          });
        });
        
      })
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
            const _24HrFeeValue = new BigNumber(
              poolStats[i.poolId]._24HrFee
            )
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
