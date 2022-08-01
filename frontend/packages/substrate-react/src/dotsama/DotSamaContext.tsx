import React, { useState, useEffect, createContext } from "react";
import {
  DotSamaContext,
  ParachainApi,
  DotSamaExtensionStatus,
  ParachainId,
  RelaychainApi,
  RelayChainId,
} from "./types";
import { ParachainNetworks, RelayChainNetworks } from "./Networks";
import { createParachainApis } from "./utils";

const PARACHAIN_PROVIDERS_DEFAULT: {
  [chainId in ParachainId]: ParachainApi;
} = Object.entries(ParachainNetworks)
  .map(([chainId, network]) => {
    return {
      chainId: chainId,
      parachainApi: undefined,
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
  }, {} as { [chainId in ParachainId]: ParachainApi });

const RELAYCHAIN_PROVIDERS_DEFAULT: {
  [chainId in RelayChainId]: RelaychainApi;
} = Object.entries(RelayChainNetworks)
  .map(([chainId, network]) => {
    return {
      chainId: chainId,
      parachainApi: undefined,
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
  }, {} as { [chainId in RelayChainId]: RelaychainApi });

export const DotsamaContext = createContext<DotSamaContext>({
  parachainProviders: PARACHAIN_PROVIDERS_DEFAULT,
  relaychainProviders: RELAYCHAIN_PROVIDERS_DEFAULT,
  extensionStatus: "initializing",
  activate: undefined,
  selectedAccount: -1,
});

export const DotSamaContextProvider = ({
  supportedParachains,
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
  children: React.ReactNode;
}) => {
  const [parachainProviders, setParachainProviders] = useState<{
    [chainId in ParachainId]: ParachainApi;
  }>(PARACHAIN_PROVIDERS_DEFAULT);
  const [relaychainProviders, setRelayChainProviders] = useState<{
    [chainId in RelayChainId]: RelaychainApi;
  }>(RELAYCHAIN_PROVIDERS_DEFAULT);

  const [extensionStatus, setExtensionStatus] = useState<DotSamaExtensionStatus>('initializing');

  const activate = async (
    selectDefaultAccount: boolean = true
  ): Promise<any[] | undefined> => {
    setExtensionStatus("connecting");

    let extensionExists = true;
    let inectedExtesions;
    try {
      const extensionPkg = await import("@polkadot/extension-dapp");
      inectedExtesions = await extensionPkg.web3Enable(appName);
      extensionExists = inectedExtesions.length !== 0;
    } catch (e) {
      console.error(e);
      extensionExists = false;
    }

    if (!extensionExists) {
      setExtensionStatus('no_extension');
      return inectedExtesions;
    }

    setExtensionStatus('connected');

    for (let i = 0; i < supportedParachains.length; i++) {
      const { chainId } = supportedParachains[i];
      const { prefix } = ParachainNetworks[chainId];

      try {
        const extensionPkg = await import("@polkadot/extension-dapp");
        const accounts = await extensionPkg.web3Accounts({
          ss58Format: prefix,
        });

        setParachainProviders((s) => {
          s[chainId].accounts = accounts.map((x, i) => ({
            address: x.address,
            name: x.meta.name ?? i.toFixed(),
          }));
          return { ... s };
        });

        if (selectDefaultAccount) {
          setSelectedAccount(accounts.length ? 0 : -1);
        }
      } catch (e) {
        console.error(e);
        continue;
      }
    }

    return inectedExtesions;
  };

  const deactivate = async (): Promise<void> => {
    setExtensionStatus("initializing");
    setSelectedAccount(-1);
  };


  useEffect(() => {
    for (let i = 0; i < supportedParachains.length; i++) {
      createParachainApis(parachainProviders, supportedParachains).then(
          setParachainProviders
      );
    }
  }, []); // eslint-disable-line  react-hooks/exhaustive-deps

  const [selectedAccount, setSelectedAccount] = useState<number | -1>(-1);

  return (
    <DotsamaContext.Provider
      value={{
        relaychainProviders,
        parachainProviders,
        setSelectedAccount,
        selectedAccount,
        activate,
        deactivate,
        extensionStatus,
      }}
    >
      {children}
    </DotsamaContext.Provider>
  );
};
