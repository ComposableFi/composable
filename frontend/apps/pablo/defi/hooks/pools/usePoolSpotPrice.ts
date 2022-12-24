import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID, fetchSpotPrice } from "@/defi/utils";
import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import { Asset, callbackGate } from "shared";
import { PoolConfig } from "@/store/pools/types";

export const usePoolSpotPrice = (
  pool: PoolConfig | undefined,
  input: [Asset, Asset] | undefined
) => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const [spotPrice, setSpotPrice] = useState<BigNumber>(new BigNumber(0));

  useEffect(() => {
    callbackGate(
      (api, poolConfig, inputConfig) => {
        const [assetOne, assetTwo] = inputConfig;
        fetchSpotPrice(
          api,
          poolConfig,
          assetTwo.getPicassoAssetId() as string,
          assetOne.getPicassoAssetId() as string
        ).then(setSpotPrice);
      },
      parachainApi,
      pool,
      input
    );
  }, [input, parachainApi, pool]);

  return {
    spotPrice,
  };
};
