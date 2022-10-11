import { useMemo } from "react";
import { useAuctionsSlice } from "@/store/auctions/auctions.slice";
import BigNumber from "bignumber.js";

export const useAuctionSpotPrice = (auctionId: number): BigNumber => {
  const { spotPrices } = useAuctionsSlice();

  const spotPrice = useMemo(() => {
    return spotPrices[auctionId.toString()] ?? new BigNumber(0)
  }, [auctionId, spotPrices]);

  return spotPrice;
};
