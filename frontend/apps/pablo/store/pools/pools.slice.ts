import { DualAssetConstantProduct, PabloLiquidityBootstrappingPool } from "shared";
import create from "zustand";

export const usePoolsSlice = create<{ liquidityPools: DualAssetConstantProduct[], liquidityBootstrappingPools: PabloLiquidityBootstrappingPool[] }>(() => ({
  liquidityPools: [],
  liquidityBootstrappingPools: []
}));

export const setPermissionedConstantProductPools = (pools: DualAssetConstantProduct[]) => usePoolsSlice.setState((state) => ({
  ...state,
  constantProductPools: pools
}));
