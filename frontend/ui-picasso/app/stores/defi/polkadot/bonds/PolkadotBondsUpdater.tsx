import { useCallback, useEffect, VoidFunctionComponent } from "react";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { fetchBonds } from "@/defi/polkadot/pallets/BondedFinance";
import { useStore } from "@/stores/root";

export const Updater: VoidFunctionComponent = () => {
  const { parachainApi: api, accounts } = usePicassoProvider();
  const { setBonds, setBondOfferCount } = useStore((state) => state.bonds);

  const updateBonds = useCallback(async () => {
    if (!api) return;
    const { bonds, bondOfferCount } = await fetchBonds(api);
    setBonds(bonds);
    setBondOfferCount(bondOfferCount);
  }, [setBonds, setBondOfferCount, api]);
  useEffect(() => {
    updateBonds();
  }, [accounts, updateBonds]);

  return null;
};
