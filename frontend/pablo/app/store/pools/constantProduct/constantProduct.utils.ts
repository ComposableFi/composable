import produce from "immer";
import { ConstantProductPoolsSlice, ConstantProductPool } from "./constantProduct.types";

export const putConstantProductPools = (
  lbpState: ConstantProductPoolsSlice["constantProductPools"],
  constantProductPools: ConstantProductPool[]
) => {
  return produce(lbpState, (draft) => {
    draft.list = [...constantProductPools];
  });
};
