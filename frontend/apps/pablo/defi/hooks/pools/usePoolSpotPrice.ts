import { useParachainApi } from "substrate-react";
import { calculateOutGivenIn, DEFAULT_NETWORK_ID } from "@/defi/utils";
import BigNumber from "bignumber.js";
import { Asset } from "shared";
import { PoolConfig } from "@/store/pools/types";
import { useLiquidity } from "@/defi/hooks";
import { useMemo } from "react";

export const usePoolSpotPrice = (
  pool: PoolConfig | undefined | null,
  input: [Asset, Asset] | undefined | null,
  isReversed: boolean
) => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  // const [spotPrice, setSpotPrice] = useState<BigNumber>(new BigNumber(0));
  const { baseAmount, quoteAmount } = useLiquidity(
    pool as PoolConfig | undefined
  );

  // useEffect(() => {
  //   callbackGate(
  //     (api, poolConfig, inputConfig) => {
  //       const [assetOne, assetTwo] = inputConfig;
  //       fetchSpotPrice(
  //         api,
  //         poolConfig,
  //         assetTwo.getPicassoAssetId() as string,
  //         assetOne.getPicassoAssetId() as string,
  //         assetTwo.getDecimals(DEFAULT_NETWORK_ID)
  //       ).then(setSpotPrice);
  //     },
  //     parachainApi,
  //     pool,
  //     input
  //   );
  // }, [input, parachainApi, pool]);

  // The below calculation is to not use pricesFor
  const spotPrice = useMemo(() => {
    const out = calculateOutGivenIn(
      baseAmount,
      quoteAmount,
      new BigNumber(1),
      new BigNumber(5),
      new BigNumber(5)
    );
    if (isReversed) {
      return new BigNumber(1).div(out);
    }

    return out;
  }, [baseAmount, quoteAmount, isReversed]);
  return {
    spotPrice,
  };
};
