import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { callbackGate, fromChainIdUnit, unwrapNumberOrHex } from "shared";
import { Assets } from "@/defi/polkadot/Assets";
import { useCirculatingSupply } from "@/apollo/hooks/useCirculatingSupply";
import { ComposableTraitsOraclePrice } from "defi-interfaces";

export const useMarketCap = () => {
  const circulatingSupply = useCirculatingSupply();
  const [picaPrice, setPicaPrice] = useState<BigNumber>(new BigNumber(0));
  const { parachainApi } = usePicassoProvider();
  useEffect(() => {
    callbackGate((api) => {
      api.query.oracle.prices(
        Assets.pica.supportedNetwork.picasso,
        (result: ComposableTraitsOraclePrice) => {
          if (!result.isEmpty) {
            const { price, block } = result.toJSON() as any;
            setPicaPrice(fromChainIdUnit(unwrapNumberOrHex(price)));
          }
        }
      );
    }, parachainApi);
  }, [parachainApi]);

  return circulatingSupply.multipliedBy(picaPrice);
};
