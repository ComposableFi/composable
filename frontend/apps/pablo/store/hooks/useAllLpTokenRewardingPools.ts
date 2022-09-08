import useStore from "@/store/useStore";
import { useMemo } from "react";

export const useAllLpTokenRewardingPools = (): any[] => {
  const {
    pools: { stableSwapPools, constantProductPools },
  } = useStore();

  return useMemo(() => {
    return [...constantProductPools.verified, ...stableSwapPools.verified];
  }, [stableSwapPools, constantProductPools]);
};
