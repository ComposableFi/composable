import { fetchSubsquid } from "../stakingRewards/helpers";

function queryPabloPool(poolId: number | string): string {
  return `query MyQuery {
  pabloPools(where: {id_eq: "${poolId.toString()}"}) {
    poolAssets {
      totalLiquidity
      totalVolume
      assetId
    }
    id
  }
}
`;
}

type PabloPool = {
  id: string;
  totalVolume: string;
  totalLiquidity: string;
};

type PoolAsset = {
  totalLiquidity: string;
  totalVolume: string;
  assetId: string;
};

type FPabloPool = {
  id: string;
  poolAssets: PoolAsset[];
};

export async function fetchPabloPool(
  poolId: number
): Promise<FPabloPool | undefined> {
  try {
    const { pabloPools } = await fetchSubsquid<{ pabloPools: FPabloPool[] }>(
      queryPabloPool(poolId)
    );

    return pabloPools?.at(0);
  } catch (e) {
    return Promise.reject(e);
  }
}

export async function fetchPabloPools(poolId: number): Promise<PabloPool[]> {
  try {
    const { pabloPools } = await fetchSubsquid<{ pabloPools: PabloPool[] }>(
      queryPabloPool(poolId)
    );
    return pabloPools;
  } catch (err: any) {
    return Promise.reject(err);
  }
}
