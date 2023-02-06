import { callbackGate, SubstrateNetworkId } from "shared";
import { useCallback, useEffect } from "react";
import { useStore } from "@/stores/root";
import { ChainApi, useDotSamaContext, useEagerConnect } from "substrate-react";

import {
  subscribeKaruraBalance,
  subscribeNativeBalance,
  subscribePicassoBalanceByAssetId,
  subscribeStatemineBalance,
} from "@/defi/polkadot/pallets/Balances";
import { TokenMetadata } from "../tokens/slice";
import { picassoAssetsList } from "@/defi/polkadot/pallets/Assets";
import { VoidFn } from "@polkadot/api/types";
import { kusamaAssetsList } from "@/defi/polkadot/pallets/Assets/kusama";
import { statemineAssetList } from "@/defi/polkadot/pallets/Assets/statemine";
import { SUBSTRATE_NETWORKS } from "shared/defi/constants";

const PolkadotBalancesUpdater = () => {
  useEagerConnect("picasso");
  // useEagerConnect("karura");

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
    if (parachainProviders.picasso.parachainApi) {
      picassoAssetsList(parachainProviders.picasso.parachainApi).then(
        (picaAssetMetadataList) => {
          console.dir(JSON.parse(JSON.stringify(picaAssetMetadataList)));
          updateTokens(picaAssetMetadataList, [], null);
        }
      );
    }

    if (relaychainProviders.kusama.parachainApi) {
      kusamaAssetsList(relaychainProviders.kusama.parachainApi).then(
        (kusamaAsset) => updateTokens([], [], kusamaAsset)
      );
    }

    if (parachainProviders.statemine.parachainApi) {
      statemineAssetList(parachainProviders.statemine.parachainApi).then(
        (statemineAssets) => updateTokens([], statemineAssets, null)
      );
    }
  }, [
    parachainProviders,
    relaychainProviders.kusama.parachainApi,
    updateTokens,
  ]);

  const picassoBalanceSubscriber = useCallback(
    async (
      chain: ChainApi,
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
            connectedAccounts[chainId as SubstrateNetworkId]?.[
              selectedAccount
            ] &&
            chain.parachainApi
          ) {
            subscribeNativeBalance(
              connectedAccounts[chainId as SubstrateNetworkId][selectedAccount]
                .address,
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
            case "statemine":
              if (
                connectedAccounts.statemine[selectedAccount] &&
                SUBSTRATE_NETWORKS.statemine.tokenId !== asset.id
              ) {
                subscribeStatemineBalance(
                  api,
                  connectedAccounts.statemine[selectedAccount].address,
                  asset,
                  chainId,
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
