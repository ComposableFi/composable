import { useCallback, useEffect, useRef } from "react";
import useStore from "@/store/useStore";
import { useParachainApi } from "substrate-react";
import { fetchPools } from "@/defi/utils";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { useRouter } from "next/router";
/**
 * Updates zustand store with all pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const {
    pools: { setPoolsList },
  } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const hasFetchedOnce = useRef(false);

  const updatePools = useCallback((url) => {
    if (parachainApi && (!hasFetchedOnce.current || url === "/pool")) {
      if (!hasFetchedOnce.current) hasFetchedOnce.current = true;
      fetchPools(parachainApi).then((pools) => {
        setPoolsList([
          ...pools.liquidityBootstrapping.verified,
          ...pools.constantProduct.verified,
          ...pools.stableSwap.verified,
        ]);
      });
    }
  }, [parachainApi, setPoolsList]);

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
