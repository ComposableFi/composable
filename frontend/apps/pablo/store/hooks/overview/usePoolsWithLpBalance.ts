import BigNumber from "bignumber.js";
import { useMemo } from "react";
import useStore from "@/store/useStore";
import { useLiquidityPoolsList } from "../useLiquidityPoolsList";

export const usePoolsWithLpBalance = () => {
    const {
        apollo,
        userLpBalances
    } = useStore();
    const liquidityPoolsWithStats = useLiquidityPoolsList();

    const liquidityProviderPositions = useMemo(() => {
        return liquidityPoolsWithStats.map(lp => {
            let lpBalance = new BigNumber(0);
            let lpPrice = new BigNumber(0);
            if (userLpBalances[lp.poolId]) {
                lpBalance = new BigNumber(userLpBalances[lp.poolId])
            }
            if (apollo[lp.lpTokenAssetId]) {
                lpPrice = new BigNumber(apollo[lp.lpTokenAssetId])
            }
            return {
                ... lp,
                lpBalance,
                lpPrice
            }
        }).filter(i => i.lpBalance.gt(0))
    }, [apollo, liquidityPoolsWithStats, userLpBalances]);

    return liquidityProviderPositions
}