import { useEffect, useState } from "react";
import { PabloConstantProductPool } from "shared";
import { useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import BigNumber from "bignumber.js";

export function useLpTokenUserBalance(
    liquidityPool: PabloConstantProductPool | undefined
): BigNumber {
    const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
    const [lpBalance, setLpBalance] = useState(new BigNumber(0));

    useEffect(() => {
        if (!liquidityPool || !selectedAccount) {
            setLpBalance(new BigNumber(0))
            return;
        }

        const lpToken = liquidityPool.getLiquidityProviderToken();
        lpToken.balanceOf(selectedAccount.address).then(setLpBalance)

    }, [liquidityPool, selectedAccount]);

    return lpBalance;
}