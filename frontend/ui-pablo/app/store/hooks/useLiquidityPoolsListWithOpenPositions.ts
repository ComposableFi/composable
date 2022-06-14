import { useMemo } from "react";
import {
  LiquidityPoolRow,
  useLiquidityPoolsList,
} from "./useLiquidityPoolsList";
import useStore from "../useStore";
import BigNumber from "bignumber.js";

export const useLiquidityPoolsListWithOpenPositions =
  (): LiquidityPoolRow[] => {
    const { userLpBalances } = useStore();
    const allPools = useLiquidityPoolsList();

    const poolIds = useMemo(() => {
      return Object.keys(userLpBalances)
        .filter((k) => {
          const bal = new BigNumber(userLpBalances[Number(k)]);
          return bal.gt(0);
        })
        .map((i) => Number(i));
    }, [userLpBalances]);

    return allPools.filter((i) => poolIds.includes(i.poolId));
  };
