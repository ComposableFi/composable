import { PoolConfig, PoolId, PoolSlice } from "./types";
import { option } from "fp-ts";
import { pipe } from "fp-ts/function";
import BigNumber from "bignumber.js";
import { StoreSlice } from "@/stores/types";

export const createPoolsSlice: StoreSlice<PoolSlice> = (set, get) => ({
  pools: {
    isLoaded: false,
    config: [],
    poolAmount: {},
    totalIssued: {},
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
    setTotalIssued: (poolId: PoolId, totalIssued: BigNumber) => {
      set((state) => {
        state.pools.totalIssued[poolId.toString()] = totalIssued;
      });
    },
  },
});
