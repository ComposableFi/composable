import useStore from "@/store/useStore";
import { useMemo } from "react";

export const useAllLpTokenRewardingPools = (): any[] => {
  const {
    pools: { stableSwapPools, constantProductPools },
  } = useStore();

  return useMemo(() => {
    const allSS = stableSwapPools.unVerified.concat(stableSwapPools.verified);
    const allCp = constantProductPools.unVerified.concat(
      constantProductPools.verified
    );

    return allSS.concat(allCp as any[]);
  }, [stableSwapPools, constantProductPools]);
};


