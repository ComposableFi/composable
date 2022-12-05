import { PabloConstantProductPool } from "shared";
import { usePoolsSlice } from "@/store/pools/pools.slice";

export const useAllLpTokenRewardingPools = (): Array<PabloConstantProductPool> => {
  const {
    constantProductPools
  } = usePoolsSlice();

  return constantProductPools;
};
