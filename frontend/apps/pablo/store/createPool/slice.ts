import { StoreSlice } from "../types";
import { PoolConfig, PoolSlice } from "./types";
import { option } from "fp-ts";
import { pipe } from "fp-ts/function";

const createPoolSlice: StoreSlice<PoolSlice> = (set, get) => ({
  pools: {
    isLoaded: false,
    config: [],
    setConfig: (poolConfig: PoolConfig[]) => {
      set((state) => {
        state.pools.config = poolConfig;
        state.pools.isLoaded = true;
      });
    },
    getPoolById: (poolId: string) => {
      console.log(get().pools.config);
      return pipe(
        get().pools.config.find((pool) => pool.poolId.toString() === poolId),
        option.fromNullable
      );
    },
  },
});

export default createPoolSlice;
