import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { callbackGate, getExistentialDeposit, toTokenUnitsBN } from "shared";
import { useCallback, useEffect } from "react";
import { useStore } from "@/stores/root";
import { ApiPromise } from "@polkadot/api";
import {
  fetchKaruraBalanceByAssetId,
  subscribePicassoBalanceByAssetId
} from "@/defi/polkadot/pallets/Balance";
import BigNumber from "bignumber.js";
import { ParachainId, RelayChainId, useDotSamaContext, useEagerConnect } from "substrate-react";

export async function subscribeNativeBalance(
  account: string,
  api: ApiPromise | undefined,
  chainId: string,
  updateBalance: (data: {
    substrateNetworkId: SubstrateNetworkId;
    balance: string;
    existentialDeposit: BigNumber;
  }) => void
) {
  if (!api) return;
  // create AccountId32 type byte array
  // and retrieve balances
  const accountId = api.createType("AccountId32", account);
  await api.query.system.account(accountId, result => {
    const blObject: any = result.toJSON();

    const {
      data: { free }
    } = blObject;

    const { decimals } = SUBSTRATE_NETWORKS[chainId as SubstrateNetworkId];
    const bnBalance = toTokenUnitsBN(free, decimals);

    const existentialDeposit = getExistentialDeposit(api);

    updateBalance({
      substrateNetworkId: chainId as SubstrateNetworkId,
      balance: bnBalance.toString(),
      existentialDeposit
    });
  });
}

export async function updateBalances(
  account: string,
  api: ApiPromise | undefined,
  chainId: string,
  updateBalance: (data: {
    substrateNetworkId: SubstrateNetworkId;
    balance: string;
    existentialDeposit: BigNumber;
  }) => void
) {
  if (!api) return;
  // create AccountId32 type byte array
  // and retrieve balances
  const accountId = api.createType("AccountId32", account);
  const queryResult = await api.query.system.account(accountId);
  const blObject: any = queryResult.toJSON();

  const {
    data: { free }
  } = blObject;

  const { decimals } = SUBSTRATE_NETWORKS[chainId as SubstrateNetworkId];
  const bnBalance = toTokenUnitsBN(free, decimals);

  const existentialDeposit = getExistentialDeposit(api);

  updateBalance({
    substrateNetworkId: chainId as SubstrateNetworkId,
    balance: bnBalance.toString(),
    existentialDeposit
  });
}

const PolkadotBalancesUpdater = () => {
  useEagerConnect("picasso");
  useEagerConnect("karura");
  const updateBalance = useStore(
    ({ substrateBalances }) => substrateBalances.updateBalance
  );
  const assets = useStore(({ substrateBalances }) => substrateBalances.assets);

  const updateAssetBalance = useStore(
    ({ substrateBalances }) => substrateBalances.updateAssetBalance
  );
  const clearBalance = useStore(
    ({ substrateBalances }) => substrateBalances.clearBalance
  );

  const {
    extensionStatus,
    selectedAccount,
    parachainProviders,
    relaychainProviders,
    connectedAccounts
  } = useDotSamaContext();

  const picassoBalanceSubscriber = useCallback(
    async (chain, asset, chainId) => {
      callbackGate(
        async (chain, asset, chainId, account) => {
          await subscribePicassoBalanceByAssetId(
            chain.parachainApi!,
            account.address,
            String(asset.meta.supportedNetwork[chainId as SubstrateNetworkId]),
            balance => {
              updateAssetBalance({
                substrateNetworkId: chainId as SubstrateNetworkId,
                assetId: asset.meta.assetId,
                balance
              });
            }
          );
        },
        chain,
        asset,
        chainId,
        chain.accounts[selectedAccount]
      );
    },
    [selectedAccount, updateAssetBalance]
  );

  // Subscribe for native balance changes
  useEffect(() => {
    if (selectedAccount !== -1) {
      Object.entries({ ...parachainProviders, ...relaychainProviders }).forEach(
        ([chainId, chain]) => {
          if (connectedAccounts[chainId as ParachainId | RelayChainId] && chain.parachainApi) {
            subscribeNativeBalance(
              connectedAccounts[chainId as ParachainId | RelayChainId][selectedAccount].address,
              chain.parachainApi,
              chainId,
              updateBalance
            ).catch(err => {
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
    if (extensionStatus !== "connected" || selectedAccount === -1) {
      return () => {};
    }

    Object.entries(parachainProviders).forEach(([chainId, chain]) =>
      callbackGate(api => {
        Object.values(assets[chainId as SubstrateNetworkId].assets).forEach(
          asset => {
            if (!asset.meta.supportedNetwork[chainId as SubstrateNetworkId]) {
              return;
            }
            switch (chainId) {
              case "picasso":
                picassoBalanceSubscriber(chain, asset, chainId);
                break;
              case "karura":
                if (connectedAccounts[chainId][selectedAccount]) {
                  fetchKaruraBalanceByAssetId(
                    api,
                    connectedAccounts[chainId][selectedAccount].address,
                    String(asset.meta.symbol)
                  ).then(balance => {
                    updateAssetBalance({
                      substrateNetworkId: chainId as SubstrateNetworkId,
                      assetId: asset.meta.assetId,
                      balance
                    });
                  });
                }
                break;
              default:
                break;
            }
          }
        );
      }, chain.parachainApi)
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [extensionStatus, selectedAccount, parachainProviders]);

  return null;
};

export default PolkadotBalancesUpdater;
