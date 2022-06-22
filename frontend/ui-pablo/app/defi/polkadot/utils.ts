import { APP_NAME } from "./constants";

export const getPolkadotSigner = async (address: string): Promise<any> => {
    const extensionPackage = await import("@polkadot/extension-dapp");
    const {
        web3FromAddress,
        web3Enable } = extensionPackage;
    await web3Enable(APP_NAME);
    const injector = await web3FromAddress(address);
    return injector.signer;
}
