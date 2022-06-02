export interface LiquiditySlice {
    liquidity: {
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
    }
    setTokenAmountInPool: (poolId: number, amounts: { baseAmount?: string; quoteAmount?: string }) => void;
    setTokenValueInPool: (poolId: number, amounts: { baseValue?: string; quoteValue?: string }) => void;
    setUserProvidedTokenAmountInPool: (poolId: number, amounts: { baseAmount?: string; quoteAmount?: string }) => void;
}
