import { useParachainApi, useSelectedAccount } from "substrate-react";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { useState } from "react";
import { Apollo, Asset, ClaimableAsset, fromChainIdUnit, LiquidityPoolFactory, LiquidityProviderToken, PabloConstantProductPool, StakingRewardPool } from "shared";
import { DEFAULT_NETWORK_ID } from "../utils";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import { calculateStakingRewardsPoolApy } from "../utils/stakingRewards";
import { fetchPabloPools } from "../subsquid/pabloPool";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";

export default function useLiquidityPools(liquidityFilter: boolean = false): {
    pools: Array<{
        pool: PabloConstantProductPool;
        rewardsPerDay: ClaimableAsset[];
        baseAsset: Asset;
        quoteAsset: Asset;
        apy: number;
        totalValueLocked: BigNumber;
        volume: BigNumber;
        lpToken: LiquidityProviderToken;
    }>
    isLoading: boolean
} {
    const [isLoading, setIsLoading] = useState(true);
    const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
    const { substrateTokens } = useStore();
    const { hasFetchedTokens, tokens } = substrateTokens;
    const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

    const [pools, setPools] = useState<Array<{
        pool: PabloConstantProductPool;
        rewardsPerDay: ClaimableAsset[];
        baseAsset: Asset;
        quoteAsset: Asset;
        apy: number;
        totalValueLocked: BigNumber;
        volume: BigNumber;
        lpToken: LiquidityProviderToken;
    }>>([]);

    useAsyncEffect(async (): Promise<void> => {
        if (!parachainApi || !hasFetchedTokens) return;
        if (liquidityFilter && !selectedAccount) return;
        setIsLoading(true);

        const apollo = new Apollo(parachainApi);
        const assets = Object.values(tokens)
        const { uniswapConstantProduct } = await LiquidityPoolFactory.fetchPermissionedPools(
            parachainApi,
            assets
        );
        
        let pools: Array<{
            pool: PabloConstantProductPool;
            rewardsPerDay: ClaimableAsset[];
            baseAsset: Asset;
            quoteAsset: Asset;
            apy: number;
            totalValueLocked: BigNumber;
            volume: BigNumber;
            lpToken: LiquidityProviderToken
        }> = [];

        for (const pool of uniswapConstantProduct) {
            const pair = pool.getPair();
            const baseAsset = assets.find(x => (x.getPicassoAssetId(true) as BigNumber).eq(pair.getBaseAsset()));
            const quoteAsset = assets.find(x => (x.getPicassoAssetId(true) as BigNumber).eq(pair.getQuoteAsset()));
            const pabloPools = await fetchPabloPools((pool.getPoolId(true) as BigNumber).toNumber());
            const volume = pabloPools.length > 0 ? fromChainIdUnit(
                BigInt(pabloPools[0].totalVolume)
            ) : new BigNumber(0);

            if (baseAsset && quoteAsset) {
                const baseAssetIdStr = baseAsset.getPicassoAssetId() as string;
                const lpToken = pool.getLiquidityProviderToken();
                const lpTokenTotalIssuance = await lpToken.totalIssued();
                let lpTokenBalance = new BigNumber(0);
                if (liquidityFilter) {
                    lpTokenBalance = await lpToken.balanceOf((selectedAccount as InjectedAccountWithMeta).address);

                    if (lpTokenBalance.lte(0)) {
                        continue;
                    }
                }

                const stakingRewardPool = await StakingRewardPool.fetchStakingRewardPool(
                    parachainApi,
                    lpToken.getPicassoAssetId(true) as BigNumber
                );

                const rewards = stakingRewardPool.getRewards();
                const rewardAssetIds = Array.from(stakingRewardPool.getRewards().keys());
                const rewardAssets = assets.filter(rewardAsset => (rewardAssetIds.includes(rewardAsset.getPicassoAssetId() as string)))

                const prices = await apollo.getPrice([baseAsset, quoteAsset, ...rewardAssets]);
                const baseAssetValue = prices[baseAssetIdStr];
                const quoteAssetValue = prices[baseAssetIdStr];
                
                const baseAssetAmount = await pool.getAssetLiquidity(baseAsset.getPicassoAssetId(true) as BigNumber)
                const quoteAssetAmount = await pool.getAssetLiquidity(quoteAsset.getPicassoAssetId(true) as BigNumber)
                const totalValueLocked = (baseAssetAmount.times(baseAssetValue)).plus(
                    quoteAssetAmount.times(quoteAssetValue)
                );

                const lpTokenValue = totalValueLocked.div(lpTokenTotalIssuance);
                // Fetch Probably from subsquid
                const lpTokensStaked = new BigNumber(0);
                let rewardsPerDay: ClaimableAsset[] = [];
                let apy = 0;
                for (const reward of rewardAssetIds) {
                    const rewardConfig = rewards.get(reward);
                    if (rewardConfig) {
                        const rewardAsset = rewardAssets.find(asset => (asset.getPicassoAssetId() as string) === reward);
                        if (rewardAsset) {
                            const rewardAssetIdStr = rewardAsset.getPicassoAssetId() as string;
                            const rewardPerDay = rewardConfig.getRewardsPerDay();
                            rewardsPerDay.push(ClaimableAsset.fromAsset(rewardAsset, rewardPerDay))
                            apy = apy + calculateStakingRewardsPoolApy(
                                prices[rewardAssetIdStr],
                                rewardPerDay,
                                lpTokenValue.times(
                                    lpTokensStaked
                                )
                            ).toNumber()
                        }
                    }
                }


                pools.push({
                    pool,
                    baseAsset,
                    quoteAsset,
                    totalValueLocked,
                    rewardsPerDay,
                    apy,
                    volume: volume.times(quoteAssetValue),
                    lpToken: pool.getLiquidityProviderToken()
                });
            }

        }

        setPools(pools);
        setIsLoading(false);
    }, [parachainApi, hasFetchedTokens, tokens, liquidityFilter, selectedAccount])

    return {
        isLoading,
        pools
    }
}