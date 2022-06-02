import { AssetMetadata, getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import _ from "lodash";
import { useMemo } from "react";
import { DailyRewards, PoolStats } from "../poolStats/poolStats.types";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";

export const useLiquidityPoolsList = (): {
  poolId: number;
  baseAsset: AssetMetadata;
  quoteAsset: AssetMetadata;
  volume: BigNumber;
  apr: BigNumber;
  tvl: BigNumber;
  dailyRewards: DailyRewards[];
  lpAssetId: string;
}[] => {
  const { poolStats, liquidity, assets } = useStore();
  const allLpRewardingPools = useAllLpTokenRewardingPools();

  const liquidityPools = useMemo(() => {
    let statsMap = Object.entries(poolStats).reduce((p, c) => {
      let poolId = Number(c[0]);
      return {
        ...p,
        [poolId]: c[1],
      };
    }, {} as { [poolId: number]: PoolStats });

    let list: any[] = [];

    allLpRewardingPools.forEach((p) => {
      let base = getAssetByOnChainId("picasso", p.pair.base);
      let quote = getAssetByOnChainId("picasso", p.pair.quote);
      const basePrice = assets[base.assetId].price;
      const quotePrice = assets[quote.assetId].price;

      let volume = new BigNumber(0),
        liq = new BigNumber(0),
        apr = new BigNumber(0),
        dailyRewards: DailyRewards[] = [];

      if (statsMap[p.poolId] && liquidity[p.poolId]) {
        let baseLiq = new BigNumber(
          liquidity[p.poolId].tokenAmounts.baseAmount
        );
        let quoteLiq = new BigNumber(
          liquidity[p.poolId].tokenAmounts.quoteAmount
        );

        baseLiq = new BigNumber(basePrice).times(baseLiq);
        quoteLiq = new BigNumber(quotePrice).times(quoteLiq);
        volume = volume.plus(statsMap[p.poolId].totalVolumeValue);
        liq = quoteLiq.plus(baseLiq);
        dailyRewards = statsMap[p.poolId].dailyRewards;
      }

      list.push({
        baseAsset: base,
        quoteAsset: quote,
        volume: volume,
        poolId: p.poolId,
        apr,
        tvl: liq,
        dailyRewards,
        lpAssetId: p.lpToken,
      });
    });

    return list;
  }, [poolStats, allLpRewardingPools.length, liquidity]);

  return liquidityPools;
};
