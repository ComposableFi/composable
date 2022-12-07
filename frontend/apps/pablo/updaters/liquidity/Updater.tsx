import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useEffect } from "react";
import { useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { usePoolsSlice } from "@/store/pools/pools.slice";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";

const Updater = () => {
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { constantProductPools } = usePoolsSlice();
  const { putLiquidityInPoolRecord, setUserLpBalance } =
    useStore();
  /**
   * For each pool, fetch its
   * base and quote token amount
   * and update it in zustand store
   * (first call)
   */
  useAsyncEffect(async (): Promise<void> => {
    if (constantProductPools.length > 0) {
      let liquidity: Record<string, { baseAmount: BigNumber, quoteAmount: BigNumber }> = {};
      // fetchPoolLiquidity(constantProductPools).then(putLiquidityInPoolRecord)
      for (const pool of constantProductPools) {
        const base = pool.getPair().getBaseAsset();
        const quote = pool.getPair().getQuoteAsset();
        const id = pool.getPoolId() as string;

        const baseAmount = await pool.getAssetLiquidity(base);
        const quoteAmount = await pool.getAssetLiquidity(quote);

        liquidity[id] = {
          baseAmount,
          quoteAmount
        }
      }
      putLiquidityInPoolRecord(liquidity)
    }
  }, [constantProductPools, putLiquidityInPoolRecord]);
  /**
   * Fetch and update LP Balances within
   * zustand store
   */
  useEffect(() => {
    if (constantProductPools.length > 0 && selectedAccount) {
      for (const pool of constantProductPools) {
        const lpToken = pool.getLiquidityProviderToken();
        const poolId: BigNumber = pool.getPoolId(true) as BigNumber;
        lpToken.balanceOf(selectedAccount.address).then(balance => {
          setUserLpBalance(poolId.toNumber(), balance.toString());
        })
      }
    }
  }, [constantProductPools, selectedAccount, setUserLpBalance]);

  return null;
};

export default Updater;
