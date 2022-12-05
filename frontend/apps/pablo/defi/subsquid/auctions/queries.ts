import { subsquidClient } from "../client";

export const queryAuctionStats = (
  poolId: number
) => subsquidClient().query(`query auctionVolumeAndLiquidity {
  pabloPools(orderBy: calculatedTimestamp_DESC, where: {poolId_eq: ${poolId.toString()}}) {
    id
    totalVolume
    totalLiquidity
  }
}`).toPromise();
