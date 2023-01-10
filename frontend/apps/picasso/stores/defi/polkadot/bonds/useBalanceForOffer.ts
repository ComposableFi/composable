import BigNumber from "bignumber.js";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { usePicassoAccount } from "@/defi/polkadot/hooks";
import { useEffect, useState } from "react";
import { fetchBalanceByAssetId } from "@/defi/polkadot/pallets/Balances";
import { usePicassoProvider } from "substrate-react";

type BondOfferBalances = {
  [key: string]: BigNumber;
};

export function useBalanceForOffer(offer: BondOffer) {
  const { parachainApi } = usePicassoProvider();
  const account = usePicassoAccount();
  const [balances, setBalances] = useState<BondOfferBalances>({});

  useEffect(() => {
    if (account && parachainApi && offer) {
      fetchBalanceByAssetId(parachainApi, account.address, offer.assetId).then(
        (result) => {
          setBalances((amount) => ({
            ...amount,
            ...{ [offer.assetId]: result },
          }));
        }
      );
    }
  }, [parachainApi, account, offer]);

  return {
    balances,
    isLoading: Object.keys(balances).length === 0,
  };
}
