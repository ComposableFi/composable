import { ApiPromise, WsProvider } from "@polkadot/api";
import React, { useState, useEffect, createContext } from "react";
import type { InjectedExtension } from "@polkadot/extension-inject/types";
import { getChainId } from "./utils";

import * as definitions from "../interfaces/definitions";
export interface ParachainApi {
  chainId: string;
  parachainApi: ApiPromise | undefined;
  apiStatus: ParachainApiStatus;
  ss58Format: number;
  accounts: { address: string; name: string }[];
}

export const ParachainContext = createContext<{
  parachainProviders: { [chainId: string]: ParachainApi };
  extensionStatus: ParachainExtensionStatus;
  activate?: () => Promise<InjectedExtension[] | undefined>;
  deactivate?: () => Promise<void>;
  selectedAccount: number | -1;
  setSelectedAccount?: (account: number | -1) => void;
}>({
  parachainProviders: {},
  extensionStatus: "initializing",
  activate: undefined,
  selectedAccount: -1,
});

export type ParachainApiStatus = "initializing" | "failed" | "connected";

export type ParachainExtensionStatus =
  | "initializing"
  | "connecting"
  | "connected"
  | "no_extension"
  | "error";

const ParachainContextProvider = ({
  supportedChains,
  children,
  appName,
}: {
  appName: string;
  supportedChains: {
    relayChain: "polkadot" | "kusama";
    parachainId: number | 0;
    ss58Format: number;
    wsUrl: string;
  }[];
  children: React.ReactNode;
}) => {
  const [parachainProviders, setParachainProviders] = useState<{
    [chainId: string]: ParachainApi;
  }>({});

  const activate = async (): Promise<InjectedExtension[] | undefined> => {
    setExtension((s) => {
      s.extensionStatus = "connecting";
      return s;
    });

    let extensionExists = true;
    let injectedExtensions;
    try {
      const extensionPkg = await import("@polkadot/extension-dapp");
      injectedExtensions = await extensionPkg.web3Enable(appName);
      extensionExists = injectedExtensions.length !== 0;
    } catch (e) {
      console.error(e);
      extensionExists = false;
    }

    if (!extensionExists) {
      setExtension((s) => {
        s.extensionStatus = "no_extension";
        return s;
      });
      return injectedExtensions;
    }

    setExtension((s) => {
      s.extensionStatus = "connected";
      return s;
    });

    for (let i = 0; i < supportedChains.length; i++) {
      const { parachainId, relayChain, ss58Format } = supportedChains[i];
      const chainId = getChainId(relayChain, parachainId);

      try {
        const extensionPkg = await import("@polkadot/extension-dapp");
        const accounts = await extensionPkg.web3Accounts({ ss58Format });

        setParachainProviders((s) => {
          s[chainId].accounts = accounts.map((x, i) => ({
            address: x.address,
            name: x.meta.name ?? i.toFixed(),
          }));
          return { ...s };
        });

        // setting default account
        setSelectedAccount(accounts.length ? 0 : -1);
      } catch (e) {
        console.error(e);
        continue;
      }
    }

    return injectedExtensions;
  };

  const deactivate = async (): Promise<void> => {
    setExtension((s) => {
      s.extensionStatus = "initializing";
      return s;
    });

    for (let i = 0; i < supportedChains.length; i++) {
      const { parachainId, relayChain } = supportedChains[i];
      const chainId = getChainId(relayChain, parachainId);
      setParachainProviders((s) => {
        s[chainId].accounts = [];
        return { ...s };
      });

      setSelectedAccount(-1);

      return Promise.resolve();
    }
  };

  const [extension, setExtension] = useState<{
    extensionStatus: ParachainExtensionStatus;
    activate: () => Promise<InjectedExtension[] | undefined>;
    deactivate: () => Promise<void>;
  }>({
    extensionStatus: "initializing",
    activate,
    deactivate,
  });

  useEffect(() => {
    for (let i = 0; i < supportedChains.length; i++) {
      const { wsUrl, parachainId, relayChain, ss58Format } = supportedChains[i];
      const chainId = getChainId(relayChain, parachainId);
      // just so we can activate ASAP (where ss58Format is needed)
      setParachainProviders((s) => {
        s[chainId] = {
          chainId,
          parachainApi: undefined,
          apiStatus: "initializing",
          accounts: [],
          ss58Format: ss58Format,
        };
        return s;
      });

      const wsProvider = new WsProvider(wsUrl);
      let parachainApi;
      if (chainId === "kusama-2019") {
        const rpc = Object.keys(definitions)
          .filter((k) => {
            if (!(definitions as any)[k].rpc) {
              return false;
            } else {
              return Object.keys((definitions as any)[k].rpc).length > 0;
            }
          })
          .reduce(
            (accumulator, key) => ({
              ...accumulator,
              [key]: (definitions as any)[key].rpc,
            }),
            {}
          );
        const types = Object.keys(definitions)
          .filter((key) => Object.keys((definitions as any)[key].types).length > 0)
          .reduce(
            (accumulator, key) => ({
              ...accumulator,
              ...(definitions as any)[key].types,
            }),
            {}
          );
        parachainApi = new ApiPromise({ provider: wsProvider, types, rpc });
      } else {
        parachainApi = new ApiPromise({ provider: wsProvider });
      }

      parachainApi.isReady
        .then((parachainApi: ApiPromise) => {
          setParachainProviders((s) => {
            if (!(chainId in parachainProviders)) {
              s[chainId] = {
                chainId,
                parachainApi: parachainApi,
                apiStatus: "connected",
                accounts: [],
                ss58Format: ss58Format,
              };
            } else {
              s[chainId].apiStatus = "connected";
              s[chainId].parachainApi = parachainApi;
            }
            return s;
          });
        })
        .catch((e: any) => {
          console.error(e);
          setParachainProviders((s) => {
            s[chainId] = {
              chainId,
              parachainApi: undefined,
              apiStatus: "failed",
              accounts: [],
              ss58Format: ss58Format,
            };
            return s;
          });
        });
    }
  }, []);

  const [selectedAccount, setSelectedAccount] = useState<number | -1>(-1);

  return (
    <ParachainContext.Provider
      value={{
        parachainProviders,
        setSelectedAccount,
        selectedAccount,
        ...extension,
      }}
    >
      {children}
    </ParachainContext.Provider>
  );
};

export default ParachainContextProvider;
