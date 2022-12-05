import { PabloConstantProductPool, PabloLiquidityBootstrappingPool } from "shared";
import create from "zustand";

export const usePoolsSlice = create<{ constantProductPools: PabloConstantProductPool[], liquidityBootstrappingPools: PabloLiquidityBootstrappingPool[] }>(() => ({
  constantProductPools: [],
  liquidityBootstrappingPools: []
}));

export const setPermissionedConstantProductPools = (pools: PabloConstantProductPool[]) => usePoolsSlice.setState((state) => ({
  ...state,
  constantProductPools: pools
}));
