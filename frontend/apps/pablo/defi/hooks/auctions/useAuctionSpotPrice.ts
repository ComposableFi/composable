import { useMemo } from "react";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";

export const useAuctionSpotPrice = (auctionId: number): BigNumber => {
  const {
    pools: {
      liquidityBootstrappingPools: { spotPrices },
    },
  } = useStore();

  const spotPrice = useMemo(() => {
    return spotPrices.reduce((acc, [lbpAuctionId, spotPrice]) => {
      if (lbpAuctionId === auctionId) {
        return new BigNumber(spotPrice)
      }
      return acc
    }, new BigNumber(0))

  }, [spotPrices, auctionId]);

  return spotPrice;
};
