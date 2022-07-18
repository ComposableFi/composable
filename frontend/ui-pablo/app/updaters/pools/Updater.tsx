import { useEffect } from "react";
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
  /**
   * Populate all pools
   * from the pallet
   */
  useEffect(() => {
    if (parachainApi) {
      fetchPools(parachainApi).then((pools) => {
        console.log("fetchPools", pools);
        setPoolsList(pools.constantProduct.verified, "ConstantProduct", true);
        // setPoolsList(pools.constantProduct.unVerified, "ConstantProduct", false)
        setPoolsList(pools.stableSwap.verified, "StableSwap", true);
        // setPoolsList(pools.stableSwap.unVerified, "StableSwap", false)
        setPoolsList(
          pools.liquidityBootstrapping.verified,
          "LiquidityBootstrapping",
          true
        );
        // setPoolsList(pools.liquidityBootstrapping.unVerified, "LiquidityBootstrapping", false)
      });
    }
  }, [parachainApi, setPoolsList]);

  const router = useRouter();

  useEffect(() => {
    if (parachainApi) {
      const handleRouteChange = (url: string, params: any) => {
        if (url === "/pool") {
          fetchPools(parachainApi).then((pools) => {
            console.log("fetchPools", pools);
            setPoolsList(
              pools.constantProduct.verified,
              "ConstantProduct",
              true
            );
            // setPoolsList(pools.constantProduct.unVerified, "ConstantProduct", false)
            setPoolsList(pools.stableSwap.verified, "StableSwap", true);
            // setPoolsList(pools.stableSwap.unVerified, "StableSwap", false)
            setPoolsList(
              pools.liquidityBootstrapping.verified,
              "LiquidityBootstrapping",
              true
            );
            // setPoolsList(pools.liquidityBootstrapping.unVerified, "LiquidityBootstrapping", false)
          });
        }
      };

      router.events.on("routeChangeStart", handleRouteChange);

      // If the component is unmounted, unsubscribe
      // from the event with the `off` method:
      return () => {
        router.events.off("routeChangeStart", handleRouteChange);
      };
    }
  }, [parachainApi, router, setPoolsList]);

  return null;
};

export default Updater;
