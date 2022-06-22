import useStore from "@/store/useStore";
import { useMemo } from "react";

export const useAllLpTokenRewardingPools = (): any[] => {
  const {
    pools: { stableSwapPools, constantProductPools },
  } = useStore();

  return useMemo(() => {
    const allSS = [...stableSwapPools.verified];
    const allCp = [
      ...
      constantProductPools.verified
    ]

    return allSS.concat(allCp as any[]);
  }, [stableSwapPools, constantProductPools]);
};


