import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { callbackGate, fromChainIdUnit, unwrapNumberOrHex } from "shared";

import { useCirculatingSupply } from "@/apollo/hooks/useCirculatingSupply";
import { ComposableTraitsOraclePrice } from "defi-interfaces";
import { useStore } from "@/stores/root";

export const useMarketCap = () => {
  const circulatingSupply = useCirculatingSupply();
  const tokens = useStore(({ substrateTokens }) => substrateTokens.tokens);
  const [picaPrice, setPicaPrice] = useState<BigNumber>(new BigNumber(0));
  const { parachainApi } = usePicassoProvider();
  useEffect(() => {
    callbackGate((api, picassoId) => {
      api.query.oracle.prices(
        picassoId.toString(),
        (result: ComposableTraitsOraclePrice) => {
          if (!result.isEmpty) {
            const { price, block } = result.toJSON() as any;
            setPicaPrice(fromChainIdUnit(unwrapNumberOrHex(price)));
          }
        }
      );
    }, parachainApi, tokens.pica.picassoId);
  }, [parachainApi, tokens]);

  return circulatingSupply.multipliedBy(picaPrice);
};
