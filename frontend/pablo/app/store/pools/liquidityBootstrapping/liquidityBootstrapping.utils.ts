import produce from "immer";
import { LBPoolSlice, LiquidityBootstrappingPool } from "./liquidityBootstrapping.types";

export const putLBPList = (
  previousState: LBPoolSlice["liquidityBootstrappingPools"],
  lbPools: LiquidityBootstrappingPool[]
) => {
  return produce(previousState, (_draft) => {
    _draft.list = [ ... lbPools ];
  });
};

export const putLBPListpotPrice = (
  previousState: LBPoolSlice["liquidityBootstrappingPools"],
  price: string,
  index: number
) => {
  return produce(previousState, (_draft) => {
    if (_draft.list[index]) {
      _draft.list[index].spotPrice = price;
    }
  });
};