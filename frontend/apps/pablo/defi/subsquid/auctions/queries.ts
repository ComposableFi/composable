import { makeClient } from "../makeClient";

export const queryAuctionStats = (
  poolId: number
) => makeClient().query(`query auctionVolumeAndLiquidity {
  pabloPools(orderBy: calculatedTimestamp_DESC, where: {poolId_eq: ${poolId.toString()}}) {
    id
    totalVolume
    totalLiquidity
  }
}`).toPromise();
