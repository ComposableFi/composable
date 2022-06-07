export const queryAuctionStats = (
  poolId: number
) => `query subsquidLiquidityPoolStatsQuery {
    pabloPools(orderBy: calculatedTimestamp_DESC, where: {poolId_eq: ${poolId.toString()}}) {
      id
      totalVolume
      totalLiquidity
    }
  }`;
