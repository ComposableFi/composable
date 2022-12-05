import { PabloConstantProductPool, Asset, BondOffer, LiquidityProviderToken } from "shared";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { usePoolsSlice } from "@/store/pools/pools.slice";

export function useBondedAsset(
    bondOffer?: BondOffer
): LiquidityProviderToken | Asset | undefined {
    const { substrateTokens } = useStore();
    const { tokens } = substrateTokens;
    const lpRewardingPools = usePoolsSlice().constantProductPools;

    if (!bondOffer) return undefined;

    const isLpBasedBond: PabloConstantProductPool | undefined =
        lpRewardingPools.find(
            (pool: PabloConstantProductPool) =>
            (pool.getLiquidityProviderToken().getPicassoAssetId(true) as BigNumber)
            .eq(bondOffer.getBondedAssetId(true) as BigNumber)
        );

    if (isLpBasedBond) {
        return isLpBasedBond.getLiquidityProviderToken();
    } else {
        return Object.values(tokens).find(asset => {
            (asset.getPicassoAssetId(true) as BigNumber).eq(bondOffer.getBondedAssetId(true) as BigNumber)
        })
    }
}
