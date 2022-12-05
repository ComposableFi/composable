import BigNumber from "bignumber.js";

export interface LiquiditySlice {
    liquidityInPool: Record<string, {
        baseAmount: BigNumber;
        quoteAmount: BigNumber;
    }>,
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
    putLiquidityInPoolRecord: (record: Record<string, { baseAmount: BigNumber; quoteAmount: BigNumber }>) => void;
    updatePoolLiquidity: (poolId: string, amounts: { baseAmount: BigNumber; quoteAmount: BigNumber }) => void;
    setUserProvidedTokenAmountInLiquidityPool: (poolId: number, amounts: { baseAmount?: string; quoteAmount?: string }) => void;
    updateUserProvidedTokenAmountInLiquidityPool: (poolId: number, amounts: { baseAmount?: string; quoteAmount?: string }) => void;
    setUserLpBalance: (poolId: number, balance: string) => void;
}
