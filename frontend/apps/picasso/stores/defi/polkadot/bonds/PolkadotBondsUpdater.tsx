import { useEffect, VoidFunctionComponent } from "react";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { subscribeBonds } from "@/defi/polkadot/pallets/BondedFinance";
import { useStore } from "@/stores/root";

export const Updater: VoidFunctionComponent = () => {
  const { parachainApi: api } = usePicassoProvider();
  const { setBonds, setBondOfferCount } = useStore((state) => state.bonds);

  useEffect(() => {
    if (!api) return;
    let unsubList: any[] = [];
    subscribeBonds(api, ([unsubs, bonds, bondOfferCount]) => {
      unsubList = unsubs;
      setBonds(bonds);
      setBondOfferCount(bondOfferCount);
    });
    return () => {
      unsubList.forEach((unsub: any) => unsub?.());
    };
  }, [api, setBondOfferCount, setBonds]);

  return null;
};
