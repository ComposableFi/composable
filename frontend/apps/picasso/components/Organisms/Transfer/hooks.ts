import { useStore } from "@/stores/root";
import { useEffect, useMemo } from "react";
import { getTransferToken } from "@/components/Organisms/Transfer/utils";
import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "@/defi/polkadot/pallets/BondedFinance";

export const useExistentialDeposit = () => {
  const tokenId = useStore((state) => state.transfers.tokenId);
  const from = useStore((state) => state.transfers.networks.from);
  const to = useStore((state) => state.transfers.networks.to);
  const allProviders = useAllParachainProviders();

  const { native, assets } = useStore(
    ({ substrateBalances }) => substrateBalances[from]
  );

  const nativeTo = useStore(
    ({ substrateBalances }) => substrateBalances[to].native
  );

  const { updateExistentialDeposit, existentialDeposit } = useStore(
    (state) => state.transfers
  );

  const isNativeToNetwork = useMemo(() => {
    const transferableTokenId = getTransferToken(from, to);
    return assets[transferableTokenId].meta.supportedNetwork[from] === 1;
  }, [assets, from, to]);

  const balance = isNativeToNetwork ? native.balance : assets[tokenId].balance;

  /**
   * Fetch existential deposit based on native asset, if transfer token is native,
   * then we will reach substrate balances to fetch this value, otherwise we can
   * fetch it based on assetED on currency factory, since we have the tokenId
   */
  useEffect(() => {
    if (isNativeToNetwork) {
      updateExistentialDeposit(nativeTo.existentialDeposit);
    } else {
      const api = allProviders[from]?.parachainApi;
      if (api) {
        api.query.currencyFactory
          .assetEd(assets[tokenId].meta.supportedNetwork[from])
          .then((ed) => {
            const existentialString = ed.toString();
            const existentialValue = fromChainIdUnit(
              new BigNumber(existentialString)
            );
            updateExistentialDeposit(
              existentialValue.isNaN() ? new BigNumber(0) : existentialValue
            );
          });
      }
    }
  }, [
    from,
    to,
    assets,
    tokenId,
    isNativeToNetwork,
    nativeTo,
    updateExistentialDeposit,
    allProviders,
  ]);

  return {
    balance,
    tokenId,
    isNativeToNetwork,
    from,
    to,
    assets,
    native,
    existentialDeposit,
  };
};
