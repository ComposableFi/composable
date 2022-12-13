import { DualAssetConstantProduct } from "shared";
import { usePoolsSlice } from "@/store/pools/pools.slice";

export const useAllLpTokenRewardingPools = (): Array<DualAssetConstantProduct> => {
  const {
    liquidityPools
  } = usePoolsSlice();

  return liquidityPools;
};
