import BigNumber from "bignumber.js";
import { Asset } from "../assets/Asset";
export interface Exchangeable {
    getSpotPrice(...args: unknown[]): Promise<BigNumber>;
    getLiquidity(assets: Asset[]): Promise<Map<string, BigNumber>>;
}