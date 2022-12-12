import { useState } from "react";
import { DualAssetConstantProduct } from "shared";
import { usePoolsSlice } from "@/store/pools/pools.slice";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import { useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";

export const usePoolsWithLpBalance = (): Array<DualAssetConstantProduct> => {
    const {
        liquidityPools
    } = usePoolsSlice();
    const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
    const [poolsWithLpBalance, setPoolsWithLpBalance] = useState<DualAssetConstantProduct[]>([]);

    useAsyncEffect(async (): Promise<void> => {
        if (!selectedAccount) {
            setPoolsWithLpBalance([]);
            return;
        }

        let poolsWithBalance = [];
        for (const pool of liquidityPools) {
            const lpToken = pool.getLiquidityProviderToken();
            const balance = await lpToken.balanceOf(selectedAccount.address);

            if (balance.gt(0)) {
                poolsWithBalance.push(pool)
            }
        }

        setPoolsWithLpBalance(poolsWithBalance);
    }, [liquidityPools, selectedAccount])

    return poolsWithLpBalance;
}