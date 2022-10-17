import { useEffect } from "react";
import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { fetchAndExtractAuctionStats, fetchAuctionSpotPrices } from "@/defi/utils/pablo/auctions";
import { fetchAuctionTrades } from "@/defi/subsquid/auctions/helpers";
import useStore from "@/store/useStore";
import {
  setAuctionsSlice,
  useAuctionsSlice,
} from "@/store/auctions/auctions.slice";

const Updater = () => {
  const {
    pools: {
      liquidityBootstrappingPools,
    },
  } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const { activePool } = useAuctionsSlice();
  /**
   * Queries initiated on an Auctions
   * LBP selection
   */
  useEffect(() => {
    const { poolId } = activePool;
    if (parachainApi && poolId !== -1) {
      fetchAndExtractAuctionStats(parachainApi, activePool)
        .then((activePoolStats) => {
          setAuctionsSlice({ activePoolStats });
        })
        .catch((err) => {
          console.error(err);
        });
    }
  }, [parachainApi, activePool]);
  /**
   * Update trade history
   * in history tab
   * add apollo here as well
   */
  useEffect(() => {
    const { poolId } = activePool;
    if (poolId !== -1) {
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
    fetchAuctionSpotPrices(parachainApi, liquidityBootstrappingPools.verified).then((spotPrices) => {
      setAuctionsSlice({ spotPrices })
    })
  }, [
    parachainApi,
    liquidityBootstrappingPools.verified,
  ]);

  return null;
};

export default Updater;
