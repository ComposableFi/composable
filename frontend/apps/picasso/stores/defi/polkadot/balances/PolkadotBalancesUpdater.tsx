import { ParachainContext } from "@/defi/polkadot/context/ParachainContext";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { SubstrateNetwork, SubstrateNetworkId } from "@/defi/polkadot/types";
import { getExistentialDeposit, toTokenUnitsBN } from "shared";

import { useCallback, useContext, useEffect } from "react";

import { useStore } from "@/stores/root";
import { ApiPromise } from "@polkadot/api";
import { fetchKaruraBalanceByAssetId, subscribePicassoBalanceByAssetId } from "@/defi/polkadot/pallets/Balance";
import BigNumber from "bignumber.js";

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
  const { updateBalance, clearBalance, updateAssetBalance, ...assets } =
    useStore(({ substrateBalances }) => substrateBalances);
  const { selectedAccount, parachainProviders } = useContext(ParachainContext);
  const picassoProvider = usePicassoProvider();

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
      clearBalance();
    }
  }, [
    selectedAccount,
    substrateNetworks,
    picassoProvider.accounts.length,
    picassoProvider.accounts,
    parachainProviders,
    updateBalance,
    clearBalance
  ]);

  const picassoBalanceSubscriber = useCallback(
    async (chain, asset, chainId) => {
      if (chain.accounts[selectedAccount]) {
        await subscribePicassoBalanceByAssetId(
          chain.parachainApi!,
          chain.accounts[selectedAccount].address,
          String(asset.meta.supportedNetwork[chainId as SubstrateNetworkId]),
          (balance) => {
            updateAssetBalance({
              substrateNetworkId: chainId as SubstrateNetworkId,
              assetId: asset.meta.assetId,
              balance
            });
          }
        );
      }
    },
    []
  );

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
  }, [selectedAccount]);

  return null;
};

export default PolkadotBalancesUpdater;
