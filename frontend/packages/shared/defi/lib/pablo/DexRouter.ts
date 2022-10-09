import { ApiPromise } from "@polkadot/api";
import { Asset } from "../Asset";

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
}