import { Asset } from "shared";
import BigNumber from "bignumber.js";
import { PoolConfig } from "@/store/createPool/types";
import { useEffect, useMemo, useState } from "react";
import useStore from "@/store/useStore";
import { fetchPabloPool } from "@/defi/subsquid/pabloPool";
import { getOraclePrice } from "@/store/oracle/slice";
import { DEFAULT_NETWORK_ID, fromChainUnits } from "@/defi/utils";

type PoolAsset = {
  asset: Asset;
  totalLiquidity: BigNumber;
  totalVolume: BigNumber;
  usdPrice: BigNumber;
};

export const usePoolTotalVolume = (pool: PoolConfig) => {
  const [poolAssets, setPoolAssets] = useState<PoolAsset[]>([]);
  const tokens = useStore((store) => store.substrateTokens.tokens);
  const isPoolFetched = useStore((store) => store.pools.isLoaded);

  useEffect(() => {
    if (isPoolFetched) {
      fetchPabloPool(pool.poolId.toNumber()).then((pabloPool) => {
        if (!pabloPool) {
          return;
        }

        const poolAssets = pabloPool.poolAssets
          .map((poolAsset) => {
            const asset = Object.values(tokens).find(
              (token) =>
                (token.getPicassoAssetId() as string) === poolAsset.assetId
            );
            if (!asset) return null;
            const price = getOraclePrice(asset.getSymbol(), "coingecko", "usd");
            return {
              asset: asset,
              totalLiquidity: new BigNumber(
                fromChainUnits(
                  poolAsset.totalLiquidity,
                  asset.getDecimals(DEFAULT_NETWORK_ID)
                )
              ),
              totalVolume: new BigNumber(
                fromChainUnits(
                  poolAsset.totalVolume,
                  asset.getDecimals(DEFAULT_NETWORK_ID)
                )
              ),
              usdPrice: price,
            };
          })
          .filter((v): v is PoolAsset => v !== null);

        setPoolAssets(poolAssets);
      });
    }
  }, [isPoolFetched, pool]);

  const totalVolume = useMemo(() => {
    if (poolAssets.length !== 2) return new BigNumber(0);

    const [poolAssetOne, poolAssetTwo] = poolAssets;

    return poolAssetOne.usdPrice.isZero()
      ? poolAssetTwo.usdPrice
          .multipliedBy(2)
          .multipliedBy(poolAssetTwo.totalVolume)
      : poolAssetOne.usdPrice
          .multipliedBy(2)
          .multipliedBy(poolAssetOne.totalVolume);
  }, [poolAssets]);

  return totalVolume;
};
