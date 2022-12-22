import { Asset } from "shared";
import BigNumber from "bignumber.js";
import { PoolConfig } from "@/store/createPool/types";
import {
  queryPabloPoolAssets,
  querySpotPriceFromPool,
} from "@/defi/subsquid/swaps/queries";
import { fromChainUnits } from "@/defi/utils";
import { getOraclePrice } from "@/store/oracle/slice";

export type GetStatsReturn =
  | {
      [key in string]: {
        asset: Asset;
        total: {
          liquidity: BigNumber;
          volume: BigNumber;
        };
        spotPrice: BigNumber;
      };
    }
  | null;

export async function getStats(pool: PoolConfig): Promise<GetStatsReturn> {
  const result = await queryPabloPoolAssets(pool.poolId.toString());
  const poolResponse = result.data;
  const assets = pool.config.assets;
  let stats: GetStatsReturn = null;
  if (result.error || typeof poolResponse === "undefined") {
    return null;
  }
  for (const asset of assets) {
    const assetId = asset.getPicassoAssetId().toString();
    const otherAsset = pool.config.assets.find(
      (a) => a.getPicassoAssetId().toString() !== assetId
    );
    const total = poolResponse.pabloPoolAssets.reduce(
      (acc, cur) => {
        acc.liquidity = acc.liquidity.plus(
          fromChainUnits(cur.totalLiquidity.toString())
        );
        acc.volume = acc.volume.plus(
          fromChainUnits(cur.totalVolume.toString())
        );

        return acc;
      },
      { liquidity: new BigNumber(0), volume: new BigNumber(0) }
    );
    stats ||= {};
    stats[assetId] = {
      total,
      asset,
      spotPrice: new BigNumber(0),
    };

    if (otherAsset) {
      const assetPrice = getOraclePrice(asset.getSymbol(), "coingecko", "usd");
      const otherAssetPrice = getOraclePrice(
        otherAsset.getSymbol(),
        "coingecko",
        "usd"
      );
      // If we have other asset, we can calculate the price based on the pair and ratio
      if (assetPrice.isZero() && !otherAssetPrice.isZero()) {
        const spotPriceResponse = await querySpotPriceFromPool(
          pool.poolId.toString(),
          asset.getPicassoAssetId().toString(),
          otherAsset.getPicassoAssetId().toString()
        );
        const spotPrice =
          spotPriceResponse.data?.pabloSpotPrice?.spotPrice || 0;
        stats[assetId] = {
          total,
          asset,
          spotPrice: new BigNumber(spotPrice).multipliedBy(otherAssetPrice),
        };
      }
      // If we don't have the other asset, and current assetPrice is zero, resolve to 0
      else {
        stats[assetId] = {
          total,
          asset,
          spotPrice: asset.getPrice(),
        };
      }
    }
  }
  return stats;
}

export function getPoolTVL(stats: GetStatsReturn): BigNumber {
  if (stats === null) {
    return new BigNumber(0);
  }

  return Object.values(stats).reduce((acc, cur) => {
    acc = acc.plus(cur.total.volume.multipliedBy(cur.spotPrice));

    return acc;
  }, new BigNumber(0));
}

export function getPoolVolume(stats: GetStatsReturn): BigNumber {
  if (stats === null) {
    return new BigNumber(0);
  }

  return Object.values(stats).reduce((acc, cur) => {
    acc = acc.plus(cur.total.liquidity.multipliedBy(cur.spotPrice));

    return acc;
  }, new BigNumber(0));
}

export function getPriceAndRatio(
  stats: {
    [p: string]: {
      asset: Asset;
      total: { liquidity: BigNumber; volume: BigNumber };
      spotPrice: BigNumber;
    };
  },
  assetOne: Asset,
  amountOne: BigNumber,
  amountTwo: BigNumber,
  assetTwo: Asset
) {
  const spotPriceOfATOB = stats[
    assetOne.getPicassoAssetId().toString()
  ].spotPrice.isZero()
    ? amountOne.div(amountTwo).isNaN()
      ? new BigNumber(0)
      : amountOne.div(amountTwo)
    : stats[assetOne.getPicassoAssetId().toString()].spotPrice;
  const spotPriceOfBToA = stats[
    assetTwo.getPicassoAssetId().toString()
  ].spotPrice.isZero()
    ? amountTwo.div(amountOne).isNaN()
      ? new BigNumber(0)
      : amountTwo.div(amountOne)
    : stats[assetTwo.getPicassoAssetId().toString()].spotPrice;
  const totalLiquidityA =
    stats[assetOne.getPicassoAssetId().toString()].total.liquidity;
  const totalLiquidityB =
    stats[assetTwo.getPicassoAssetId().toString()].total.liquidity;
  const ratioA = totalLiquidityA.isZero()
    ? 100
    : amountOne.div(totalLiquidityA).multipliedBy(100).toNumber();
  const ratioB = totalLiquidityB.isZero()
    ? 100
    : amountTwo.div(totalLiquidityB).multipliedBy(100).toNumber();
  return { spotPriceOfATOB, spotPriceOfBToA, ratioA, ratioB };
}
