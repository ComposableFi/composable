import { fetchSubsquid } from "../stakingRewards/helpers";

function queryPabloPool(poolId: number): string {
  return `query auctionVolumeAndLiquidity {
    pabloPools(orderBy: calculatedTimestamp_DESC, where: {poolId_eq: ${poolId.toString()}}) {
      id
      totalVolume
      totalLiquidity
    }
  }`
}

type PabloPool = {
  id: string;
  totalVolume: string;
  totalLiquidity: string;
}

export async function fetchPabloPools(poolId: number): Promise<PabloPool[]> {
  try {
    const { pabloPools } = await fetchSubsquid<{ pabloPools: PabloPool[] }>(queryPabloPool(poolId));
    return pabloPools;
  } catch (err: any) {
    return Promise.reject(err);
  }
}