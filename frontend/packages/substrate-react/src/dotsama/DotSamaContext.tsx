import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';
import React, { useState, useEffect, createContext } from 'react';
import {
  DotSamaContext,
  ParachainApi,
  DotSamaExtensionStatus,
  ParachainId,
  RelaychainApi,
  RelayChainId,
} from './types';
import { ParachainNetworks, RelayChainNetworks } from './Networks';

const PARACHAIN_PROVIDERS_DEFAULT: {
  [chainId in ParachainId]: ParachainApi;
} = Object.entries(ParachainNetworks)
  .map(([chainId, network]) => {
    return {
      chainId: chainId,
      parachainApi: undefined,
      apiStatus: 'initializing',
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
      apiStatus: 'initializing',
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
  extensionStatus: 'initializing',
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
  const [parachainProviders, setParachainProviders] = useState<
    { [chainId in ParachainId]: ParachainApi }
  >(PARACHAIN_PROVIDERS_DEFAULT);
  const [relaychainProviders, setRelayChainProviders] = useState<
    { [chainId in RelayChainId]: RelaychainApi }
  >(RELAYCHAIN_PROVIDERS_DEFAULT);

  const activate = async (selectDefaultAccount: boolean = true): Promise<any[] | undefined> => {
    setExtension(s => {
      s.extensionStatus = 'connecting';
      return s;
    });

    let extensionExists = true;
    let inectedExtesions;
    try {
      const extensionPkg = await import('@polkadot/extension-dapp');
      inectedExtesions = await extensionPkg.web3Enable(appName);
      extensionExists = inectedExtesions.length !== 0;
    } catch (e) {
      console.error(e);
      extensionExists = false;
    }

    if (!extensionExists) {
      setExtension(s => {
        s.extensionStatus = 'no_extension';
        return s;
      });
      return inectedExtesions;
    }

    setExtension(s => {
      s.extensionStatus = 'connected';
      return s;
    });

    for (let i = 0; i < supportedParachains.length; i++) {
      const { chainId } = supportedParachains[i];
      const { prefix } = ParachainNetworks[chainId];

      try {
        const extensionPkg = await import('@polkadot/extension-dapp');
        const accounts = await extensionPkg.web3Accounts({
          ss58Format: prefix,
        });

        setParachainProviders(s => {
          s[chainId].accounts = accounts.map((x, i) => ({
            address: x.address,
            name: x.meta.name ?? i.toFixed(),
          }));
          return { ...s };
        });

        if (selectDefaultAccount) {
          // setting default account
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
    setExtension(s => {
      s.extensionStatus = 'initializing';
      return s;
    });

    for (let i = 0; i < supportedParachains.length; i++) {
      setParachainProviders(s => {
        const { chainId } = supportedParachains[i];
        s[chainId].accounts = [];
        return { ...s };
      });

      setSelectedAccount(-1);

      return Promise.resolve();
    }
  };

  const [extension, setExtension] = useState<{
    extensionStatus: DotSamaExtensionStatus;
    activate: () => Promise<any[] | undefined>;
    deactivate: () => Promise<void>;
  }>({
    extensionStatus: 'initializing',
    activate,
    deactivate,
  });

  useEffect(() => {
    for (let i = 0; i < supportedParachains.length; i++) {
      const { rpcUrl, chainId, rpc, types } = supportedParachains[i];
      const { prefix } = ParachainNetworks[chainId];

      // just so we can activate ASAP (where ss58Format is needed)
      // setParachainProviders(s => {
      //   s[chainId] = {
      //     parachainApi: undefined,
      //     apiStatus: 'initializing',
      //     accounts: [],
      //     prefix,
      //     chainId,
      //   };
      //   return s;
      // });

      const wsProvider = new WsProvider(rpcUrl);
      const parachainApi = new ApiPromise({ provider: wsProvider, rpc, types });

      parachainApi.isReady
        .then((parachainApi: ApiPromise) => {
          setParachainProviders(s => {
            if (!(chainId in parachainProviders)) {
              s[chainId] = {
                chainId,
                parachainApi: parachainApi,
                apiStatus: 'connected',
                accounts: [],
                prefix,
              };
            } else {
              s[chainId].apiStatus = 'connected';
              s[chainId].parachainApi = parachainApi;
            }
            return s;
          });
        })
        .catch((e: any) => {
          console.error(e);
          setParachainProviders(s => {
            s[chainId] = {
              chainId,
              parachainApi: undefined,
              apiStatus: 'failed',
              accounts: [],
              prefix,
            };
            return s;
          });
        });
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
        ...extension,
      }}
    >
      {children}
    </DotsamaContext.Provider>
  );
};
