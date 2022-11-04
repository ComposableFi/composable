import { PabloConstantProductPool } from "./PabloConstantProductPool";
import { ApiPromise } from "@polkadot/api";
import { Asset } from "../Asset";
import { DexRouter } from "./DexRouter";
import { BasePabloPool } from "./BasePabloPool";
import BigNumber from "bignumber.js";

export class LiquidityPoolFactory {
    static async fetchPool(
        api: ApiPromise,
        poolId: number,
        assets: Asset[]
    ): Promise<
        PabloConstantProductPool | null
    > {
        try {
            const pool = await api.query.pablo.pools(poolId);
            const decodedJSON: any = pool.toJSON();
            if (decodedJSON) {
                if ("constantProduct" in decodedJSON) {
                    return PabloConstantProductPool.fromJSON(
                        new BigNumber(poolId),
                        api,
                        assets,
                        decodedJSON.constantProduct
                    );
                }
            }
            throw new Error("Pool with ID not found")
        } catch (err) {
            console.error(err)
            return null;
        }
    }

    static async fetchPermissionedPools(api: ApiPromise, assets: Asset[] = []): Promise<{
        uniswapConstantProduct: PabloConstantProductPool[];
    }> {
        let pools = {
            uniswapConstantProduct: [] as PabloConstantProductPool[],
        };
        try {
            const poolCount = await api.query.pablo.poolCount();
            const poolCountBn = new BigNumber(poolCount.toString());
            const dexRouter = new DexRouter(api);

            const fetchPoolPromises: Promise<
                PabloConstantProductPool | null
            >[] = [];

            for (let poolIndex = 0; poolIndex < poolCountBn.toNumber(); poolIndex++) {
                fetchPoolPromises.push(LiquidityPoolFactory.fetchPool(api, poolIndex, assets));
            }

            let allConstantProductPools = await Promise.all(fetchPoolPromises);
            let isPermissionedPools = allConstantProductPools
                .filter(x => x !== null)
                .map(pool => {
                    return dexRouter.isPermissioned(pool as BasePabloPool)
                });

            const permissionedStatus = await Promise.all(isPermissionedPools);
            const uniswapConstantProduct = allConstantProductPools.filter((basePool) => {
                if (basePool) {
                    const poolId: BigNumber = basePool.getPoolId(true) as BigNumber;
                    const isPermissionedStatus = permissionedStatus
                        .find((_permissionedStatus) => (_permissionedStatus.poolId.eq(poolId)))
                    return isPermissionedStatus && isPermissionedStatus.isPermissioned
                }
                return false
            }) as PabloConstantProductPool[];

            pools.uniswapConstantProduct = uniswapConstantProduct;
            return pools;
        } catch (err) {
            console.error(err)
            return pools;
        }
    }
}