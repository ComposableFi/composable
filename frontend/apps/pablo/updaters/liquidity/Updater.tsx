import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useEffect } from "react";
import { useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { usePoolsSlice } from "@/store/pools/pools.slice";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";

const Updater = () => {
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { liquidityPools } = usePoolsSlice();
  const { putLiquidityInPoolRecord, setUserLpBalance } =
    useStore();
  /**
   * For each pool, fetch its
   * base and quote token amount
   * and update it in zustand store
   * (first call)
   */
  useAsyncEffect(async (): Promise<void> => {
    if (liquidityPools.length > 0) {
      let liquidity: Record<string, { baseAmount: BigNumber, quoteAmount: BigNumber }> = {};
      for (const pool of liquidityPools) {

        const pair = Object.keys(pool.getAssets().assets);
        const base = pair[0];
        const quote = pair[1];
        const id = pool.getPoolId() as string;
        const baseAmount = await pool.getAssetLiquidity(new BigNumber(base));
        const quoteAmount = await pool.getAssetLiquidity(new BigNumber(quote));

        liquidity[id] = {
          baseAmount,
          quoteAmount
        }
      }
      putLiquidityInPoolRecord(liquidity)
    }
  }, [liquidityPools, putLiquidityInPoolRecord]);
  /**
   * Fetch and update LP Balances within
   * zustand store
   */
  useEffect(() => {
    if (liquidityPools.length > 0 && selectedAccount) {
      for (const pool of liquidityPools) {
        const lpToken = pool.getLiquidityProviderToken();
        const poolId: BigNumber = pool.getPoolId(true) as BigNumber;
        lpToken.balanceOf(selectedAccount.address).then(balance => {
          setUserLpBalance(poolId.toNumber(), balance.toString());
        })
      }
    }
  }, [liquidityPools, selectedAccount, setUserLpBalance]);

  return null;
};

export default Updater;
