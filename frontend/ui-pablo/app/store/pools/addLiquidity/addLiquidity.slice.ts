import { StoreSlice } from "../../types";
import { AddLiquiditySlice } from "./addLiquidity.types";
import {
  putFormFieldAddLiquidity,
  putPoolMetadataAddLiquidity,
} from "./addLiquidity.utils";

const createAddLiquiditySlice: StoreSlice<AddLiquiditySlice> = (set) => ({
  addLiquidity: {
    form: {
      baseAssetSelected: "none",
      quoteAssetSelected: "none",
      baseAmount: "0",
      quoteAmount: "0",
    },
    pool: {
      poolId: -1,
      owner: "",
      pair: {
        base: -1,
        quote: -1
      },
      lpToken: "",
      fee: 0,
      ownerFee: 0,
      balance: {
        base: "0",
        quote: "0",
      },
    },
    setFormField: (
      formFeildInput: Partial<AddLiquiditySlice["addLiquidity"]["form"]>
    ) =>
      set((prev: AddLiquiditySlice) => ({
        addLiquidity: putFormFieldAddLiquidity(
          prev.addLiquidity,
          formFeildInput
        ),
      })),
    setPoolMetadata: (
      pool: Partial<AddLiquiditySlice["addLiquidity"]["pool"]>
    ) =>
      set((prev: AddLiquiditySlice) => ({
        addLiquidity: putPoolMetadataAddLiquidity(prev.addLiquidity, pool),
      })),
  },
});

export default createAddLiquiditySlice;
