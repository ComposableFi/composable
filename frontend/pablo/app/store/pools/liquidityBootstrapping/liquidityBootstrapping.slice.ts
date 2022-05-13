import { StoreSlice } from "../../types";
import {
  LiquidityBootstrappingPool,
  LBPoolSlice,
} from "./liquidityBootstrapping.types";
import {
  putLBPList,
  putLBPListpotPrice
} from "./liquidityBootstrapping.utils";

const createLiquidityBootstrappingPoolSlice: StoreSlice<LBPoolSlice> = (
  set
) => ({
  liquidityBootstrappingPools: { list: [] },
  putLBPList: (lbPools: LiquidityBootstrappingPool[]) =>
    set((prev: LBPoolSlice) => ({
      liquidityBootstrappingPools: putLBPList(prev.liquidityBootstrappingPools, lbPools),
    })),
  putLBPSpotPrice: (price: string, index: number) =>
  set((prev: LBPoolSlice) => ({
    liquidityBootstrappingPools: putLBPListpotPrice(prev.liquidityBootstrappingPools, price, index),
  }))
});

export default createLiquidityBootstrappingPoolSlice;
