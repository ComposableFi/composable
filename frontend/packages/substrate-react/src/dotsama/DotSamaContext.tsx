import React, {
  createContext,
  ReactNode,
  useCallback,
  useEffect,
  useMemo,
  useState,
} from "react";
import {
  ChainApi,
  ConnectedAccounts,
  DotSamaContext,
  DotSamaExtensionStatus,
  SupportedWalletId,
} from "./types";
import { parachainNetworks, RelayChainNetworks } from "./Networks";
import type {
  InjectedAccount,
  InjectedAccountWithMeta,
  InjectedExtension,
} from "@polkadot/extension-inject/types";
import { decodeAddress, encodeAddress } from "@polkadot/util-crypto";
import { ParachainId, RelaychainId } from "shared";
import { createChainApi } from "../dotsama/utils";

const DEFAULT_ACCOUNTS: ConnectedAccounts = {
  picasso: [],
  karura: [],
  kusama: [],
  statemine: [],
};

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

const PARACHAIN_PROVIDERS_DEFAULT: {
  [chainId in ParachainId]: ChainApi;
} = Object.entries(parachainNetworks)
  .map(([chainId, network]) => {
    return {
      chainId: chainId,
      ChainApi: undefined,
      apiStatus: "initializing",
      prefix: network.prefix,
      accounts: [],
    };
  })
  .reduce((acc, curr) => {
    return {
      ...acc,
      [curr.chainId]: curr,
    };
  }, {} as { [chainId in ParachainId]: ChainApi });

const RELAYCHAIN_PROVIDERS_DEFAULT: {
  [chainId in RelaychainId]: ChainApi;
} = Object.entries(RelayChainNetworks)
  .map(
    ([chainId, network]): {
      chainId: string;
      ChainApi: undefined;
      prefix: any;
      accounts: any[];
      apiStatus: "initializing";
    } => {
      return {
        chainId: chainId,
        ChainApi: undefined,
        apiStatus: "initializing",
        prefix: network.prefix,
        accounts: [],
      };
    }
  )
  .reduce((acc, curr) => {
    return {
      ...acc,
      [curr.chainId]: curr,
    };
  }, {} as { [chainId in RelaychainId]: ChainApi });

export const DotsamaContext = createContext<DotSamaContext>({
  signer: undefined,
  parachainProviders: PARACHAIN_PROVIDERS_DEFAULT,
  relaychainProviders: RELAYCHAIN_PROVIDERS_DEFAULT,
  extensionStatus: "initializing",
  activate: undefined,
  selectedAccount: -1,
  connectedAccounts: DEFAULT_ACCOUNTS,
});

export const DotSamaContextProvider = ({
  supportedParachains,
  supportedRelaychains,
  children,
  appName,
}: {
  appName: string;
  supportedParachains: {
    chainId: ParachainId;
    rpcUrl: string;
    rpc: any;
    types: any;
  }[];
  supportedRelaychains?: {
    chainId: RelaychainId;
    rpcUrl: string;
    rpc: any;
    types: any;
  }[];
  children: ReactNode;
}) => {
  const [parachainProviders, setParachainProviders] = useState<{
    [chainId in ParachainId]: ChainApi;
  }>(PARACHAIN_PROVIDERS_DEFAULT);
  const [relaychainProviders, setRelayChainProviders] = useState<{
    [chainId in RelaychainId]: ChainApi;
  }>(RELAYCHAIN_PROVIDERS_DEFAULT);

  const [connectedAccounts, setConnectedAccounts] =
    useState<ConnectedAccounts>(DEFAULT_ACCOUNTS);
  const [extensionInjected, setInjectedExtension] = useState<
    InjectedExtension | undefined
  >(undefined);
  const [extensionStatus, setExtensionStatus] =
    useState<DotSamaExtensionStatus>("initializing");

  const activate = useCallback(
    async (
      walletId: SupportedWalletId = SupportedWalletId.Polkadotjs,
      selectDefaultAccount: boolean = false
    ): Promise<any | undefined> => {
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

      let accountMap: ConnectedAccounts = DEFAULT_ACCOUNTS;
      for (const element of supportedParachains) {
        const { chainId } = element;
        const { prefix } = parachainNetworks[chainId];

        try {
          let accounts = await injectedExtension.accounts.get();
          if (accounts === undefined)
            throw new Error("Unable to fetch accounts from extension.");
          accounts = mapAccounts(walletId, accounts, prefix);
          if (accounts === undefined)
            throw new Error("Unable to fetch accounts from extension.");

          accountMap = { ...accountMap, [chainId]: accounts };

          if (selectDefaultAccount) {
            setSelectedAccount(accounts.length ? 0 : -1);
          }
        } catch (e) {
          console.error(e);
        }
      }
      setConnectedAccounts(accountMap);

      if (supportedRelaychains?.length) {
        for (const relayChain of supportedRelaychains) {
          const { chainId } = relayChain;
          const { prefix } = RelayChainNetworks[chainId];
          let accounts = await injectedExtension.accounts.get();
          if (accounts === undefined)
            throw new Error("Unable to fetch accounts from extension.");
          accounts = mapAccounts(walletId, accounts, prefix);

          // @ts-ignore
          accountMap = { ...accountMap, [chainId]: accounts };
        }
      }

      return injectedExtension;
    },
    [appName, supportedParachains, supportedRelaychains]
  );

  const deactivate = async (): Promise<void> => {
    setExtensionStatus("initializing");
    setSelectedAccount(-1);
  };

  useEffect(() => {
    createChainApi(parachainProviders, supportedParachains).then(
      setParachainProviders
    );
    if (supportedRelaychains) {
      createChainApi(relaychainProviders, supportedRelaychains).then(
        setRelayChainProviders
      );
    }

    // only called on first render
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const [selectedAccount, setSelectedAccount] = useState<number | -1>(-1);

  const signer = useMemo(() => {
    if (extensionInjected) {
      return extensionInjected.signer;
    }
    return undefined;
  }, [extensionInjected]);

  return (
    <DotsamaContext.Provider
      value={{
        signer,
        relaychainProviders,
        parachainProviders,
        setSelectedAccount,
        selectedAccount,
        activate,
        deactivate,
        extensionStatus,
        connectedAccounts,
      }}
    >
      {children}
    </DotsamaContext.Provider>
  );
};
