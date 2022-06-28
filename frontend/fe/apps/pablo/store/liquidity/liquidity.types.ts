export interface LiquiditySlice {
    poolLiquidity: {
        [poolId: number]: {
            tokenAmounts: {
                baseAmount: string;
                quoteAmount: string;
            },
            value: {
                baseValue: string;
                quoteValue: string;
            }
        }
    },
    userProvidedLiquidity: {
        [poolId: number]: {
            tokenAmounts: {
                baseAmount: string;
                quoteAmount: string;
            }
        }
    },
    userLpBalances: {
        [poolId: number]: string;
    },
    setTokenAmountInLiquidityPool: (poolId: number, amounts: { baseAmount?: string; quoteAmount?: string }) => void;
    setTokenValueInLiquidityPool: (poolId: number, amounts: { baseValue?: string; quoteValue?: string }) => void;
    setUserProvidedTokenAmountInLiquidityPool: (poolId: number, amounts: { baseAmount?: string; quoteAmount?: string }) => void;
    updateUserProvidedTokenAmountInLiquidityPool: (poolId: number, amounts: { baseAmount?: string; quoteAmount?: string }) => void;
    setUserLpBalance: (poolId: number, balance: string) => void;
}
