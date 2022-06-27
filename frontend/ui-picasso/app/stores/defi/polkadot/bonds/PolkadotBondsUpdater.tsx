import { useEffect, VoidFunctionComponent } from "react";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { fetchBonds } from "@/defi/polkadot/pallets/BondedFinance";
import { useStore } from "@/stores/root";

export const Updater: VoidFunctionComponent = () => {
  const { parachainApi: api, accounts } = usePicassoProvider();
  const { setBonds, setBondOfferCount } = useStore((state) => state.bonds);

  const updateBonds = async () => {
    if (!api) return;
    const { bonds, bondOfferCount } = await fetchBonds(api);
    setBonds(bonds);
    setBondOfferCount(bondOfferCount);
  };

  useEffect(() => {
    updateBonds();
  }, [accounts]);

  return null;
};
