import { StateCreator } from "zustand";
import { TokensSlice } from "@/store/tokens/types";
import { SwapsSlice } from "@/store/swaps/types";
import { LiquiditySlice } from "@/store/liquidity/types";
import { RemoveLiquiditySlice } from "@/store/removeLiquidity/types";
import { CreatePoolSlice } from "@/store/createPool/types";
import { PoolStatsSlice } from "@/store/poolStats/types";

export type StoreSlice<T> = StateCreator<
  AllSlices,
  [
    ["zustand/subscribeWithSelector", never],
    ["zustand/immer", never],
    ["zustand/devtools", never]
  ],
  [],
  T
>;

export type AllSlices = TokensSlice &
  SwapsSlice &
  LiquiditySlice &
  RemoveLiquiditySlice &
  PoolStatsSlice &
  CreatePoolSlice;
