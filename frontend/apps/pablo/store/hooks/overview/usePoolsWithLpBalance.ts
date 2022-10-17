import BigNumber from "bignumber.js";
import { useMemo } from "react";
import useStore from "@/store/useStore";
import { useAllLpTokenRewardingPools } from "../useAllLpTokenRewardingPools";
import { ConstantProductPool, StableSwapPool } from "@/defi/types";

export interface StableSwapPoolWithLpBalance extends StableSwapPool { lpBalance: BigNumber }
export interface ConstantProductPoolWithLpBalance extends ConstantProductPool { lpBalance: BigNumber }

export const usePoolsWithLpBalance = (): Array<StableSwapPoolWithLpBalance & ConstantProductPoolWithLpBalance> => {
    const {
        userLpBalances
    } = useStore();
    const lpRewardingPools = useAllLpTokenRewardingPools();

    const lpPools = useMemo(() => {
        return lpRewardingPools.map(i => {
            if (userLpBalances[i.poolId]) {
                if (new BigNumber(userLpBalances[i.poolId]).gt(0)) {
                    return { ...i, lpBalance: new BigNumber(userLpBalances[i.poolId]) };
                }
            }
            return null;
        }).filter(i => i !== null) as Array<StableSwapPoolWithLpBalance & ConstantProductPoolWithLpBalance>;
    }, [lpRewardingPools, userLpBalances]);

    return lpPools;
}