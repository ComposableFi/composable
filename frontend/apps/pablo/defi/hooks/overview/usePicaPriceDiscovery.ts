import useStore from "@/store/useStore";
import { usePoolSpotPrice } from "@/defi/hooks/pools/usePoolSpotPrice";
import { getOraclePrice } from "@/store/oracle/slice";
import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import { PoolConfig } from "@/store/pools/types";
import { Asset } from "shared";

export const usePicaPriceDiscovery = () => {
  const pools = useStore((store) => store.pools.config);
  const [picaPrice, setPicaPrice] = useState(new BigNumber(0));
  const [picaUSDTPool, setPicaUSDTPool] = useState<PoolConfig | undefined>(
    undefined
  );
  let [pica, usdt] = picaUSDTPool?.config.assets ?? [
    new Asset("", "", "", "pica"),
    new Asset("", "", "", "pica"), // This is intentionally set as invalid.
  ];

  if (pica?.getSymbol() === "USDT") {
    [pica, usdt] = [usdt, pica];
  }

  const { spotPrice } = usePoolSpotPrice(picaUSDTPool, [pica, usdt]);

  useEffect(() => {
    const found = pools.find((pool) =>
      pool.config.assets.some((asset) => asset.getSymbol() === "PICA")
    );

    if (found) {
      setPicaUSDTPool(found);
    }
  }, [pools]);

  useEffect(() => {
    const usdtPrice = getOraclePrice("USDT", "coingecko", "usd");
    setPicaPrice(usdtPrice.multipliedBy(new BigNumber(1).div(spotPrice)));
  }, [spotPrice.toString(16)]);

  return picaPrice;
};
