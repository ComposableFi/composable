import { useEffect } from "react";
import {
  setAuctionsSpotPrice,
  useAuctionsSlice,
} from "@/store/auctions/auctions.slice";
import { usePoolsSlice } from "@/store/pools/pools.slice";

const Updater = () => {
  const {
    liquidityBootstrappingPools
  } = usePoolsSlice();
  /**
   * This effect is called to show prices
   * on auctions page
   */
  useEffect(() => {
    for (const pool of liquidityBootstrappingPools) {
      pool.getSpotPrice().then((spotPrice) => {
        setAuctionsSpotPrice(pool.getPoolId() as string, spotPrice)
      })
    }
  }, [
    liquidityBootstrappingPools,
  ]);

  return null;
};

export default Updater;
