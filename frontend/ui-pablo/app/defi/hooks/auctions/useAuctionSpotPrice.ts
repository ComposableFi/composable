import { useMemo } from "react";
import useStore from "@/store/useStore";

export const useAuctionSpotPrice = (auctionId: number): string => {
  const {
    pools: {
      liquidityBootstrappingPools: { spotPrices },
    },
  } = useStore();

  const spotPrice = useMemo(() => {
    let e = spotPrices.find((s) => s[0] === auctionId);
    return e ? e[1] : "0";
  }, [spotPrices, auctionId]);

  return spotPrice;
};
