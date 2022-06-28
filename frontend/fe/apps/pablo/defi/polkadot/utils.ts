import BigNumber from "bignumber.js";
import { getAsset } from "./Assets";
import { APP_NAME } from "./constants";
import { AssetId } from "./types";

export const getPolkadotSigner = async (address: string): Promise<any> => {
    const extensionPackage = await import("@polkadot/extension-dapp");
    const {
        web3FromAddress,
        web3Enable } = extensionPackage;
    await web3Enable(APP_NAME);
    const injector = await web3FromAddress(address);
    return injector.signer;
}

export function getPairDecimals(base: AssetId, quote: AssetId): {baseDecimals: BigNumber; quoteDecimals: BigNumber} {
    const baseDecimals = new BigNumber(10).pow(getAsset(base).decimals);
    const quoteDecimals = new BigNumber(10).pow(getAsset(quote).decimals);
    return {
        baseDecimals,
        quoteDecimals
    }
}