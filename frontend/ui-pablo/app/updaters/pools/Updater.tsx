import { useEffect } from "react";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { useParachainApi } from "substrate-react";
import {
  StableSwapPool,
  ConstantProductPool,
  LiquidityBootstrappingPool,
} from "@/store/pools/pools.types";
import { decodeCpp, decodeLbp, decodeSsp, fetchPools } from "./utils";
import _ from "lodash";

function isVerifiedPool(
  verifiedPoolIds: number[],
  pool: { poolId: number }
): boolean {
  return verifiedPoolIds.some((p) => p === pool.poolId);
}

/**
 * Updates zustand store with all pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const {
    pools: {
      setPoolsList,
    },
  } = useStore();
  const { parachainApi } = useParachainApi("picasso");
  /**
   * Populate all pools
   * from the pallet
   */
  useEffect(() => {
    if (parachainApi) {
      fetchPools(parachainApi).then(pools => {
        console.log('fetchPools', pools)
        setPoolsList(pools.constantProduct.verified, "ConstantProduct", true)
        setPoolsList(pools.constantProduct.unVerified, "ConstantProduct", false)
        setPoolsList(pools.stableSwap.verified, "StableSwap", true)
        setPoolsList(pools.stableSwap.unVerified, "StableSwap", false)
        setPoolsList(pools.liquidityBootstrapping.verified, "LiquidityBootstrapping", true)
        setPoolsList(pools.liquidityBootstrapping.unVerified, "LiquidityBootstrapping", false)
      })
    }
  }, [parachainApi]);


  return null;
};

export default Updater;
