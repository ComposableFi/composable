import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import useStore from "@/store/useStore";
import { useMemo } from "react";

export const useAllLpTokenRewardingPools = (): Array<StableSwapPool | ConstantProductPool> => {
  const {
    pools: { stableSwapPools, constantProductPools },
  } = useStore();

  return useMemo(() => {
    return [...constantProductPools.verified, ...stableSwapPools.verified];
  }, [stableSwapPools, constantProductPools]);
};
