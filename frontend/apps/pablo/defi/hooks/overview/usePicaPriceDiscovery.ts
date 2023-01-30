import useStore from "@/store/useStore";
import { usePoolSpotPrice } from "@/defi/hooks/pools/usePoolSpotPrice";
import { getOraclePrice } from "@/store/oracle/slice";
import { useCallback, useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import { Asset } from "shared";

export const usePicaPriceDiscovery = () => {
  const picaUSDTPool = useStore(
    useCallback((store) => {
      return store.pools.config.find((pool) =>
        pool.config.assets.some((asset) => asset.getSymbol() === "PICA")
      );
    }, [])
  );
  const [picaPrice, setPicaPrice] = useState(new BigNumber(0));
  let [pica, usdt] = picaUSDTPool?.config.assets ?? [
    new Asset("", "", "", "pica"),
    new Asset("", "", "", "pica"), // This is intentionally set as invalid.
  ];

  if (pica?.getSymbol() === "USDT") {
    [pica, usdt] = [usdt, pica];
  }

  const { spotPrice } = usePoolSpotPrice(picaUSDTPool, [pica, usdt]);

  useEffect(() => {
    const usdtPrice = getOraclePrice("USDT", "coingecko", "usd");
    setPicaPrice(
      spotPrice.isZero()
        ? spotPrice
        : usdtPrice.multipliedBy(new BigNumber(1).div(spotPrice))
    );
  }, [spotPrice]);

  return picaPrice;
};
