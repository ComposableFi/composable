import BigNumber from "bignumber.js";
import { ApiPromise } from "@polkadot/api";
import { Asset } from "./Asset";

export class LiquidityProviderToken extends Asset {
    protected __underlyingAssets: Asset[];

    constructor(
        api: ApiPromise,
        underlyingAssets: Asset[],
        tokenAssetId: BigNumber
    ) {
        super(
            api,
            tokenAssetId,
            `LP ${underlyingAssets.map(x => x.getSymbol()).join("/")}`,
            `${underlyingAssets.map(x => x.getSymbol()).join("/")}`,
            "-"
        );
        this.__underlyingAssets = underlyingAssets;
    }

    getUnderlyingAssets(): Asset[] {
        return this.__underlyingAssets;
    }
}