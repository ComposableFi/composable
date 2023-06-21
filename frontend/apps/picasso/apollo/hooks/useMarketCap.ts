import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import {
  callbackGate,
  fromChainIdUnit,
  isPalletSupported,
  unwrapNumberOrHex,
} from "shared";

import { useCirculatingSupply } from "@/apollo/hooks/useCirculatingSupply";
import { ComposableTraitsOraclePrice } from "defi-interfaces";
import { useStore } from "@/stores/root";

export const useMarketCap = () => {
  const circulatingSupply = useCirculatingSupply();
  const tokens = useStore(({ substrateTokens }) => substrateTokens.tokens);
  const [picaPrice, setPicaPrice] = useState<BigNumber>(new BigNumber(0));
  const { parachainApi } = usePicassoProvider();
  useEffect(() => {
    callbackGate(
      (api, picassoId) => {
        isPalletSupported(api)("Oracle")
          ? api.query.oracle.prices(
              picassoId.toString(),
              (result: ComposableTraitsOraclePrice) => {
                if (!result.isEmpty) {
                  setPicaPrice(
                    fromChainIdUnit(
                      unwrapNumberOrHex((result as any).price.toString())
                    )
                  );
                }
              }
            )
          : null;
      },
      parachainApi,
      tokens.pica.chainId.picasso
    );
  }, [parachainApi, tokens]);

  return circulatingSupply.multipliedBy(picaPrice);
};
