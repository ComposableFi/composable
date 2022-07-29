import useStore from "@/store/useStore";
import { useEffect, useMemo } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { fetchPoolLiquidity } from "@/defi/utils";
import { fetchBalanceByAssetId } from "@/defi/utils";
import _ from "lodash";

const PICK = ["poolId", "pair", "lpToken"];
const Updater = () => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { pools, putLiquidityInPoolRecord, setUserLpBalance, liquidityInPool } =
    useStore();

  /**
   * Select pools tracking
   * liquidity
   */
  const allPools = useMemo(() => {
    return [
      ...pools.constantProductPools.verified.map((p) => _.pick(p, PICK)),
      ...pools.stableSwapPools.verified.map((p) => _.pick(p, PICK)),
    ];
  }, [pools]);
  /**
   * For each pool, fetch its
   * base and quote token amount
   * and update it in zustand store
   * (first call)
   */
  useEffect(() => {
    if (allPools.length > 0 && parachainApi) {
      fetchPoolLiquidity(parachainApi, allPools as any[]).then(
        putLiquidityInPoolRecord
      );
    }
  }, [allPools, parachainApi, putLiquidityInPoolRecord]);
  /**
   * Fetch and update LP Balances within
   * zustand store
   */
  useEffect(() => {
    if (allPools.length > 0 && selectedAccount !== undefined && parachainApi !== undefined) {
      allPools.forEach((pool) => {
        if (pool.poolId !== undefined && pool.pair !== undefined && pool.lpToken !== undefined) {
          fetchBalanceByAssetId(
            parachainApi,
            selectedAccount.address,
            pool.lpToken
          ).then((lpBalance) => {
            setUserLpBalance(pool.poolId as number, lpBalance);
          });
        }
      });
    }
  }, [parachainApi, allPools, selectedAccount, setUserLpBalance]);

  return null;
};

export default Updater;
