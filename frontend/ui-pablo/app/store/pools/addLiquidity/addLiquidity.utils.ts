import produce from "immer";
import { AddLiquiditySlice } from "./addLiquidity.types";

export const putFormFieldAddLiquidity = (
  addLiqState: AddLiquiditySlice["addLiquidity"],
  formFeildInput: Partial<AddLiquiditySlice["addLiquidity"]["form"]>
) => {
  return produce(addLiqState, (draft) => {
    draft.form.baseAssetSelected =
      formFeildInput.baseAssetSelected ?? addLiqState.form.baseAssetSelected;
    draft.form.quoteAssetSelected =
      formFeildInput.quoteAssetSelected ?? addLiqState.form.quoteAssetSelected;
    draft.form.quoteAmount =
      formFeildInput.quoteAmount ?? addLiqState.form.quoteAmount;
    draft.form.baseAmount =
      formFeildInput.baseAmount ?? addLiqState.form.baseAmount;
  });
};

export const putPoolMetadataAddLiquidity = (
  addLiqState: AddLiquiditySlice["addLiquidity"],
  pool: Partial<AddLiquiditySlice["addLiquidity"]["pool"]>
) => {
  return produce(addLiqState, (draft) => {
    draft.pool.poolId = pool.poolId ?? addLiqState.pool.poolId;
    draft.pool.balance.base =
      pool.balance && pool.balance.base
        ? pool.balance.base
        : addLiqState.pool.balance.base;
    draft.pool.balance.quote =
      pool.balance && pool.balance.quote
        ? pool.balance.quote
        : addLiqState.pool.balance.quote;
    draft.pool.lpToken = pool.lpToken ?? addLiqState.pool.lpToken;
  });
};
