import { useEffect } from "react";
import { fetchAndExtractAuctionStats } from "@/defi/utils/pablo/auctions";
import { fetchAuctionTrades } from "@/defi/subsquid/auctions/helpers";
import {
  setAuctionsSlice,
  setAuctionsSpotPrice,
  useAuctionsSlice,
} from "@/store/auctions/auctions.slice";
import { usePoolsSlice } from "@/store/pools/pools.slice";

const Updater = () => {
  const {
    liquidityBootstrappingPools
  } = usePoolsSlice();
  const { activePool } = useAuctionsSlice();
  /**
   * Queries initiated on an Auctions
   * LBP selection
   */
  useEffect(() => {
    if (activePool) {
      fetchAndExtractAuctionStats(activePool)
        .then((activePoolStats) => {
          setAuctionsSlice({ activePoolStats });
        })
        .catch((err) => {
          console.error(err);
        });
    }
  }, [activePool]);
  /**
   * Update trade history
   * in history tab
   * add apollo here as well
   */
  useEffect(() => {
    if (activePool) {
      fetchAuctionTrades(activePool)
        .then((activePoolTradeHistory) => {
          setAuctionsSlice({ activePoolTradeHistory });
        })
        .catch((err) => {
          console.error(err);
        });
    }
  }, [activePool]);
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
