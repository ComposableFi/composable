import { ApiPromise, WsProvider } from "@polkadot/api";
import { ParachainNetwork, SubstrateNetwork, SupportedWalletId } from "../types";
import type {
    InjectedAccount,
    InjectedAccountWithMeta,
    InjectedExtension,
} from "@polkadot/extension-inject/types";
import { decodeAddress, encodeAddress } from "@polkadot/util-crypto";
import { setChainApi, setHasInitialized, useSubstrateReact, setExtensionStatus, setInjectedExtension, setChainConnectedAccounts, setState } from "./store/extension.slice";

export const initialize = async (networks: Array<ParachainNetwork | SubstrateNetwork>): Promise<boolean> => {
    for (let i = 0; i < networks.length; i++) {
        const { chainId, wsUrl } = networks[i];
        const provider = new WsProvider(wsUrl);
        const api = new ApiPromise({ provider });

        try {
            await api.isReady;

            setChainApi(chainId, {
                ...networks[i],
                api,
                apiStatus: "connected",
                connectedAccounts: []
            });
        } catch (e) {
            setChainApi(chainId, {
                ...networks[i],
                api,
                apiStatus: "failed",
                connectedAccounts: []
            });
            console.error(e);
            continue;
        }
    }

    setHasInitialized(true);
    return true;
};

const truncate_regex = /^([a-zA-Z0-9]{4})[a-zA-Z0-9]+([a-zA-Z0-9]{4})$/;
function mapAccounts(
    source: string,
    list: InjectedAccount[],
    ss58Format?: number
): InjectedAccountWithMeta[] {
    return list.map(
        ({ address, genesisHash, name, type }): InjectedAccountWithMeta => ({
            address:
                address.length === 42
                    ? address
                    : encodeAddress(decodeAddress(address), ss58Format),
            meta: { genesisHash, name, source },
            type,
        })
    );
}

export const activate = async (
    appName: string,
    walletId: SupportedWalletId = SupportedWalletId.Polkadotjs,
): Promise<void> => {
    const { chainApi, hasInitialized } = useSubstrateReact.getState();

    if (!hasInitialized) {
        return Promise.reject(new Error("Substrate React has not been initialized!"));
    }

    setExtensionStatus("connecting");
    let injectedExtension, extensionError;

    try {
        if (!window.injectedWeb3) throw new Error("Extension not installed.");

        let extension = window.injectedWeb3[walletId];
        if (!extension) throw new Error("Extension not installed.");

        injectedExtension = await extension.enable(appName);
    } catch (e) {
        console.error(e);
        extensionError = e;
    }

    if (injectedExtension === undefined) {
        setExtensionStatus("no_extension");
        return Promise.reject(extensionError);
    }

    setExtensionStatus("connected");
    localStorage.setItem("wallet-id", walletId);
    setInjectedExtension(injectedExtension as InjectedExtension);

    for (const chain of Object.values(chainApi)) {
        const { prefix, chainId } = chain;

        try {
            let accounts = await injectedExtension.accounts.get();
            // talisman adds ethereum wallets as well, we dont want
            // them here for now
            accounts = accounts.filter((x) => !x.address.startsWith("0x"));
            let accountsWithMeta = mapAccounts(walletId, accounts, prefix);
            const connectedAccounts = accountsWithMeta.map(
                (x) => {
                    const regexMatch = x.address.match(truncate_regex);
                    const nameFallback = regexMatch
                        ? `${regexMatch[1]}...${regexMatch[2]}`
                        : x.address;
                    return {
                        address: x.address,
                        name: x.meta.name ?? nameFallback,
                    };
                }
            );
            setChainConnectedAccounts(chainId, connectedAccounts)
        } catch (error) {
            console.error(error);
            continue;
        }
    }
};

export const deactivate = () => {
    setState({
        selectedAccount: undefined,
        extensionStatus: "initializing",
        signer: undefined,
        injectedExtension: undefined
    })
}