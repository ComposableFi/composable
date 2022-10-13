import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import { callbackGate, fromChainIdUnit, unwrapNumberOrHex } from "shared";
import { u128 } from "@polkadot/types-codec";

export const useCirculatingSupply = () => {
  const { parachainApi } = usePicassoProvider();
  const [circulatingSupply, setCirculatingSupply] = useState<BigNumber>(
    new BigNumber(0)
  );

  useEffect(() => {
    callbackGate((api) => {
      api.query.balances.totalIssuance((totalIssuance: u128) =>
        setCirculatingSupply(
          fromChainIdUnit(unwrapNumberOrHex(totalIssuance.toHex()))
        )
      );
    }, parachainApi);
  }, [parachainApi]);

  return circulatingSupply;
};
