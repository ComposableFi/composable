import {
  StableSwapPool,
  ConstantProductPool,
  LiquidityBootstrappingPool,
} from "@/defi/types";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { decodeLbp, decodeCpp, decodeSsp } from "./index";

export async function fetchPool(
  parachainApi: ApiPromise,
  poolId: number
): Promise<
  StableSwapPool | ConstantProductPool | LiquidityBootstrappingPool | null
> {
  try {
    const pool = await parachainApi.query.pablo.pools(poolId);
    const decodedPool: any = pool.toHuman();
    if (!decodedPool) throw new Error("Pool with ID not found");

    if ("LiquidityBootstrapping" in decodedPool) {
      const currentBlock = await parachainApi.query.system.number();
      const currentBlockBN = new BigNumber(currentBlock.toString());

      return decodeLbp(
        decodedPool.LiquidityBootstrapping,
        poolId,
        currentBlockBN
      );
    }

    if ("ConstantProduct" in decodedPool) {
      return decodeCpp(decodedPool.ConstantProduct, poolId);
    }
    if ("StableSwap" in decodedPool) {
      return decodeSsp(decodedPool.StableSwap, poolId);
    }

    return null;
  } catch (err) {
    console.error(err)
    return null;
  }
}

export async function isVerifiedPool(
  parachainApi: ApiPromise,
  pool: ConstantProductPool | LiquidityBootstrappingPool | StableSwapPool
): Promise<number> {
  try {
    let dexRoute: any = await parachainApi.query.dexRouter.dexRoutes(
      pool.pair.base,
      pool.pair.quote
    );
    let dexRouteReverse: any = await parachainApi.query.dexRouter.dexRoutes(
      pool.pair.quote,
      pool.pair.base
    );

    dexRoute = dexRoute.toJSON();
    dexRouteReverse = dexRouteReverse.toJSON();

    if (!dexRoute && !dexRouteReverse) return -1;
    dexRoute = dexRoute ? dexRoute : dexRouteReverse;
    if (!dexRoute.direct) return -1;

    if (dexRoute.direct.length && dexRoute.direct[0] === pool.poolId) {
      return pool.poolId;
    } else {
      return -1;
    }
  } catch (err: any) {
    console.error(err.message);
    return -1;
  }
}

export async function fetchPools(parachainApi: ApiPromise): Promise<{
  stableSwap: {
    verified: StableSwapPool[];
    unVerified: StableSwapPool[];
  };
  constantProduct: {
    verified: ConstantProductPool[];
    unVerified: ConstantProductPool[];
  };
  liquidityBootstrapping: {
    verified: LiquidityBootstrappingPool[];
    unVerified: LiquidityBootstrappingPool[];
  };
}> {
  let pools = {
    stableSwap: {
      verified: [] as StableSwapPool[],
      unVerified: [] as StableSwapPool[],
    },
    constantProduct: {
      verified: [] as ConstantProductPool[],
      unVerified: [] as ConstantProductPool[],
    },
    liquidityBootstrapping: {
      verified: [] as LiquidityBootstrappingPool[],
      unVerified: [] as LiquidityBootstrappingPool[],
    },
  };

  try {
    const poolCount = await parachainApi.query.pablo.poolCount();
    const poolCountBn = new BigNumber(poolCount.toString());

    const fetchPoolPromises: Promise<
      StableSwapPool | ConstantProductPool | LiquidityBootstrappingPool | null
    >[] = [];

    for (let poolIndex = 0; poolIndex < poolCountBn.toNumber(); poolIndex++) {
      fetchPoolPromises.push(fetchPool(parachainApi, poolIndex));
    }

    let allPools = await Promise.all(fetchPoolPromises);
    allPools = allPools.filter((pool) => !!pool);

    let allVerifiedPoolIds = await Promise.all(
      allPools.map((i: any) => isVerifiedPool(parachainApi, i))
    );
    allVerifiedPoolIds = allVerifiedPoolIds.filter((i) => i !== -1);

    let lbPool: LiquidityBootstrappingPool[] = allPools.filter(
      (i: any) => !!i.sale
    ) as LiquidityBootstrappingPool[];
    let cpPool: ConstantProductPool[] = allPools.filter(
      (i: any) => !!i.baseWeight
    ) as ConstantProductPool[];
    let ssPool: StableSwapPool[] = allPools.filter(
      (i: any) => !!i.amplificationCoefficient
    ) as StableSwapPool[];

    pools.liquidityBootstrapping.verified = lbPool.filter((p) =>
      allVerifiedPoolIds.includes(p.poolId)
    );
    // these might be needed in future so not removing
    // pools.liquidityBootstrapping.unVerified = lbPool.filter(
    //   (p) => !allVerifiedPoolIds.includes(p.poolId)
    // );

    pools.constantProduct.verified = cpPool.filter((p) =>
      allVerifiedPoolIds.includes(p.poolId)
    );
    // these might be needed in future so not removing
    // pools.constantProduct.unVerified = cpPool.filter(
    //   (p) => !allVerifiedPoolIds.includes(p.poolId)
    // );

    pools.stableSwap.verified = ssPool.filter((p) =>
      allVerifiedPoolIds.includes(p.poolId)
    );
    // these might be needed in future so not removing    
    // pools.stableSwap.unVerified = ssPool.filter(
    //   (p) => !allVerifiedPoolIds.includes(p.poolId)
    // );

    return pools;
  } catch (err) {
    console.error(err)
    return pools;
  }
}

export function getLPTokenPair(
  constantProductPool: Array<ConstantProductPool | StableSwapPool>,
  currencyId: string
) {
  const constantProduct = constantProductPool.find(
    (constantProduct) => constantProduct.lpToken === currencyId
  );
  return constantProduct ? constantProduct.pair : null;
}
