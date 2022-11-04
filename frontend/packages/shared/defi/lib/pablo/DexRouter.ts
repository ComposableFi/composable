import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { Asset } from "../Asset";
import { BasePabloPool } from "./BasePabloPool";

export class DexRouter {
    protected __api: ApiPromise;

    constructor(
        api: ApiPromise
    ) {
        this.__api = api;
    }

    public async getDexRoute(base: Asset, quote: Asset): Promise<{ direct: number[] } | null> {
        try {
            let dexRoute: any = await this.__api.query.dexRouter.dexRoutes(
                base.getPicassoAssetId(),
                quote.getPicassoAssetId()
            );
            let dexRouteReverse: any = await this.__api.query.dexRouter.dexRoutes(
                quote.getPicassoAssetId(),
                base.getPicassoAssetId()
            );

            dexRoute = dexRoute.toJSON() as { direct: number[] } | null;
            dexRouteReverse = dexRouteReverse.toJSON() as { direct: number[] } | null;

            if (!!dexRoute) return dexRoute;
            if (!!dexRouteReverse) return dexRouteReverse;

            return dexRoute;
        } catch (err: any) {
            console.error("[getDexRoute] ", err.message);
            return Promise.reject(err);
        }
    }

    public async isPermissioned(pool: BasePabloPool): Promise<{ isPermissioned: boolean; poolId: BigNumber }> {
        let status = { isPermissioned: false, poolId: pool.getPoolId(true) as BigNumber }
        try {
            let dexRoute: any = await this.__api.query.dexRouter.dexRoutes(
                pool.getPair().getBaseAsset().toString(),
                pool.getPair().getQuoteAsset().toString()
            );
            let dexRouteReverse: any = await this.__api.query.dexRouter.dexRoutes(
                pool.getPair().getQuoteAsset().toString(),
                pool.getPair().getBaseAsset().toString()
            );

            let _dexRoute: { direct: number[] } | null = dexRoute.toJSON() as { direct: number[] } | null;
            let _dexRouteReverse: { direct: number[] } | null = dexRouteReverse.toJSON() as { direct: number[] } | null;

            if (!!_dexRoute && !!_dexRouteReverse) return status;
            if (_dexRoute || _dexRouteReverse) {
                if (_dexRoute) { status.isPermissioned = _dexRoute.direct[0] === (pool.getPoolId(true) as BigNumber).toNumber() }
                if (_dexRouteReverse) { status.isPermissioned = _dexRouteReverse.direct[0] === (pool.getPoolId(true) as BigNumber).toNumber() }
            }

            return status;
        } catch (err: any) {
            console.error("[DexRouter isPermissioned] ", err.message);
            return status;
        }
    }
}