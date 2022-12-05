import { useState } from "react";
import { PabloConstantProductPool } from "shared";
import { usePoolsSlice } from "@/store/pools/pools.slice";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import { useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";

export const usePoolsWithLpBalance = (): Array<PabloConstantProductPool> => {
    const {
        constantProductPools
    } = usePoolsSlice();
    const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
    const [poolsWithLpBalance, setPoolsWithLpBalance] = useState<PabloConstantProductPool[]>([]);

    useAsyncEffect(async (): Promise<void> => {
        if (!selectedAccount) {
            setPoolsWithLpBalance([]);
            return;
        }

        let poolsWithBalance = [];
        for (const pool of constantProductPools) {
            const lpToken = pool.getLiquidityProviderToken();
            const balance = await lpToken.balanceOf(selectedAccount.address);

            if (balance.gt(0)) {
                poolsWithBalance.push(pool)
            }
        }

        setPoolsWithLpBalance(poolsWithBalance);
    }, [constantProductPools, selectedAccount])

    return poolsWithLpBalance;
}