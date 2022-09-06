import { useAllLpTokenRewardingPools } from "@/store/hooks/useAllLpTokenRewardingPools";
import BigNumber from "bignumber.js";
import { useMemo, useState } from "react";

export function useStakingRewardsPoolApy(
    poolId: string | undefined
): BigNumber {
    // const lpRewardingPools = useAllLpTokenRewardingPools();
    // const isLpBasedRewardPool = useMemo(() => {
    //     return lpRewardingPools.find(pool => pool.lpToken === poolId);
    // }, [lpRewardingPools])

    const [apy, setApy] = useState(
        new BigNumber(0)
    )

    return apy;
}