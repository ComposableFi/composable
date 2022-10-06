import BigNumber from "bignumber.js";

export interface Exchangeable {
    getSpotPrice(): Promise<BigNumber>;
    getAssetLiquidity(assetId: string): Promise<BigNumber>;
}