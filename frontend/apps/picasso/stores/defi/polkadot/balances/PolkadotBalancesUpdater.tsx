import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { callbackGate } from "shared";
import { useCallback, useEffect } from "react";
import { useStore } from "@/stores/root";
import {
  ParachainApi,
  ParachainId,
  RelayChainId,
  useDotSamaContext,
  useEagerConnect,
} from "substrate-react";

import {
  subscribeKaruraBalance,
  subscribeNativeBalance,
  subscribePicassoBalanceByAssetId,
} from "@/defi/polkadot/pallets/Balances";
import { TokenMetadata } from "../tokens/slice";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import {
  karuraAssetsList,
  picassoAssetsList,
} from "@/defi/polkadot/pallets/Assets";
import { VoidFn } from "@polkadot/api/types";
import { AcalaPrimitivesCurrencyCurrencyId } from "@acala-network/types/interfaces/types-lookup";
import { ApiPromise } from "@polkadot/api";
import { kusamaAssetsList } from "@/defi/polkadot/pallets/Assets/kusama";

const PolkadotBalancesUpdater = () => {
  useEagerConnect("picasso");
  useEagerConnect("karura");

  const isLoaded = useStore((state) => state.substrateTokens.isLoaded);

  const updateTokens = useStore(
    ({ substrateTokens }) => substrateTokens.updateTokens
  );
  const tokens = useStore(({ substrateTokens }) => substrateTokens.tokens);

  const updateBalance = useStore(
    ({ substrateBalances }) => substrateBalances.updateBalance
  );

  const clearBalance = useStore(
    ({ substrateBalances }) => substrateBalances.clearBalance
  );

  const {
    extensionStatus,
    selectedAccount,
    parachainProviders,
    relaychainProviders,
    connectedAccounts,
  } = useDotSamaContext();

  /**
   * This effect fetches
   * metadata for tokens and
   * should be called almost
   * after API creation
   */
  useEffect(() => {
    callbackGate(
      async (_picaApi, _karApi, _kusamaApi) => {
        const picaAssetMetadataList = await picassoAssetsList(_picaApi);
        const karuraAssetMetadataList = await karuraAssetsList(_karApi);
        const kusamaAssetMetadata = await kusamaAssetsList(_kusamaApi);
        updateTokens(
          picaAssetMetadataList,
          karuraAssetMetadataList,
          kusamaAssetMetadata
        );
      },
      parachainProviders.picasso.parachainApi,
      parachainProviders.karura.parachainApi,
      relaychainProviders.kusama.parachainApi
    );
  }, [parachainProviders, updateTokens]);

  const picassoBalanceSubscriber = useCallback(
    async (
      chain: ParachainApi,
      tokenMetadata: TokenMetadata,
      chainId,
      accounts
    ) => {
      callbackGate(
        async (api, tokenMetadata, chainId, account) => {
          await subscribePicassoBalanceByAssetId(
            api,
            account.address,
            tokenMetadata,
            (balance) => {
              updateBalance({
                network: chainId as SubstrateNetworkId,
                tokenId: tokenMetadata.id,
                balance,
              });
            }
          );
        },
        chain.parachainApi,
        tokenMetadata,
        chainId,
        accounts[selectedAccount]
      );
    },
    [selectedAccount, updateBalance]
  );

  // Subscribe for native balance changes
  useEffect(() => {
    if (selectedAccount !== -1) {
      let subscriptionList: Array<VoidFn | undefined> = [];

      Object.entries({ ...parachainProviders, ...relaychainProviders }).forEach(
        ([chainId, chain]) => {
          console.log("Subscribing native balance", chainId);
          if (
            connectedAccounts[chainId as RelayChainId | ParachainId] &&
            chain.parachainApi
          ) {
            subscribeNativeBalance(
              connectedAccounts[chainId as ParachainId | RelayChainId][
                selectedAccount
              ].address,
              chain.parachainApi,
              chainId,
              SUBSTRATE_NETWORKS[chainId as SubstrateNetworkId].tokenId,
              updateBalance
            ).then((subscription) => {
              subscriptionList.push(subscription);
            });
          }
        }
      );

      return function unsubNativeBalances() {
        console.log("Clearing Native Subscriptions. ", subscriptionList.length);
        return subscriptionList.forEach((x) => {
          x?.();
        });
      };
    } else if (selectedAccount === -1) {
      clearBalance();
    }
  }, [
    parachainProviders,
    relaychainProviders,
    selectedAccount,
    connectedAccounts,
    updateBalance,
    clearBalance,
  ]);

  // Subscribe non-native token balances
  useEffect(() => {
    let unsubList: any[];
    unsubList = [];
    if (
      extensionStatus !== "connected" ||
      selectedAccount === -1 ||
      !isLoaded
    ) {
      return () => {};
    }

    Object.entries(parachainProviders).forEach(([chainId, chain]) =>
      callbackGate((api) => {
        Object.values(tokens).forEach((asset) => {
          switch (chainId) {
            case "picasso":
              if (SUBSTRATE_NETWORKS.picasso.tokenId !== asset.id) {
                picassoBalanceSubscriber(
                  chain,
                  asset,
                  chainId,
                  connectedAccounts[chainId]
                );
              }
              break;
            case "karura":
              // Ignore native token since for that we need to fetch system
              if (
                connectedAccounts.karura[selectedAccount] &&
                SUBSTRATE_NETWORKS.karura.tokenId !== asset.id
              ) {
                subscribeKaruraBalance(
                  api,
                  connectedAccounts.karura[selectedAccount].address,
                  asset,
                  (balance) => {
                    updateBalance({
                      network: chainId as SubstrateNetworkId,
                      tokenId: asset.id,
                      balance,
                    });
                  }
                );
              }
              break;
            default:
              break;
          }
        });

        return () => {
          unsubList.forEach((unsub) => {
            unsub.then((call: any) => call?.());
          });
        };
      }, chain.parachainApi)
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [extensionStatus, selectedAccount, parachainProviders, isLoaded]);

  return null;
};

export default PolkadotBalancesUpdater;
