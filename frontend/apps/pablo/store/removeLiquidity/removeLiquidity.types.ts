export interface RemoveLiquiditySlice {
    removeLiquidity: {
        poolId: number;
        setRemoveLiquidity: (
            stats: {
                poolId: number;
            }
        ) => void;
        resetRemoveLiquidity: () => void;
    }
}