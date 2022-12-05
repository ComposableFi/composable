import { useCallback, useEffect, useRef } from "react";
import useStore from "@/store/useStore";
import { useParachainApi } from "substrate-react";
import { LiquidityPoolFactory } from "shared";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { useRouter } from "next/router";
import { setPermissionedConstantProductPools } from "@/store/pools/pools.slice";
/**
 * Updates zustand store with all pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const {
    substrateTokens
  } = useStore();
  const { tokens, hasFetchedTokens } = substrateTokens;

  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const hasFetchedOnce = useRef(false);

  const updatePools = useCallback((url) => {
    const assets = Object.values(tokens);
    if (parachainApi && hasFetchedTokens && assets.length > 0 && (!hasFetchedOnce.current || url === "/pool")) {
      if (!hasFetchedOnce.current) hasFetchedOnce.current = true;
      LiquidityPoolFactory.fetchPermissionedPools(parachainApi, assets).then((pools) => {
        setPermissionedConstantProductPools(pools.uniswapConstantProduct);
      });
    }
  }, [hasFetchedTokens, parachainApi, tokens]);

  /**
   * Populate all pools
   * from the pallet
   */
  useEffect(() => {
    updatePools("");
  }, [updatePools]);

  const router = useRouter();

  useEffect(() => {
      router.events.on("routeChangeStart", updatePools);

      // If the component is unmounted, unsubscribe
      // from the event with the `off` method:
      return () => {
        router.events.off("routeChangeStart", updatePools);
      };
  }, [router, updatePools]);

  return null;
};

export default Updater;
