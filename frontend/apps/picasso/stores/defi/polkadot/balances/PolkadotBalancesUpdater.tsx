import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { callbackGate } from "shared";
import { useCallback, useEffect } from "react";
import { useStore } from "@/stores/root";
import {
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
import { karuraAssetsList, picassoAssetsList } from "@/defi/polkadot/pallets/Assets";

const PolkadotBalancesUpdater = () => {
  useEagerConnect("picasso");
  useEagerConnect("karura");

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
      async (_picaApi, _karApi) => {
        const picaAssetMetadataList = await picassoAssetsList(_picaApi);
        const karuraAssetMetadataList = await karuraAssetsList(_karApi);
        updateTokens(
          picaAssetMetadataList,
          karuraAssetMetadataList
        )
      },
      parachainProviders.picasso.parachainApi,
      parachainProviders.karura.parachainApi,
    )
  }, [parachainProviders, updateTokens])

  const picassoBalanceSubscriber = useCallback(
    async (chain, tokenMetadata: TokenMetadata, chainId) => {
      callbackGate(
        async (chain, tokenMetadata, chainId, account) => {
          await subscribePicassoBalanceByAssetId(
            chain.parachainApi!,
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
        chain,
        tokenMetadata,
        chainId,
        chain.accounts[selectedAccount]
      );
    },
    [selectedAccount, updateBalance]
  );

  // Subscribe for native balance changes
  useEffect(() => {
    if (selectedAccount !== -1) {
      Object.entries({ ...parachainProviders, ...relaychainProviders }).forEach(
        ([chainId, chain]) => {
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
            ).catch((err) => {
              console.error(err);
            });
          }
        }
      );
    } else if (selectedAccount === -1) {
      clearBalance();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [parachainProviders, relaychainProviders, selectedAccount]);

  // Subscribe non-native token balances
  useEffect(() => {
    let unsubList: any[];
    unsubList = [];
    if (extensionStatus !== "connected" || selectedAccount === -1) {
      return () => {};
    }

    Object.entries(parachainProviders).forEach(([chainId, chain]) =>
      callbackGate((api) => {
        Object.values(tokens).forEach((asset) => {
          switch (chainId) {
            case "picasso":
              picassoBalanceSubscriber(chain, asset, chainId);
              break;
            case "karura":
              if (connectedAccounts.karura[selectedAccount]) {
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

        return function cleanUp() {
          unsubList.forEach((unsub) => {
            unsub.then((call: any) => call?.());
          });
        };
      }, chain.parachainApi)
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [extensionStatus, selectedAccount, parachainProviders]);

  return null;
};

export default PolkadotBalancesUpdater;
