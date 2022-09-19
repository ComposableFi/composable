import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { SubstrateNetwork, SubstrateNetworkId } from "@/defi/polkadot/types";
import { callbackGate, getExistentialDeposit, toTokenUnitsBN } from "shared";

import { useCallback, useEffect } from "react";

import { useStore } from "@/stores/root";
import { ApiPromise } from "@polkadot/api";
import { fetchKaruraBalanceByAssetId, subscribePicassoBalanceByAssetId } from "@/defi/polkadot/pallets/Balance";
import BigNumber from "bignumber.js";
import { useDotSamaContext, useEagerConnect } from "substrate-react";

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
  await api.query.system.account(accountId, (result) => {
    console.log("Processing balance update...");
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

const PolkadotBalancesUpdater = ({
  substrateNetworks
}: {
  substrateNetworks: SubstrateNetwork[];
}) => {
  useEagerConnect("picasso");
  const { updateBalance, clearBalance, updateAssetBalance, ...assets } =
    useStore(({ substrateBalances }) => substrateBalances);
  const { selectedAccount, parachainProviders, relaychainProviders } = useDotSamaContext();
  const picassoProvider = usePicassoProvider();

  // Subscribe for native balance changes
  useEffect(() => {
    if (selectedAccount !== -1 && picassoProvider.accounts.length) {
      Object.entries(parachainProviders).forEach(([chainId, chain]) => {
        if (picassoProvider.accounts[selectedAccount] && chain.parachainApi) {
          subscribeNativeBalance(
            picassoProvider.accounts[selectedAccount].address,
            chain.parachainApi,
            chainId,
            updateBalance
          ).catch((err) => {
            console.error(err);
          });
        }
      });
    } else if (selectedAccount === -1) {
      console.log("selectedAccount is not specified");
      clearBalance();
    } else {
      console.log("picassoProvider is not available");
    }
  }, [
    selectedAccount,
    substrateNetworks,
    picassoProvider.accounts.length,
    picassoProvider.accounts,
    parachainProviders,
    picassoProvider.parachainApi,
    updateBalance,
    clearBalance
  ]);

  const picassoBalanceSubscriber = useCallback(
    async (chain, asset, chainId) => {
      callbackGate(async (account) => {
        await subscribePicassoBalanceByAssetId(
          chain.parachainApi!,
          account.address,
          String(asset.meta.supportedNetwork[chainId as SubstrateNetworkId]),
          (balance) => {
            updateAssetBalance({
              substrateNetworkId: chainId as SubstrateNetworkId,
              assetId: asset.meta.assetId,
              balance
            });
          }
        );
      }, chain.accounts[selectedAccount]);
    },
    []
  );

  // Subscribe non-native token balances
  useEffect(() => {
    if (selectedAccount !== -1 && picassoProvider.accounts.length) {
      Object.entries(parachainProviders).forEach(([chainId, chain]) => {
        if (chain.parachainApi) {
          Object.values(assets[chainId as SubstrateNetworkId].assets).forEach(
            (asset) => {
              if (!asset.meta.supportedNetwork[chainId as SubstrateNetworkId]) {
                return;
              }
              switch (chainId) {
                case "picasso":
                  picassoBalanceSubscriber(chain, asset, chainId);
                  break;
                case "karura":
                  fetchKaruraBalanceByAssetId(
                    chain.parachainApi!,
                    chain.accounts[selectedAccount].address,
                    String(asset.meta.symbol)
                  ).then((balance) => {
                    updateAssetBalance({
                      substrateNetworkId: chainId as SubstrateNetworkId,
                      assetId: asset.meta.assetId,
                      balance
                    });
                  });
                default:
                  break;
              }
            }
          );
        }
      });
    }
  }, [selectedAccount, picassoProvider, parachainProviders]);

  return null;
};

export default PolkadotBalancesUpdater;
