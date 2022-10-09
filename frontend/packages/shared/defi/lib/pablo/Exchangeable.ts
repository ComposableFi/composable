import BigNumber from "bignumber.js";
import { Asset } from "../Asset";
export interface Exchangeable {
    getSpotPrice(): Promise<BigNumber>;
    getLiquidity(assets: Asset[]): Promise<Map<string, BigNumber>>;
}