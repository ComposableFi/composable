import { StoreSlice } from "../../types";
import { ConstantProductPool, ConstantProductPoolsSlice } from "./constantProduct.types";
import { putConstantProductPools } from "./constantProduct.utils";

const createConstantProductPoolsSlice: StoreSlice<ConstantProductPoolsSlice> = (
  set
) => ({
  constantProductPools: { list: [] },
  putConstantProductPools: (constantProductPools: ConstantProductPool[]) =>
    set((prev: ConstantProductPoolsSlice) => ({
      constantProductPools: putConstantProductPools(prev.constantProductPools, constantProductPools),
    })),
});

export default createConstantProductPoolsSlice;
