import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { useEffect, useState } from "react";
import { fetchBalanceByAssetId } from "@/defi/polkadot/pallets/Balance";
import { BondOfferBalances } from "@/defi/polkadot/pallets/BondedFinance";

export function useBalanceForOffer(offer: BondOffer | undefined) {
  const { parachainApi } = usePicassoProvider();
  const account = useSelectedAccount();
  const [balances, setBalances] = useState<BondOfferBalances>({});

  useEffect(() => {
    if (account && parachainApi && offer) {
      fetchBalanceByAssetId(parachainApi, account.address, offer.assetId).then(
        (result) => {
          setBalances((amount) => ({
            ...amount,
            ...{ [offer.assetId]: result }
          }));
        }
      );
    }
  }, [parachainApi, account, offer]);

  return {
    balances,
    isLoading: Object.keys(balances).length === 0
  };
}
