import { StoreSlice } from "../types";
import { PoolConfig, PoolSlice } from "./types";
import { option } from "fp-ts";
import { pipe } from "fp-ts/function";

const createPoolsSlice: StoreSlice<PoolSlice> = (set, get) => ({
  pools: {
    isLoaded: false,
    config: [],
    poolAmount: {},
    setConfig: (poolConfig: PoolConfig[]) => {
      set((state) => {
        state.pools.config = poolConfig;
        state.pools.isLoaded = true;
      });
    },
    getPoolById: (poolId: string) =>
      pipe(
        get().pools.config.find((pool) => pool.poolId.toString() === poolId),
        option.fromNullable
      ),
    setPoolAmount: (poolId: string, payload) => {
      set((state) => {
        state.pools.poolAmount[poolId] = payload;
      });
    },
  },
});

export default createPoolsSlice;
