import {
  ParachainApi,
  ParachainContext,
} from "@/defi/polkadot/context/ParachainContext";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { SubstrateNetwork, SubstrateNetworkId } from "@/defi/polkadot/types";
import { toTokenUnitsBN } from "@/utils/BN";

import { useContext, useEffect } from "react";

import { useStore } from "@/stores/root";

export async function updateBalances(
  account: string,
  chainApi: ParachainApi,
  chainId: string,
  updateBalance: (data: {
    substrateNetworkId: SubstrateNetworkId;
    balance: string;
  }) => void
) {
  const { parachainApi } = chainApi;
  if (!parachainApi) return;
  // create AccountId32 type byte array
  // and retrieve balances
  const accountId = parachainApi.createType("AccountId32", account);
  const queryResult = await parachainApi.query.system.account(accountId);
  const blObject: any = queryResult.toJSON();

  const {
    data: { free },
  } = blObject;

  const { decimals } = SUBSTRATE_NETWORKS[chainId as SubstrateNetworkId];
  const bnBalance = toTokenUnitsBN(free, decimals);

  updateBalance({
    substrateNetworkId: chainId as any,
    balance: bnBalance.toString(),
  });
}

const PolkadotBalancesUpdater = ({
  substrateChains,
}: {
  substrateChains: SubstrateNetwork[];
}) => {
  const { updateBalance, clearBalance } = useStore(
    ({ substrateBalances }) => substrateBalances
  );
  const { selectedAccount, parachainProviders } = useContext(ParachainContext);
  const picassoProvider = usePicassoProvider();

  useEffect(() => {
    if (selectedAccount !== -1 && picassoProvider.accounts.length) {
      Object.entries(parachainProviders).forEach(([id, chain]) => {
        if (picassoProvider.accounts[selectedAccount] && chain.parachainApi) {
          updateBalances(
            picassoProvider.accounts[selectedAccount].address,
            chain,
            id,
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
    substrateChains,
    picassoProvider.accounts.length,
    picassoProvider.accounts,
    parachainProviders,
    updateBalance,
    clearBalance,
  ]);

  return null;
};

export default PolkadotBalancesUpdater;
